use std::collections::HashMap;
use std::io::SeekFrom;
use std::path::PathBuf;
use std::{fs::File, io::Seek};
use std::{
    fs::{self},
    io::{Read, Write},
};

use aisp::io::endianness::EndianAware;
use aisp::packfile::packfile::find_base_dir;
use aisp::packfile::packfile_header::PackFileEntry;
use aisp::util::scramble::{self, scramble};
use aisp::{
    io::{reader::AiReader, reader_scramble::AiScrambleReader, serializable::Serializable},
    packfile::packfile_header::PackFileHeader,
};
use argp::FromArgs;
use walkdir::WalkDir;

#[derive(FromArgs, Debug)]
#[argp(subcommand, name = "analyze", description = "anaylze files")]
pub struct Args {
    #[argp(positional, description = "path to .hed file")]
    path: String,
    //
    // #[argp(positional, description = "data file idx")]
    // file_idx: u32,
}

fn analyze_single(args: &Args) {
    let file = File::open(&args.path).expect("failed to open file");
    let mut reader = AiReader::new_le(file);

    let mut pack = PackFileHeader::new();
    pack.deserialize(&mut reader).expect("failed to parse");

    if pack.files.is_empty() {
        println!("Packfile has no files!");
        return;
    }

    let data_dir_path = find_base_dir(&pack, &args.path);

    let mut fileid_entries: HashMap<u32, Vec<&PackFileEntry>> = HashMap::new();

    for entry in &pack.files {
        fileid_entries.entry(entry.file_id).or_default().push(entry);
    }

    let holes = find_holes(&pack, &fileid_entries, &data_dir_path);

    detect_file_sizes(&pack, &holes, &data_dir_path);
}
pub fn analyze_all(args: &Args) {
    let data_dir = PathBuf::from(&args.path);

    let head_iterator = WalkDir::new(&args.path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_type().is_file() && e.path().extension().is_some_and(|ext| ext == "hed")
        });

    for header_file in head_iterator {
        let file_path = header_file.path().to_str().expect("file");

        let mut reader = AiReader::new_le(File::open(file_path).expect("failed to open file"));

        let mut pack = PackFileHeader::new();
        match pack.deserialize(&mut reader) {
            Ok(_) => (),
            Err(e) => {
                println!("Failed to deserialize [{}] [{}]", file_path, e);
                continue;
            }
        };

        let holes = {
            let mut fileid_entries: HashMap<u32, Vec<&PackFileEntry>> = HashMap::new();

            for entry in &pack.files {
                fileid_entries.entry(entry.file_id).or_default().push(entry);
            }
            find_holes(&pack, &fileid_entries, &data_dir)
        };

        if holes.is_empty() {
            continue;
        }

        println!("\n{} holes[{}]", file_path, holes.len());
        for hole in &holes {
            println!(
                "[{:04}] {:#08x} size={:#x}",
                hole.file_id, hole.offset, hole.size
            )
        }
    }
}

