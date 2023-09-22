use crate::architecture::Architecture;

pub struct Kernel {
    architecture: Architecture,
}

impl Kernel {
    pub fn new() -> Self {
        // Perform the architecture-specific initialization.
        log::info!("Starting architecture-specific initialization");
        let architecture = Architecture::new();
        log::info!("Architecture-specific initialization completed");

        Self { architecture }
    }
}
