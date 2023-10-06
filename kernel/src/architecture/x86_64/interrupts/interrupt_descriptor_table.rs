use core::arch;
use core::marker::PhantomPinned;
use core::mem;
use core::pin::Pin;

use const_default::ConstDefault;

use crate::architecture::x86_64::interrupts::exception_handlers;
use crate::architecture::x86_64::interrupts::interrupt_descriptor::InterruptDescriptor;
use crate::architecture::x86_64::interrupts::interrupt_descriptor_table_descriptor::InterruptDescriptorTableDescriptor;

const INTERRUPT_DESCRIPTOR_TABLE_ENTRIES: usize = 256;

#[derive(ConstDefault)]
pub struct InterruptDescriptorTable {
    entries: [InterruptDescriptor; INTERRUPT_DESCRIPTOR_TABLE_ENTRIES],
    _pin: PhantomPinned,
}

// TODO: This struct should be allocated on the heap, using `Box::pin`,
// from within the `InterruptDescriptorTable::new` method.
// However, we don't yet have a global allocator available for use.
// Update this when a global allocator is implemented.
static mut INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable =
    <InterruptDescriptorTable as ConstDefault>::DEFAULT;

impl InterruptDescriptorTable {
    pub fn new() -> Pin<&'static mut InterruptDescriptorTable> {
        let mut interrupt_descriptor_table = InterruptDescriptorTable::default();

        // Initialize the exception handlers.
        interrupt_descriptor_table.entries[3]
            .set_handler(exception_handlers::breakpoint_handler as *const ());

        // Pin the Interrupt Descriptor Table in memory.
        //
        // TODO: This is currently done via a static variable, however it should
        // be really allocated on the heap.
        // Update this when a global allocator is implemented.
        //
        // NOTE: This operation is safe, as the Interrupt Descriptor Table is initialized here
        // and thus no one can access it but us.
        let pinned_interrupt_descriptor_table = unsafe {
            INTERRUPT_DESCRIPTOR_TABLE = interrupt_descriptor_table;
            Pin::new_unchecked(&mut INTERRUPT_DESCRIPTOR_TABLE)
        };

        // Load the Interrupt Descriptor Table.
        pinned_interrupt_descriptor_table.load();

        pinned_interrupt_descriptor_table
    }

    fn load(&self) {
        // Construct an Interrupt Descriptor Table descriptor.
        let descriptor = InterruptDescriptorTableDescriptor::new(
            mem::size_of_val(&self.entries) as u16,
            (&self.entries as *const _) as u64,
        );

        // Load the Interrupt Descriptor Table using the descriptor.
        unsafe { arch::asm!("lidt [{}]", in(reg) &descriptor, options(nostack)) };
    }
}

impl Default for InterruptDescriptorTable {
    fn default() -> Self {
        Self {
            entries: [InterruptDescriptor::default(); INTERRUPT_DESCRIPTOR_TABLE_ENTRIES],
            _pin: PhantomPinned,
        }
    }
}

pub fn initialize() {
    let interrupt_descriptor_table = InterruptDescriptorTable::new();
}
