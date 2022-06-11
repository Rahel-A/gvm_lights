use clap::{Arg, Command};
use gvm_lights::{GvmBleClient, ControlMessage, LightCmd};
use std::str::FromStr;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let brightness = "brightness";
    let temperature = "temperature";
    let hue = "hue";
    let saturation = "saturation";
    let server = "server";
    let light = "light";
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
        .get_matches();

    let cmd = match matches.value_of(light) {
        Some("on") => { 
            println!("on");
            ControlMessage::Light(LightCmd::On)
        },
        Some("off") => {
            println!("off");
            ControlMessage::Light(LightCmd::Off)
        },
        _ => {
            if matches.is_present(brightness) {
                let br = u8::from_str(matches.value_of(brightness).ok_or("No value for brightness")?)?;
                println!("br={}", br);
                ControlMessage::SetBrightness(br)
            } else if matches.is_present(temperature) {
                let t = u16::from_str(matches.value_of(temperature).ok_or("No value for temperature")?)?;
                println!("temp={}", t);
                ControlMessage::SetTemperature(t)
            } else if matches.is_present(hue) {
                let hue = u16::from_str(matches.value_of(hue).ok_or("No value for hue")?)?;
                println!("hue={}", hue);
                ControlMessage::SetHue(hue)
            } else if matches.is_present(saturation) {
                let sat = u8::from_str(matches.value_of(saturation).ok_or("No value for sat")?)?;
                println!("sat={}", sat);
                ControlMessage::SetSaturation(sat)
            } else {
                panic!("Not recognized command");
            }
        }
    };

    let client1 = GvmBleClient::new("A4:C1:38:EE:86:C1").await?;
    let client2 = GvmBleClient::new("A4:C1:38:8D:61:45").await?;
    
    client1.send_to(&cmd).await?.close().await?;
    time::sleep(Duration::from_millis(300)).await;
    client2.send_to(&cmd).await?.close().await?;
    Ok(())
}
