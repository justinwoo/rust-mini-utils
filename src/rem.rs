// runemacs

use std::env;
use std::process::Command;

fn main() {
    let args = env::args();
    let mut cmd: Vec<String> = vec![String::from("add")];
    cmd.append(&mut args.skip(1).collect());

    Command::new("runemacs")
        .args(cmd)
        .spawn()
        .expect("Error:")
        .wait()
        .expect("Error:");
}
