extern crate claxon;
extern crate hound;
extern crate alac;

use std::path::Path;

pub struct MediaInfo {
    pub format: String,
    pub depth: u32,
    pub rate: u32,
}

impl MediaInfo {
    pub fn is_audiophile_grade_audio(&self) -> bool {
        (self.depth >= 16 && self.rate >= 44100) && (self.depth > 16 || self.rate > 44100)
    }

    pub fn from_path(path: &Path) -> Option<MediaInfo> {
        let ext = path.extension().unwrap_or_default().to_str().unwrap();

        match ext.to_lowercase().as_str() {
            "flac" => Some(MediaInfo::from_flac(path)),
            "wav" => MediaInfo::from_wav(path),
            "m4a" => MediaInfo::from_alac(path),
            "alac" => MediaInfo::from_alac(path),
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

    fn from_alac(path: &Path) -> Option<MediaInfo> {
        use std::fs::File;

        let file = File::open(path).expect("Error opening file!");
        let result = alac::Reader::new(&file);
        match result {
            Ok(r) => {
                let stream_info = r.stream_info();

                Some(MediaInfo {
                    format: "Alac".to_string(),
                    depth: stream_info.bit_depth() as u32,
                    rate: stream_info.sample_rate(),
                })
            },
            _ => None
        }

    }
}

impl std::fmt::Display for MediaInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _rate = (self.rate as f32) / 1000.0;

        write!(f, "{} - {}-bit, {}kHz", self.format, self.depth, _rate)
    }
}
