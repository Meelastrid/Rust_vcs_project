use compress::zlib;
use flate2::bufread::ZlibDecoder;
use flate2::bufread::ZlibEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use fs_extra::file::read_to_string;
use hex::encode;
use hex_literal::hex;
use sha1::{Digest, Sha1};
use std::env;
use std::fs;
use std::fs::{metadata, write, File};
use std::io::prelude::*;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
// use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str;
extern crate fs_extra;
use fs_extra::dir::get_size;
use serde_derive::{Deserialize, Serialize};

// use std::sync::mpsc::RecvTimeoutError;

use crate::init::init;
use crate::status::status;

pub fn run_command(args: &Vec<String>) -> std::io::Result<()> {
    match args[1].as_str() {
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

pub fn get_size_of_the_file(path: String) -> u64 {
    let file_lenth = fs::metadata(path).unwrap();
    file_lenth.len()
}

pub fn get_hash(data: &String) -> String {
    let mut hasher = Sha1::new();

    hasher.update(data);
    let hash_sum_u8 = hasher.finalize();
    hex::encode(hash_sum_u8)
}

pub fn create_blob_object(path: String, vcs_dir: String, commit: String) {
    let objects_dir_path = vcs_dir + "/objects";

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

    fs::create_dir(objects_dir_path.clone() + "/" + object_dir_name).unwrap();
    let file_path = PathBuf::from(objects_dir_path + "/" + object_dir_name).join(object_file_name);

    write(&file_path, data).unwrap();

    let ref_prefix: &str = &commit[0..2];
    let ref_file_name: &str = &commit[2..];
    let ref_path = PathBuf::from(objects_dir_path + "/" + ref_prefix).join(object_file_name);
    fs::create_dir(objects_dir_path.clone() + "/" + ref_prefix).unwrap();
    let mut refdata: String = object_hash + &path;
    write(&ref_path, data).unwrap();
}

pub fn create_tree_object(d: String, vcs_dir: String, parent: String) {
    let folder_size_string = get_size(d).unwrap().to_string();
    let folder_size: &str = &folder_size_string[..];
    let object_type: String = "tree".to_string();
    let mut data = object_type;
    data.push(' ');
    data.push_str(folder_size);
    data.push('\0');

    let object_hash = get_hash(&data);
    let object_dir: &str = "objects";
    let object_pref: &str = &object_hash[0..2];
    let object_file: &str = &object_hash[2..];

    let mut file_path = PathBuf::from(vcs_dir).join(object_dir).join(object_pref);
    if !file_path.exists() {
        fs::create_dir_all(file_path.clone()).unwrap();
    }
    file_path = file_path.join(object_file);
    write(&file_path, data).unwrap();


    let mut ref_path = PathBuf::from(vcs_dir).join("refs").join(object_pref);
    if !file_path.exists() {
        fs::create_dir_all(file_path.clone()).unwrap();
    }
}

//pub fn create_tree_object(d: String) {
//    let dir_path = fs::read_dir(d.clone()).unwrap();
//    for path in dir_path {
//        let folder_size_string = get_size(path.unwrap().path().display().to_string())
//            .unwrap()
//            .to_string();
//        let folder_size: &str = &folder_size_string[..];
//        let object_type: String = "tree".to_string();
//        let mut data = object_type;
//        data.push(' ');
//        data.push_str(folder_size);
//        data.push('\0');
//
//        let object_hash = get_hash(&data);
//        let object_dir: &str = &object_hash[0..2];
//        let object_file: &str = &object_hash[2..];
//
//        let dir_path: String = d.clone();
//        if !Path::new(&(dir_path.clone() + "/vcs/" + object_dir)).exists() {
//            fs::create_dir(dir_path.clone() + "/vcs/" + object_dir).unwrap();
//        }
//        let file_path = PathBuf::from(d.clone() + "/vcs/" + object_dir).join(object_file);
//        //    println!("{}", data);
//        write(&file_path, data).unwrap();
//    }
//}

pub fn get_file_name_from_path(path: &str) {
    let ancestors = Path::new(&path).file_name().unwrap().to_str().unwrap();
    println!("File name was {}", ancestors);
}

pub fn create_object(t: String, path: String) {
    // match t {
    //     // "blob" => get_hash(),
    // }
}

pub fn create_commit(path: String, branch: String, parent: String, message: String) {
    //create branch, write current commit hash there
    let mut branch_path = PathBuf::from(path).join("branches").join(branch);
    let commit_hash = get_hash(&parent);
    write(&branch_path, commit_hash).unwrap();

    //create commit object, write current commit hash there
    let commit_dir_name: &str = &commit_hash[0..2];
    let commit_file_name: &str = &commit_hash[2..];

    fs::create_dir(path.clone() + "/.vcs/objects/" + commit_dir_name).unwrap();
    let commit_path = PathBuf::from(path + "/.vcs/objects/" + commit_dir_name).join(commit_file_name);
    write(&branch_path, commit_hash).unwrap();
    create_objects(path, path + "/.vcs", commit_hash)

}

pub fn create_objects(path: String, vcs_dir: String, commit: String) {
    if metadata(&path).unwrap().is_file() {
        create_blob_object(path, vcs_dir, commit);
    } else if metadata(&path).unwrap().is_dir() && !path.contains(".vcs") {
        create_tree_object(path.clone(), vcs_dir.clone(), commit);
        let paths = fs::read_dir(&path).unwrap();
        for p in paths {
            create_objects(p.as_ref().unwrap().path().display().to_string(), vcs_dir.clone(), commit);
        }
    }
}
