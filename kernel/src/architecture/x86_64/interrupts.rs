use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::architecture::x86_64::global_descriptor_table;

lazy_static::lazy_static! {
    static ref INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(global_descriptor_table::DOUBLE_FAULT_INTERRUPT_STACK_TABLE_INDEX)
        };
        idt
    };
}

pub fn initialize_interrupt_descriptor_table() {
    INTERRUPT_DESCRIPTOR_TABLE.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    log::info!("EXCEPTION: Breakpoint\n{stack_frame:#?}");
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!("EXCEPTION: Double Fault (error={error_code})\n{stack_frame:#?}");
}
