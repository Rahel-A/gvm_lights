use dotenv::dotenv;
use gvm_server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("{}:{}",
        dotenv::var("APP_HOST").unwrap_or("0.0.0.0".to_string()),
        dotenv::var("APP_PORT").unwrap_or("8631".to_string()));

    pretty_env_logger::init();
    dotenv().ok();

    match dotenv::var("APP_CLIENTS") {
        Ok(gvm_clients) => {
            let mut server = Server::new(address, gvm_clients).await?;
            server.run().await?;
        }
        _ => panic!("Can't initialise server without target GVM lights")
    };
    Ok(())
}
