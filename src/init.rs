// use util.rs;
use crate::{create_initial_commit, get_hash};
use std::fs;
use std::fs::write;
use std::path::PathBuf;

pub fn init(args: &Vec<String>) -> std::io::Result<()> {
    let vcs_path = PathBuf::from(".vcs");
    fs::create_dir_all(vcs_path.join("objects"))?;
    fs::create_dir_all(vcs_path.join("branches"))?;

    write(vcs_path.join("HEAD"), "master").unwrap();
    write(vcs_path.join("config"), &args[3])?;

    let parent = "null".to_string();

    create_initial_commit(&args[3], "master", "null", "Initial commit");
    println!("Initialized VCS repository in {}", args[3]);
    println!(
        "Created commit:\n[master {}] Initial commit",
        get_hash(&parent)
    );
    Ok(())
}
