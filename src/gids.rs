use std::process::Command;

fn main() {
    Command::new("git")
        .args(vec![String::from("diff"), String::from("--staged")])
        .spawn()
        .expect("Error:")
        .wait()
        .expect("Error:");
}
