use clap::{Arg, Command, PossibleValue};
use gvm_lights::{GvmBleClient, ServerMessage, ControlMessage, LightCmd, ModeCmd};
use gvm_lights::encode;
use log::info;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use dotenv::dotenv;

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
                  .takes_value(false))
        .arg(Arg::new(state)
                  .long(state)
                  .short('i')
                  .takes_value(false))
        .arg(Arg::new(client)
                  .long(client)
                  .default_value("255")
                  .validator(|s: &str| if s == "all" { Ok(255) } else { s.parse::<u8>() })
                  .short('c')
                  .takes_value(true))
        .get_matches();

    if matches.is_present(server) {
        println!("Opening server on interface: {}", address);
        let listener = TcpListener::bind(address).await?;

        let mut clients: Vec<GvmBleClient> = Vec::new();
        match dotenv::var("clients") {
            Ok(_clients) => {
                let mut counter = 1;
                for bt_address in _clients.split(',').collect::<Vec<_>>().into_iter() {
                    clients.push(GvmBleClient::new(counter, bt_address).await?);
                    counter = counter + 1;
                };
            },
            _ => panic!("Can't initialise server without target GVM lights")
        };


        loop {
            let (mut socket, _) = listener.accept().await?;
            socket.readable().await?;

            let mut buffer = [0; 50];
            let n = socket.try_read(&mut buffer)?;
            info!("Received message from client! {:?}", &buffer[..n]);

            match serde_json::from_slice(&buffer[..n]) {
                Ok(ServerMessage{client, msg}) => {
                    let filtered_clients: Vec<_> = clients
                        .clone()
                        .into_iter()
                        // select either a target client or ALL clients
                        .filter(|gvm_client| gvm_client.id() == client || client == 255)
                        .collect();
                    for gvm_client in filtered_clients {
                        // client can send multiple commands (actions)
                        for action in msg.iter() {
                            // get list of states of the gvm client to send back
                            let states = match action {
                                ControlMessage::ReadState() =>
                                    gvm_client.get_state()
                                        .await
                                        .unwrap(),
                                _ => {
                                    gvm_client.send_to(&encode(&action).unwrap())
                                        .await
                                        .expect("Failed to send message to GVM Client");
                                    vec![]
                                },
                            };
                            // if there is a message to send back, do it here...
                            if !states.is_empty() {
                                let cmd_json = serde_json::to_string(&states
                                    .into_iter()
                                    .map(|state| ServerMessage{client:gvm_client.id(), msg:vec![state]})
                                    .collect::<Vec<ServerMessage>>())?;
                                socket.write(cmd_json.as_bytes()).await?;
                                socket.write("\n".as_bytes()).await?;
                                socket.flush().await?;
                            };
                        }
                    }
                },
                _ => eprintln!("Received unexpected message from client"),
            }
        }
    }
    else {
        let target = matches.value_of(client).unwrap().parse();
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
            } else if matches.is_present(state) {
                ControlMessage::ReadState()
            } else {
                panic!("Not recognized command");
            };
        let mut stream = TcpStream::connect(address).await?;
        let cmd_json = serde_json::to_string(&ServerMessage{client:target.unwrap(), msg:vec!(cmd)})?;
        stream.write_all(cmd_json.as_bytes()).await?;

        info!("Sending message to server! {:?}", &cmd_json);
        if let ControlMessage::ReadState() = cmd {
            loop {
                let mut buffer = [0; 500];
                let n = stream.read(&mut buffer).await?;
                if let Ok(msgs) = serde_json::from_slice::<Vec<ServerMessage>>(&buffer[..n]) {
                    for ServerMessage{client:_, msg} in msgs {
                        info!("Received message from server! {:?}", msg);
                    };
                }
                else {
                     panic!("Unknown message from server: {:?}", String::from_utf8(buffer[..n].to_vec()));
                };
                break;
            }
        }
    };

    Ok(())
}
