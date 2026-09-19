use tower::ServiceBuilder;
use vercel_runtime::axum::VercelLayer;
use vercel_runtime::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let state = rchat_server::vercel_app_state()
        .await
        .map_err(|error| error.to_string())?;
    let router = rchat_server::app(state);
    let app = ServiceBuilder::new()
        .layer(VercelLayer::new())
        .service(router);
    vercel_runtime::run(app).await
}
