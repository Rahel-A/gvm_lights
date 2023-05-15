use clap::{Arg, Command, PossibleValue, ArgMatches};
use gvm_server::gvm_node_command::{GvmNodeCommand, LightCmd, ModeCmd};

pub mod client;
pub use client::Client;

pub fn find_command(matches: &ArgMatches) -> Option<GvmNodeCommand> {
    if let Some(s) = matches.get_one::<String>("light") {
        Some(match s.as_str() {
            "on" => GvmNodeCommand::Light(LightCmd::On),
            "off" => GvmNodeCommand::Light(LightCmd::Off),
            _ => panic!("Incorrect argument passed")
        })
    } else if let Some(s) = matches.get_one::<String>("mode") {
        Some(match s.as_str() {
            "CT" => GvmNodeCommand::Mode(ModeCmd::ColourTemp),
            "HS" => GvmNodeCommand::Mode(ModeCmd::HueSat),
            "Sc" => GvmNodeCommand::Mode(ModeCmd::Scenes),
            _ => panic!("Incorrect argument passed")
        })
    } else if let Some(br) = matches.get_one::<u8>("brightness") {
        Some(GvmNodeCommand::Brightness(*br))
    } else if let Some(t) = matches.get_one::<u16>("temperature") {
        Some(GvmNodeCommand::Temperature(*t))
    } else if let Some(h) = matches.get_one::<u16>("hue") {
        Some(GvmNodeCommand::Hue(*h))
    } else if let Some(sat) = matches.get_one::<u8>("saturation") {
        Some(GvmNodeCommand::Saturation(*sat))
    } else if let Some(sc) = matches.get_one::<u8>("scene") {
        Some(GvmNodeCommand::Scene(*sc))
    } else if let Some(r) = matches.get_one::<u8>("rgb") {
        Some(GvmNodeCommand::RGB(*r))
    } else {
        None
    }
}

pub fn cli() -> Command<'static> {
    let validator_u8 = clap::value_parser!(u8);
    let validator_u16 = clap::value_parser!(u16);

    Command::new("GVM Lights")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::new("light")
                  .long("light")
                  .short('l')
                  .takes_value(true)
                  .value_parser([
                      PossibleValue::new("on"),
                      PossibleValue::new("off")]))
        .arg(Arg::new("brightness")
                  .long("brightness")
                  .short('b')
                  .value_parser(validator_u8)
                  .takes_value(true))
        .arg(Arg::new("temperature")
                  .long("temperature")
                  .short('t')
                  .value_parser(validator_u16)
                  .takes_value(true))
        .arg(Arg::new("hue")
                  .long("hue")
                  .short('h')
                  .value_parser(validator_u16)
                  .takes_value(true))
        .arg(Arg::new("saturation")
                  .long("saturation")
                  .short('s')
                  .value_parser(validator_u8)
                  .takes_value(true))
        .arg(Arg::new("mode")
                  .long("mode")
                  .short('m')
                  .takes_value(true)
                  .value_parser([
                      PossibleValue::new("CT"),
                      PossibleValue::new("HS"),
                      PossibleValue::new("Sc")]))
        .arg(Arg::new("scene")
                  .long("scene")
                  .short('z')
                  .value_parser(validator_u8)
                  .takes_value(true))
        .arg(Arg::new("rgb")
                  .long("rgb")
                  .short('r')
                  .value_parser(validator_u8)
                  .takes_value(true))
        .arg(Arg::new("state")
                  .long("state")
                  .short('i')
                  .takes_value(false))
        .arg(Arg::new("client")
                  .long("client")
                  .default_value("255")
                  .value_parser(validator_u8)
                  .short('c')
                  .takes_value(true))
}

