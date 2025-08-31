use camellia::{
    Camellia128,
    cipher::{KeyInit, generic_array::GenericArray},
};
use rsa::{
    self, BigUint, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
    rand_core::{OsRng, RngCore},
    traits::PublicKeyParts,
};

use crate::{
    crypt::encryption::Crypter,
    net::{net_error::NetError, stream_buffer::StreamBuffer},
};

/**

ClientSendPublicKey
    1. client generates an rsa key with two 64 bit primes, in turn the public key becomes 128 bits.
    2. client sends this public key to server,
    client_next_state: ClientWaitingSharedKeys
    server_next_state: ServerWaitingKeyExchange

ServerWaitingKeyExchange
    1. client sends public key (rsa n) which is a 128 bit key. each prime is 64 bits
    2. server then generates an encryption key and a decryption key
    3. server then encrypts theese keys with the public key it recieved, and sends them to client
    server_next_state: Ready
    client_next_state: ClientWaitingSharedKeys

ClientWaitingSharedKeys
    1. client recieves a decryption and encryption key, encrypted with the public key sent earlier
    client_next_state: Ready
*/
enum State {
    Init = 1,

    ClientSendPublicKey = 2,
    ClientWaitingSharedKeys = 3,

    ServerWaitingKeyExchange = 4,
    ServerGenerateKeys = 5,

    Ready = 6,
}

const RSA_E: u32 = 65537;

pub struct CamelliaCrypter {
    state: State,

    // the rsa is for key exchange. probably similar to diffie hellman
    rsa_priv: Option<RsaPrivateKey>,
    rsa_pub: Option<RsaPublicKey>,

    encrypt_key: [u8; 0x10],
    decrypt_key: [u8; 0x10],

    encrypt_ctx: Option<camellia::Camellia128>,
    decrypt_ctx: Option<camellia::Camellia128>,
}

impl CamelliaCrypter {
    pub fn new_server() -> Self {
        // todo!("Generate public key and initialize it");

        let rsa_priv = RsaPrivateKey::new(&mut OsRng, 64).expect("Failed to create priv key");
        let rsa_pub = RsaPublicKey::from(&rsa_priv);

        Self {
            rsa_priv: Some(rsa_priv),
            rsa_pub: Some(rsa_pub),

            state: State::ClientSendPublicKey,

            encrypt_key: [0u8; 0x10],
            decrypt_key: [0u8; 0x10],

            encrypt_ctx: None,
            decrypt_ctx: None,
        }
    }

    pub fn new_client() -> Self {
        Self {
            state: State::ServerWaitingKeyExchange,

            rsa_priv: None,
            rsa_pub: None,

            encrypt_key: [0u8; 0x10],
            decrypt_key: [0u8; 0x10],

            encrypt_ctx: None,
            decrypt_ctx: None,
        }
    }
}

