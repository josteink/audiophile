//use std::fs;
use std::env;

extern crate claxon;

use std::ffi::OsStr;

fn main() {
    let root_dir = get_root_dir();
    let mask = format!("{}/**/*.flac", root_dir);

    // let files = glob::glob("../**/*.flac").expect("Error listing files!");
    let files = glob::glob(&mask).expect("Error listing files!");

    for item in files {
        let path = item.unwrap();
        if path.is_file() && path.extension() == Some(OsStr::new("flac")) {
            let (depth, rate) = get_bit_depth_of_flac(&path);
            if depth > 16 || rate > 44100 {
                println!("{} ({} - {}Hz)", path.display(), depth, rate);
            }
        }
    }
}

fn get_root_dir() -> std::string::String {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let dir = &args[1];
    dir.to_string()
}

fn get_bit_depth_of_flac(path: &std::path::Path) -> (u32, u32) {
    let reader = claxon::FlacReader::open(path).unwrap();
    let metadata = reader.streaminfo();

    (metadata.bits_per_sample, metadata.sample_rate)
}
