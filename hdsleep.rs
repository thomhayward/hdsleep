use std::process::Command;

const HD_PARM: &str = "/usr/bin/hdparm";
const DEVICES: [&str; 2] = ["/dev/sda", "/dev/sdb"];
const TIMEOUT: u16 = 60;

fn main() {
    Command::new(HD_PARM)
        .arg("-S")
        .arg(&TIMEOUT.to_string())
        .args(&DEVICES)
        .status()
        .expect("Failed to set timeout with hdparm");

    Command::new(HD_PARM)
        .arg("-y")
        .args(&DEVICES)
        .status()
        .expect("Failed to set timeout with hdparm");
}
