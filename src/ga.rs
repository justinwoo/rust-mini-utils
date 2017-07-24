use std::env;
use std::process::Command;

fn main() {
    let args = env::args();
    match args.len() {
        1 => {
            println!("Please come back with files you want to add");
        }
        _ => {
            let mut cmd: Vec<String> = vec![String::from("add")];
            cmd.append(&mut args.skip(1).collect());

            Command::new("git")
                .args(cmd)
                .spawn()
                .expect("Error:")
                .wait()
                .expect("Error:");
        }
    }
}
