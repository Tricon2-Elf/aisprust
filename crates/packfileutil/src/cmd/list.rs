use std::fs::File;

use aisp::{
    io::{reader::AiReader, serializable::Serializable},
    packfile::packfile_header::PackFileHeader,
    util::{
        self,
        scramble::{ScrambleImpl::SXor, ScrambleSXOR},
    },
};
use argp::FromArgs;

#[derive(FromArgs, Debug)]
#[argp(subcommand, name = "list", description = "list hed file")]
pub struct Args {
    #[argp(positional, description = "path to .hed file")]
    path: String,

    #[argp(positional, description = "path to .exe file")]
    exe_path: String,
}

pub fn list(args: &Args) {
    let file = File::open(&args.path).expect("failed to open file");
    let mut reader = AiReader::new_le(file);

    util::scramble::calc_scramble_key_crc(&args.exe_path);

    let mut pack = match util::scramble::calc_scramble_key(&args.exe_path) {
        Some(key) => PackFileHeader::with_scramble(SXor(ScrambleSXOR::from_crc(0x67c7))),
        None => PackFileHeader::new(),
        // println!(
        //     "{:?}",
        //     scramble_key
        //         .iter()
        //         .map(|x| format!("{:02x}", x))
        //         .collect::<Vec<String>>()
        //         .join(" ")
        // );
    };

    pack.deserialize(&mut reader).expect("failed to parse");
}
