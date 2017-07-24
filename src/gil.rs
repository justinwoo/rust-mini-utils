use std::process::Command;

fn main() {
    Command::new("git")
        .args(vec![
            String::from("log"),
            String::from("--decorate"),
            String::from("--stat"),
            String::from("--graph"),
            String::from(
                "--pretty=format:\"%d %Cgreen%h%Creset (%ar - %Cred%an%Creset), %s%n\"",
            ),
        ])
        .spawn()
        .expect("Error:")
        .wait()
        .expect("Error:");
}
