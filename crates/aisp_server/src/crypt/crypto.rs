use crate::{
    crypt::{camellia::CamelliaProvider, crypto_stream::CryptStream},
    net::{net_error::NetError, stream_buffer::StreamBuffer},
};

// pub enum State {
//     Busy,
//     Ready,
// }
//
pub enum CryptoType {
    Camellia(CryptStream<CamelliaProvider>),
    Blowfish, //(BlowfishCrypter),
    Rijndael, //(RijndaelCrypter),
    None,
}

pub trait NetworkCrypto {
    fn handle_incoming(
        &mut self,
        input: &mut StreamBuffer,
        output: &mut StreamBuffer,
    ) -> Result<usize, NetError>;

    fn handle_outgoing(
        &mut self,
        input: &mut StreamBuffer,
        output: &mut StreamBuffer,
    ) -> Result<usize, NetError>;
}

impl NetworkCrypto for CryptoType {
    fn handle_incoming(
        &mut self,
        encrypted: &mut StreamBuffer,
        decrypted: &mut StreamBuffer,
    ) -> Result<usize, NetError> {
        match self {
            CryptoType::Camellia(crypt) => crypt.handle_incoming(encrypted, decrypted),
            // CryptoType::Blowfish(crypt) => crypt.handle_incoming(encrypted, decrypted),
            // CryptoType::Rijndael(crypt) => crypt.handle_incoming(encrypted, decrypted),
            CryptoType::None => {
                decrypted.incoming.extend_from_slice(&encrypted.incoming);
                Ok(encrypted.incoming.len())
            }
            _ => panic!("Stuff"),
        }
    }

    fn handle_outgoing(
        &mut self,
        decrypted: &mut StreamBuffer,
        encrypted: &mut StreamBuffer,
    ) -> Result<usize, NetError> {
        match self {
            CryptoType::Camellia(crypt) => crypt.handle_outgoing(decrypted, encrypted),
            // CryptoType::Blowfish(crypt) => crypt.handle_outgoing(encrypted, decrypted),
            // CryptoType::Rijndael(crypt) => crypt.handle_outgoing(encrypted, decrypted),
            CryptoType::None => {
                encrypted.outgoing.extend_from_slice(&decrypted.outgoing);
                Ok(decrypted.outgoing.len())
            }
            _ => panic!("Stuff"),
        }
    }
}
