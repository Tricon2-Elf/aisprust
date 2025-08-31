use crate::{packets::Packet, util::fixed_array::FixedArray};
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Debug, Deserialize, Serialize)]
pub struct MailData {
    mail_id: u64,
    _0x0008: u32,
    _0x000c: u32,
    _0x0010: u32,
    _0x0014: FixedArray<37>,
    _0x003c: u32,
    _0x0040: FixedArray<37>,
    _0x0065: FixedArray<20>,
    _0x0079: FixedArray<91>,
    _0x00d4: FixedArray<751>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xf96d)]
pub struct MailDeleteRequest {
    mail_id: u64,
    type_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xe501)]
pub struct MailDeleteResponse {
    result: u32,
    mail_id: u64,
    type_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x8d92)]
pub struct MailBoxGetDataRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x147a)]
pub struct MailBoxGetDataResponse {
    pub result: u32,
    pub mail: Vec<MailData>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x1292)]
pub struct MailOpenRequest {
    // is this u64?
    mail_id: u64,
    _0x000a: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xdf76)]
pub struct MailOpenResponse {
    result: u32,
    mail_id: u64,
    type_id: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x34bc)]
pub struct MailPostRequest {
    dist_id: u32,
    dist_name: String,
    subject: String,
    body: String,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xfead)]
pub struct MailProtectCancelRequest {
    mail_id: u64,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x024c)]
pub struct MailProtectRequest {
    mail_id: u64,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xc3e4)]
pub struct MailProtectResponse {
    result: u32,
}
