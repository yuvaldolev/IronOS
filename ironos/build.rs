use std::path::PathBuf;

fn main() {
    // Set by Cargo, build scripts should use this directory for output files.
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    // Set by Cargo's artifact dependency feature, see
    // https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies
    let kernel = PathBuf::from(std::env::var_os("CARGO_BIN_FILE_KERNEL_kernel").unwrap());

    // Create a BIOS disk image.
    let bios_image = out_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_image)
        .unwrap();

    // Create an UEFI disk image (optional).
    let uefi_image = out_dir.join("uefi.img");
    bootloader::UefiBoot::new(&kernel)
        .create_disk_image(&uefi_image)
        .unwrap();

    // Pass the disk image paths as env variables to the `main.rs`.
    println!("cargo:rustc-env=BIOS_IMAGE={}", bios_image.display());
    println!("cargo:rustc-env=UEFI_IMAGE={}", uefi_image.display());
}
