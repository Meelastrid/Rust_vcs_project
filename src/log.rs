use std::fs;
use std::path::PathBuf;
use crate::status::get_current_branch;
pub fn log() -> std::io::Result<()> {
    println!("Log:");
    let vcs_dir = "test/.vcs".to_string();
    let br = get_current_branch();
    let commit = fs::read_to_string("test/.vcs/branches/".to_string() + &br).unwrap();
    let mut ref_prefix: &str = &commit[0..2];
    let mut ref_file_name: &str = &commit[2..];
    let mut objects_dir_path = vcs_dir + "/objects";
    let mut ref_path = PathBuf::from(objects_dir_path + "/" + ref_prefix).join(ref_file_name);
    let mut c: String = "valid".to_string();

    while c != "null" {
    let vcs_dir = "test/.vcs".to_string();
    let contents = fs::read_to_string(ref_path.clone()).unwrap();
    println!("{contents}");
    let mut lns = contents.lines();
    lns.next();
    c = lns.next().unwrap().to_string();
    ref_prefix = &c[0..2];
    ref_file_name = &c[2..];
    objects_dir_path = vcs_dir + "/objects";
    ref_path = PathBuf::from(objects_dir_path + "/" + ref_prefix).join(ref_file_name);


    }


    Ok(())
}
