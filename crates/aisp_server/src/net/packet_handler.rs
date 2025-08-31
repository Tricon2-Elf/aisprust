use aisp_packet::packets::PacketId;

use crate::net::{net_error::NetError, vce_peer::VcePeer};

#[async_trait::async_trait]
pub trait ServerHandler: Send + Sync {
    fn on_packet(&mut self, peer: &mut VcePeer, packet: &PacketId) -> Result<(), NetError>;

    // fn outgoing(&mut self, peer: &VcePeer) -> Result<Vec<PacketId>, NetError>;
}
