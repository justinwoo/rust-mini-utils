use std::process::Command;

pub fn get_branch_name() -> String {
    let output = Command::new("git")
        .args(vec![
            String::from("rev-parse"),
            String::from("--abbrev-ref"),
            String::from("HEAD"),
        ])
        .output()
        .expect("Error:");

    let string = String::from_utf8(output.stdout).expect("why didn't this decode");
    return string.trim().to_owned();
}
