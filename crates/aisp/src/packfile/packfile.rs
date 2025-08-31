use std::path::PathBuf;

use crate::packfile::packfile_header::PackFileHeader;

pub struct PackFile {}

impl PackFile {}

pub fn find_base_dir(header: &PackFileHeader, header_file_path: &String) -> PathBuf {
    let mut data_dir_path = PathBuf::from(&header_file_path);
    data_dir_path.pop();

    {
        let mut buf2 = PathBuf::from(&header.base_path);
        buf2.pop();
        buf2.pop();

        // file_name == leaf
        while data_dir_path.file_name() == buf2.file_name() {
            println!("{:#?}", data_dir_path.file_name());
            data_dir_path.pop();
            buf2.pop();
        }
    }

    data_dir_path
}
