extern crate audiophile;

use audiophile::MediaInfo;

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
