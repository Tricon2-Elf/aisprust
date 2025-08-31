use crate::{
    packets::{Packet, robo::cls_7991A0},
    shared::chara::{CharaData, CharaParam, CharaVisual, MoveData},
    util::fixed_array::FixedArray,
};
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

use crate::shared::item;

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct NotifyMoveData {
    chara_id: u32,
    move_data: MoveData,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct AvatarData {
    pub avatar_id: u32,
    pub chara: CharaData,
    pub _0x0268: [CharaParam; 8],
    pub _0x03a8: u32,
    pub _0x03ac: u32,
    pub _0x03b0: u8,
    pub _0x03b4: cls_7991A0,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x29a4)]
pub struct AvatarCreateRequest {
    name: String,

    // dafuq is this. its supposed to be model id?. is it overflowing or something
    // on client?
    model_id: u32,
    visual: CharaVisual,
    slot_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x788f)]
pub struct AvatarCreateResponse {
    pub result: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x765a)]
pub struct AvatarDestroyRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x04f6)]
pub struct AvatarGetCreateInfoRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xad9e)]
pub struct AvatarGetDataRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xb055)]
pub struct AvatarGetDataResponse {
    pub result: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x6747)]
pub struct AvatarDataResponse {
    pub result: u32,
    pub name: String,
    pub model_id: u32,
    pub visual: CharaVisual,
    pub reg_islandid: u32,
    pub slot_id: u32,
    pub equips: [item::ItemSlotInfo; 30],
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x7d78)]
pub struct AvatarNotifyData {
    pub result: u32,
    pub avatar_data: AvatarData,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x113d)]
pub struct AvatarSelectRequest {
    slot_id: u32,
}
#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x2c5f)]
pub struct AvatarSelectResponse {
    pub result: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xa5ad)]
pub struct AvatarGetCreateInfoResponse {
    pub m_builds: Vec<u32>, // model ids
    pub m_faces: Vec<u8>,
    pub m_hairstyles: Vec<u32>,
    pub m_haircolors: Vec<u8>,
    pub m_equips: Vec<item::ItemSlotInfo>, //TODO: this is probably 2 u8s

    pub f_builds: Vec<u32>, // model ids
    pub f_faces: Vec<u8>,
    pub f_hairstyles: Vec<u32>,
    pub f_haircolors: Vec<u8>,
    pub f_equips: Vec<item::ItemSlotInfo>, //TODO: this is probably 2 u8s
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x9483)]
pub struct AvatarMove {
    pub result: u32,
    pub moves: [MoveData; 2],
}
