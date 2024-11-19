fn main() {     //KEYBOARD LAYOUT CHANGER
    //get current layout
    let output = std::process::Command::new("setxkbmap").arg("-query").output().expect("failed to execute process");
    let output = String::from_utf8_lossy(&output.stdout);
    //let lang:&str = if output.contains("tr\n") { "us" } else { "tr" };
    std::process::Command::new("setxkbmap")
        .args(&["-layout", if output.contains("tr\n") { "us" } else { "tr" }])
        .output()
        .expect("failed to execute process");
}