use crate::packets::Packet;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

use crate::shared::server;

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xdb57)]
pub struct ChannelCheckResponse {
    result: u32,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct ChannelInfo {
    pub channel_id: u32,

    // TODO: this is probably player count or something.
    // _0x0008 has to be over 4
    pub _0x0004: u32,
    pub _0x0008: u32,

    pub server_info: server::ServerInfo,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x23bf)]
pub struct ChannelListGetMapRequest {
    map_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x0300)]
pub struct ChannelListGetRequest {}

#[derive(Packet, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xf27f)]
pub struct ChannelListGetResponse {
    pub result: u32,
    pub channels: Vec<ChannelInfo>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xffe1)]
pub struct ChannelSelectRequest {
    channel_id: u32,
}

#[derive(Packet, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xffea)]
pub struct ChannelSelectResponse {
    pub result: u32,
    pub areasv_info: server::ServerInfo,
    pub map_id: u32,

    // 30000000 is myroom probably
    pub map_serial_id: u32,
}
