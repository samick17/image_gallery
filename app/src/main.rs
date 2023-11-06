use std::env;
use std::process;
use cli;
use image_utils;
use wasm_runner;
use std::io::{
    BufReader,
    Read,
};
use std::io::{
    Write,
};
use std::fs::File;

 fn read_file(file_path: &str) -> Vec<u8> {
    let f = File::open(file_path).unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    buffer
}

pub fn write_file(buffer: Vec<u8>, file_path: &str) -> () {
    let mut file = File::create(file_path).unwrap();
    file.write_all(&buffer).unwrap();
}

fn path_cwd(file_path: &str) -> String {
    let cur_dir = env::current_dir().unwrap();
    let cwd = cur_dir.to_str().unwrap();
    format!("{}/{}", cwd, file_path)
}

fn print_menu() {
    println!("{}", "[ImageGallery]");
    println!("{}", "- resize");
    println!("{}", "- wasm");
    println!("{}", "- exit");
}

fn main() {
    loop {
        let prompt = cli::prompt(">");
        let args = cli::parse_kwargs(&prompt);
        if let Some(cmd) = args.get("cmd") {
            match cmd.as_str() {
                "help" => print_menu(),
                "resize" => {
                    let src_file_path = path_cwd("../testfiles/pianos_keys_musical_instrument_120891_1080x1920.jpeg");
                    let file_ext = ".jpeg";
                    let file_array = read_file(&src_file_path);
                    let out_file_array = image_utils::resize_image(file_array, &file_ext, 256, 256);
                    let dest_file_path = path_cwd("../testfiles/256.jpeg");
                    write_file(out_file_array, &dest_file_path);
                },
                "wasm" => {
                    let wasm_file_path = "./wasm/image_utils_wasm.wasm";
                    {
                        let fn_name = "get_value_i32";
                        let result = wasm_runner::execute_wasm::<i32>(wasm_file_path, fn_name);
                        println!("WASM(get_value_i32): {}", result);
                    }
                    {
                        let fn_name = "get_value_f32";
                        let result = wasm_runner::execute_wasm::<f32>(wasm_file_path, fn_name);
                        println!("WASM(get_value_f32): {}", result);
                    }
                },
                "exit" => process::exit(0),
                _ => {
                    println!("Unknown command: {:?}", prompt);
                },
            }
        }
    }
}
