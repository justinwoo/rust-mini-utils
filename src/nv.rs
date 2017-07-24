use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    Command::new("gvim")
      .args(args)
      .spawn()
      .expect("Error:")
      .wait()
      .expect("Error:");
}
