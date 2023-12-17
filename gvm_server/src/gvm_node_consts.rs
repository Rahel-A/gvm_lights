pub const MIN_TEMPERATURE: u32 = 3200; //Kelvins
pub const MAX_TEMPERATURE: u32 = 5600; //Kelvins

pub fn kelvin_to_mireds(kelvins: u32) -> f32 {
    1_000_000 as f32 / kelvins as f32
}

///
/// ```
/// use gvm_server::gvm_node_consts::*;
/// assert_eq!(mirads_to_kelvin(kelvin_to_mirads(MAX_TEMPERATURE)),
///            MAX_TEMPERATURE);
/// ```
pub fn mireds_to_kelvin(mirads: f32) -> u32 {
    (1_000_000 as f32 / mirads) as u32
}
