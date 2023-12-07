use dotenv::dotenv;
use futures_util::Future;
use tokio::signal;

#[cfg(not(feature = "mqtt"))]
use gvm_server::run;
#[cfg(feature = "mqtt")]
use gvm_server::mqtt_run;

use gvm_server::GvmServerResult;

#[cfg(not(feature = "mqtt"))]
async fn run_server(
    address: String,
    nodes: Option<String>,
    shutdown: impl Future,
) -> GvmServerResult<()> {
    custom_run(address, nodes, shutdown).await
}

#[tokio::main]
async fn main() -> GvmServerResult<()> {
    let address = format!(
        "{}:{}",
        dotenv::var("APP_HOST").unwrap_or("0.0.0.0".to_string()),
        dotenv::var("APP_PORT").unwrap_or("8631".to_string())
    );

    pretty_env_logger::init();
    dotenv().ok();

    // If `APP_CLIENTS` was not provided, will search for gvm nodes
    match dotenv::var("APP_CLIENTS") {
        Ok(gvm_nodes) => run_server(address, Some(gvm_nodes), signal::ctrl_c()).await,
        Err(_) => run_server(address, None, signal::ctrl_c()).await,
    }
}
