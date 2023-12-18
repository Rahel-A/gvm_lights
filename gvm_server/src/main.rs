use dotenv::dotenv;
use futures_util::Future;
use tokio::signal;

#[cfg(not(feature = "mqtt"))]
use gvm_server::custom_run;
#[cfg(feature = "mqtt")]
use gvm_server::mqtt_run;
use tokio::sync::oneshot;

use gvm_server::GvmServerResult;

#[cfg(not(feature = "mqtt"))]
async fn run_server(address: String, nodes: Option<String>) -> GvmServerResult<()> {
    custom_run(address, nodes, signal::ctrl_c()).await
}

#[cfg(feature = "mqtt")]
async fn run_server(address: String, nodes: Option<String>) -> GvmServerResult<()> {
    use log::trace;

    let node_id = dotenv::var("MQTT_NODE_ID").ok();
    let mut user = None;
    if let Ok(username) = dotenv::var("MQTT_USER") {
        if let Ok(password) = dotenv::var("MQTT_PASSWORD") {
            user = Some((username, password));
        }
    };
    let (tx, mut rx) = oneshot::channel::<()>();
    tokio::spawn(async move {
        trace!("Waiting for ctrl_c");
        let _ = signal::ctrl_c().await;
        trace!("ctrl_c received");
        rx.close()
    });

    mqtt_run(node_id, address, user, nodes, tx).await
}

#[tokio::main]
async fn main() -> GvmServerResult<()> {
    let address = format!(
        "{}:{}",
        dotenv::var("APP_HOST").unwrap_or("0.0.0.0".to_string()),
        dotenv::var("APP_PORT").unwrap_or("1883".to_string())
    );

    pretty_env_logger::init();
    dotenv().ok();

    // If `APP_CLIENTS` was not provided, will search for gvm nodes
    match dotenv::var("APP_CLIENTS") {
        Ok(gvm_nodes) => run_server(address, Some(gvm_nodes)).await,
        Err(_) => run_server(address, None).await,
    }
}
