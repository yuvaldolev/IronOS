#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader_api::info::FrameBufferInfo;
use bootloader_api::BootInfo;
use bootloader_x86_64_common::logger::LockedLogger;
use conquer_once::spin::OnceCell;

use kernel::Kernel;

bootloader_api::entry_point!(kernel_main);

static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

/// Invoked by the bootloader to start the kernel.
fn kernel_main(boot_information: &'static mut BootInfo) -> ! {
    // Extract the framebuffer from to boot information struct.
    let (framebuffer, framebuffer_info) = extract_framebuffer(boot_information);

    // Initialize the kernel framebuffer logger.
    initialize_logger(framebuffer, framebuffer_info);

    // Initialize the Kernel.
    log::info!("IronOS Kernel is initializing");
    let _kernel = Kernel::new();
    // x86_64::instructions::interrupts::int3();
    log::info!("IronOS Kernel initialization completed!");

    loop {}
}

/// Invoked on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("KERNEL PANIC: {info}");
    loop {}
}

fn extract_framebuffer(
    boot_information: &'static mut BootInfo,
) -> (&'static mut [u8], FrameBufferInfo) {
    // Free the doubly wrapped framebuffer from the boot info struct.
    let framebuffer_optional = &mut boot_information.framebuffer;

    // Free the wrapped framebuffer from the FFI-safe abstraction provided by bootloader_api.
    let framebuffer_option = framebuffer_optional.as_mut();

    // Unwrap the framebuffer.
    let framebuffer = framebuffer_option.unwrap();

    // Extract the framebuffer info and, to satisfy the borrow checker, clone it.
    let framebuffer_info = framebuffer.info().clone();

    // Get the framebuffer's mutable raw byte slice.
    let raw_framebuffer = framebuffer.buffer_mut();

    (raw_framebuffer, framebuffer_info)
}

fn initialize_logger(framebuffer: &'static mut [u8], framebuffer_info: FrameBufferInfo) {
    let logger =
        LOGGER.get_or_init(move || LockedLogger::new(framebuffer, framebuffer_info, true, false));
    log::set_logger(logger).expect("Logger already set");
    log::set_max_level(log::LevelFilter::Trace);
}
