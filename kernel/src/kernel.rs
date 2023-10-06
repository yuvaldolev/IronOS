use crate::architecture;

pub fn initialize() {
    // Perform the architecture-specific initialization.
    log::info!("Starting architecture-specific initialization");
    let architecture = architecture::initialize();
    log::info!("Architecture-specific initialization completed");
}