impl Crypter for CamelliaCrypter {
    fn handle_incoming(
        &mut self,
        encrypted: &mut StreamBuffer,
        decrypted: &mut StreamBuffer,
    ) -> Result<usize, NetError> {
        match self.state {
            State::ServerWaitingKeyExchange => {
                const SIZE: usize = 0x10;
                if encrypted.incoming.len() < SIZE {
                    // return Err(NetError::WaitData);
                    return Ok(0);
                }

                let rsa_n = &encrypted.incoming[0..SIZE];

                self.rsa_pub = match RsaPublicKey::new(
                    BigUint::from_bytes_le(rsa_n),
                    BigUint::new(vec![RSA_E]),
                ) {
                    Ok(rsa) => Some(rsa),
                    Err(e) => return Err(NetError::Generic(e.to_string())),
                };

                self.state = State::ServerGenerateKeys;

                // let mut rng_inst = rand::rng();
                let mut rng_inst = OsRng;

                // self.encrypt_key = rand::random::<[u8; 0x10]>();
                rng_inst
                    .try_fill_bytes(&mut self.encrypt_key)
                    .expect("Failed to fill encrypt key");
                // self.decrypt_key = rand::random::<[u8; 0x10]>();
                rng_inst
                    .try_fill_bytes(&mut self.decrypt_key)
                    .expect("Failed to fill decrypt key");

                self.encrypt_ctx = Some(Camellia128::new(GenericArray::from_slice(
                    &self.encrypt_key,
                )));
                self.decrypt_ctx = Some(Camellia128::new(GenericArray::from_slice(
                    &self.decrypt_key,
                )));

                // send encrypt_key
                let encrypted_encrypt_key = match self
                    .rsa_pub
                    .as_mut()
                    .expect("rsa_pub is null")
                    .encrypt(&mut rng_inst, Pkcs1v15Encrypt, &self.encrypt_key)
                {
                    Ok(data) => data,
                    Err(e) => return Err(NetError::Generic(e.to_string())),
                };
                encrypted.outgoing.extend_from_slice(&encrypted_encrypt_key);

                // send decrypt_key
                let encrypted_decrypt_key = match self
                    .rsa_pub
                    .as_mut()
                    .expect("rsa_pub is null")
                    .encrypt(&mut rng_inst, Pkcs1v15Encrypt, &self.decrypt_key)
                {
                    Ok(data) => data,
                    Err(e) => return Err(NetError::Generic(e.to_string())),
                };
                encrypted.outgoing.extend_from_slice(&encrypted_decrypt_key);

                self.state = State::Ready;
                Ok(SIZE)
            }
            State::ClientWaitingSharedKeys => {
                const KEY_SIZE: usize = 0x10;
                const SIZE: usize = KEY_SIZE * 2;
                if encrypted.incoming.len() < SIZE {
                    // return Err(NetError::WaitData);
                    return Ok(0);
                }

                let encrypt_key = &encrypted.incoming[0x00..KEY_SIZE];
                let decrypt_key = &encrypted.incoming[KEY_SIZE..KEY_SIZE * 2];

                self.encrypt_key = match self
                    .rsa_priv
                    .as_ref()
                    .expect("client with no rsa keys.")
                    .decrypt(Pkcs1v15Encrypt, encrypt_key)
                {
                    Ok(key) => key[0..0x10].try_into().expect("failed to encrypt"),
                    Err(e) => return Err(NetError::Generic(e.to_string())),
                };

                self.decrypt_key = match self
                    .rsa_priv
                    .as_ref()
                    .expect("client with no rsa keys.")
                    .decrypt(Pkcs1v15Encrypt, decrypt_key)
                {
                    Ok(key) => key[0..0x10].try_into().expect("failed to encrypt"),
                    Err(e) => return Err(NetError::Generic(e.to_string())),
                };

                self.encrypt_ctx = Some(Camellia128::new(GenericArray::from_slice(
                    &self.encrypt_key,
                )));
                self.decrypt_ctx = Some(Camellia128::new(GenericArray::from_slice(
                    &self.decrypt_key,
                )));

                self.state = State::Ready;

                Ok(SIZE)
            }

            State::Ready => {
                const HEADER: usize = 4;
                if encrypted.incoming.len() < HEADER {
                    return Ok(0);
                }

                let data_size =
                    u32::from_le_bytes(encrypted.incoming[0..HEADER].try_into().expect("bad len"))
                        as usize;

                let mut total_size = data_size + HEADER;
                // if block isnt aligned then wait for next block
                if (data_size & 0xF) != 0 {
                    total_size += 0x10;
                }

                if encrypted.incoming.len() < total_size {
                    return Ok(0);
                }

                for i in 0..data_size / 0x10 {
                    let data_block: &[u8] =
                        &encrypted.incoming[HEADER + (0x10 * i)..HEADER + (0x10 * (i + 1))];

                    // self.decrypt_ctx
                    //     .expect("no decryption")
                    //     .decrypt_block(data_block);
                    // self.decrypt_ctx.expect("No decryption").k[0] += 1;

                    decrypted.incoming.extend_from_slice(data_block);
                }
                if (data_size & 0xF) != 0 {
                    let offset = HEADER + (data_size & !0xF);
                    let data_block: &[u8] = &encrypted.incoming[offset..offset + 0x10];

                    // self.decrypt_ctx
                    //     .expect("no decryption")
                    //     .decrypt_block(&data_block);
                    // self.decrypt_ctx.k[0] += 1;

                    decrypted
                        .incoming
                        .extend_from_slice(&data_block[0..(data_size & 0xF)]);
                }

                Ok(total_size)
            }
            _ => Ok(0),
        }
    }
    fn handle_outgoing(
        &mut self,
        decrypted: &mut StreamBuffer,
        encrypted: &mut StreamBuffer,
    ) -> Result<usize, NetError> {
        match self.state {
            State::ClientSendPublicKey => {
                let public_key_bytes = &self
                    .rsa_pub
                    .as_ref()
                    .expect("client with no rsa keys")
                    .n()
                    .to_bytes_le();
                // let public_key_bytes = self
                //     .rsa_priv
                //     .as_ref()
                //     .expect("client with no rsa keys")
                //     .n()
                //     .to_bytes_le();

                if public_key_bytes.len() != 0x10 {
                    panic!("public key has to be 0x10 bytes {:?}", public_key_bytes);
                }

                encrypted.outgoing.extend_from_slice(public_key_bytes);

                self.state = State::ClientWaitingSharedKeys;
                Ok(0)
            }

            State::Ready => {
                todo!("Do encryption");
            }

            _ => Ok(0),
        }
    }
}
