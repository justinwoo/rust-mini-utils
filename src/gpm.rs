use std::process::Command;

fn main() {
    Command::new("git")
        .args(vec![
            String::from("push"),
            String::from("-u"),
            String::from("mine"),
            String::from("$(git rev-parse --abbrev-ref HEAD)"),
        ])
        .spawn()
        .expect("Error:")
        .wait()
        .expect("Error:");
}
