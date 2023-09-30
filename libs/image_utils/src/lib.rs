use std::io::{
    Cursor,
};
use std::path::Path;
use std::ffi::OsStr;
use image::io::Reader as ImageReader;
use image::imageops::FilterType;
use image::{ImageFormat};
#[cfg(feature = "native_fs")]
use std::fs::File;
#[cfg(feature = "native_fs")]
use std::io::{
    BufReader,
    Read,
    Write,
};

#[cfg(feature = "native_fs")]
pub fn read_file(file_path: &str) -> Vec<u8> {
    let f = File::open(file_path).unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    buffer
}

#[cfg(feature = "native_fs")]
pub fn write_file(buffer: Vec<u8>, file_path: &str) -> () {
    let mut file = File::create(file_path).unwrap();
    file.write_all(&buffer).unwrap();
}

pub fn resize_image(bytes: Vec<u8>, dest_file_path: &str, new_width: u32, new_height: u32) -> Vec<u8> {
    // let bytes = read_file(file_path);
    let img2 = ImageReader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap();
    let mut buff = Cursor::new(Vec::new());
    // let file_ext = format!(".{}", Path::new(dest_file_path).extension().unwrap().to_str().unwrap());
    // println!("{}", file_ext);
    let file_ext = Path::new(dest_file_path).extension().and_then(OsStr::to_str).unwrap();
    let image_format = match file_ext {
        ".jpg" => ImageFormat::Jpeg,
        ".png" => ImageFormat::Png,
        _ => ImageFormat::Jpeg,
    };
    img2.resize(new_width, new_height, FilterType::CatmullRom).write_to(&mut buff, image_format).expect("Error encoding image to JPEG");
    buff.into_inner()
    // write_file(buff.into_inner(), dest_file_path);
}
