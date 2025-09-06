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
}

impl<H: ServerHandler> VceServer<H> {
    // pub fn new(backend: ServerBackend, compressor: CompressionType, encryption: CryptType, handler: Arc<H>) -> Self
    pub fn new(backend: ServerBackend, handler: H) -> Self {
        Self {
            backend,

            handler,

            peers: Vec::new(),
        }
    }

    fn handle_incoming(&mut self) {
        // println!("Handle incoming");
        if let Ok(stream) = self.backend.accept() {
            println!("Got connection {}", stream.peer_address().expect("peer"));

            // TODO: create new compressiontype and crypttype. copying doesnt work as it is not
            // stateless.
            let vce_peer = VcePeer::new(stream, CompressionType::None, CryptoType::None);
            // let vce_peer = VcePeer::new(
            //     stream,
            //     CompressionType::None,
            //     CryptoType::Camellia(CryptStream::new_server(
            //         CamelliaProvider::new(),
            //         CamelliaProvider::new(),
            //     )),
            // );

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
