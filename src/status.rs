use crate::util::get_hash_file;
use array_tool::vec::{Intersect, Uniq};
use std::collections::HashMap;
use std::fs::{self, metadata, DirEntry, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;
pub fn get_current_branch() -> String {
    fs::read_to_string("test/.vcs/HEAD").unwrap()
}
pub fn get_files_in_commit() -> HashMap<String, String> {
    let mut files: HashMap<String, String> = HashMap::new();
    let br = get_current_branch();
    let commit = fs::read_to_string("test/.vcs/branches/".to_string() + &br).unwrap();
    let reader = BufReader::new(
        File::open("test/.vcs/objects/".to_string() + &commit[..2] + "/" + &commit[2..]).unwrap(),
    );
    let mut commit_read = false;
    for line in reader.lines() {
        if !commit_read {
            commit_read = true;
            continue;
        }
        for (prefix, suffix) in line.unwrap().split_once(" ") {
            files.insert(prefix.to_string(), suffix.to_string());
        }
    }
    files
}

pub fn check_modified_files() -> Vec<String> {
    let mut res_files: Vec<String> = Vec::new();
    let files = get_files_in_commit();
    for (file, hash) in files.iter() {
        if PathBuf::from(file).exists() && !(get_hash_file(file.clone()) == hash.clone()) {
            res_files.push(file.clone());
        }
    }
    res_files
}

fn collect_all_files_in_dir(dir: String) -> Vec<String> {
    let mut res_files: Vec<String> = Vec::new();
    if metadata(&dir).unwrap().is_file() {
        res_files.push(dir)
    } else if metadata(&dir).unwrap().is_dir() && !dir.contains(".vcs") {
        let paths = fs::read_dir(&dir).unwrap();
        for p in paths {
            res_files.extend(collect_all_files_in_dir(
                p.unwrap().path().display().to_string(),
            ));
        }
    }
    res_files
}

pub fn nothing_to_commit() -> bool {
    let my = collect_all_files_in_dir("test".to_string());
    let in_commit: Vec<String> = get_files_in_commit().keys().cloned().collect();
    let added = my.uniq(in_commit.clone());
    let deleted = in_commit.uniq(my);
    let modified = check_modified_files();
    if added.is_empty() && deleted.is_empty() && modified.is_empty() {
        return true;
    }
    false
}
pub fn status() -> std::io::Result<()> {
    //let my = check_new_files();
    let br = get_current_branch();
    let commit = fs::read_to_string("test/.vcs/branches/".to_string() + &br).unwrap();
    let my = collect_all_files_in_dir("test".to_string());
    let in_commit: Vec<String> = get_files_in_commit().keys().cloned().collect();
    let added = my.uniq(in_commit.clone());
    let deleted = in_commit.uniq(my);
    let modified = check_modified_files();
    println!("Status:");
    println!("Branch: {}", br);
    println!("Commit: {}", commit);
    println!("Modified: {:?}", modified);
    println!("Added: {:?}", added);
    println!("Deleted: {:?}", deleted);
    Ok(())
}
