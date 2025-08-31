use crate::packets::Packet;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xc202)]
pub struct Ping {
    pub cur_time: u32,
}
