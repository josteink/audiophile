//use std::fs;
use std::env;

extern crate claxon;

use std::ffi::OsStr;

struct MediaInfo {
    path: std::string::String,
    depth: u32,
    rate: u32
}

fn main() {
    let root_dir = get_root_dir();
    let mask = format!("{}/**/*.flac", root_dir);
    let files = glob::glob(&mask).expect("Error listing files!");

    for item in files {
        let path = item.unwrap();
        if path.is_file() && path.extension() == Some(OsStr::new("flac")) {
            let media_info = get_media_info_for_flac(&path);
            if media_info.depth > 16 || media_info.rate > 44100 {
                println!("{} ({} - {}Hz)", media_info.path, media_info.depth, media_info.rate);
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

fn get_media_info_for_flac(path: &std::path::Path) -> MediaInfo {
    let reader = claxon::FlacReader::open(path).unwrap();
    let metadata = reader.streaminfo();

    MediaInfo {
        path: path.display().to_string(),
        depth: metadata.bits_per_sample,
        rate: metadata.sample_rate
    }
}
