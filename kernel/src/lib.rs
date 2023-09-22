#![no_std]
#![feature(abi_x86_interrupt)]

mod architecture;
mod kernel;

pub use kernel::Kernel;
