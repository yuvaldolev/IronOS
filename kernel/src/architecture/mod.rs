#[cfg(target_arch = "x86_64")]
mod x86_64;

// TODO: This might be a x86-64 specific thing - so more abstraction might be needed here...
#[cfg(target_arch = "x86_64")]
pub use x86_64::global_descriptor_table;

#[cfg(target_arch = "x86_64")]
pub use x86_64::interrupts;
