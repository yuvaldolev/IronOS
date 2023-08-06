#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader_api::info::FrameBufferInfo;
use bootloader_api::BootInfo;
use bootloader_x86_64_common::logger::LockedLogger;
use conquer_once::spin::OnceCell;

bootloader_api::entry_point!(kernel_main);

static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

/// Invoked by the bootloader to start the kernel.
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // Free the doubly wrapped framebuffer from the boot info struct.
    let frame_buffer_optional = &mut boot_info.framebuffer;

    // Free the wrapped framebuffer from the FFI-safe abstraction provided by bootloader_api.
    let frame_buffer_option = frame_buffer_optional.as_mut();

    // Unwrap the framebuffer.
    let frame_buffer = frame_buffer_option.unwrap();

    // Extract the framebuffer info and, to satisfy the borrow checker, clone it.
    let frame_buffer_info = frame_buffer.info().clone();

    // Get the framebuffer's mutable raw byte slice.
    let raw_frame_buffer = frame_buffer.buffer_mut();

    // Finally, initialize the logger using the last two variables.
    init_logger(raw_frame_buffer, frame_buffer_info);

    // Kernel initialization completed.
    log::info!("IronOS Kernel is initialized!");

    loop {}
}

/// Invoked on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("KERNEL PANIC: {info}");
    loop {}
}

fn init_logger(buffer: &'static mut [u8], info: FrameBufferInfo) {
    let logger = LOGGER.get_or_init(move || LockedLogger::new(buffer, info, true, false));
    log::set_logger(logger).expect("Logger already set");
    log::set_max_level(log::LevelFilter::Trace);
}
