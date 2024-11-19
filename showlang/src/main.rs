use std::process::Command;
fn main() {
    let output = Command::new("setxkbmap").arg("-query").output().expect("failed to execute process");
    let output = String::from_utf8_lossy(&output.stdout);
    let lang:&str = if output.contains("tr\n") { "TR" } else { "EN" };
    println!("{}", lang);
}
