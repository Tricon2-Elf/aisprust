use std::fs::File;

use aisp::{
    io::{reader::AiReader, serializable::Serializable},
    packfile::packfile_header::PackFileHeader,
};
use argp::FromArgs;

#[derive(FromArgs, Debug)]
#[argp(subcommand, name = "dump", description = "dump hed file")]
pub struct Args {
    #[argp(positional, description = "path to .hed file")]
    path: String,
}

pub fn dump(args: &Args) {
    let file = File::open(&args.path).expect("failed to open file");
    let mut reader = AiReader::new_le(file);

    let mut pack = PackFileHeader::new();
    pack.deserialize(&mut reader).expect("failed to parse");
}
