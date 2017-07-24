use std::process::Command;

fn main() {
    Command::new("git")
        .arg(String::from("branch"))
        .spawn()
        .expect("Error:")
        .wait()
        .expect("Error:");
}
