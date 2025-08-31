use crate::{
    io::{
        reader::EndianReader,
        reader_limited::LimitedReader,
        reader_scramble::AiScrambleReader,
        serializable::Serializable,
        writer::{AiWriter, EndianWriter},
    },
    util::scramble::{ScrambleAddKey, ScrambleImpl},
};
// use chrono::{TimeZone, Utc};
use std::io::{Cursor, Error, ErrorKind, Result, Seek, Write};

// for ai sp@ce
const SCRAMBLE_KEY: [u8; 0x40] = [
    0x77, 0x95, 0x94, 0xcc, 0xb1, 0x18, 0x42, 0x19, 0x0c, 0xc2, 0x73, 0x3a, 0xca, 0x0b, 0x03, 0x68,
    0x53, 0xa5, 0x89, 0xd3, 0x1b, 0x25, 0x01, 0x41, 0x4e, 0xfb, 0x83, 0x3d, 0xfc, 0xbf, 0x65, 0x3c,
    0xe6, 0x3b, 0xce, 0xac, 0x30, 0x38, 0x1e, 0xa2, 0x57, 0x67, 0xdc, 0x02, 0x62, 0xc6, 0x5e, 0x0f,
    0xbd, 0x5a, 0x94, 0x2d, 0xf4, 0xdd, 0x08, 0x95, 0x87, 0x49, 0x38, 0x8c, 0x86, 0xcd, 0xa0, 0x6a,
];

// enum PackFileType {
//     AiSpace,
//     WizardryOnline,
//     PandoraSaga,
// }

#[derive(Debug)]
pub struct PackFileEntry {
    pub folder_path: String,
    pub file_name: String,

    pub file_id: u32,
    pub offset: u32,
    pub size: u32,
    pub file_time: i64,
}

// #[derive(Debug)]
pub struct PackFileHeader {
    pub ver_maj: u16,
    pub ver_min: u16,
    pub base_path: String,

    pub data_scramble_key: Vec<u8>,
    pub files: Vec<PackFileEntry>,

    header_scramble_key: Option<ScrambleImpl>, // used for header, if not SCRAMBLE_KEY is used
}

impl PackFileHeader {
    pub fn new() -> Self {
        Self {
            ver_maj: 0,
            ver_min: 0,

            base_path: String::new(),

            data_scramble_key: Vec::new(),
            files: Vec::new(),

            header_scramble_key: None,
        }
    }

    pub fn with_scramble(header_scramble_key: ScrambleImpl) -> Self {
        Self {
            ver_maj: 0,
            ver_min: 0,

            base_path: String::new(),

            data_scramble_key: Vec::new(),
            files: Vec::new(),

            header_scramble_key: Some(header_scramble_key),
        }
    }

    fn parse_section_1<R: EndianReader>(&mut self, reader: &mut R) -> Result<()> {
        self.base_path = reader.read_ai_string_unicode()?;

        // self.base_path = self.base_path.replace("%04x", "{:04x}");
        //
        // if self.base_path.contains("%") {
        //     panic!("base path [{}] contains format specifier", self.base_path);
        // }

        let _unk_int_1 = reader.read_u32()?;

        if self.ver_maj >= 1 && self.ver_min >= 1 {
            // 1.1
            let _unk_int_2 = reader.read_u32()?;
        }

        // println!("Base path [{}]", self.base_path);

        Ok(())
    }

    fn parse_section_2<R: EndianReader>(&mut self, reader: &mut R) -> Result<()> {
        let scramble_len = reader.read_u32()?;

        self.data_scramble_key = vec![0u8; scramble_len as usize];
        reader.read(&mut self.data_scramble_key)?;

        Ok(())
    }

    fn parse_section_3<R: EndianReader>(&mut self, reader: &mut R) -> Result<()> {
        let count = reader.read_u32()?;

        for _ in 0..count {
            let folder_path = reader.read_ai_string_unicode()?;
            let file_name = reader.read_ai_string_unicode()?;

            let dat_file_id = reader.read_u32()?;
            let dat_file_offset = reader.read_u32()?;
            let data_size = reader.read_u32()?;
            let file_time = reader.read_i64()?;

            // let date_time = Utc.timestamp_opt(file_time as i64, 0).unwrap();

            // println!(
            //     "{} {} {:#x} {:#x} {:#x} {}",
            //     folder_path, file_name, dat_file_id, dat_file_offset, data_size, file_time
            // );

            let entry = PackFileEntry {
                folder_path,
                file_name,

                file_id: dat_file_id,
                offset: dat_file_offset,
                size: data_size,
                file_time,
            };
            self.files.push(entry);
        }
        Ok(())
    }
}

