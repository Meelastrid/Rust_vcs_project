use util::{
    create_initial_commit,
     get_hash, run_command,
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
}
