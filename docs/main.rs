use crate::database::load_mysql_pool;
use docs::docs_server::DocsServer;
use docs_service::DocsService;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use lazy_static::lazy_static;
use log::info;
use serde::{Deserialize, Serialize};
use tonic::{transport::Server, Request, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{self, CorsLayer};
use logging_timer::time;

pub mod database;
pub mod docs_cache;
pub mod docs_mapper;
pub mod docs_service;
pub mod macros;
pub mod queries;
pub mod utils;

pub mod docs {
    tonic::include_proto!("docs");
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
}

#[time("info")]
fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    lazy_static! {
        static ref PRIVATE_KEY: DecodingKey = DecodingKey::from_secret(
            std::env::var("PRIVATE_KEY")
                .expect("PRIVATE_KEY not set")
                .as_bytes()
        );
    }
    log::info!("Checking auth");
    let token = String::from_utf8(
        req.metadata()
            .get("authorization")
            .ok_or(Status::unauthenticated("Missing auth token"))?
            .as_bytes()
            .to_vec(),
    )
    .map_err(|_| Status::unauthenticated("Invalid auth token"))?;
    log::debug!("{:?}", token);
    let res =
        jsonwebtoken::decode::<Claims>(&token, &PRIVATE_KEY, &Validation::new(Algorithm::HS256))
            .map_or_else(
                |e| {
                    Err(Status::unauthenticated(format!(
                        "Invalid auth token: {}",
                        e
                    )))
                },
                |_| Ok(req),
            );
    res
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // Set default env logger to info
    std::env::set_var("RUST_LOG", "info");
    env_logger::builder().init();

	let addr = "0.0.0.0:8080".parse().unwrap();
    let cors = CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_headers(cors::Any);
    let docs_service = DocsServer::with_interceptor(DocsService::new(), check_auth);

    load_mysql_pool().await;

    info!("Listening on {:#?}", addr);
    Server::builder()
		.accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(docs_service)
        .serve(addr)
        .await
        .unwrap();
    Ok(())
}
