// use util.rs;
use crate::{create_initial_commit, create_objects, get_hash};
use std::env;
use std::fs;
use std::fs::{metadata, write};
use std::path::PathBuf;
use std::sync::mpsc::RecvTimeoutError;

pub fn init(args: &Vec<String>) -> std::io::Result<()> {
    let path: String = args[3].clone() + "/.vcs";
    fs::create_dir(&path)?;
    fs::create_dir(path.clone() + "/objects")?;
    //fs::create_dir(path.clone() + "/refs")?;
    fs::create_dir(path.clone() + "/branches")?;
    let file_path = PathBuf::from(&path).join("HEAD");
    write(&file_path, "master").unwrap();
    let file_path = PathBuf::from(&path).join("config");
    write(
        &file_path,
        "[core]
	repositoryformatversion = 0
	filemode = false
	bare = false",
    )?;
    let file_path = PathBuf::from(&path).join("description");
    write(&file_path, "").unwrap();
    let parent = "null".to_string();
    let p_hash = get_hash(&parent);

    //create_objects(args[3].clone(), args[3].clone() + "/.vcs", parent);
    create_initial_commit(
        args[3].clone(),
        "master".to_string(),
        parent.clone(),
        "Initial commit".to_string(),
    );
    println!("Initialized VCS repository in {}", args[3]);
    println!(
        "Created commit:\n[master {}] Initial commit",
        get_hash(&parent)
    );
    Ok(())
}
