use flate2::bufread::ZlibDecoder;
use flate2::bufread::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::fs::{metadata, write, File};
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::str;
extern crate fs_extra;
use flate2::write::GzEncoder;
use fs_extra::dir::get_size;
use std::io::copy;

// use std::sync::mpsc::RecvTimeoutError;

use crate::commit::commit;
use crate::init::init;
use crate::jump::jump;
use crate::log::log;
use crate::merge::merge;
use crate::new_branch::new_branch;
use crate::status::status;

pub fn run_command(args: &Vec<String>) -> std::io::Result<()> {
    match args[1].as_str() {
        "init" => init(args),
        "status" => status(),
        "commit" => commit(args[3].clone()),
        "log" => log(),
        "new_branch" => new_branch(args[3].clone()),
        "jump" => jump(args[2].clone(), args[3].clone()),
        "merge" => merge(args[3].clone()),
        "help" => help(),
        _ => help(),
    }
}

pub fn help() -> std::io::Result<()> {
    println!("vcs is a lightweight version control system.");
    println!("Available commands:");
    println!();
    println!("vcs init --path <directory path>");
    println!("vcs status");
    println!("vcs log");
    println!("vcs commit --message <message>");
    println!("vcs new_branch --name <branch_name>");
    println!("vcs jump --branch <branch_name>");
    println!("vcs jump --commit <commit_hash>");
    println!("vcs merge --branch <branch_name>");

    Ok(())
}

pub fn get_command_line_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let len: usize = args.len();
    if len < 2 {
        panic!("No arguments for vcs. See `vcs help` for help.");
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

pub fn compress_zlib(path: &PathBuf) -> Vec<u8> {
    let f = File::open(path).unwrap();
    let b = BufReader::new(f);
    let mut z = ZlibEncoder::new(b, Compression::fast());
    let mut buffer = Vec::new();
    z.read_to_end(&mut buffer).unwrap();
    // dbg!(&buffer);
    buffer
}

pub fn get_size_of_the_file(path: &PathBuf) -> u64 {
    let file_lenth = fs::metadata(path).unwrap();
    file_lenth.len()
}

pub fn get_hash(data: &str) -> String {
    let mut hasher = Sha1::new();

    hasher.update(data);
    let hash_sum_u8 = hasher.finalize();
    hex::encode(hash_sum_u8)
}

pub fn get_hash_file(path: &PathBuf) -> String {
    let mut hasher = Sha1::new();
    let mut file = fs::File::open(&path).unwrap();
    io::copy(&mut file, &mut hasher).unwrap();
    let hash_sum_u8 = hasher.finalize();
    hex::encode(hash_sum_u8)
}

pub fn compress_file(source: &PathBuf, target: &PathBuf) {
    let mut input = BufReader::new(File::open(source).unwrap());
    let output = File::create(target).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    copy(&mut input, &mut encoder).unwrap();
    encoder.finish().unwrap();
}

pub fn create_blob_object(path: &PathBuf, vcs_dir: PathBuf, commit: String) {
    let objects_dir_path = vcs_dir.join("objects");

    let file_size_string = get_size_of_the_file(path).to_string();
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

    let object_hash = get_hash_file(path);
    let object_dir_name: &str = &object_hash[0..2];
    let object_file_name: &str = &object_hash[2..];

    fs::create_dir_all(objects_dir_path.join(object_dir_name)).unwrap();
    let file_path = PathBuf::from(&objects_dir_path)
        .join(object_dir_name)
        .join(object_file_name);

    //write(&file_path, &data).unwrap();
    compress_file(path, &file_path);

    let ref_prefix: &str = &commit[0..2];
    let ref_file_name: &str = &commit[2..];
    let ref_path = PathBuf::from(&objects_dir_path)
        .join(ref_prefix)
        .join(ref_file_name);
    if !ref_path.exists() {
        fs::create_dir_all(&objects_dir_path.join(ref_prefix)).unwrap();
    }
    let refdata: String = path.display().to_string() + " " + &object_hash + "\n";
    let mut ref_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&ref_path)
        .unwrap();
    ref_file.write_all(refdata.as_bytes()).unwrap();
}

pub fn create_tree_object(d: String, vcs_dir: String) {
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
}

pub fn get_file_name_from_path(path: &str) {
    let ancestors = Path::new(&path).file_name().unwrap().to_str().unwrap();
    println!("File name was {}", ancestors);
}

pub fn create_initial_commit(path: &str, branch: &str, parent: &str, message: &str) {
    //create branch, write current commit hash there
    let branch_path = PathBuf::from(".vcs/branches").join(branch);
    let commit_hash = get_hash(parent);
    write(&branch_path, commit_hash.clone()).unwrap();

    //create commit object, write current commit hash there
    let commit_dir_name: &str = &commit_hash[0..2];
    let commit_file_name: &str = &commit_hash[2..];
    fs::create_dir(".vcs/objects/".to_string() + commit_dir_name).unwrap();
    let commit_path =
        PathBuf::from(".vcs/objects/".to_string() + commit_dir_name).join(commit_file_name);
    write(&commit_path, message.to_string() + "\n" + &parent + "\n").unwrap();

    //create objects for commit
    create_objects(
        PathBuf::from(path.to_string()),
        PathBuf::from(".vcs"),
        commit_hash,
    )
}

pub fn create_objects(path: PathBuf, vcs_dir: PathBuf, commit: String) {
    if metadata(&path).unwrap().is_file() {
        create_blob_object(&path, vcs_dir, commit);
    } else if metadata(&path).unwrap().is_dir() && !path.display().to_string().contains(".vcs") {
        //create_tree_object(path.clone(), vcs_dir.clone(), commit.clone());
        let paths = fs::read_dir(&path).unwrap();
        for p in paths {
            create_objects(p.unwrap().path(), vcs_dir.clone(), commit.clone());
        }
    }
}
