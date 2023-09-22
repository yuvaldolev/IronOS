#[cfg(target_arch = "x86_64")]
use crate::architecture::x86_64::X86_64;

pub struct Architecture {
    #[cfg(target_arch = "x86_64")]
    api: X86_64,
}

impl Architecture {
    pub fn new() -> Self {
        #[cfg(target_arch = "x86_64")]
        let api = X86_64::new();

        Self { api }
    }
}
