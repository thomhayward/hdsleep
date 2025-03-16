use std::process::Command;

const HD_PARM: &str = "/usr/bin/hdparm";
const TIMEOUT: u16 = 60;
const DEVICES: [&str; 2] = [
    "/dev/disk/by-id/ata-ST6000DM003-2CY186_ZCT2LADB",
    "/dev/disk/by-id/ata-ST6000DM003-2CY186_WCT3JNNM",
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

    for drive in DEVICES {
        // Setting the timeout will spin-up a sleeping drive. Only attempt to
        // set the timeout if the drive is active.
        if check_status(drive)
            .inspect(|status| eprintln!("{drive}: {status:?}"))
            .is_some_and(|status| status == DriveStatus::Active)
        {
            Command::new(HD_PARM)
                .arg("-S")
                .arg(&TIMEOUT.to_string())
                .arg(drive)
                .status()
                .expect("Failed to set timeout with hdparm");
        }

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
