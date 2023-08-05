use std::process::{self, Command};

fn main() {
    // Read the environment variables that were set in build script.
    let uefi_image = env!("UEFI_IMAGE");

    // Run IronOS in QEMU.
    let mut qemu = Command::new("qemu-system-x86_64");
    qemu.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
    qemu.arg("-drive")
        .arg(format!("format=raw,file={uefi_image}"));
    let exit_status = qemu.status().unwrap();
    process::exit(exit_status.code().unwrap_or(-1));
}
