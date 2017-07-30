use std::process::Command;

fn main() {
    Command::new("git")
        .args(vec![
            String::from("pull"),
            String::from("--prune")
        ])
        .spawn()
        .expect("Error:")
        .wait()
        .expect("Error:");
}
