use std::env;
use std::process;
use cli;
use image_utils;

fn path_cwd(file_path: &str) -> String {
    let cur_dir = env::current_dir().unwrap();
    let cwd = cur_dir.to_str().unwrap();
    format!("{}/{}", cwd, file_path)
}

fn main() {
    loop {
        let prompt = cli::prompt(">");
        match prompt.as_str() {
            "resize" => {
                let src_file_path = path_cwd("../testfiles/pianos_keys_musical_instrument_120891_1080x1920.jpeg");
                let dest_file_path = path_cwd("../testfiles/256.jpeg");
                image_utils::resize_image(&src_file_path, &dest_file_path, 256, 256);
            },
            "exit" => process::exit(0),
            _ => {
                println!("{:?}", prompt);
            },
        }
    }
}
