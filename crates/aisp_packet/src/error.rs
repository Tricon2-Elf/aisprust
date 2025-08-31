// use serde::de::value::Error;

#[derive(Debug)]
pub enum ParseError {
    Truncated,
    UnknownPacket(u16),
    SerdeDeserialize(serde::de::value::Error),
    SerdeSerialize(serde::de::value::Error),
}
