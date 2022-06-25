use clap::{Arg, Command, PossibleValue};
use gvm_lights::{ControlMessage, LightCmd, ModeCmd};
use log::info;
use dotenv::dotenv;
use gvm_server::Server;
use gvm_cli::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // argument strings
    let brightness = "brightness";
    let temperature = "temperature";
    let hue = "hue";
    let saturation = "saturation";
    let server = "server";
    let light = "light";
    let scene = "scene";
    let mode = "mode";
    let rgb = "rgb";
    let client = "client";
    let state = "state";
    let validator_u8 = clap::value_parser!(u8);
    let validator_u16 = clap::value_parser!(u16);

    let address = format!("{}:{}",
        dotenv::var("APP_HOST").unwrap_or("0.0.0.0".to_string()),
        dotenv::var("APP_PORT").unwrap_or("8631".to_string()));

    pretty_env_logger::init();
    dotenv().ok();

    let matches = Command::new("GVM Lights")
        .version("0.1.0")
        .arg(Arg::new(light)
                  .long(light)
                  .short('l')
                  .takes_value(true)
                  .value_parser([
                      PossibleValue::new("on"),
                      PossibleValue::new("off")]))
        .arg(Arg::new(brightness)
                  .long(brightness)
                  .short('b')
                  .value_parser(validator_u8)
                  .takes_value(true))
        .arg(Arg::new(temperature)
                  .long(temperature)
                  .short('t')
                  .value_parser(validator_u16)
                  .takes_value(true))
        .arg(Arg::new(hue)
                  .long(hue)
                  .short('h')
                  .value_parser(validator_u16)
                  .takes_value(true))
        .arg(Arg::new(saturation)
                  .long(saturation)
                  .short('s')
                  .value_parser(validator_u8)
                  .takes_value(true))
        .arg(Arg::new(mode)
                  .long(mode)
                  .short('m')
                  .takes_value(true)
                  .value_parser([
                      PossibleValue::new("CT"),
                      PossibleValue::new("HS"),
                      PossibleValue::new("Sc")]))
        .arg(Arg::new(scene)
                  .long(scene)
                  .short('z')
                  .value_parser(validator_u8)
                  .takes_value(true))
        .arg(Arg::new(rgb)
                  .long(rgb)
                  .short('r')
                  .value_parser(validator_u8)
                  .takes_value(true))
        .arg(Arg::new(server)
                  .long(server)
                  .short('x')
                  .takes_value(false))
        .arg(Arg::new(state)
                  .long(state)
                  .short('i')
                  .takes_value(false))
        .arg(Arg::new(client)
                  .long(client)
                  .default_value("255")
                  .value_parser(validator_u8)
                  .short('c')
                  .takes_value(true))
        .get_matches();

    if matches.is_present(server) {
        match dotenv::var("clients") {
            Ok(gvm_clients) => {
                let mut server = Server::new(address, gvm_clients).await?;
                server.run().await?;
            }
            _ => panic!("Can't initialise server without target GVM lights")
        };
    }
    else {
        let target = matches.get_one::<u8>(client);
        let cmd =
            if let Some(s) = matches.get_one::<String>(light) {
                match s.as_str() {
                    "on" => ControlMessage::Light(LightCmd::On),
                    "off" => ControlMessage::Light(LightCmd::Off),
                    _ => panic!("Incorrect argument passed")
                }
            } else if let Some(s) = matches.get_one::<String>(mode) {
                match s.as_str() {
                    "CT" => ControlMessage::Mode(ModeCmd::ColourTemp),
                    "HS" => ControlMessage::Mode(ModeCmd::HueSat),
                    "Sc" => ControlMessage::Mode(ModeCmd::Scenes),
                    _ => panic!("Incorrect argument passed")
                }
            } else if let Some(br) = matches.get_one::<u8>(brightness) {
                ControlMessage::Brightness(*br)
            } else if let Some(t) = matches.get_one::<u16>(temperature) {
                ControlMessage::Temperature(*t)
            } else if let Some(hue) = matches.get_one::<u16>(hue) {
                ControlMessage::Hue(*hue)
            } else if let Some(sat) = matches.get_one::<u8>(saturation) {
                ControlMessage::Saturation(*sat)
            } else if let Some(scene) = matches.get_one::<u8>(scene) {
                ControlMessage::Scene(*scene)
            } else if let Some(rgb) = matches.get_one::<u8>(rgb) {
                ControlMessage::RGB(*rgb)
            } else if matches.contains_id(state) {
                ControlMessage::ReadState()
            } else {
                panic!("Not recognized command");
            };
        info!("Parsed arguments into: {:?}", cmd);

        let mut client = Client::new(address, *target.unwrap()).await?;
        client.send_message(vec!(cmd)).await?;
        if ControlMessage::ReadState() == cmd {
            let msg = client.receive_message().await?;
            info!("Received message from server: {:?}", msg);
        }
    };

    Ok(())
}
