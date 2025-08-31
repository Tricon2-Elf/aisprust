use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

use aisp::{
    io::{reader::AiReader, reader_scramble::AiScrambleReader, serializable::Serializable},
    packfile::packfile_header::PackFileHeader,
    util::scramble::{ScrambleAddKey, ScrambleImpl},
};
use argp::FromArgs;
use walkdir::WalkDir;

#[derive(FromArgs, Debug)]
#[argp(
    subcommand,
    name = "dump_all",
    description = "dump all hed files in dir"
)]
pub struct Args {
    #[argp(positional, description = "base path to data dir")]
    base_path: String,

    #[argp(positional, description = "output directory")]
    output_dir: String,
}

pub fn dump_all(args: &Args) {
    // let mut data_path = PathBuf::from(&args.base_path);

    // let file = File::open(&args.path).expect("failed to open file");
    // let mut reader = AiReader::new_le(file);
    //
    // let mut pack = PackFileHeader::new();
    // pack.deserialize(&mut reader).expect("failed to parse");

    let data_dir = PathBuf::from(&args.base_path);
    let out_dir = PathBuf::from(&args.output_dir);

    let head_iterator = WalkDir::new(&args.base_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_type().is_file() && e.path().extension().is_some_and(|ext| ext == "hed")
        });

    // head_iterator.par_iter().for_each(|file| {
    //
    for header_file in head_iterator {
        let file_path = header_file.path().to_str().expect("file");
        println!("\n{}", file_path);

        let mut reader = AiReader::new_le(File::open(file_path).expect("failed to open file"));

        let mut pack = PackFileHeader::new();
        match pack.deserialize(&mut reader) {
            Ok(_) => (),
            Err(_) => {
                println!("Failed to deserialize");
                continue;
            }
        };

        for file_entry in &pack.files {
            let out_file_path = out_dir
                .join(file_entry.folder_path.replace("\\", "/"))
                .join(file_entry.file_name.replace("\\", "/"));

            let data_file_offset = match fmt_c::format(
                &pack.base_path,
                vec![fmt_c::FormatArg::Uint(file_entry.file_id as u64)],
            ) {
                Err(_) => continue,
                Ok(val) => val,
            };

            let data_path = data_dir.join(&data_file_offset);

            let mut raw_data_reader = AiReader::new_le(match File::open(&data_path) {
                Ok(fd) => fd,
                Err(e) => {
                    println!(
                        "Failed to open file [{}]",
                        data_path
                            .to_str()
                            .expect("failed to convert path to string?????")
                    );
                    continue;
                }
            });
            if let Err(e) = raw_data_reader.seek(SeekFrom::Start(file_entry.offset as u64)) {
                println!("Failed to seek file [{}] {}", file_entry.file_name, e);
                continue;
            }

            let mut scramble_reader = AiScrambleReader::new(
                raw_data_reader,
                ScrambleImpl::AddKey(ScrambleAddKey::from_key(&pack.data_scramble_key)),
            );

            let mut data = vec![0u8; file_entry.size as usize];
            scramble_reader
                .read_exact(&mut data)
                .expect("failed to read scramble");

            std::fs::create_dir_all(out_file_path.parent().unwrap()).unwrap();

            let mut out_file = match File::create(&out_file_path) {
                Ok(fd) => fd,
                Err(e) => {
                    println!(
                        "Failed to open output file [{}]",
                        out_file_path
                            .to_str()
                            .expect("failed to convert path to string 2??????")
                    );
                    continue;
                }
            };

            if let Err(e) = out_file.write_all(&data) {
                println!("Failed to write file [{}] {}", out_file_path.to_str().expect("Faield to convert path to string"), e);
            }
        }
    }
}
