#![no_std]
#![feature(abi_x86_interrupt)]

mod architecture;

pub use architecture::interrupts;
