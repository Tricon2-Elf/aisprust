use crate::packets::Packet;
use crate::util::fixed_array::FixedArray;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

#[repr(u32)]
enum TimeZone {
    Morning = 0,
    Daytime = 1,
    Evening = 2,
    Night = 3,
    EarlyMorning = 4,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x5f53)]
pub struct TimeZoneGetRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xcd38)]
pub struct TimeZoneGetResponse {
    pub result: u32,
    pub time_zone: u32, // 1 = daytime or evening, 2 = night or evening
    pub time: u32,
    pub time_zone_max: u32,
    pub flag: u8,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x61e7)]
pub struct MoneyDataGetRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xdc19)]
pub struct MoneyDataGetResponse {
    pub result: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xbf17)]
pub struct MoneyNpsPointsRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x3cf5)]
pub struct MoneyNpsPointsResponse {
    pub result: u32,
    pub total: u64,
    pub limit: u64,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x79a1)]
pub struct UpdateOptionRequest {
    pub flag: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xb314)]
pub struct UpdateOptionResponse {
    pub result: u32,
}
