use std::process::Command;

fn main() {
    Command::new("git")
        .args(vec![String::from("branch")])
        .spawn()
        .expect("Error:")
        .wait()
        .expect("Error:");
}
