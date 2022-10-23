use util::{create_initial_commit, get_command_line_args, get_hash, run_command};

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
    run_command(&args).unwrap();
}
