use std::process::Command;

fn main() {
    Command::new("git")
        .args(vec![String::from("diff")])
        .spawn()
        .expect("Error:")
        .wait()
        .expect("Error:");
}
