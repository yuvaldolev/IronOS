use const_default::ConstDefault;

use crate::architecture::x86_64::interrupts::interrupt_descriptor_type_attributes::InterruptDescriptorTypeAttributes;

#[derive(ConstDefault, Default)]
#[repr(C, packed)]
pub struct InterruptDescriptor {
    offset_1: u16,
    selector: u16,
    interrupt_stack_table: u8,
    type_attributes: u8,
    offset_2: u16,
    offset_3: u32,
    zero: u32,
}

impl InterruptDescriptor {
    pub fn set_handler(&mut self, handler: *const ()) {
        // Set the handler's offset.
        let offset = handler as usize;
        self.offset_1 = offset as u16;
        self.offset_2 = (offset >> 16) as u16;
        self.offset_3 = (offset >> 32) as u32;

        // Set the selector to the Global Descriptor Table's code segment.
        self.selector = 8;

        // Set the type attributes.
        self.type_attributes = (InterruptDescriptorTypeAttributes::PRESENT
            | InterruptDescriptorTypeAttributes::DPL_RING_0
            | InterruptDescriptorTypeAttributes::GATE_TYPE_INTERRUPT)
            .bits();
    }
}
