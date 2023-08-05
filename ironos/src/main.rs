use std::env;
use std::fs;

fn main() {
    let current_exe = env::current_exe().unwrap();

    let bios_target = current_exe.with_file_name("ironos_bios.img");
    let uefi_target = current_exe.with_file_name("ironos_uefi.img");

    fs::copy(env!("BIOS_IMAGE"), &bios_target).unwrap();
    fs::copy(env!("UEFI_IMAGE"), &uefi_target).unwrap();

    println!("BIOS disk image at {}", bios_target.display());
    println!("UEFI disk image at {}", uefi_target.display());
}
