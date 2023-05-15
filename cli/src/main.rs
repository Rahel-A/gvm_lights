use dotenv::dotenv;
use gvm_cli::Client;
use gvm_server::gvm_server_event::GvmServerEvent;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = format!(
        "{}:{}",
        dotenv::var("APP_HOST").unwrap_or("0.0.0.0".to_string()),
        dotenv::var("APP_PORT").unwrap_or("8631".to_string())
    );

    pretty_env_logger::init();
    dotenv().ok();

    let matches = gvm_cli::cli().get_matches();

    let target = matches.get_one::<u8>("client");
    let arg = gvm_cli::find_command(&matches);
    info!("Parsed arguments into: {:?}", arg);

    let id = *target.unwrap();
    let clients = Client::new(address, id).await?;
    for mut target in clients.into_iter().filter(|c| id == 255 || c.uid == id) {
        let states = if matches.contains_id("state") {
            target
                .send(GvmServerEvent::GetNodeStatus(target.uid), true)
                .await
        } else {
            let cmd = arg.expect("No command detected in arguments");
            target
                .send(GvmServerEvent::NodeCommand(target.uid, cmd), false)
                .await
        };

        if let Ok(states) = states {
            println!("Received message: {states:#?}");
        };
    }

    Ok(())
}
