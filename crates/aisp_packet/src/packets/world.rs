use crate::packets::Packet;
use crate::util::fixed_array::FixedArray;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

use crate::shared::server;

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct WorldEntry {
    pub world_id: u32,
    pub name: FixedArray<97>,
    pub description: FixedArray<766>,
    pub _0x0364: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x6676)]
pub struct WorldListRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xee7e)]
pub struct WorldListResponse {
    pub result: u32,
    pub world_list: Vec<WorldEntry>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x7fe7)]
pub struct WorldSelectRequest {
    pub world_id: u32,
}

#[derive(Packet, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x3491)]
pub struct WorldSelectResponse {
    pub result: u32,
    // server address and ports of CProtoMsg
    pub msgsv_addrs: Vec<server::ServerInfo>,
    pub otp: FixedArray<20>,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct MapLinkData {
    pub _0x0000: f32,
    pub _0x0004: f32,
    pub _0x0008: f32,
    pub _0x000c: u8,
    pub _0x0010: f32,
    pub _0x0014: f32,
}

// TODO: THis is a notify message
#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x30c8)]
pub struct MapLinkGetDataRequest {
    pub map_id: u32,
    pub channel_id: u32,
}
#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x6c4e)]
pub struct MapLinkGetDataResponse {
    pub result: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x5755)]
pub struct MapLinkNotifyData {
    pub result: u32,
    pub maplink_data: MapLinkData,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x2810)]
pub struct MapEnterRequest {
    pub map_id: u32,
    pub channel_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x1dcd)]
pub struct MapEnterResponse {
    pub result: u32,
}
