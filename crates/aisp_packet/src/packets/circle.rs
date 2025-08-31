use crate::packets::Packet;
use crate::util::fixed_array::FixedArray;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct MemberData {
    _0x0000: u32,
    _0x0004: FixedArray<0x25>,
    _0x002c: u32,
}
#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x05ed)]
pub struct CircleChangeCoreAuthorityRequest {
    circle_id: u64,
    avatar_id: u32,
    authority: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x9514)]
pub struct CircleChatInRequest {
    circle_id: u64,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x05e5)]
pub struct CircleChatOutRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x3d7f)]
pub struct CircleChatPostRequest {
    msg_id: u32,
    message: String,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x1048)]
pub struct CircleCreateRequest {
    name: String,
    mark_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xdb5f)]
pub struct CircleGetDataRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x90ad)]
pub struct CircleGetDataResponse {
    // CProtoMsg_client::recv_get_circle_data_r
    pub result: u32,
    pub circle_data: Vec<CircleData>,
    pub auth_level: Vec<u32>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xd895)]
pub struct CircleMarkChangeRequest {
    circle_id: u64,
    mark_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xd0ef)]
pub struct CircleMarkChangeResponse {
    result: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x1b70)]
pub struct CircleMemberJoinAnswerRequest {
    answer: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x83c1)]
pub struct CircleMemberJoinMemberCancelRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xab2d)]
pub struct CircleMemberJoinMemberRequest {
    avatar_id: u32,
    circle_id: u64,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xdc3a)]
pub struct CircleMemberJoinMemberResponse {
    result: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xbf32)]
pub struct CircleMemberKickRequest {
    circle_id: u64,
    avatar_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x2d2b)]
pub struct CircleMessageChangeRequest {
    circle_id: u64,
    message: String,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xcbfa)]
pub struct CircleNotifyChatIn {
    circle_id: u64,
    avatar_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xbbc4)]
pub struct CircleNotiftyChatOut {
    circle_id: u64,
    avatar_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x7382)]
pub struct CircleResignRequest {
    circle_id: u64,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xbf0e)]
pub struct CircleNotifyMember {
    circle_id: u64,
    member_data: Vec<MemberData>,
    already_login: Vec<bool>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xc097)]
pub struct CircleChangeCoreAuthorityResponse {
    result: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xbb59)]
pub struct CircleLeaderChangeResponse {
    result: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xa9c1)]
pub struct CircleChatPostResponse {
    msg_id: u32,
    result: u32,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct CircleData {
    circle_id: u64,
    _0x0008: FixedArray<0x2e>,
    _0x0038: u32,
    _0x003c: FixedArray<0x25>,
    _0x0061: FixedArray<0x14>,
    _0x0075: FixedArray<0x2ef>,
}

#[derive(Packet, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x9888)]
pub struct CircleNotifyJoinRequest {
    from_avatar_id: u32,
    circle_data: CircleData,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x81c6)]
pub struct CircleChatInResponse {
    result: u32,
    change_flags: u32,
    avatar_ids: Vec<u32>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x8fed)]
pub struct CircleNotifyJoinRequestResult {
    result: u32,
}
