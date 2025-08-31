use crate::{shared::item, util::fixed_array::FixedArray};
use serde::{Deserialize, Serialize};

use serde_big_array::BigArray;

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct CharaVisual {
    pub blood_type: u32,
    pub month: u8,
    pub day: u8,
    pub gender: u32,
    pub chara_id: u32,
    pub face: u8,
    pub hairstyle: u32,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct cls_798720 {
    _0x0000: f32,
    _0x0004: f32,
    _0x0008: f32,
    _0x000c: f32,
    move_data: MoveData,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct cls_798B80 {
    _0x0000: u32,
    _0x0004: u32,
    _0x0008: u32,
    _0x000c: u32,
    _0x0010: u8,
    _0x0011: u8,
}
#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct cls_798C00 {
    _0x0000: u32,
    _0x0004: u32,
    _0x0008: u32,
    _0x000c: u32,
}
#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct cls_79A180 {
    _0x0000: u32,
    _0x0004: u32,
    _0x0008: u32,
    _0x000c: u32,
}
#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct cls_798C60 {
    _0x0000: [u32; 5],
    _0x0014: [u32; 5],
    _0x0028: [u32; 5],
    _0x003c: [u32; 5],
}
#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct cls_798D10 {
    _0x0000: u64,
    _0x0008: u32,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct cls_798B10 {
    _0x0000: u8,
    _0x0008: u64,
    _0x0010: u64,
    _0x0018: u64,
}
#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct cls_798D50 {
    _0x0000: u32,
    _0x0008: cls_798B10,
}
#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct cls_798D80 {
    _0x0000: cls_798B80,
    _0x0014: cls_798C00,
    _0x0024: cls_79A180,
    _0x0034: cls_798C60,
    _0x0088: cls_798D10,
    _0x0098: u32,
    _0x00a0: cls_798D50,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct CharaData {
    pub chara_id: u32, // slot id
    pub _0x0004: u32,  // character id
    pub name: FixedArray<37>,
    pub visual: CharaVisual,
    pub _0x0048: u32, // pointer??
    pub _0x004c: cls_798720,
    pub _0x006c: [f32; 2],
    #[serde(with = "BigArray")]
    pub equipment: [item::ItemSlotInfo; 30],
    pub _0x0164: u32,
    pub _0x0168: u32,
    pub _0x016c: u32,
    pub _0x0170: [f32; 2],
    pub _0x0178: cls_798D80,
    pub _0x0240: cls_798B10,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct CharaParam {
    _0x0000: u32,
    _0x0004: u32,
    _0x0008: u32,
    param_type: u32,
    params_values: [u32; 5],
    _0x0024: u8,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct MoveData {
    pub position: [f32; 3],
    pub yaw: i8,
    pub _0x000d: u8,
}
