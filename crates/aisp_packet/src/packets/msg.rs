use crate::util::fixed_array::FixedArray;
use crate::{packets::Packet, shared::item};

use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct Parameter {
    value: FixedArray<181>,
}

#[derive(Packet, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x2e64)]
pub struct CmdExecRequest {
    _0x0000: u32,

    // command string?
    _0x0004: FixedArray<97>,

    // probably an array of arguments
    #[serde(with = "BigArray")]
    _0x0064: [Parameter; 11],

    _0x0f64: u32,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct CommentInfo {
    _0x0000: u16,
    _0x0004: u32,
    _0x0008: u32,
    _0x000c: u32,
    _0x0010: u32,
    _0x0014: u32,

    _0x0018: FixedArray<0x181>,

    _0x019c: f32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xd19a)]
pub struct LiveContestNotifyCommentDelete {
    comment_id: u32,
}

#[derive(Packet, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xec7f)]
pub struct LiveContestNotifyCommentForward {
    comment: CommentInfo,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x34ef)]
pub struct LoginRequest {
    user_id: u32,
    otp: FixedArray<20>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x0ad0)]
pub struct LogoutRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xa50e)]
pub struct PlackardGetCommentLogRequest {
    plackard_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xeb2e)]
pub struct PostTalkRequest {
    msg_id: u32,
    dist_id: u32,
    message: String,
    balloon_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xb7b9)]
pub struct LogoutResponse {
    pub result: u32,
}

// #[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
// #[packet(0x90ad)]
// pub struct CircleGetDataResponse {
//     // CProtoMsg_client::recv_get_circle_data_r
//     result: u32,
//     circle_data: Vec<CircleData>,
//     auth_level: Vec<u32>,
// }

//TODO: between

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x1fea)]
pub struct LoginResponse {
    // CProtoMsg_client::recv_login_r
    pub result: u32,
}
