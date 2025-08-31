use crate::packets::Packet;
use crate::shared::chara::{CharaData, CharaParam};
use crate::util::fixed_array::FixedArray;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::shared::item;

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct cls_7991A0 {
    _0x0000: FixedArray<49>,
    _0x0034: u32,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct RoboData {
    robo_id: u32,
    _0x0004: u32,
    _0x0008: u32,
    _0x000c: u32,
    _0x0010: u16,

    chara_data: CharaData,

    _0x0278: [CharaParam; 8],
    _0x03b8: u32,
    _0x03bc: u32,
    _0x03c0: [u32; 5],
    _0x03d4: cls_7991A0,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x44ce)]
pub struct RoboGetListRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xf606)]
pub struct RoboGetListResponse {
    pub result: u32,
    pub robo_datas: Vec<RoboData>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xdcbf)]
pub struct RoboGetObtainedSkillListRequest {
    pub robo_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x1159)]
pub struct RoboGetObtainedSkillListResponse {
    pub result: u32,
    pub robo_id: u32,
    pub skill_id: Vec<u32>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x9305)]
pub struct RoboVoiceTypeUpdateRequest {
    pub voice_type: u8,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x8f10)]
pub struct RoboVoiceTypeUpdateResponse {
    pub result: u32,
    pub voice_type: u8,
}
