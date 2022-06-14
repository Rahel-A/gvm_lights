use clap::{Arg, Command, PossibleValue};
use gvm_lights::{GvmBleClient, ControlMessage, LightCmd, ModeCmd};
use gvm_lights::encode;
use std::str::FromStr;
use std::time::Duration;
use log::info;
use tokio::time;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt};
use dotenv::dotenv;

#[cfg(not(debug_assertions))]
async fn get_current_state(clients: Vec<GvmBleClient>) -> Vec<GvmBleClient> {
    clients
}
#[cfg(debug_assertions)]
async fn get_current_state(clients: Vec<GvmBleClient>) -> Vec<GvmBleClient> {
    let tasks: Vec<_> = clients
        .into_iter()
        .map(|client| {
            tokio::spawn(async {
                client.get_state().await.expect("Failed to read current state");
                client
            })
        })
        .collect();
    // await the tasks for resolve's to complete and give back our items
    let mut clients = vec![];
    for task in tasks {
        clients.push(task.await.unwrap());
    };
    clients
}

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
    let validator_u8 = |s:&str| s.parse::<u8>();
    let validator_u16 = |s:&str| s.parse::<u16>();

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
                  .possible_values(&["on", "off"]))
        .arg(Arg::new(brightness)
                  .long(brightness)
                  .short('b')
                  .validator(validator_u8)
                  .takes_value(true))
        .arg(Arg::new(temperature)
                  .long(temperature)
                  .short('t')
                  .validator(validator_u16)
                  .takes_value(true))
        .arg(Arg::new(hue)
                  .long(hue)
                  .short('h')
                  .validator(validator_u16)
                  .takes_value(true))
        .arg(Arg::new(saturation)
                  .long(saturation)
                  .short('s')
                  .validator(validator_u8)
                  .takes_value(true))
        .arg(Arg::new(mode)
                  .long(mode)
                  .short('m')
                  .takes_value(true)
                  .possible_values([
                      PossibleValue::new("CT"),
                      PossibleValue::new("HS"),
                      PossibleValue::new("Sc")]))
        .arg(Arg::new(scene)
                  .long(scene)
                  .short('z')
                  .validator(validator_u8)
                  .takes_value(true))
        .arg(Arg::new(rgb)
                  .long(rgb)
                  .short('r')
                  .validator(validator_u8)
                  .takes_value(true))
        .arg(Arg::new(server)
                  .long(server)
                  .short('x')
                  .exclusive(true)
                  .takes_value(false))
        .get_matches();

    if matches.is_present(server) {
        println!("Opening server on interface: {}", address);
        let listener = TcpListener::bind(address).await?;

        let mut clients: Vec<GvmBleClient> = Vec::new();
        match dotenv::var("clients") {
            Ok(_clients) =>
                for bt_address in _clients.split(',').collect::<Vec<_>>().into_iter() {
                    clients.push(GvmBleClient::new(bt_address).await?)
                },
            _ => panic!("Can't initialise server without target GVM lights")
        };

        clients = get_current_state(clients).await;

        loop {
            let (socket, _) = listener.accept().await?;
            socket.readable().await?;

            let mut buffer = [0; 50];
            let n = socket.try_read(&mut buffer)?;
            info!("Received message from client! {:?}", &buffer[..n]);

            let cmd = encode(&serde_json::from_slice(&buffer[..n])?)?;
            clients[0].send_to(&cmd).await?;
            time::sleep(Duration::from_millis(30)).await;
            clients[1].send_to(&cmd).await?;
        }
    }
    else {
        let cmd =
            if matches.is_present(light) {
                match matches.value_of(light) {
                    Some("on") => ControlMessage::Light(LightCmd::On),
                    Some("off") => ControlMessage::Light(LightCmd::Off),
                    _  => panic!("Incorrect argument passed")
                }
            } else if let Some(br) = matches.value_of(brightness) {
                ControlMessage::Brightness(br.parse().unwrap())
            } else if let Some(t) = matches.value_of(temperature) {
                ControlMessage::Temperature(t.parse().unwrap())
            } else if let Some(hue) = matches.value_of(hue) {
                ControlMessage::Hue(hue.parse().unwrap())
            } else if let Some(sat) = matches.value_of(saturation) {
                ControlMessage::Saturation(sat.parse().unwrap())
            } else if matches.is_present(mode) {
                println!("mode={}", mode);
                match matches.value_of(mode) {
                    Some("CT") => ControlMessage::Mode(ModeCmd::ColourTemp),
                    Some("HS") => ControlMessage::Mode(ModeCmd::HueSat),
                    Some("Sc") => ControlMessage::Mode(ModeCmd::Scenes),
                    _ => panic!("Incorrect argument passed")
                }
            } else if let Some(scene) = matches.value_of(scene) {
                ControlMessage::Scene(scene.parse().unwrap())
            } else if let Some(rgb) = matches.value_of(rgb) {
                ControlMessage::RGB(rgb.parse().unwrap())
            } else {
                panic!("Not recognized command");
            };
        let mut stream = TcpStream::connect(address).await?;
        let cmd_json = serde_json::to_string(&cmd)?;
        stream.write_all(cmd_json.as_bytes()).await?;
        info!("Sending message to server! {:?}", &cmd_json);
    };

    Ok(())
}
