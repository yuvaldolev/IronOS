use std::process::{self, Command};

fn main() {
    // Read the environment variable that was set in build script.
    let bios_image = env!("BIOS_IMAGE");

    // Run IronOS in BIOS mode using QEMU.
    let mut qemu = Command::new("qemu-system-x86_64");
    qemu.arg("-drive")
        .arg(format!("format=raw,file={bios_image}"));
    let exit_status = qemu.status().unwrap();
    process::exit(exit_status.code().unwrap_or(-1));
}
