// use std::fs;
// use std::fs::{write, metadata};
use std::env;
// use std::path::PathBuf;
// use std::sync::mpsc::RecvTimeoutError;
// use hex_literal::hex;
// use sha1::{Sha1, Digest};

//pub fn run_command(args: Vec<String>) -> std::io::Result<()> {
//    match args[2].as_str() {
//        "init" => init(args),
//        "status" => status(),
//
//        _ => Ok(()),
//
//    }
//}
//
//pub fn get_command_line_args() -> Vec<String> {
//    let args: Vec<String> = env::args().collect();
//    let len: usize = args.len();
//    if len < 3 {
//        panic!("No arguments for vcs");
//    }
//    args
//}

// pub fn get_hash(path: String) {
//     let mut hasher = Sha1::new();
//     hasher.update(b"hello world");
// }

// fn create_object(t: String, path: String) {
//     // match t {
//     //     // "blob" => get_hash
//     // }
// }

// pub fn create_objects(path: String){
//     if metadata(&path).unwrap().is_file() == true {
//         println!("{}", path);
//     }
//     else {
//         if metadata(&path).unwrap().is_dir() == true {
//             let paths = fs::read_dir("./").unwrap();
//             println!("{}", path);
//             for path in paths {
//                 create_objects(path.unwrap().path().display().to_string());
//         }

//     }
//         // println!("{}", path.unwrap().path().display());
//     }
// }

// pub fn init(args: Vec<String>) -> std::io::Result<()> {
//     let path: String = args[4].clone() + "/.vcs";
//     fs::create_dir(&path)?;
//     println!("Initialized VCS repository in {}", args[4]);
//     fs::create_dir(path.clone() + &"/objects".to_string())?;
//     fs::create_dir(path.clone() + &"/refs".to_string())?;
//     let file_path = PathBuf::from(&path).join("HEAD");
//     write(&file_path, "");
//     let file_path = PathBuf::from(&path).join("config");
//     write(&file_path, "[core]
// 	repositoryformatversion = 0
// 	filemode = false
// 	bare = false")?;
//     let file_path = PathBuf::from(&path).join("description");
//     write(&file_path, "");
//     return Ok(());
// }

// pub fn status() -> std::io::Result<()>{
//     println!("Hello");
//     Ok(())
// }

// pub fn commit() {

// }

// pub fn jump_to_branch() {

// }

// pub fn jump_to_commit() {

// }

// pub fn new_brach() {

// }

// pub fn merge() {

// }

// pub fn log() {

// }