#[derive(Debug)]
struct FileHoleEntry {
    pub file_id: u32,
    pub offset: usize,
    pub size: usize,
}
fn find_holes(
    pack: &PackFileHeader,
    fileid_entries: &HashMap<u32, Vec<&PackFileEntry>>,
    data_dir_path: &PathBuf,
) -> Vec<FileHoleEntry> {
    let max_file_idx = *fileid_entries
        .keys()
        .max()
        .expect("header file has no files?");

    let mut hole_entries: Vec<FileHoleEntry> = Vec::new();

    // let unused_file_ids: Vec<u32> = (0..max_file_idx)
    //     .filter(|a| !fileid_entries.contains_key(a))
    //     .collect();
    //
    // println!("{:#?}", unused_file_ids);

    for file_id in 0..max_file_idx {
        let rel_file_path = match fmt_c::format(
            &pack.base_path,
            vec![fmt_c::FormatArg::Uint(file_id as u64)],
        ) {
            Err(_) => continue,
            Ok(val) => val,
        };

        let data_path = data_dir_path.join(&rel_file_path);

        let data_file_info = match fs::metadata(data_path) {
            Ok(info) => info,
            Err(_) => continue,
        };

        // if !fileid_entries.contains_key(&file_id) {
        //     hole_entries.push(FileHoleEntry {
        //         file_id,
        //         offset: 0x0,
        //         size: data_file_info.len() as u32,
        //     });
        //     continue;
        // }

        match fileid_entries.get(&file_id) {
            None => {
                hole_entries.push(FileHoleEntry {
                    file_id,
                    offset: 0x0,
                    size: data_file_info.len() as usize,
                });
                continue;
            }

            Some(entries) => {
                let mut sorted_entries: Vec<&PackFileEntry> = entries.clone();
                sorted_entries.sort_by_key(|e| e.offset);

                let mut last_end: usize = 0x0;

                for file_entry in &sorted_entries {
                    if file_entry.offset as usize != last_end {
                        hole_entries.push(FileHoleEntry {
                            file_id,
                            offset: last_end,
                            size: file_entry.offset as usize - last_end,
                        });
                    }
                    last_end = file_entry.offset as usize + file_entry.size as usize
                }

                if last_end != data_file_info.len() as usize {
                    hole_entries.push(FileHoleEntry {
                        file_id,
                        offset: last_end,
                        size: data_file_info.len() as usize - last_end,
                    });
                }
            }
        }
    }

    // println!("{:#?}", hole_entries);

    hole_entries
}

fn detect_file_sizes(pack: &PackFileHeader, holes: &Vec<FileHoleEntry>, data_dir_path: &PathBuf) {
    let block_size = pack.data_scramble_key.len() * 4;

    for hole in holes {
        let rel_file_path = match fmt_c::format(
            &pack.base_path,
            vec![fmt_c::FormatArg::Uint(hole.file_id as u64)],
        ) {
            Err(_) => continue,
            Ok(val) => val,
        };
        let data_path = data_dir_path.join(&rel_file_path);

        let mut file_reader = AiReader::new_le(File::open(data_path).expect("failed to open file"));

        let mut scramble_buf = vec![0u8; hole.size];
        file_reader.seek(SeekFrom::Start(hole.offset as u64));
        file_reader.read_exact(&mut scramble_buf);

        println!("{:?}", hole);
        for offset in (0..hole.size).step_by(block_size) {
            let block_data = &scramble_buf[offset..(offset + block_size).min(hole.size)];

            let mut score_tuples: Vec<(usize, f64)> = {
                let mut score_buf = vec![0.0f64; pack.data_scramble_key.len()];
                // let mut best_score_idx = 0;
                // let mut best_score_val: f64 = 0.0;
                //
                for scramble_off in 0..(pack.data_scramble_key.len() - 1) {
                    let decrambled =
                        scramble::unscramble(&block_data, &pack.data_scramble_key, scramble_off);

                    let chi_score = chi_squared(&decrambled);
                    score_buf[scramble_off] = chi_score;

                    // if chi_score > best_score_val {
                    //     best_score_val = chi_score;
                    //     best_score_idx = scramble_off;
                    // }
                }

                score_buf
                    .iter()
                    .enumerate()
                    .map(|(idx, &val)| (idx, val))
                    .collect()
            };
            score_tuples.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            println!(
                "{:#x}: {:?}",
                offset,
                score_tuples
                    .iter()
                    .take(3)
                    .map(|tup| format!("[{}] = {}", tup.0, tup.1))
                    .collect::<Vec<String>>(),
            );
            // println!(
            //     "{:#x}: [{}] = {} {}",
            //     offset,
            //     best_score_idx,
            //     best_score_val,
            //     (offset + best_score_idx) % pack.scramble_key.len()
            // );

            // break;
        }
    }
}

// some entropy function
fn chi_squared(window: &[u8]) -> f64 {

    let mut counts = [0usize; 256];
    for &b in window {
        counts[b as usize] += 1;
    }

    let expected = window.len() as f64 / 256.0;
    counts.iter().fold(0.0, |acc, &obs| {
        let diff = obs as f64 - expected;
        acc + diff * diff / expected
    })
}
