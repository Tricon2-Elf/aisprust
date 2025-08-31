use crate::packets::Packet;
use crate::util::fixed_array::FixedArray;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

use crate::shared::server;

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct FurnitureData {
    pub _0x0000: u32,
    pub _0x0004: u32,
    pub _0x0008: u32,
    pub _0x000c: u32,
    pub _0x0010: [f32; 3],
    pub _0x001c: u8,
    pub _0x001d: u8,
    pub _0x0020: u32,
}

// TODO: this is a notify packet
#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xe868)]
pub struct MyRoomGetFurnitureRequest {
    pub map_id: u32,
    pub channel_id: u32,
}
#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x943d)]
pub struct MyRoomGetFurnitureResponse {
    pub result: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xa64a)]
pub struct MyRoomNotifyFurnitue {
    pub furn: FurnitureData,
}
