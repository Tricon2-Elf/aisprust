use crate::packets::Packet;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xF24B)]
pub struct AuthenticateRequest {
    pub login_id: String,
    pub login_pw: String,
    pub adcode: String,
    pub exe_type: u8,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xd845)]
pub struct AuthenticateResponseFalure {
    pub result: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xd4ab)]
pub struct AuthenticateResponseDevelop {
    pub user_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x2d66)]
pub struct NotifyLogout {
    pub reason: u32,
}

// #[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
// #[packet(0x6676)]
// pub struct WorldlistRequest {}
//
// #[derive(PartialEq, Debug, Deserialize, Serialize)]
// pub struct WorldEntry {
//     pub world_id: u32,
//     pub name: FixedArray<97>,
//     pub description: FixedArray<766>,
//     pub _0x0364: u32,
// }
//
// #[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
// #[packet(0xee7e)]
// pub struct WorldlistResponse {
//     pub result: u32,
//     pub world_list: Vec<WorldEntry>,
// }
//
// #[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
// #[packet(0x7fe7)]
// pub struct SelectWorldRequest {
//     pub world_id: u32,
// }
//
// #[derive(PartialEq, Debug, Deserialize, Serialize)]
// pub struct MessageServerInfo {
//     pub port: u16,
//
//     pub address: FixedArray<65>,
// }
//
// #[derive(Packet, PartialEq, Debug, Deserialize, Serialize)]
// #[packet(0x3491)]
// pub struct SelectWorldResponse {
//     pub result: u32,
//     // server address and ports of CProtoMsg
//     pub msgsv_addrs: Vec<MessageServerInfo>,
//     #[serde(with = "BigArray")]
//     pub otp: [u8; 20],
// }
