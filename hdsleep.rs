use std::process::Command;

const HD_PARM: &str = "/usr/bin/hdparm";
const TIMEOUT: u16 = 60;
const DEVICES: [&str; 2] = [
    "ata-ST6000DM003-2CY186_ZCT2LADB",
    "ata-ST6000DM003-2CY186_WCT3JNNM",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum DriveStatus {
    Active,
    Standby,
    Sleeping,
}

fn main() {
    let uid = unsafe { libc::geteuid() };
    if uid != 0 {
        println!("{}: must be run as root.", env!("CARGO_BIN_NAME"));
        std::process::exit(1);
    }

    let mut drives = DEVICES.to_vec();
    drives.retain(|drive| check_status(drive).is_some_and(|status| status == DriveStatus::Active));
    if drives.is_empty() {
        println!("{}: no active drives.", env!("CARGO_BIN_NAME"));
        std::process::exit(0);
    }

    for drive in drives {
        Command::new(HD_PARM)
            .arg("-S")
            .arg(&TIMEOUT.to_string())
            .arg(drive)
            .status()
            .expect("Failed to set timeout with hdparm");

        Command::new(HD_PARM)
            .arg("-y")
            .arg(drive)
            .status()
            .expect("Failed to put drive(s) to sleep");
    }
}

fn check_status(dev: &str) -> Option<DriveStatus> {
    let output = Command::new(HD_PARM).arg("-C").arg(dev).output().ok()?;
    if output.status.success() {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            if stdout.contains("active") || stdout.contains("idle") {
                return Some(DriveStatus::Active);
            }

            if stdout.contains("standby") {
                return Some(DriveStatus::Standby);
            }

            if stdout.contains("sleeping") {
                return Some(DriveStatus::Sleeping);
            }
        }
    }

    None
}
