use hyper::Body;
use std::task::{Context, Poll};
use tonic::body::BoxBody;
use tower::layer::Layer;
use tower::Service;
#[derive(Debug, Clone)]
pub struct TonicLogger<S> {
    inner: S,
}

impl<S> Service<hyper::Request<Body>> for TonicLogger<S>
where
    S: Service<hyper::Request<Body>, Response = hyper::Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: hyper::Request<Body>) -> Self::Future {
        // This is necessary because tonic internally uses `tower::buffer::Buffer`.
        // See https://github.com/tower-rs/tower/issues/547#issuecomment-767629149
        // for details on why this is necessary
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            // Do extra async work here...
            let response = inner.call(req).await?;
            let status = response
                .headers()
                .get("grpc-status")
                .map(|s| s.to_str().unwrap_or_default())
                .unwrap_or_default();

            if status.is_empty() || status != "0" {
                let message = response
                    .headers()
                    .get("grpc-message")
                    .map(|s| s.to_str().unwrap_or_default())
                    .unwrap_or_default();
                let message = urlencoding::decode(message).unwrap_or(message.into());
                log::error!("response failed: {} {}", status, message);
            }
            Ok(response)
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct TonicLoggerLayer;

impl<S> Layer<S> for TonicLoggerLayer {
    type Service = TonicLogger<S>;

    fn layer(&self, service: S) -> Self::Service {
        TonicLogger { inner: service }
    }
}
