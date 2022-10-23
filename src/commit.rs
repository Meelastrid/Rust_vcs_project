use crate::status::nothing_to_commit;
use crate::util::compress_file;
use crate::util::compress_zlib;
use crate::util::get_hash_file;
use crate::util::get_size_of_the_file;
use sha1::{Digest, Sha1};
use std::fs::OpenOptions;
use std::fs::{self, metadata, write};
use std::io::prelude::*;
use std::path::PathBuf;

pub fn get_hash(data: &String) -> String {
    let mut hasher = Sha1::new();

    hasher.update(data);
    let hash_sum_u8 = hasher.finalize();
    hex::encode(hash_sum_u8)
}
pub fn get_workdir() -> String {
    fs::read_to_string(PathBuf::from(".vcs").join("config")).unwrap()
}

pub fn create_blob_object(path: &PathBuf, vcs_dir: &PathBuf, commit: String) {
    let objects_dir_path = &PathBuf::from(vcs_dir).join("objects");

    let file_size_string = get_size_of_the_file(path).to_string();
    let file_size: &str = &file_size_string[..];
    let object_type: String = "blob".to_string();
    let mut data = object_type;
    data.push(' ');
    data.push_str(file_size);
    data.push('\0');
    let zlib_text = compress_zlib(path);
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
    let ref_path = PathBuf::from(objects_dir_path.join(ref_prefix)).join(ref_file_name);
    if !file_path.exists() {
        fs::create_dir(objects_dir_path.join(ref_prefix)).unwrap();
    }
    let refdata: String = path.display().to_string() + " " + &object_hash + "\n";
    let mut ref_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&ref_path)
        .unwrap();
    ref_file.write_all(refdata.as_bytes()).unwrap();
}

pub fn create_objects(path: &PathBuf, vcs_dir: &PathBuf, commit: String) {
    if metadata(&path).unwrap().is_file() {
        create_blob_object(path, vcs_dir, commit);
    } else if metadata(&path).unwrap().is_dir() && !path.display().to_string().contains(".vcs") {
        //create_tree_object(path.clone(), vcs_dir.clone(), commit.clone());
        let paths = fs::read_dir(&path).unwrap();
        for p in paths {
            create_objects(&p.as_ref().unwrap().path(), vcs_dir, commit.clone());
        }
    }
}

pub fn commit(message: String) -> std::io::Result<()> {
    if nothing_to_commit() {
        println!("Nothing to commit.");
        return Ok(());
    }
    let current_branch: String = fs::read_to_string(PathBuf::from(".vcs").join("HEAD")).unwrap();
    let current_commit =
        fs::read_to_string(PathBuf::from(".vcs").join("branches").join(&current_branch)).unwrap();
    let branch_path = PathBuf::from(".vcs/branches").join(current_branch);
    let commit_hash = get_hash_file(
        &PathBuf::from(".vcs")
            .join("objects")
            .join(current_commit[0..2].to_string())
            .join(current_commit[2..].to_string()),
    );
    write(&branch_path, &commit_hash).unwrap();

    //create commit object, write current commit hash there
    let commit_dir_name: &str = &commit_hash[0..2];
    let commit_file_name: &str = &commit_hash[2..];
    fs::create_dir_all(PathBuf::from(".vcs").join("objects").join(commit_dir_name)).unwrap();
    let commit_path =
        PathBuf::from(".vcs/objects/".to_string() + commit_dir_name).join(commit_file_name);

    write(&commit_path, message + "\n").unwrap();
    let mut commit_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&commit_path)
        .unwrap();
    commit_file
        .write_all((current_commit + "\n").as_bytes())
        .unwrap();

    //create objects for commit
    let workdir = get_workdir();
    create_objects(&PathBuf::from(workdir), &PathBuf::from(".vcs"), commit_hash);
    Ok(())
}
