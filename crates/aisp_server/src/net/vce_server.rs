use crate::{
    compression::compression::CompressionType,
    crypt::{camellia::CamelliaProvider, crypto::CryptoType, crypto_stream::CryptStream},
    net::{packet_handler::ServerHandler, server_backend::ServerBackend, vce_peer::VcePeer},
};

pub struct VceServer<H: ServerHandler> {
    pub backend: ServerBackend,
    // pub compressor: CompressionType,
    // pub encryption: CryptType,
    handler: H,

    pub peers: Vec<VcePeer>,

    is_encrypted: bool,
}

impl<H: ServerHandler> VceServer<H> {
    // pub fn new(backend: ServerBackend, compressor: CompressionType, encryption: CryptType, handler: Arc<H>) -> Self
    pub fn new(backend: ServerBackend, handler: H, is_encrypted: bool) -> Self {
        Self {
            backend,

            handler,

            peers: Vec::new(),

            is_encrypted,
        }
    }
    pub fn new_tcp(listen_str: &str, handler: H, is_encrypted: bool) -> Self {
        Self {
            backend: ServerBackend::listen_tcp(listen_str).expect("Failed to listen server"),

            handler,

            peers: Vec::new(),

            is_encrypted,
        }
    }

    fn handle_incoming(&mut self) {
        // println!("Handle incoming");
        if let Ok(stream) = self.backend.accept() {
            println!(
                "Got connection {} enc: {}",
                stream.peer_address().expect("peer"),
                self.is_encrypted
            );

            let crypto_type = match self.is_encrypted {
                false => CryptoType::None,
                true => CryptoType::new_server_camellia(),
            };
            // TODO: create new compressiontype and crypttype. copying doesnt work as it is not
            // stateless.
            // let vce_peer = VcePeer::new(stream, CompressionType::None, CryptoType::None);
            let vce_peer = VcePeer::new(stream, CompressionType::None, crypto_type);

            self.peers.push(vce_peer);
        }
    }

    pub fn tick(&mut self) {
        self.handle_incoming();

        // println!("Handle peers");
        for (id, peer) in self.peers.iter_mut().enumerate() {
            if let Err(e) = peer.tick(&mut self.handler) {
                println!("Peer {} error: {:?}", id, e);
                // self.handler.on_peer_disconnect(*id).await;
            }
        }
    }
}
