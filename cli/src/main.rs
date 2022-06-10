use clap::{Arg, Command};
use gvm_lights::{GvmBleClient, ControlMessage, LightCmd};
use std::str::FromStr;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("GVM Lights")
        .version("0.1.0")
        .arg(Arg::new("l")
                  .long("light")
                  .takes_value(true)
                  .possible_values(&["on", "off"]))
        .arg(Arg::new("b")
                  .long("br")
                  .takes_value(true))
        .arg(Arg::new("t")
                  .long("temp")
                  .takes_value(true))
        .arg(Arg::new("h")
                  .long("hue")
                  .takes_value(true))
        .arg(Arg::new("s")
                  .long("sat")
                  .takes_value(true))
        .get_matches();

    let cmd = match matches.value_of("light") {
        Some("on") => { 
            println!("on");
            ControlMessage::Light(LightCmd::On)
        },
        Some("off") => {
            println!("off");
            ControlMessage::Light(LightCmd::Off)
        },
        _ => {
            if matches.is_present("br") {
                let br = u8::from_str(matches.value_of("br").ok_or("No value for brightness")?)?;
                println!("br={}", br);
                ControlMessage::SetBrightness(br)
            } else if matches.is_present("t") {
                let t = u16::from_str(matches.value_of("t").ok_or("No value for temperature")?)?;
                println!("t={}", t);
                ControlMessage::SetTemperature(t)
            } else if matches.is_present("hue") {
                let hue = u16::from_str(matches.value_of("hue").ok_or("No value for hue")?)?;
                println!("hue={}", hue);
                ControlMessage::SetHue(hue)
            } else if matches.is_present("sat") {
                let sat = u8::from_str(matches.value_of("sat").ok_or("No value for sat")?)?;
                println!("sat={}", sat);
                ControlMessage::SetSaturation(sat)
            } else {
                panic!("Not recognized command");
            }
        }
    };

    let client = GvmClient::new("192.168.4.2").await?;
    
    let sent_bytes = client.send_to(&"192.168.4.1", &cmd).await?;
    println!("{:?}", sent_bytes);
    Ok(())
}
