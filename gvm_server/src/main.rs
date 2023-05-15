use dotenv::dotenv;
use gvm_server::GvmServerResult;
use tokio::signal;

#[tokio::main]
async fn main() -> GvmServerResult<()> {
    let address = format!(
        "{}:{}",
        dotenv::var("APP_HOST").unwrap_or("0.0.0.0".to_string()),
        dotenv::var("APP_PORT").unwrap_or("8631".to_string())
    );

    pretty_env_logger::init();
    dotenv().ok();

    match dotenv::var("APP_CLIENTS") {
        Ok(gvm_nodes) => gvm_server::run(address, Some(gvm_nodes), signal::ctrl_c()).await,
        Err(_) => gvm_server::run(address, None, signal::ctrl_c()).await,
    }
}
