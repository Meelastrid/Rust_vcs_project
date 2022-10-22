use flate2::read;
use crate::util::get_hash_file;
use crate::commit::commit;
use array_tool::vec::{Intersect, Uniq, Union};
use std::collections::HashMap;
use std::fs::{self, write, metadata, File};
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

pub fn get_current_branch() -> String {
    fs::read_to_string("test/.vcs/HEAD").unwrap()
}

pub fn get_head_in_branch(branch: String) -> String {
    fs::read_to_string(["test/.vcs/branches", &branch].join("/")).unwrap()
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

pub fn get_files_in_branch(br: String) -> HashMap<String, String> {
    let mut files: HashMap<String, String> = HashMap::new();
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

pub fn get_parent(commit: String) -> String {

        let vcs_dir = "test/.vcs/objects".to_string();
        let ref_prefix: String = commit[0..2].to_string();
        let ref_file_name: String = commit[2..].to_string();
        let contents = fs::read_to_string([vcs_dir, ref_prefix, ref_file_name].join("/")).unwrap();

        let mut lns = contents.lines();
        lns.next();
        lns.next().unwrap().to_string()
}

pub fn find_common_commit(branch1: String, branch2: String) -> String {
    let branch1_commit = get_head_in_branch(branch1);
    let mut branch2_commit = get_head_in_branch(branch2);

    while branch2_commit != "null" {
        let mut temp = branch1_commit.clone();
        while temp != "null" {
            if temp == branch2_commit {
                return branch2_commit; }
                temp = get_parent(temp);
            }
            branch2_commit = get_parent(branch2_commit);
        }
        "null".to_string()
}
pub fn check_modified_files_between_commits(commit1: String, commit2: String) -> Vec<String> {

    let mut files_in_commit1: HashMap<String, String> = HashMap::new();
    let mut files_in_commit2: HashMap<String, String> = HashMap::new();
    let reader1 = BufReader::new(
        File::open("test/.vcs/objects/".to_string() + &commit1[..2] + "/" + &commit1[2..]).unwrap(),
    );
    let mut commit_read = 2;
    for line in reader1.lines() {
        if commit_read > 0 {
            commit_read -= 1;
            continue;
        }
        for (prefix, suffix) in line.unwrap().split_once(" ") {
            files_in_commit1.insert(prefix.to_string(), suffix.to_string());
        }
    }
    let reader2 = BufReader::new(
        File::open("test/.vcs/objects/".to_string() + &commit2[..2] + "/" + &commit2[2..]).unwrap(),
    );
    commit_read = 2;
    for line in reader2.lines() {
        if commit_read > 0 {
            commit_read -= 1;
            continue;
        }
        for (prefix, suffix) in line.unwrap().split_once(" ") {
            files_in_commit2.insert(prefix.to_string(), suffix.to_string());
        }
    }

    let files_list1: Vec<String> = files_in_commit1.keys().cloned().collect();
    let files_list2: Vec<String> = files_in_commit2.keys().cloned().collect();
    let common_files: Vec<String> = files_list1.intersect(files_list2);
    let mut diff: Vec<String> = Vec::new();
    for file in common_files {
        if files_in_commit1[file.as_str()] != files_in_commit2[file.as_str()] {
            diff.push(file);
        }
    }
    diff
}

pub fn merge(branch: String) -> std::io::Result<()> {
    if get_current_branch() != "master" {
        println!("Not master. Merge can only be done to master branch.");
        return Ok(());
    }

    if !nothing_to_commit() {
        println!("Branch dirty. Please commit your changes before merging.");
        return Ok(());
    }
    let our_files: HashMap<String, String> = get_files_in_branch("master".to_string());
    let our_files_list: Vec< String> = get_files_in_branch("master".to_string()).keys().cloned().collect();
    let their_files: HashMap<String, String> = get_files_in_branch(branch.clone());
    let their_files_list: Vec<String> = get_files_in_branch(branch.clone()).keys().cloned().collect();
    let common_commit = find_common_commit("master".to_string(), branch.clone());


    let my_modified = check_modified_files_between_commits(get_head_in_branch("master".to_string()), common_commit);
    let their_modified = check_modified_files_between_commits(get_head_in_branch("master".to_string()), get_head_in_branch(branch.clone()));
    let conflict = my_modified.intersect(their_modified.clone());
    if !conflict.is_empty() {
    println!("Merge confilict: file has been changed both in master and branch.");
    println!("Files: {:?}", conflict);
    println!("Aborting...");
    return Ok(());

    }
    let files_to_add = their_files_list.uniq(our_files_list.clone());
    let files_to_remove = our_files_list.uniq(their_files_list);

    for fa in files_to_add.clone() {
        let mut f =
            File::open("test/.vcs/objects/".to_string() + &their_files[&fa][0..2] + "/" + &their_files[&fa][2..]).unwrap();
        let mut data = Vec::new();
        f.read_to_end(&mut data).unwrap();
        let mut gz = read::GzDecoder::new(&data[..]);
        let mut text = String::new();
        gz.read_to_string(&mut text).unwrap();
        let path = std::path::Path::new(&fa);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        write(&path, text.clone()).unwrap();
    }
    for fr in files_to_remove.clone() {
    fs::remove_file("test/".to_string() + &fr).unwrap();
    }
    for fm in their_modified.clone() {
        let mut f =
            File::open("test/.vcs/objects/".to_string() + &their_files[&fm][0..2] + "/" + &their_files[&fm][2..]).unwrap();
        let mut data = Vec::new();
        f.read_to_end(&mut data).unwrap();
        let mut gz = read::GzDecoder::new(&data[..]);
        let mut text = String::new();
        gz.read_to_string(&mut text).unwrap();
        let path = std::path::Path::new(&fm);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        write(&path, text.clone()).unwrap();
    }

    commit("Merged branch ".to_string() + &branch).unwrap();
    fs::remove_file("test/.vcs/branches/".to_string() + &branch).unwrap();




    println!("Successfully created merge commit:");
    println!("[master {}] Merged branch {}", get_head_in_branch("master".to_string()), branch);
    println!("Files added: {:?}", files_to_add);
    println!("Files to deleted: {:?}", files_to_remove);
    println!("Files to modified: {:?}", their_modified);
    println!("Deleted branch: {}", branch);
    Ok(())
}
