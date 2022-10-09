use fs_extra::file::read_to_string;
use hex_literal::hex;
use sha1::{Sha1, Digest};
use std::fs;
use std::fs::{write, metadata, File};
use std::io::Read;
use compress::zlib;
use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::io::prelude::*;
use flate2::read::GzDecoder;
use::hex::encode;
use flate2::bufread::ZlibEncoder;
use flate2::bufread::ZlibDecoder;
use flate2::Compression;
// use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str;
extern crate fs_extra;
use fs_extra::dir::get_size;

// use std::sync::mpsc::RecvTimeoutError;

use crate::init::init;
use crate::status::status;

    pub fn run_command(args: &Vec<String>) -> std::io::Result<()> {
        match args[2].as_str() {
            "init" => init(args),
            "status" => status(),


            _ => Ok(()),

        }
    }

pub fn get_command_line_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let len: usize = args.len();
    if len < 3 { 
        panic!("No arguments for vcs");
    }
    args
}

pub fn decompress_zlib(bytes: Vec<u8>) -> io::Result<String> {
    let mut z = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    println!("{}", s);
    Ok(s)
}

pub fn compress_zlib(path: String) -> Vec<u8> {
    let f = File::open(path).unwrap();
    let b = BufReader::new(f);
    let mut z = ZlibEncoder::new(b, Compression::fast());
    let mut buffer = Vec::new();
    z.read_to_end(&mut buffer).unwrap();
    // dbg!(&buffer);
    buffer
}

pub fn get_size_of_the_file(path: String) -> u64{
    let file_lenth = fs::metadata(path).unwrap().len();
    // println!("{}", file_lenth);
    file_lenth
}

pub fn get_hash(data: &String) -> String {
    let mut hasher = Sha1::new();
    
    hasher.update(data);
    let hash_sum_u8 = hasher.finalize();
    let hash_sum_str = hex::encode(hash_sum_u8);

    hash_sum_str
}

pub fn create_blob_object(path: String, args: &Vec<String>) {
    println!("{}", &path);
    let objects_dir_path = args[4].clone() + "/.vcs" + "/objects";

    let file_size_string = get_size_of_the_file(path.clone()).to_string();
    let file_size: &str = &file_size_string[..];
    let object_type: String = "blob".to_string();
    let mut data = object_type;
    data.push(' ');
    data.push_str(file_size);
    data.push('\0');
    let zlib_text = compress_zlib(path);
    // let zlib_text_string = str::from_utf8(&zlib_text).unwrap();
    let zlib_text_string = format!("{:?}", &zlib_text);
    let zlib_text_slice: &str = &zlib_text_string[..];
    data.push_str(zlib_text_slice);

    let object_hash = get_hash(&data);
    let object_dir_name: &str = &object_hash[0..2];
    let object_file_name: &str = &object_hash[2..];

    fs::create_dir(objects_dir_path.clone() + "/" + &object_dir_name.to_string());
    let file_path = PathBuf::from(objects_dir_path + "/" + &object_dir_name.to_string()).join(object_file_name);
    write(&file_path, data);
}

pub fn create_tree_object() {
    let dir_path = fs::read_dir("/home/meelastrid/Education/EduRust/VcsProject/vcs/src/.vcs/objects").unwrap();
    for path in dir_path {
       let folder_size_string = get_size(path.unwrap().path().display().to_string(),).unwrap().to_string();
       let folder_size: &str = &folder_size_string[..];
       let object_type: String = "tree".to_string();
       let mut data = object_type;
       data.push(' ');
       data.push_str(folder_size);
       data.push('\0');

       let object_hash = get_hash(&data);
       let object_dir: &str = &object_hash[0..2];
       let object_file: &str = &object_hash[2..];

       let dir_path: String = "/home/meelastrid/Education/EduRust/VcsProject/vcs/src/.vcs/objects".to_string();
       if !Path::new(&(dir_path.clone() + "/" + &object_dir.to_string())).exists() {
           fs::create_dir(dir_path.clone() + "/" + &object_dir.to_string());
       }
       let file_path = PathBuf::from(dir_path + "/" + &object_dir.to_string()).join(object_file);
    //    println!("{}", data);
       write(&file_path, data);

    }
}

pub fn get_file_name_from_path(path: &str) {
    let ancestors = Path::new(& path).file_name().unwrap().to_str().unwrap();
    println!("File name was {}", ancestors);
}

pub fn create_object(t: String, path: String) {
    // match t {
    //     // "blob" => get_hash(),
    // }
} 

pub fn create_objects(path: String, args: &Vec<String>){
    if metadata(&path).unwrap().is_file() == true {
        // println!("entered file: {}", path);
        create_blob_object(path, args);
    }
    else {
        if metadata(&path).unwrap().is_dir() == true && path != "/home/meelastrid/Education/EduRust/VcsProject/vcs/src/.vcs"{
            let paths = fs::read_dir(&path).unwrap();
            // println!("entered dir: {}", path);
            // println!("entered dir: {}", path);

            let mut cnt = 0;
            for p in paths {
                cnt = cnt + 1;
                create_objects(p.as_ref().unwrap().path().display().to_string(), args);
                // println!("entered for dir: {}", p.unwrap().path().display().to_string());
        }

    }
        // println!("{}", path.unwrap().path().display());
    }
}