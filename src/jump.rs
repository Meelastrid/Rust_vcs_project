use flate2::read;
use std::collections::HashMap;
use std::fs::{self, write, File};
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;


pub fn get_workdir() -> String {
    fs::read_to_string(".vcs/config").unwrap()
}

pub fn restore_files_in_commit(commit: String) {
    let mut files: HashMap<String, String> = HashMap::new();
    let reader = BufReader::new(
        File::open(".vcs/objects/".to_string() + &commit[..2] + "/" + &commit[2..]).unwrap(),
    );
    let mut header_read = 2;
    for line in reader.lines() {
        if header_read != 0 {
            header_read -= 1;
            continue;
        }
        for (prefix, suffix) in line.unwrap().split_once(" ") {
            files.insert(prefix.to_string(), suffix.to_string());
        }
    }
    for (file, hash) in files.iter() {
        let mut f =
            File::open(".vcs/objects/".to_string() + &hash[..2] + "/" + &hash[2..]).unwrap();

        let mut data = Vec::new();
        f.read_to_end(&mut data).unwrap();
        let mut gz = read::GzDecoder::new(&data[..]);
        let mut text = String::new();
        gz.read_to_string(&mut text).unwrap();
        let path = std::path::Path::new(file);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        write(&path, text.clone()).unwrap();
    }
}
pub fn delete_all_files(path: String) {
    for entry in fs::read_dir(path).unwrap() {
        if entry
            .as_ref()
            .unwrap()
            .path()
            .display()
            .to_string()
            .contains(".vcs")
        {
            continue;
        }
        if entry.as_ref().unwrap().file_type().unwrap().is_dir() {
            fs::remove_dir_all(entry.unwrap().path()).unwrap();
        } else {
            fs::remove_file(entry.unwrap().path()).unwrap();
        }
    }
}

pub fn restore_all_files_branch(name: String) -> std::io::Result<()> {
    let commit = fs::read_to_string(".vcs/branches/".to_string() + &name).unwrap();

    restore_files_in_commit(commit);

    let path: String = ".vcs".to_string();
    let file_path = PathBuf::from(&path).join("HEAD");
    write(&file_path, name).unwrap();

    Ok(())
}

pub fn jump(t: String, name: String) -> std::io::Result<()> {
    let workdir = get_workdir();
    delete_all_files(workdir);
    if t == "--branch" {
        restore_all_files_branch(name)?;
    } else if t == "--commit" {
        restore_files_in_commit(name);
    }
    Ok(())
}
