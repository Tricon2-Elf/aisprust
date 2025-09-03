use std::cmp;

use camellia::cipher::KeyInit;
use rsa::{
    self, BigUint, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
    rand_core::{OsRng, RngCore},
    traits::PublicKeyParts,
};

use crate::{
    crypt::crypto::NetworkCrypto,
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
#[derive(PartialEq, Debug)]
enum State {
    Init = 1,

    ClientSendPublicKey = 2,
    ClientWaitingSharedKeys = 3,

    ServerWaitingKeyExchange = 4,
    ServerGenerateKeys = 5,

    ClientReady = 6,
    ServerReady = 7,
}

pub trait CryptProvider {
    fn initialize(&mut self, key: &[u8]) -> Result<(), std::io::Error>;
    fn reinitialize(&mut self) -> Result<(), std::io::Error>;

    fn encrypt(&mut self, data: &mut [u8]) -> Result<(), std::io::Error>;
    fn decrypt(&mut self, data: &mut [u8]) -> Result<(), std::io::Error>;
}

const RSA_E: u32 = 65537;

pub struct CryptStream<T> {
    state: State,

    // the rsa is for key exchange. probably similar to diffie hellman
    rsa_priv: Option<RsaPrivateKey>,
    rsa_pub: Option<RsaPublicKey>,

    // one of theese is send and recv keys. depending on client/server.
    s2c_key: [u8; 0x10],
    c2s_key: [u8; 0x10],

    s2c_ctx: T,
    c2s_ctx: T,
}

impl<T: CryptProvider> CryptStream<T> {
    pub fn new_client(encrypt_provider: T, decrypt_provider: T) -> Self {
        // todo!("Generate public key and initialize it");

        let rsa_priv = RsaPrivateKey::new(&mut OsRng, 64).expect("Failed to create priv key");
        let rsa_pub = RsaPublicKey::from(&rsa_priv);

        Self {
            rsa_priv: Some(rsa_priv),
            rsa_pub: Some(rsa_pub),

            state: State::ClientSendPublicKey,

            s2c_key: [0u8; 0x10],
            c2s_key: [0u8; 0x10],

            s2c_ctx: encrypt_provider,
            c2s_ctx: decrypt_provider,
        }
    }

    pub fn new_server(encrypt_provider: T, decrypt_provider: T) -> Self {
        Self {
            state: State::ServerWaitingKeyExchange,

            rsa_priv: None,
            rsa_pub: None,

            s2c_key: [0u8; 0x10],
            c2s_key: [0u8; 0x10],

            s2c_ctx: encrypt_provider,
            c2s_ctx: decrypt_provider,
        }
    }
}

impl<T: CryptProvider> NetworkCrypto for CryptStream<T> {
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

                println!("rsa pub key {:?}", rsa_n);
                // println!("{:?}", &encrypted.incoming);

                self.rsa_pub = match RsaPublicKey::new_with_max_size(
                    BigUint::from_bytes_le(rsa_n),
                    BigUint::from(RSA_E),
                    128,
                ) {
                    Ok(rsa) => Some(rsa),
                    Err(e) => return Err(NetError::Generic(e.to_string())),
                };

                self.state = State::ServerGenerateKeys;

                // let mut rng_inst = rand::rng();
                let mut rng_inst = OsRng;

                rng_inst
                    .try_fill_bytes(&mut self.s2c_key)
                    .expect("Failed to fill encrypt key");
                rng_inst
                    .try_fill_bytes(&mut self.c2s_key)
                    .expect("Failed to fill decrypt key");

                // ok so techinically rsa wants the mesasge to be under n. set the last byte to 0
                // to hopefully get below n always
                //
                self.s2c_key[15] = 0;
                self.c2s_key[15] = 0;
                println!("c2s key {:?}", self.s2c_key);
                println!("s2c key {:?}", self.c2s_key);

                if let Err(e) =  self.s2c_ctx.initialize(&self.s2c_key) {
                    panic!("Failed to initialize cryoto 1 {}", e);
                }
                if let Err(e) = self.c2s_ctx.initialize(&self.c2s_key) {
                    panic!("Failed to initialize cryoto 2 {}", e);
                }

                let (pub_rsa_e,pub_rsa_n) = {
                    let rsa = self.rsa_pub.as_ref().expect("pub");

                    (rsa.e(), rsa.n())
                };

                // send encrypt_key
                // NOTE: rsa lib sucks and is too picky. do it manually
                let encrypted_s2c_key = BigUint::from_bytes_le(&self.s2c_key)
                    .modpow(pub_rsa_e, pub_rsa_n)
                    .to_bytes_le();
                encrypted.outgoing.extend_from_slice(&encrypted_s2c_key);

                // encrypt and send key to
                // NOTE: rsa lib sucks and is too picky. do it manually
                let encrypted_c2s_key = BigUint::from_bytes_le(&self.c2s_key)
                    .modpow(pub_rsa_e, pub_rsa_n)
                    .to_bytes_le();
                encrypted.outgoing.extend_from_slice(&encrypted_c2s_key);

                self.state = State::ServerReady;
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

                println!("encrypt/c2s key raw {:?}", encrypt_key);
                println!("decrypt/s2c key raw {:?}", decrypt_key);

                self.s2c_key = match self
                    .rsa_priv
                    .as_ref()
                    .expect("client with no rsa keys.")
                    .decrypt(Pkcs1v15Encrypt, encrypt_key)
                {
                    Ok(key) => key[0..0x10].try_into().expect("failed to encrypt"),
                    Err(e) => return Err(NetError::Generic(e.to_string())),
                };

                self.c2s_key = match self
                    .rsa_priv
                    .as_ref()
                    .expect("client with no rsa keys.")
                    .decrypt(Pkcs1v15Encrypt, decrypt_key)
                {
                    Ok(key) => key[0..0x10].try_into().expect("failed to encrypt"),
                    Err(e) => return Err(NetError::Generic(e.to_string())),
                };

                println!("encrypt/c2s key {:?}", self.s2c_key);
                println!("decrypt/s2c key {:?}", self.c2s_key);

                self.s2c_ctx.initialize(&self.s2c_key);
                self.c2s_ctx.initialize(&self.c2s_key);

                self.state = State::ClientReady;

                Ok(SIZE)
            }

            State::ServerReady => {
                // decrypts client to server
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
                    total_size += 0x10 - (data_size & 0xF);
                }

                if encrypted.incoming.len() < total_size {
                    return Ok(0);
                }
                // println!("Recevied {:?}", &encrypted.incoming);

                for i in 0..data_size / 0x10 {
                    let mut data_block = [0; 0x10];
                    data_block.copy_from_slice(
                        &encrypted.incoming[HEADER + (0x10 * i)..HEADER + (0x10 * (i + 1))],
                    );

                    self.c2s_ctx.decrypt(&mut data_block);

                    decrypted.incoming.extend_from_slice(&data_block);
                }
                if (data_size & 0xF) != 0 {
                    let offset = HEADER + (data_size & !0xF);
                    let mut data_block = [0; 0x10];
                    data_block.copy_from_slice(&encrypted.incoming[offset..offset + 0x10]);

                    self.c2s_ctx.decrypt(&mut data_block);

                    decrypted
                        .incoming
                        .extend_from_slice(&data_block[..(data_size & 0xF)]);
                }

                // println!("Decrypted {:?}", &decrypted.incoming);

                Ok(total_size)
            }

            State::ClientReady => {
                // decrypts server to client
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
                    let mut data_block = [0; 0x10];
                    data_block.copy_from_slice(
                        &encrypted.incoming[HEADER + (0x10 * i)..HEADER + (0x10 * (i + 1))],
                    );

                    self.s2c_ctx.decrypt(&mut data_block);

                    decrypted.incoming.extend_from_slice(&data_block);
                }
                if (data_size & 0xF) != 0 {
                    let offset = HEADER + (data_size & !0xF);
                    let mut data_block = [0; 0x10];
                    data_block.copy_from_slice(&encrypted.incoming[offset..offset + 0x10]);

                    self.s2c_ctx.decrypt(&mut data_block);

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

            State::ClientReady => {
                // encrypts client to server
                let data_size = cmp::min(decrypted.outgoing.len(), 1400);

                if data_size < 0x10 {
                    return Ok(0);
                }

                // send info to server
                encrypted
                    .outgoing
                    .extend_from_slice(&(data_size as u32).to_le_bytes());

                for i in 0..data_size / 0x10 {
                    let mut data_block = [0; 0x10];
                    data_block.copy_from_slice(&decrypted.outgoing[(0x10 * i)..(0x10 * (i + 1))]);

                    self.c2s_ctx.encrypt(&mut data_block);

                    encrypted.outgoing.extend_from_slice(&data_block);
                }
                if (data_size & 0xF) != 0 {
                    let offset = data_size & !0xF;
                    let size = data_size & 0xF;

                    // real vce fills this with random. seems like a waste..
                    let mut data_block = [0; 0x10];
                    data_block[..size]
                        .copy_from_slice(&decrypted.outgoing[offset..(offset + size)]);

                    self.c2s_ctx.encrypt(&mut data_block);

                    encrypted.outgoing.extend_from_slice(&data_block);
                }

                Ok(data_size)
            }

            State::ServerReady => {
                // encrypts server to client
                let data_size = cmp::min(decrypted.outgoing.len(), 1400);
                // let data_size = decrypted.outgoing.len();
                if data_size < 1 {
                    return Ok(0);
                }

                // println!("Outgoing data {} {:?}", data_size, decrypted.outgoing);

                // send info to server
                encrypted
                    .outgoing
                    .extend_from_slice(&(data_size as u32).to_le_bytes());

                for i in 0..data_size / 0x10 {
                    let mut data_block = [0; 0x10];
                    data_block.copy_from_slice(&decrypted.outgoing[(0x10 * i)..(0x10 * (i + 1))]);

                    self.s2c_ctx.encrypt(&mut data_block);

                    encrypted.outgoing.extend_from_slice(&data_block);
                }
                if (data_size & 0xF) != 0 {
                    let offset = data_size & !0xF;
                    let size = data_size & 0xF;

                    // real vce fills this with random. seems like a waste..
                    let mut data_block = [0; 0x10];
                    data_block[..size]
                        .copy_from_slice(&decrypted.outgoing[offset..(offset + size)]);

                    self.s2c_ctx.encrypt(&mut data_block);

                    encrypted.outgoing.extend_from_slice(&data_block);
                }
                // println!("Outgoing encrypted {:?}", encrypted.outgoing);

                Ok(data_size)
            }

            _ => Ok(0),
        }
    }
}
