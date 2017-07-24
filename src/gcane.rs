use std::process::Command;

fn main() {
    Command::new("git")
        .args(vec![
            String::from("commit"),
            String::from("--amend"),
            String::from("--no-edit")
        ])
        .spawn()
        .expect("Error:")
        .wait()
        .expect("Error:");
}
