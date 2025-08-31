use crate::packets::Packet;
use crate::util::fixed_array::FixedArray;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x4646)]
pub struct AreasvEnterRequest {
    user_id: u32,
    otp: FixedArray<20>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x0149)]
pub struct AreasvEnterResponse {
    pub result: u32,
    pub obj_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xF7B9)]
pub struct AreasvLeaveRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xe31d)]
pub struct AreasvLeaveResponse {
    pub result: u32,
}
