mod mini_utils;
use std::env;
use std::process::Command;

fn main() {
    let args = env::args();
    let branch_name = mini_utils::get_branch_name();
    let mut cmd: Vec<String> = vec![
        String::from("push"),
        String::from("-u"),
        String::from("origin"),
        branch_name
    ];
    cmd.append(&mut args.skip(1).collect());

    Command::new("git")
        .args(cmd)
        .spawn()
        .expect("Error:")
        .wait()
        .expect("Error:");
}
