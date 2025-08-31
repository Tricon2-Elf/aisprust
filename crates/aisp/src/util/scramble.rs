use std::fs;

pub trait Scramble {
    fn unscramble_base(&self, data: &mut [u8], base_offset: usize);
    fn scramble_base(&self, data: &mut [u8], base_offset: usize);

    fn unscramble(&self, data: &mut [u8]) {
        self.unscramble_base(data, 0);
    }

    fn scramble(&self, data: &mut [u8]) {
        self.scramble_base(data, 0);
    }
}

#[derive(Clone)]
pub enum ScrambleImpl {
    AddKey(ScrambleAddKey),
    SXor(ScrambleSXOR),
}

#[derive(Clone)]
pub struct ScrambleAddKey {
    key: Vec<u8>,
}

#[derive(Clone)]
pub struct ScrambleSXOR {
    initial: u8,
    add: u8,
}

impl ScrambleImpl {
    pub fn from_crc(crc: u16) -> Self {
        Self::SXor(ScrambleSXOR::from_crc(crc))
    }
}

impl ScrambleAddKey {
    pub fn new() -> Self {
        Self { key: Vec::new() }
    }

    pub fn from_key(key: &[u8]) -> Self {
        Self { key: key.to_vec() }
    }
}

impl ScrambleSXOR {
    pub fn new() -> Self {
        Self {
            initial: 0x00,
            add: 0x00,
        }
    }

    pub fn from_key(initial: u8, add: u8) -> Self {
        Self { initial, add }
    }

    pub fn from_crc(crc: u16) -> Self {
        Self {
            initial: (crc >> 8) as u8,
            add: crc as u8,
        }
    }
}

impl Default for ScrambleAddKey {
    fn default() -> Self {
        Self::new()
    }
}
impl Default for ScrambleSXOR {
    fn default() -> Self {
        Self::new()
    }
}

impl Scramble for ScrambleImpl {
    fn scramble_base(&self, data: &mut [u8], base_offset: usize) {
        match self {
            ScrambleImpl::AddKey(k) => k.scramble_base(data, base_offset),
            ScrambleImpl::SXor(k) => k.scramble_base(data, base_offset),
        }
    }

    fn unscramble_base(&self, data: &mut [u8], base_offset: usize) {
        match self {
            ScrambleImpl::AddKey(k) => k.unscramble_base(data, base_offset),
            ScrambleImpl::SXor(k) => k.unscramble_base(data, base_offset),
        }
    }
}

impl Scramble for ScrambleAddKey {
    fn unscramble_base(&self, data: &mut [u8], base_offset: usize) {
        if self.key.is_empty() {
            return;
        }
        for i in 0..data.len() {
            let idx = ((base_offset as u64) + (i as u64)) % (self.key.len() as u64);
            data[i] = data[i].wrapping_sub(self.key[idx as usize]);
        }
    }
    fn scramble_base(&self, data: &mut [u8], base_offset: usize) {
        if self.key.is_empty() {
            return;
        }

        for i in 0..data.len() {
            let idx = ((base_offset as u64) + (i as u64)) % (self.key.len() as u64);
            data[i] = data[i].wrapping_add(self.key[idx as usize]);
        }
    }
}

impl Scramble for ScrambleSXOR {
    fn unscramble_base(&self, data: &mut [u8], base_offset: usize) {
        let mut last_byte: u8 = self.initial;

        // TODO: this does not work with streams
        for i in 0..data.len() {
            let cur_byte = data[i];

            let a1 = last_byte.wrapping_add((i + base_offset) as u8);
            let byte = (cur_byte ^ a1).wrapping_sub(self.add);
            data[i] = byte;
            last_byte = cur_byte;
        }
    }
    fn scramble_base(&self, data: &mut [u8], base_offset: usize) {
        let mut last_byte: u8 = self.initial;

        // TODO: this does not work with streams, cause of the initial thing
        for i in 0..data.len() {
            let a1 = last_byte.wrapping_add((i + base_offset) as u8);
            let byte = data[i].wrapping_add(self.add) ^ a1;
            data[i] = byte;
            last_byte = byte;
        }
        // todo!("todoa");
        // for i in 0..data.len() {
        //     let idx = ((base_offset as u64) + (i as u64)) % (self.key.len() as u64);
        //     data[i] = data[i].wrapping_add(self.key[idx as usize]);
        // }
    }
}

pub fn unscramble(in_data: &[u8], scramble_key: &[u8], base_offset: usize) -> Vec<u8> {
    let mut data = in_data.to_vec();

    for i in 0..data.len() {
        let idx = (base_offset as u64 + (i as u64)) % (scramble_key.len() as u64);
        data[i] = data[i].wrapping_sub(scramble_key[idx as usize]);
    }

    data
}

pub fn scramble(in_data: &[u8], scramble_key: &[u8], base_offset: usize) -> Vec<u8> {
    let mut data = in_data.to_vec();

    for i in 0..data.len() {
        let idx = (base_offset as u64 + (i as u64)) % (scramble_key.len() as u64);
        data[i] = data[i].wrapping_add(scramble_key[idx as usize]);
    }

    data
}

pub fn calc_scramble_key(exe_path: &String) -> Option<Vec<u8>> {
    use pe_parser::pe::parse_portable_executable;
    use pe_parser::section::SectionFlags;

    let pe_data = match fs::read(exe_path) {
        Ok(data) => data,
        Err(_) => return None,
    };

    let pe_info = match parse_portable_executable(&pe_data) {
        Ok(pe) => pe,
        Err(_) => return None,
    };

    // println!("{}", pe_info);

    let last_code_section = pe_info.section_table.iter().rev().find(|section| {
        if let Some(section_chara) = section.get_characteristics() {
            return section_chara.intersects(SectionFlags::IMAGE_SCN_CNT_CODE);
        }
        false
    });

    match last_code_section {
        Some(section) => {
            let section_data: &[u8] = &pe_data[section.pointer_to_raw_data as usize
                ..section.pointer_to_raw_data as usize + section.size_of_raw_data as usize];

            let mut scramble_key = vec![0u8; 0x40];

            for (i, b) in section_data.iter().enumerate() {
                let scramble_idx = i % scramble_key.len();
                let value = scramble_key[scramble_idx]
                    .overflowing_add(*b)
                    .0
                    .overflowing_add(i as u8)
                    .0;
                scramble_key[scramble_idx] = value;
                // scramble_key[scramble_idx] = (scramble_key[scramble_idx] + b + i as u8) & 0xFF;
            }

            Some(scramble_key)
        }
        None => None,
    }
}

pub fn calc_scramble_key_crc(exe_path: &String) -> Option<u16> {
    let pe_data = match fs::read(exe_path) {
        Ok(data) => data,
        Err(_) => return None,
    };

    use crc;

    let crc_ctx: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_MCRF4XX);
    let checksum = crc_ctx.checksum(&pe_data);

    Some(checksum)
}
