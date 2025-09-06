use core::time;

// #[repr(u8)]
pub enum VceCodec {
    PacketData(Vec<u8>),
    Ping(u64),
    Pong(u64),
    Terminated(u32),
    DirectContact,
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
                    packet_size = (packet_size << 8) | (data[2] as usize);
                }
                if header_param >= 2 {
                    packet_size = (packet_size << 8) | (data[3] as usize);
                }
                if header_param >= 3 {
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
                let time = {
                    let mut timebuf = [0u8; size_of::<u64>()];
                    timebuf.copy_from_slice(&data[1..9]);

                    u64::from_le_bytes(timebuf)
                };

                Ok((VceCodec::Ping(time), 9))
            }
            2 => {
                let time = {
                    let mut timebuf = [0u8; size_of::<u64>()];
                    timebuf.copy_from_slice(&data[1..9]);

                    u64::from_le_bytes(timebuf)
                };

                Ok((VceCodec::Pong(time), 9))
            }
            3 => {
                let reason = {
                    let mut buf = [0u8; size_of::<u32>()];
                    buf.copy_from_slice(&data[1..5]);

                    u32::from_le_bytes(buf)
                };

                Ok((VceCodec::Terminated(reason), 5))
            }
            4 => {
                todo!("Add unknown support");

                Ok((VceCodec::DirectContact, 0))
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
            VceCodec::Ping(time) => {
                let mut buffer = Vec::new();
                buffer.push(0x10);
                buffer.extend_from_slice(&time.to_le_bytes());

                Ok(buffer)
            }
            VceCodec::Pong(time) => {
                let mut buffer = Vec::new();
                buffer.push(0x20);
                buffer.extend_from_slice(&time.to_le_bytes());

                Ok(buffer)
            }
            VceCodec::Terminated(reason) => {
                let mut buffer = Vec::new();
                buffer.push(0x30);
                buffer.extend_from_slice(&reason.to_le_bytes());

                Ok(buffer)
            }
            VceCodec::DirectContact => {
                todo!("Add DirectContact support");

                Ok(vec![])
            }
        }
    }
}
