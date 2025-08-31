use aisp::io::reader::AiReader;
use aisp::io::reader_scramble::AiScrambleReader;
use aisp::io::serializable::Serializable;
use aisp::packfile::packfile_header::PackFileHeader;
use std::{
    fs::{self, File},
    io::{Read, Write},
};

use crate::cmd::{analyze, dump, dump_all, list};

mod cmd;

use argp::FromArgs;

#[derive(FromArgs, Debug)]
#[argp(subcommand)]
enum CommandType {
    List(list::Args),
    Dump(dump::Args),
    DumpAll(dump_all::Args),
    Analyze(analyze::Args),
}

#[derive(FromArgs, Debug)]
#[argp(description = "top args")]
struct TopLevelArgs {
    #[argp(subcommand)]
    cmd: CommandType,
}

fn main() {
    let args: TopLevelArgs = argp::parse_args_or_exit(argp::DEFAULT);
    println!("{:#?}", args);

    match args.cmd {
        CommandType::List(arg) => list::list(&arg),
        CommandType::Dump(arg) => dump::dump(&arg),
        CommandType::DumpAll(arg) => dump_all::dump_all(&arg),
        CommandType::Analyze(arg) => analyze::analyze_all(&arg),
    };
}
