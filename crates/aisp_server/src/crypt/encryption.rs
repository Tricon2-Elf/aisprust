use crate::{
    crypt::{blowfish::BlowfishCrypter, camellia::CamelliaCrypter, rijndael::RijndaelCrypter},
    net::{net_error::NetError, stream_buffer::StreamBuffer},
};

// pub enum State {
//     Busy,
//     Ready,
// }
//
pub enum CryptType {
    Camellia(CamelliaCrypter),
    Blowfish(BlowfishCrypter),
    Rijndael(RijndaelCrypter),
    None,
}

pub trait Crypter {
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

impl Crypter for CryptType {
    fn handle_incoming(
        &mut self,
        encrypted: &mut StreamBuffer,
        decrypted: &mut StreamBuffer,
    ) -> Result<usize, NetError> {
        match self {
            CryptType::Camellia(crypt) => crypt.handle_incoming(encrypted, decrypted),
            CryptType::Blowfish(crypt) => crypt.handle_incoming(encrypted, decrypted),
            CryptType::Rijndael(crypt) => crypt.handle_incoming(encrypted, decrypted),
            CryptType::None => {
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
            CryptType::Camellia(crypt) => crypt.handle_outgoing(encrypted, decrypted),
            CryptType::Blowfish(crypt) => crypt.handle_outgoing(encrypted, decrypted),
            CryptType::Rijndael(crypt) => crypt.handle_outgoing(encrypted, decrypted),
            CryptType::None => {
                encrypted.outgoing.extend_from_slice(&decrypted.outgoing);
                Ok(decrypted.outgoing.len())
            }
            _ => panic!("Stuff"),
        }
    }
}
