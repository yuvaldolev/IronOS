mod architecture;
mod architecture_api;

#[cfg(target_arch = "x86_64")]
mod x86_64;

pub use architecture::Architecture;
