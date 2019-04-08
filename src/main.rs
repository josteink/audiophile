extern crate claxon;
extern crate hound;

use std::path::Path;

struct MediaInfo {
    format: String,
    depth: u32,
    rate: u32,
}

impl MediaInfo {
    fn is_audiophile_grade_audio(&self) -> bool {
        self.depth > 16 || self.rate > 44100
    }

    fn from_path(path: &Path) -> Option<MediaInfo> {
        let ext = path.extension().unwrap_or_default().to_str().unwrap();

        match ext.to_lowercase().as_str() {
            "flac" => Some(MediaInfo::from_flac(path)),
            "wav" => MediaInfo::from_wav(path),
            _ => None,
        }
    }

    fn from_flac(path: &Path) -> MediaInfo {
        let reader = claxon::FlacReader::open(path).unwrap();
        let metadata = reader.streaminfo();

        MediaInfo {
            format: "Flac".to_string(),
            depth: metadata.bits_per_sample,
            rate: metadata.sample_rate,
        }
    }

    fn from_wav(path: &Path) -> Option<MediaInfo> {
        let result = hound::WavReader::open(path);

        match result {
            Ok(reader) => {
                let metadata = reader.spec();

                Some(MediaInfo {
                    format: "Wav".to_string(),
                    depth: metadata.bits_per_sample as u32,
                    rate: metadata.sample_rate,
                })
            }
            _ => None,
        }
    }
}

impl std::fmt::Display for MediaInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _rate = (self.rate as f32) / 1000.0;

        write!(f, "{} - {}-bit, {}kHz", self.format, self.depth, _rate)
    }
}

fn main() {
    let root_dir = get_root_dir();
    let mask = format!("{}/**/*.*", root_dir);
    let files = glob::glob(&mask).expect("Error listing files!");

    for item in files {
        let path = item.unwrap();
        if path.is_file() {
            let media_info = MediaInfo::from_path(&path);

            match media_info {
                None => (),
                Some(info) => {
                    if info.is_audiophile_grade_audio() {
                        println!("{} ({})", path.display(), info);
                    }
                }
            }
        }
    }
}

fn get_root_dir() -> std::string::String {
    use std::env;
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let dir = &args[1];
    dir.to_string()
}
