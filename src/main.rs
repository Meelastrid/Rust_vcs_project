use std::fs;
use util::{
    compress_zlib, create_blob_object, create_initial_commit, create_objects, create_tree_object,
    decompress_zlib, get_hash, run_command,
};

use crate::util::get_command_line_args;

mod commands;
mod commit;
mod init;
mod jump;
mod log;
mod merge;
mod new_branch;
mod status;
mod util;

fn main() {
    let args = get_command_line_args();
    let ok = run_command(&args);
    // create_objects("/home/meelastrid/Education/EduRust/VcsProject/vcs/src".to_string(), &args);
    //fs::create_dir("test/vcs").unwrap();
    //create_tree_object("test".to_string());
    // println!("{}", s);
}
