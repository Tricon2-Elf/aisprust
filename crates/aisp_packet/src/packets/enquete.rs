use crate::packets::Packet;
use crate::util::fixed_array::FixedArray;
use aisp_packet_macros::Packet;
use serde::{Deserialize, Serialize};

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x0352)]
pub struct EnqueteAnswerRequest {
    enquete_ids: Vec<u32>,
    answer_idxs: Vec<u32>,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x615a)]
pub struct EnqueteAnswerResponse {
    pub result: u32,
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0xc578)]
pub struct EnqueteGetRequest {}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct EnqueteData {
    pub enquet_id: u32,

    pub question: FixedArray<181>,

    // #[serde(with = "BigArray")]
    pub answers: [FixedArray<61>; 10],
}

#[derive(Packet, Default, PartialEq, Debug, Deserialize, Serialize)]
#[packet(0x24ee)]
pub struct EnqueteGetResponse {
    pub result: u32,
    pub enquetes: Vec<EnqueteData>,
}
