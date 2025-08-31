use crate::packets::Packet;
use crate::util::fixed_array::FixedArray;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x0cbc)]
pub struct MascotGetCountRequest {
    map_id: u32,
    channel_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x7790)]
pub struct MascotGetCountResponse {
    pub result: u32,
    pub count: u32,
    pub serial_id: u32,
    pub name: Vec<u8>,
}
