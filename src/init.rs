// use util.rs;
use crate::{create_initial_commit, get_hash};
use std::fs;
use std::fs::write;
use std::path::PathBuf;

pub fn init(args: &Vec<String>) -> std::io::Result<()> {
    let path: String = ".vcs".to_string();
    fs::create_dir(&path)?;
    fs::create_dir(path.clone() + "/objects")?;
    //fs::create_dir(path.clone() + "/refs")?;
    fs::create_dir(path.clone() + "/branches")?;
    let file_path = PathBuf::from(&path).join("HEAD");
    write(&file_path, "master").unwrap();
    let file_path = PathBuf::from(&path).join("config");
    write(
        &file_path,
        args[3].clone(),
    )?;
    //let file_path = PathBuf::from(&path).join("description");
    //write(&file_path, "").unwrap();
    let parent = "null".to_string();

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
