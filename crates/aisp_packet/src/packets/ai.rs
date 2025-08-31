use crate::packets::Packet;
use crate::util::fixed_array::FixedArray;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct AiDownload {
    pub _0x0000: u64,
    pub _0x0008: u32,
    pub _0x000c: u32,
    pub _0x0010: u8,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x1d3f)]
pub struct AiDownloadListGetRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xbee1)]
pub struct AiDownloadListGetResponse {
    pub result: u32,
    pub downs: Vec<AiDownload>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xe30d)]
pub struct AiUploadRateGetRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xb2bc)]
pub struct AiUploadRateGetResponse {
    pub rate: u32,
}
