use crate::packets::Packet;
use crate::util::fixed_array::FixedArray;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xf582)]
pub struct QuestWorkGetRequest {}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x4bed)]
pub struct QuestHistoryGetRequest {}
