use hyper::Body;
use std::{
    task::{Context, Poll},
};
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
            if !response.status().is_success() {
                log::error!("Response status: {:?}", response);
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
