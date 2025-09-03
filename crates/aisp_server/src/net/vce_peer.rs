use crate::{
    compression::compression::CompressionType,
    crypt::crypto::{CryptoType, NetworkCrypto},
    net::{
        net_error::NetError, packet_handler::ServerHandler, server_backend::NetworkStream,
        stream_buffer::StreamBuffer, vce_codec::VceCodec,
    },
};
use std::{
    any::Any,
    collections::VecDeque,
    io::ErrorKind,
    sync::{Arc, Mutex},
};

use aisp_packet::packets::PacketId;

enum PeerState {
    Closed,

    Ready,
}

pub struct VcePeer {
    pub network: NetworkStream,
    pub compressor: CompressionType,
    pub encryption: CryptoType,

    pub state: PeerState,

    pub buffer: StreamBuffer,
    pub encrypted_buffer: StreamBuffer,

    outgoing_packets: Arc<Mutex<VecDeque<PacketId>>>,
}

impl VcePeer {
    pub fn new(
        stream: NetworkStream,
        compression: CompressionType,
        encryption: CryptoType,
    ) -> Self {
        Self {
            network: stream,
            compressor: compression,
            encryption,

            state: PeerState::Ready,

            buffer: StreamBuffer::new(),
            encrypted_buffer: StreamBuffer::new(),

            outgoing_packets: Arc::new(VecDeque::new().into()),
        }
    }

    pub fn queue_packet(&mut self, packet: PacketId) {
        let mut queue = self.outgoing_packets.lock().unwrap();
        queue.push_back(packet);
    }

    fn handle_packets<H: ServerHandler>(&mut self, handler: &mut H) {
        while self.buffer.incoming.len() > 1 {
            let (codec, codec_size) = match VceCodec::from_bytes(&self.buffer.incoming) {
                Ok(data) => data,
                Err(e) => {
                    println!("ERROR: {}", e);
                    break;
                }
            };

            match codec {
                VceCodec::PacketData(packet_data) => {
                    println!("Incoming {:?}", packet_data);
                    let packet_cls = PacketId::from_bytes(&packet_data);

                    match packet_cls {
                        Ok(packet) => {
                            if let Err(e) = handler.on_packet(self, &packet) {
                                println!("Packet handler error {:?}", e);
                                // if e == NetError::PacketNoHandler {
                                //     println!("No handler")
                                // }
                            }
                        }

                        Err(e) => println!("ERROR: {:?}", e),
                    }
                }
                _ => unimplemented!("Unimplemented codec {:?}", codec.type_id()),
            }

            self.buffer.incoming.drain(0..codec_size);
        }

        // TODO: get pending PacketIds from handler and serialize them and write it.
    }

    fn process_incoming(&mut self) -> Result<(), NetError> {
        //TODO: move into class to prevent huge stack usage.
        let mut recv_buffer = [0u8; 0x4000];

        match self.network.receive(&mut recv_buffer) {
            Ok(0) => {
                self.state = PeerState::Closed;
            }
            Ok(n) => {
                let recv_data = &recv_buffer[0..n];

                // println!("Received {} bytes", n);
                // let mut hex_str = String::new();
                // hex_write(&mut hex_str, recv_data, HexConfig::default());
                // println!("{}", hex_str);

                self.encrypted_buffer.incoming.extend_from_slice(recv_data);

                let size = self
                    .encryption
                    .handle_incoming(&mut self.encrypted_buffer, &mut self.buffer)?;
                self.encrypted_buffer.incoming.drain(0..size);

                // {
                //     Ok(size) => {
                //         // println!("decrypted {}", size);
                //         self.encrypted_buffer.incoming.drain(0..size);
                //     }
                //     Err(e) => panic!("{:?}", e),
                // }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => return Err(NetError::Generic(e.to_string())),
        }

        Ok(())
    }

    fn process_outgoing(&mut self) -> Result<(), NetError> {
        {
            let mut queue = self.outgoing_packets.lock().unwrap();

            while let Some(packet) = queue.pop_front() {
                let packet_data = match packet.to_bytes() {
                    Ok(data) => data,
                    Err(e) => {
                        println!("Failed to serialize packet. [{:?}]", packet);

                        continue;
                    }
                };

                let codec = VceCodec::PacketData(packet_data);

                let codec_payload = match codec.to_bytes() {
                    Ok(data) => data,
                    Err(e) => {
                        println!("Failed to serialize codec.");
                        continue;
                    }
                };

                // println!("-> {:?}",  packet);
                // println!("\t {:?}", codec_payload);
                // self.buffer
                //     .outgoing
                //     .extend_from_slice(&payload_len.to_be_bytes());
                // self.buffer.outgoing.extend_from_slice(&payload);
                // println!("Queuing {} bytes", codec_payload.len());
                self.buffer.outgoing.extend_from_slice(&codec_payload);
            }
        }

        let size = self
            .encryption
            .handle_outgoing(&mut self.buffer, &mut self.encrypted_buffer)?;
        self.buffer.outgoing.drain(0..size);

        if !self.encrypted_buffer.outgoing.is_empty() {
            // println!("Sending {} bytes", self.encrypted_buffer.outgoing.len());
            self.network
                .send(self.encrypted_buffer.outgoing.as_mut_slice())
                .expect("faield to send data");
            self.encrypted_buffer.clear_outgoing();
        }

        Ok(())
    }

    pub fn tick<H: ServerHandler>(&mut self, handler: &mut H) -> Result<(), NetError> {
        self.process_incoming()?;
        self.process_outgoing()?;

        self.handle_packets(handler);

        Ok(())
    }
}
