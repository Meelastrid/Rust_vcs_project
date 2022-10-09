// use util.rs;
use crate::create_objects;
use std::fs;
use std::fs::{write, metadata};
use std::env;
use std::path::PathBuf;
use std::sync::mpsc::RecvTimeoutError;

pub fn init(args: &Vec<String>) -> std::io::Result<()> {
    let path: String = args[4].clone() + "/.vcs";
    fs::create_dir(&path)?;
    println!("Initialized VCS repository in {}", args[4]);
    fs::create_dir(path.clone() + &"/objects".to_string())?;
    fs::create_dir(path.clone() + &"/refs".to_string())?;
    let file_path = PathBuf::from(&path).join("HEAD");
    write(&file_path, "");
    let file_path = PathBuf::from(&path).join("config");
    write(&file_path, "[core]
	repositoryformatversion = 0
	filemode = false
	bare = false")?;
    let file_path = PathBuf::from(&path).join("description");
    write(&file_path, "");

    create_objects(path, args);

    return Ok(());
}