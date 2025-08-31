#[derive(Debug)]
pub enum NetError {
    Generic(String),
    NeedData,

    DecompressionError(String),
    CompressionError(String),

    PacketNoHandler,
}
