use std::fmt;

#[derive(Debug)]
pub enum MqttError {
    InvalidTopic,
    InvalidPayload,
    UnavailableNodeStatus,
    IO(std::io::Error),
}

impl From<std::io::Error> for MqttError {
    fn from(err: std::io::Error) -> MqttError {
        MqttError::IO(err)
    }
}

impl fmt::Display for MqttError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MqttError::InvalidTopic => "invalid topic".fmt(f),
            MqttError::InvalidPayload => "invalid payload".fmt(f),
            MqttError::UnavailableNodeStatus => "unavailable node status".fmt(f),
            MqttError::IO(_) => "IO error".fmt(f),
        }
    }
}

impl std::error::Error for MqttError {}
