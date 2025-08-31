use crate::packets::Packet;
use crate::util::fixed_array::FixedArray;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::shared::chara::{CharaData, CharaParam, CharaVisual};
use crate::shared::item;

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct NpcData {
    pub _0x0000: u32,
    pub chara_data: CharaData,
    pub _0x0268: u8,
}

// TODO: this probably sends a notify mesasge
#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x461b)]
pub struct NpcGetDataRequest {
    pub map_id: u32,
    pub channel_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x4403)]
pub struct NpcGetDataResponse {
    pub result: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xcd67)]
pub struct NpcNotifyData {
    pub result: u32,
    pub npc_data: NpcData,
}
