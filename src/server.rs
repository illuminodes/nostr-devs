use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), HatError> {
    start_logger();
    if let Err(error) = server().await {
        error!("Sever Errored Out: {}", error);
    }
    Ok(())
}
async fn server() -> Result<(), HatError> {
    info!("Loading server routes...");
    let static_files = static_files_router()?;
    let router = pages_router(static_files);
    let listener = tcp_listener().await?;
    info!("Server started at: http://{}", hat::TCP_ADDRESS);
    axum::serve(listener, router).await?;
    Ok(())
}
fn pages_router(static_files: axum::Router) -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(hat::homepage))
        .route("/event/:route", axum::routing::get(hat::event_page))
        .nest("/", static_files)
}
fn static_files_router() -> Result<axum::Router, HatError> {
    use tower_http::services::ServeDir;
    let src_path_buf = std::env::current_dir()?.join("public");
    let src_path = src_path_buf.to_str().ok_or(HatError::PathNotFound)?;

    Ok(axum::Router::new()
        .nest_service("/styles", ServeDir::new(format!("{}/styles", src_path)))
        .nest_service("/js", ServeDir::new(format!("{}/js", src_path)))
        .nest_service("/assets", ServeDir::new(format!("{}/assets", src_path))))
}
fn start_logger() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::filter::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();
}
async fn tcp_listener() -> Result<tokio::net::TcpListener, HatError> {
    Ok(tokio::net::TcpListener::bind(hat::TCP_ADDRESS).await?)
}


#[derive(Debug)]
pub enum HatError {
    PathNotFound,
    IoError(std::io::Error),
}
impl std::fmt::Display for HatError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HatError::IoError(error) => write!(f, "IO error: {}", error),
            HatError::PathNotFound => write!(f, "Path not found"),
        }
    }
}
impl From<std::io::Error> for HatError {
    fn from(error: std::io::Error) -> Self {
        HatError::IoError(error)
    }
}
impl Into<std::io::Error> for HatError {
    fn into(self) -> std::io::Error {
        match self {
            HatError::IoError(error) => error,
            _ => std::io::Error::new(std::io::ErrorKind::Other, self.to_string()),
        }
    }
}
