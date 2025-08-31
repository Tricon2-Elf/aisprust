use crate::packets::Packet;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x62bc)]
pub struct VersionCheck {
    pub cl_major: u32,
    pub cl_minor: u32,
    pub cl_ver: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xb6b4)]
pub struct VersionCheckResponse {
    pub result: u32,
    pub sv_major: u32,
    pub sv_minor: u32,
    pub sv_ver: u32,
}
