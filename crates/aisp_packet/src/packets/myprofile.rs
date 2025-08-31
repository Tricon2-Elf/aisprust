use crate::packets::Packet;
use crate::util::fixed_array::FixedArray;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct ProfileData {
    _0x0000: u32,
    _0x0004: u32,
    _0x0008: u32,
    like_things: [FixedArray<31>; 3],
    like_description: [FixedArray<91>; 3],
    avatar_description: FixedArray<901>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xa063)]
pub struct MyProfileAvatarEditRequest {
    pub profile: ProfileData,
    pub job_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x873b)]
pub struct MyProfileAvatarEditResponse {
    pub result: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x3915)]
pub struct MyProfileAvatarGetRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xef6f)]
pub struct MyProfileCloseRequest {}
