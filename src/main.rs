//use std::fs;
use std::env;

extern crate claxon;

struct MediaInfo {
    depth: u32,
    rate: u32
}

fn main() {
    let root_dir = get_root_dir();
    let mask = format!("{}/**/*.flac", root_dir);
    let files = glob::glob(&mask).expect("Error listing files!");

    for item in files {
        let path = item.unwrap();
        if path.is_file() {
            let media_info = get_media_info_for(&path);

            match media_info {
                None => (),
                Some(info) => {
                    if is_audiophile_grade_audio(&info) {
                        println!("{} ({} - {}Hz)", path.display(), info.depth, info.rate);
                    }
                }
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

fn get_media_info_for(path: &std::path::Path) -> Option<MediaInfo> {
    let ext = path.extension().unwrap_or_default().to_str().unwrap();

    match ext {
        "flac" => get_media_info_for_flac(path),
        _ => None
    }
}

fn get_media_info_for_flac(path: &std::path::Path) -> Option<MediaInfo> {
    let reader = claxon::FlacReader::open(path).unwrap();
    let metadata = reader.streaminfo();

    Some(MediaInfo {
        depth: metadata.bits_per_sample,
        rate: metadata.sample_rate
    })
}

fn is_audiophile_grade_audio(media_info: &MediaInfo) -> bool {
    media_info.depth > 16 || media_info.rate > 44100
}
