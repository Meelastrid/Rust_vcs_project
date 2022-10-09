use util::{get_hash, compress_zlib, decompress_zlib, create_blob_object, run_command, create_objects, create_tree_object};

use crate::util::get_command_line_args;

mod commands;
mod commit;
mod init;
mod jump_to_branch;
mod jump_to_commit;
mod log;
mod new_branch;
mod status;
mod merge;
mod util;

fn main() {

   // let args = get_command_line_args();
   // run_command(&args);
   // create_objects("/home/meelastrid/Education/EduRust/VcsProject/vcs/src".to_string(), &args);
   create_tree_object();
   // println!("{}", s);

}