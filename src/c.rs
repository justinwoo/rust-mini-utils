use std::process::Command;

fn main() {
    Command::new("cd")
        .arg(String::from("~/Code"))
        .spawn()
        .expect("Error:")
        .wait()
        .expect("Error:");
}
