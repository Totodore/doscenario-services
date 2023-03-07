use crate::database::load_mysql_pool;
use docs::docs_server::DocsServer;
use docs_service::DocsService;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use lazy_static::lazy_static;
use log::info;
use logging_timer::time;
use serde::{Deserialize, Serialize};
use tonic::{transport::Server, Request, Status};
use tower_http::cors::{self, CorsLayer};
use doscenario_utils::tonic_logger::TonicLoggerLayer;

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
#[derive(Debug, Clone)]
pub struct UserId(String);

#[time("debug")]
fn check_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
    lazy_static! {
        static ref PRIVATE_KEY: DecodingKey = DecodingKey::from_secret(
            std::env::var("PRIVATE_KEY")
                .expect("PRIVATE_KEY not set")
                .as_bytes()
        );
    }
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
                |c| {
                    req.extensions_mut().insert(UserId(c.claims.sub));
                    Ok(req)
                },
            );
    res
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // Set default env logger to info
    std::env::set_var("RUST_LOG", "info");
    env_logger::builder().init();

    let addr = "0.0.0.0:9090".parse().unwrap();
    let docs_service = DocsServer::with_interceptor(DocsService::new(), check_auth);

    load_mysql_pool().await;

    info!("Listening on {:#?}", addr);
    Server::builder()
		.layer(TonicLoggerLayer)
        .add_service(docs_service)
        .serve(addr)
        .await
        .unwrap();
    Ok(())
}
