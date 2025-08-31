use crate::packets::Packet;
use crate::util::fixed_array::FixedArray;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct TagData {
    _0x0000: u32,
    _0x0004: FixedArray<61>,
}

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct FriendData {
    _0x0000: u32,
    _0x0004: FixedArray<37>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x0f97)]
pub struct FriendLinkTagGetRequest {
    avatar_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x239e)]
pub struct FriendLinkTagGetResponse {
    pub result: u32,
    pub avatar_id: u32,
    pub tagdata: Vec<TagData>,
    pub slot: Vec<u32>,
    pub questionnaire_tagdata: Vec<TagData>,
    pub questionnaire_slot: Vec<u32>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x805f)]
pub struct FriendGetListDataRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x2411)]
pub struct FriendGetListDataResponse {
    pub result: u32,
    pub friend_data: Vec<FriendData>,
    pub already_in: Vec<bool>,
    pub comment: Vec<FixedArray<145>>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xc88f)]
pub struct FriendLinkTagGetFreeRequest {}
