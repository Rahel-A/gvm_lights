use crc_any::CRC;

pub enum LightCmd {
    On,
    Off,
}

pub enum ControlMessage {
    Light(LightCmd),
    SetBrightness(u8),
    SetTemperature(u16),
    SetHue(u16),
    SetSaturation(u8),
}

pub fn encode(msg: &ControlMessage) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let dev_id = b"00";
    let dev_type = b"30";

    let (cmd, param) = match msg {
        ControlMessage::Light(param) => {
            match param {
                LightCmd::On  => (b"00", b"01".to_vec()),
                LightCmd::Off => (b"00", b"00".to_vec()),
            }
        },
        ControlMessage::SetBrightness(*br) => {
            let br = if br > 100 { 100 }
                        else { br }; 
            (b"02", hex::encode_upper([br]).into_bytes())
        },
        ControlMessage::SetTemperature(t) => {
            let t = if t < 3200 { 3200 }
                    else {
                        if t > 5600 { 5600 }
                        else { t }
                    };
            (b"03", hex::encode_upper([(t / 100) as u8]).into_bytes())
        },
        ControlMessage::SetHue(hue) => {
            let hue = if hue > 360 { 360 }
                      else { hue };
            (b"04", hex::encode_upper([(hue / 5) as u8]).into_bytes())
        },
        ControlMessage::SetSaturation(sat) => {
            let sat = if sat > 100 { 100 }
                      else { sat };
            (b"05", hex::encode_upper([sat]).into_bytes())
        },
    };

    let mut buf = Vec::<u8>::new();
    buf.extend_from_slice(b"4C5409");
    buf.extend_from_slice(dev_id);
    buf.extend_from_slice(dev_type);
    buf.extend_from_slice(b"5700");
    buf.extend_from_slice(cmd);
    buf.extend_from_slice(b"01");
    buf.extend_from_slice(&param);
    
    let buf_decoded = hex::decode(&buf)?;
    let mut crc16 = CRC::crc16xmodem();
    crc16.digest(&buf_decoded);
    let crc_str = hex::encode_upper(crc16.get_crc_vec_be());
    
    buf.extend_from_slice(crc_str.as_bytes());
    Ok(buf)
}
