use crate::util::fixed_array::FixedArray;

pub enum PartSocket {
    PartHead = 0,
    PartUpperBody1 = 1,
    PartUpperBody2 = 2,
    PartUpperBody3 = 3,
    PartLowerBody1 = 4,
    PartLowerBody2 = 5,
    PartHands = 6,
    PartSocks = 7,
    PartIndoorFootwear = 8,
    PartOutdoorFootwear = 9,
    PartUnderwearTop = 10,
    PartUnderwearBottom = 11,
    PartFace = 12,
    PartHairstyle = 13,
    PartNeckAccessory = 14,
    PartHeadAccessoryR = 15,
    PartHeadAccessoryL = 16,
    PartEarAccessoryR = 17,
    PartEarAccessoryL = 18,
    PartHandheldR = 19,
    PartHandheldL = 20,
    PartWristAccessoryR = 21,
    PartWristAccessoryL = 22,
    PartArmAccessoryR = 23,
    PartArmAccessoryL = 24,
    PartWaistAccessoryR = 25,
    PartWaistAccessoryL = 26,
    PartBack = 27,
    PartHipAccessory = 28,
}

#[derive(PartialEq, Debug, Default, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct ItemSlotInfo {
    pub id: u32,
    pub socket: u32,
}

#[derive(PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct ItemData {
    pub key: u32,                  // used as key in item manager
    pub sorted_list_priority: u32, // used as priority in sorted list

    pub item_id: u32,  // used for item in data
    pub skill_id: u32, // used in skill manager

    // #[serde(with = "BigArray")]
    pub name: FixedArray<97>,

    // seems like rest is item types, 20 is skill?
    // [0-11] = ?
    // [12-14] = ?
    // [15-16] = ?
    // [17] = ?
    // [20] = skill
    pub category: u32,

    pub socket_1: u32, // PartSocket
    pub socket_2: u32, // PartSocket

    // #[serde(with = "BigArray")]
    pub description: FixedArray<769>,
    // #[serde(with = "BigArray")]
    pub limit_desc: FixedArray<193>,

    // flags
    // 0x2 = non tradable
    pub flags: u32,
    pub _0x0448: u16,
    pub _0x044c: u32, // used as key in some map. might only be used for category [0-11]
    pub _0x0450: u32,
    pub emotion_id: u32,
    pub _0x0458: u32,
}

impl ItemData {
    pub fn from_id_name(id: u32, name: String) -> Self {
        Self {
            key: id,
            sorted_list_priority: id,
            item_id: id,
            name: FixedArray::<97>::from_str(&name),
            ..Default::default()
        }
    }
}

impl Default for ItemData {
    fn default() -> Self {
        Self {
            key: 0,
            sorted_list_priority: 0,
            item_id: 0,
            skill_id: 0,
            name: FixedArray::<97>::from_str("N/A"),
            category: 0,
            socket_1: 0,
            socket_2: 0,
            description: FixedArray::<769>::from_str("N/A"),
            limit_desc: FixedArray::<193>::from_str("N/A"),
            flags: 0,
            _0x0448: 0,
            _0x044c: 0,
            _0x0450: 0,
            emotion_id: 0,
            _0x0458: 0,
        }
    }
}
