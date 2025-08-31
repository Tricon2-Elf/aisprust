use crate::packets::Packet;
use crate::util::fixed_array::FixedArray;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

use crate::shared::item;

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xc8ea)]
pub struct ItemGetBaseListRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xc7a9)]
pub struct ItemGetBaseListResponse {
    pub result: u32,
    pub items: Vec<item::ItemData>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x2a9a)]
pub struct ItemGetListRequest {}

// TODO: this request is probably supposed to give a standard result response, but also send a
// notify response
#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xa522)]
pub struct ItemGetListResponse {
    pub result: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x3768)]
pub struct ItemEquipStartRequest {
    pub obj_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x6448)]
pub struct ItemEquipStartResponse {
    pub result: u32,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct CharaOrder {
    _0x0000: u32,
    _0x0004: FixedArray<193>,
    _0x00c5: u8,
    _0x00c6: u8,
    _0x00c7: u8,
    _0x00c8: u8,
    _0x00cc: [u64; 10], // TODO: this is two u32s, too lazy.
    _0x011c: u32,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct JobOrder {
    _0x0000: u32,
    _0x0004: FixedArray<193>,
    _0x00c8: [u64; 10], // TODO: this is two u32s, too lazy.
    _0x0118: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xf74c)]
pub struct EquipOrderListRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x2dae)]
pub struct EquipOrderListResponse {
    pub result: u32,
    pub chara_order: Vec<CharaOrder>,
    pub job_order: Vec<JobOrder>,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct FurnitureBase {
    _0x0000: u32,
    _0x0004: u32,
    _0x0008: u32,
}
#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x2fda)]
pub struct FurnitureGetBaseListRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xa0d1)]
pub struct FurnitureGetBaseListResponse {
    pub result: u32,
    pub furniture: Vec<FurnitureBase>,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct EmotionBase {
    _0x0000: u32,
    _0x0004: FixedArray<96>,
    _0x0064: u8,
    _0x0065: u8,
    _0x0066: bool,
    _0x0067: bool,
    _0x0068: bool,
    _0x0069: bool,
}
#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x7fcd)]
pub struct EmotionGetBaseListRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x28e3)]
pub struct EmotionGetBaseListResponse {
    pub result: u32,
    pub emotions: Vec<EmotionBase>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xfd42)]
pub struct EmotionGetObtainedListRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xc3d7)]
pub struct EmotionGetObtainedListResponse {
    pub result: u32,
    pub emotion_ids: Vec<u32>,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct UccAdvFigureData {
    _0x0000: u32,
    _0x0004: u32,
    _0x0008: FixedArray<96>,
    _0x0068: bool,
    _0x006c: u32,
    _0x0070: u32,
    _0x0074: u32,
    _0x0078: u32,
    _0x007c: u32,
    _0x0080: u32,
    _0x0084: u32,
    _0x0088: [u32; 30],
    _0x0100: [u32; 30],
    _0x0178: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x86dd)]
pub struct UccAdvFigureBaseListRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x878a)]
pub struct UccAdvFigureBaseListResponse {
    pub result: u32,
    pub adv_figures: Vec<UccAdvFigureData>,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct UccVoiceData {
    _0x0000: u32,
    _0x0004: u32,
    _0x0008: FixedArray<96>,
    _0x0068: bool,
    _0x0069: FixedArray<765>,
    _0x0368: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x1149)]
pub struct UccVoiceBaseListRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xbb8f)]
pub struct UccVoiceBaseListResponse {
    pub result: u32,
    pub voice_data: Vec<UccVoiceData>,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct NiconiCommonsData {
    _0x0000: u32,
    _0x0004: u32,
    _0x0008: u32,
    _0x000c: FixedArray<96>,
    _0x006c: bool,
    _0x0070: u32,
}
#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x97b7)]
pub struct NiconiCommonsBaseListRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xe60c)]
pub struct NiconiCommonsBaseListResponse {
    pub result: u32,
    pub commons_base: Vec<NiconiCommonsData>,
}

// TODO: this request is probably supposed to give a standard result response, but also send a
// notify response
#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x7d29)]
pub struct MissionDataRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x47f9)]
pub struct MissionDataResponse {
    pub result: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x04b4)]
pub struct MapDataEnterEndRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xbe02)]
pub struct MapDataEnterEndResponse {
    pub result: u32,
}
