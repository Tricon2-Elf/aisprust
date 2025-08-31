use crate::packets::Packet;
use crate::shared::chara::{CharaData, CharaParam, CharaVisual};
use crate::shared::item;
use crate::util::fixed_array::FixedArray;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct cls_79ADA0 {
    _0x0000: u32,
    _0x0004: FixedArray<49>,
    _0x0035: FixedArray<49>,
    _0x0066: FixedArray<97>,
    _0x00c7: FixedArray<601>,
    _0x0320: u32,
    chara_visual: CharaVisual,
    #[serde(with = "BigArray")]
    pub equip: [item::ItemSlotInfo; 30],
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct HeroineTicketBase {
    _0x0000: u32,
    _0x0004: u32,
    _0x0008: FixedArray<193>,
    _0x00c9: FixedArray<37>,
    _0x00ee: FixedArray<601>,
    _0x0348: [cls_79ADA0; 12],
    _0x3558: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x25ce)]
pub struct HeroineGetTicketBaseRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x16e6)]
pub struct HeroineGetTicketBaseResponse {
    pub heroine_tickets: Vec<HeroineTicketBase>,
}
