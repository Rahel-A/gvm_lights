use clap::{Arg, Command, PossibleValue};
use gvm_lights::{GvmBleClient, ControlMessage, LightCmd, ModeCmd};
use gvm_lights::encode;
use std::str::FromStr;
use std::time::Duration;
use log::info;
use tokio::time;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let brightness = "brightness";
    let temperature = "temperature";
    let hue = "hue";
    let saturation = "saturation";
    let server = "server";
    let light = "light";
    let scene = "scene";
    let mode = "mode";
    let rgb = "rgb";
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
                  .takes_value(true))
        .arg(Arg::new(temperature)
                  .long(temperature)
                  .short('t')
                  .takes_value(true))
        .arg(Arg::new(hue)
                  .long(hue)
                  .short('h')
                  .takes_value(true))
        .arg(Arg::new(saturation)
                  .long(saturation)
                  .short('s')
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
                  .takes_value(true))
        .arg(Arg::new(rgb)
                  .long(rgb)
                  .short('r')
                  .takes_value(true))
        .arg(Arg::new(server)
                  .long(server)
                  .short('x')
                  .takes_value(false))
        .get_matches();

    if matches.is_present(server) {
        let listener = TcpListener::bind("127.0.0.1:8631").await?;

        let client1 = GvmBleClient::new("A4:C1:38:EE:86:C1").await?;
        let client2 = GvmBleClient::new("A4:C1:38:8D:61:45").await?;
        client1.get_state().await?;
        client2.get_state().await?;

        loop {
            let (socket, _) = listener.accept().await?;
            socket.readable().await?;

            let mut buffer = [0; 50];
            let n = socket.try_read(&mut buffer)?;
            info!("Received message from client! {:?}", &buffer[..n]);

            client1.send_to(&buffer[..n]).await?;
            time::sleep(Duration::from_millis(30)).await;
            client2.send_to(&buffer[..n]).await?;
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
            } else if matches.is_present(brightness) {
                let br = u8::from_str(matches.value_of(brightness).ok_or("No value for brightness")?)?;
                println!("br={}", br);
                ControlMessage::Brightness(br)
            } else if matches.is_present(temperature) {
                let t = u16::from_str(matches.value_of(temperature).ok_or("No value for temperature")?)?;
                println!("temp={}", t);
                ControlMessage::Temperature(t)
            } else if matches.is_present(hue) {
                let hue = u16::from_str(matches.value_of(hue).ok_or("No value for hue")?)?;
                println!("hue={}", hue);
                ControlMessage::Hue(hue)
            } else if matches.is_present(saturation) {
                let sat = u8::from_str(matches.value_of(saturation).ok_or("No value for sat")?)?;
                println!("sat={}", sat);
                ControlMessage::Saturation(sat)
            } else if matches.is_present(mode) {
                println!("mode={}", mode);
                match matches.value_of(mode) {
                    Some("CT") => ControlMessage::Mode(ModeCmd::ColourTemp),
                    Some("HS") => ControlMessage::Mode(ModeCmd::HueSat),
                    Some("Sc") => ControlMessage::Mode(ModeCmd::Scenes),
                    _ => panic!("Incorrect argument passed")
                }
            } else if matches.is_present(scene) {
                let scene = u8::from_str(matches.value_of(scene).ok_or("No value for scene")?)?;
                println!("scene={}", scene);
                ControlMessage::Scene(scene)
            } else if matches.is_present(rgb) {
                let rgb = u8::from_str(matches.value_of(rgb).ok_or("No value for rgb")?)?;
                println!("rgb={}", rgb);
                ControlMessage::RGB(rgb)
            } else {
                panic!("Not recognized command");
            };
        let mut stream = TcpStream::connect("127.0.0.1:8631").await?;
        stream.write_all(&encode(&cmd)?).await?;
        info!("Sending message to server! {:?}", &cmd);

    };

    Ok(())
}