impl Default for PackFileHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Serializable for PackFileHeader {
    // fn deserialize(&mut self, reader: &mut dyn EndianReader) -> Result<()> {
    fn deserialize<R: EndianReader>(&mut self, reader: &mut R) -> Result<()> {
        let mut magic = [0u8; 0x4];
        reader.read_exact(&mut magic)?;

        if &magic != b"FPMF" {
            return Err(Error::new(ErrorKind::InvalidData, "Bad magic"));
        }

        self.ver_maj = reader.read_u16()?;
        self.ver_min = reader.read_u16()?;

        let data_size = reader.read_u32()?;
        let _data_pack_type = reader.read_u32()?;

        // println!("{} {}", data_size, data_pack_type);

        let scramble_key = match &self.header_scramble_key {
            Some(key) => key.clone(),
            None => ScrambleImpl::AddKey(ScrambleAddKey::from_key(SCRAMBLE_KEY.as_ref())),
        };

        let mut inner_reader =
            AiScrambleReader::new(LimitedReader::new(reader, data_size as usize), scramble_key);

        // let mut inner_reader_scr =
        //     AiScrambleReader::new(LimitedReader::new(reader, data_size as usize), scramble_key);
        //
        // let mut inner_data = vec![0u8; data_size as usize];
        // inner_reader_scr.read_exact(&mut inner_data);
        //
        // std::fs::File::create("/home/txt/Documents/RE/aispace/test/testwizard.bin")
        //     .expect("")
        //     .write_all(&inner_data);
        //
        // let mut inner_reader = AiReader::new_le(Cursor::new(inner_data));
        //
        // while inner_reader.stream_position()? < (data_size as u64) {
        for _ in 0..3 {
            let section_type = inner_reader.read_u32()?;
            let section_size = inner_reader.read_u32()?;

            let section_end = inner_reader.stream_position()? + (section_size as u64);

            let mut section_reader = LimitedReader::new(&mut inner_reader, section_size as usize);

            // println!("Section [{}] size [{:#x}]", section_type, section_size);

            match section_type {
                1 => self.parse_section_1(&mut section_reader),
                2 => self.parse_section_2(&mut section_reader),
                3 => self.parse_section_3(&mut section_reader),
                _ => Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid section {}", section_type),
                )),
            }?;

            if inner_reader.stream_position()? > section_end {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Read beyond end of section",
                ));
            }

            if inner_reader.stream_position()? < section_end {
                inner_reader
                    .seek(std::io::SeekFrom::Start(section_end))
                    .expect("Failed to seek to next section");
            }
        }

        Ok(())
    }

    // fn serialize(&self, reader: &mut dyn EndianReader) -> Result<()> {
    fn serialize<R: EndianWriter>(&self, writer: &mut R) -> Result<()> {
        let mut inner_buffer = Cursor::new(Vec::<u8>::new());
        let mut inner_writer = AiWriter::new_le(&mut inner_buffer);

        {
            let mut section_1_buffer = Cursor::new(Vec::<u8>::new());
            let mut section_1_writer = AiWriter::new_le(&mut section_1_buffer);

            // let base_path = self.base_path.replace("{:04x}", "%04x");
            //
            // if base_path.contains("{") {
            //     panic!("Base path [{}] contains rust fmt", base_path);
            // }

            // section_1_writer.write_ai_string_unicode(&base_path)?;
            section_1_writer.write_ai_string_unicode(&self.base_path)?;

            section_1_writer.write_u32(0)?;

            if self.ver_maj >= 1 && self.ver_min >= 1 {
                // 1.1
                section_1_writer.write_u32(0x40000)?;
            }

            inner_writer.write_u32(1)?; // type
            inner_writer.write_u32(section_1_buffer.position() as u32)?;
            inner_writer.write_all(section_1_buffer.get_ref())?;
        }
        {
            let mut section_2_buffer = Cursor::new(Vec::<u8>::new());
            let mut section_2_writer = AiWriter::new_le(&mut section_2_buffer);

            section_2_writer.write_u32(self.data_scramble_key.len() as u32)?;
            section_2_writer.write_all(&self.data_scramble_key)?;

            inner_writer.write_u32(2)?; // type
            inner_writer.write_u32(section_2_buffer.position() as u32)?;
            inner_writer.write_all(section_2_buffer.get_ref())?;
        }
        {
            let mut section_3_buffer = Cursor::new(Vec::<u8>::new());
            let mut section_3_writer = AiWriter::new_le(&mut section_3_buffer);

            section_3_writer.write_u32(self.files.len() as u32)?;

            for file in self.files.iter() {
                section_3_writer.write_ai_string_unicode(&file.folder_path)?;
                section_3_writer.write_ai_string_unicode(&file.file_name)?;

                section_3_writer.write_u32(file.file_id)?;
                section_3_writer.write_u32(file.offset)?;
                section_3_writer.write_u32(file.size)?;
                section_3_writer.write_i64(file.file_time)?;
            }

            inner_writer.write_u32(3)?; // type
            inner_writer.write_u32(section_3_buffer.position() as u32)?;
            inner_writer.write_all(section_3_buffer.get_ref())?;
        }

        writer.write_all(b"FPMF")?;
        writer.write_u16(self.ver_maj)?;
        writer.write_u16(self.ver_min)?;

        writer.write_u32(inner_buffer.position() as u32)?;
        writer.write_u32(3)?;
        writer.write_all(inner_buffer.get_ref())?;

        Ok(())
    }
}
