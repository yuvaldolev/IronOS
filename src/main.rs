#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Invoked by the bootloader to start the kernel.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

/// Invoked on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
