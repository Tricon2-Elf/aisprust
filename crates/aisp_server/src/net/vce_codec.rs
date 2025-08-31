#[repr(u8)]
pub enum VceCodec {
    PacketData(Vec<u8>) = 1,
    Ping = 2,
    Pong = 3,
    Terminated = 4,
    Unknown = 5,
}

impl VceCodec {
    pub fn from_bytes(data: &[u8]) -> Result<(Self, usize), String> {
        if data.is_empty() {
            return Err("data is empty".into());
        }

        let header = data[0];
        let header_type = (header >> 4) & 0xF;
        let header_param = header & 0xF;

        match header_type {
            0 => {
                // read little endian size
                let mut packet_size = data[1] as usize;
                if header_param >= 1 {
                    // packet_size = (data[2] as usize) << (8);
                    packet_size = (packet_size << 8) | (data[2] as usize);
                }
                if header_param >= 2 {
                    // packet_size += (data[3] as usize) << (8 * 2);
                    packet_size = (packet_size << 8) | (data[3] as usize);
                }
                if header_param >= 3 {
                    // packet_size += (data[4] as usize) << (8 * 3);
                    packet_size = (packet_size << 8) | (data[4] as usize);
                }

                let data_start = 2 + header_param as usize;
                let data_end = data_start + packet_size;

                if data_end > data.len() {
                    return Err("data beyond end".into());
                }

                let payload = &data[data_start..data_end];

                Ok((VceCodec::PacketData(payload.into()), data_end))
            }
            1 => {
                todo!("Add ping support");

                Ok((VceCodec::Ping, 9))
            }
            2 => {
                todo!("Add pong support");

                Ok((VceCodec::Pong, 9))
            }
            3 => {
                todo!("Add Terminated support");

                Ok((VceCodec::Terminated, 5))
            }
            4 => {
                todo!("Add unknown support");

                Ok((VceCodec::Unknown, 0))
            }

            _ => {
                panic!("Invalid codec {}. param={}", header_type, header_param);
                Err("invalid codec".into())
            }
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, ()> {
        match self {
            VceCodec::PacketData(data) => {
                // packet
                let payload_size = data.len() as u32;

                let mut buffer = Vec::new();
                buffer.push(0x03); // just do a 4 size header. more bytes the merrier
                buffer.extend_from_slice(&payload_size.to_le_bytes());
                buffer.extend_from_slice(data);

                Ok(buffer)
            }
            VceCodec::Ping => {
                todo!("Add ping support");

                Ok(vec![])
            }
            VceCodec::Pong => {
                todo!("Add pong support");

                Ok(vec![])
            }
            VceCodec::Terminated => {
                todo!("Add Terminated support");

                Ok(vec![])
            }
            VceCodec::Unknown => {
                todo!("Add unknown support");

                Ok(vec![])
            }
        }
    }
}
