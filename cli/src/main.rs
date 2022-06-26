use log::info;
use dotenv::dotenv;
use gvm_server::Server;
use gvm_cli::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("{}:{}",
        dotenv::var("APP_HOST").unwrap_or("0.0.0.0".to_string()),
        dotenv::var("APP_PORT").unwrap_or("8631".to_string()));

    pretty_env_logger::init();
    dotenv().ok();

    let matches = gvm_cli::cli().get_matches();

    if matches.is_present("server") {
        match dotenv::var("clients") {
            Ok(gvm_clients) => {
                let mut server = Server::new(address, gvm_clients).await?;
                server.run().await?;
            }
            _ => panic!("Can't initialise server without target GVM lights")
        };
    }
    else {
        let target = matches.get_one::<u8>("client");
        let cmd = gvm_cli::find_command(&matches).expect("No command detected in arguments");
        info!("Parsed arguments into: {:?}", cmd);

        let mut client = Client::new(address, *target.unwrap()).await?;
        if let Some(states) = client.send_message(vec!(cmd)).await? {
            println!("Received message: {states:?}");
        }
    };

    Ok(())
}
