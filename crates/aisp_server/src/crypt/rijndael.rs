use crate::{
    crypt::encryption::Crypter,
    net::{net_error::NetError, stream_buffer::StreamBuffer},
};

enum State {
    // Init = 1,
    //
    // Client_SendPublicKey = 2,
    // Client_WaitingSharedKeys = 3,
    //
    // Server_WaitingKeyExchange = 4,
    // Server_DeriveKeys = 5,
    Ready = 6,
}

pub struct RijndaelCrypter {
    state: State,
}

impl RijndaelCrypter {
    fn new_server() -> Self {
        todo!("Generate public key and initialize it");
        Self {
            // state: State::Client_SendPublicKey,
            state: State::Ready,
        }
    }

    fn new_client() -> Self {
        Self {
            // state: State::Server_WaitingKeyExchange,
            state: State::Ready,
        }
    }
}

impl Crypter for RijndaelCrypter {
    fn handle_incoming(
        &mut self,
        input: &mut StreamBuffer,
        output: &mut StreamBuffer,
    ) -> Result<usize, NetError> {
        panic!("Implement");
        match self.state {
            // State::Server_WaitingKeyExchange => {
            //     if input.len() < 0x10 {
            //         return Err(NetError::WaitData);
            //     }
            //
            //     let public_key = &input[0..0x10];
            //
            //     self.state = State::Server_DeriveKeys;
            //     todo!(
            //         "Generate bytes, pass to ekeygen with shared, do some encryption stuff with send buffer, and write it to outgoing buffer"
            //     );
            //
            //     todo!(
            //         "Generate bytes, pass to ekeygen with shared, do some encryption stuff for send buffer, and write to outgoing buffer"
            //     );
            //
            //     // write derived keys to output buffer
            //     // probably needs to be in its own state if its designed like this.. and we would
            //     // need to have data to send instead of an update function/thread and buffers thats
            //     // flushed
            //
            //     // output.outgoing.extend_from_slice(&)
            //     self.state = State::Ready;
            // }
            // State::Client_WaitingSharedKeys => {
            //     if input.len() < 0x20 {
            //         return Err(NetError::WaitData);
            //     }
            //
            //     let decryption_key_scrambled = &input[0x00..0x10];
            //     let encryption_key_scrambled = &input[0x10..0x20];
            //
            //     todo!(
            //         "decrypt send keys, and pass it to camellia ekeygen to get decryption and encryption key"
            //     );
            //
            //     self.state = State::Ready;
            // }
            State::Ready => {
                todo!("Do encryption");

                // Err()
            }
        }
    }
    fn handle_outgoing(
        &mut self,
        input: &mut StreamBuffer,
        output: &mut StreamBuffer,
    ) -> Result<usize, NetError> {
        panic!("Implement");
        match self.state {
            // State::Client_SendPublicKey => {
            //     output.outgoing.extend_from_slice(&self.public_key[0..0x10]);
            //
            //     self.state = State::WaitingSharedKeys;
            //     Ok(0)
            // }
            //
            State::Ready => {
                todo!("Do encryption");

                // Err()
            }
        }
    }
}
