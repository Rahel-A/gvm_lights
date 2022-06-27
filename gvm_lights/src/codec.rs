use crc_any::CRC;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum LightCmd {
    On,
    Off,
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ModeCmd {
    ColourTemp,
    HueSat,
    Scenes,
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ControlMessage {
    Light(LightCmd),
    Brightness(u8),
    Temperature(u16),
    Hue(u16),
    Saturation(u8),
    RGB(u8),
    Scene(u8),
    Mode(ModeCmd),
    ReadState()
}

impl From<ControlMessage> for String {
    fn from(msg: ControlMessage) -> String {
        let (cmd, param) = match msg {
            ControlMessage::Light(param) => {
                match param {
                    LightCmd::On  => (b"00", b"01".to_vec()),
                    LightCmd::Off => (b"00", b"00".to_vec()),
                }
            },
            ControlMessage::RGB(rgb) => { // TODO: what is this (hue?)?
                let rgb = if rgb >= 255 { 255 }
                          else { rgb };
                (b"01", hex::encode_upper([rgb]).into_bytes())
            },
            ControlMessage::Brightness(br) => {
                let br = if br > 100 { 100 }
                            else { br };
                (b"02", hex::encode_upper([br]).into_bytes())
            },
            ControlMessage::Temperature(t) => {
                let t = if t < 3200 { 3200 }
                        else {
                            if t > 5600 { 5600 }
                            else { t }
                        };
                (b"03", hex::encode_upper([(t / 100) as u8]).into_bytes())
            },
            ControlMessage::Hue(hue) => {
                let hue = if hue > 360 { 360 }
                          else { hue };
                (b"04", hex::encode_upper([(hue / 5) as u8]).into_bytes())
            },
            ControlMessage::Saturation(sat) => {
                let sat = if sat > 100 { 100 }
                          else { sat };
                (b"05", hex::encode_upper([sat]).into_bytes())
            },
            ControlMessage::Mode(mode) => {
                match mode {
                    ModeCmd::ColourTemp  => (b"06", b"01".to_vec()),
                    ModeCmd::HueSat => (b"06", b"02".to_vec()),
                    ModeCmd::Scenes => (b"06", b"03".to_vec()),
                }
            },
            ControlMessage::Scene(scene) => {
                let scene = if scene > 8 { 0 }
                          else { scene };
                (b"07", hex::encode_upper([scene]).into_bytes())
            },
            _ => panic!("Unable to convert this message")
        };
        format!("{cmd:?}01{param:?}")
    }
}

pub fn encode(msg: &ControlMessage) -> Result<Vec<u8>, hex::FromHexError> {
    let dev_id = b"00";
    let dev_type = b"30";

    let mut buf = Vec::<u8>::new();
    buf.extend_from_slice(b"4C5409");
    buf.extend_from_slice(dev_id);
    buf.extend_from_slice(dev_type);
    buf.extend_from_slice(b"5700");
    buf.extend_from_slice(String::from(*msg).as_bytes());
    
    let buf_decoded = hex::decode(&buf)?;
    let mut crc16 = CRC::crc16xmodem();
    crc16.digest(&buf_decoded);
    let crc_str = hex::encode_upper(crc16.get_crc_vec_be());

    buf.extend_from_slice(crc_str.as_bytes());

    buf.extend_from_slice(b"FF");

    Ok(buf)
}
