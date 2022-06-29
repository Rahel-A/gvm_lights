use log::info;
use dotenv::dotenv;
use gvm_cli::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("{}:{}",
        dotenv::var("APP_HOST").unwrap_or("0.0.0.0".to_string()),
        dotenv::var("APP_PORT").unwrap_or("8631".to_string()));

    pretty_env_logger::init();
    dotenv().ok();

    let matches = gvm_cli::cli().get_matches();

    let target = matches.get_one::<u8>("client");
    let cmd = gvm_cli::find_command(&matches).expect("No command detected in arguments");
    info!("Parsed arguments into: {:?}", cmd);

    let id = *target.unwrap();
    let clients = Client::new(address, id).await?;
    for mut target in clients.into_iter().filter(|c| id == 255 || c.uid == id) {
        if let Some(states) = target.send_message(vec!(cmd)).await? {
            println!("Received message: {states:?}");
        }
    }

    Ok(())
}
