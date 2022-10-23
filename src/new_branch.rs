use std::fs::{self, write};
use std::path::PathBuf;
pub fn new_branch(name: String) -> std::io::Result<()> {
    let current_branch: String = fs::read_to_string(PathBuf::from(".vcs").join("HEAD")).unwrap();
    let current_commit =
        fs::read_to_string(PathBuf::from(".vcs").join("branches").join(&current_branch)).unwrap();
    let path: String = ".vcs".to_string();
    let file_path = [path.clone(), "HEAD".to_string()].join("/");
    let branch_path = PathBuf::from([path, "branches".to_string(), name.clone()].join("/"));
    if branch_path.exists() {
        println!("Branch already exists: {}", name);
        return Ok(());
    }

    write(&file_path, &name).unwrap();
    write(&branch_path, &current_commit).unwrap();
    Ok(())
}
