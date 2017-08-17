use std::process::Command;

fn main() {
    Command::new("git")
        .arg(String::from("status"))
        .spawn()
        .expect("Error:")
        .wait()
        .expect("Error:");
}
