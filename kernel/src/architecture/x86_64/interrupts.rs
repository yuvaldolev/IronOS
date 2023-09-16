use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::instructions;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::architecture::x86_64::global_descriptor_table;

#[derive(Clone, Copy)]
#[repr(u8)]
enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static::lazy_static! {
    static ref INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // CPU exceptions.
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt.set_handler_fn(non_maskable_interrupt_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available.set_handler_fn(device_not_available_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(global_descriptor_table::DOUBLE_FAULT_INTERRUPT_STACK_TABLE_INDEX)
        };
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present.set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.x87_floating_point.set_handler_fn(x87_floating_point_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point.set_handler_fn(simd_floating_point_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        idt.vmm_communication_exception.set_handler_fn(vmm_communication_exception_handler);
        idt.security_exception.set_handler_fn(security_exception_handler);

        // Interrupts.
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        for interrupt in PIC_1_OFFSET + 1..=255 {
            idt[interrupt as usize].set_handler_fn(unhandled_interrupt_handler);
        }
        idt
    };
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        self.as_u8() as usize
    }
}

pub fn initialize_interrupt_descriptor_table() {
    INTERRUPT_DESCRIPTOR_TABLE.load();
}

pub fn initialize_pics() {
    unsafe { PICS.lock().initialize() };
}

pub fn enable() {
    instructions::interrupts::enable();
    instructions::hlt();
}

extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    log::info!("EXCEPTION: Divide Error\n{stack_frame:#?}");
}

extern "x86-interrupt" fn debug_handler(stack_frame: InterruptStackFrame) {
    log::info!("EXCEPTION: Debug\n{stack_frame:#?}");
}

extern "x86-interrupt" fn non_maskable_interrupt_handler(stack_frame: InterruptStackFrame) {
    log::info!("EXCEPTION: Non-Maskable Interrupt\n{stack_frame:#?}");
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    log::info!("EXCEPTION: Breakpoint\n{stack_frame:#?}");
}

extern "x86-interrupt" fn overflow_handler(stack_frame: InterruptStackFrame) {
    log::info!("EXCEPTION: Overflow\n{stack_frame:#?}");
}

extern "x86-interrupt" fn bound_range_exceeded_handler(stack_frame: InterruptStackFrame) {
    log::info!("EXCEPTION: Bound Range Exceeded\n{stack_frame:#?}");
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    log::info!("EXCEPTION: Invalid Opcode\n{stack_frame:#?}");
}

extern "x86-interrupt" fn device_not_available_handler(stack_frame: InterruptStackFrame) {
    log::info!("EXCEPTION: Device Not Available\n{stack_frame:#?}");
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!("EXCEPTION: Double Fault (error={error_code})\n{stack_frame:#?}");
}

extern "x86-interrupt" fn invalid_tss_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    log::info!("EXCEPTION: Invalid TSS (error={error_code})\n{stack_frame:#?}");
}

extern "x86-interrupt" fn segment_not_present_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    log::info!("EXCEPTION: Segment Not Present (error={error_code})\n{stack_frame:#?}");
}

extern "x86-interrupt" fn stack_segment_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    log::info!("EXCEPTION: Stack Segment Fault (error={error_code})\n{stack_frame:#?}");
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    log::info!("EXCEPTION: General Protection Fault (error={error_code})\n{stack_frame:#?}");
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    log::info!("EXCEPTION: Page Fault (error={error_code:?})\n{stack_frame:#?}");
}

extern "x86-interrupt" fn x87_floating_point_handler(stack_frame: InterruptStackFrame) {
    log::info!("EXCEPTION: X87 Floating-Point\n{stack_frame:#?}");
}

extern "x86-interrupt" fn alignment_check_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    log::info!("EXCEPTION: Alignment Check (error={error_code})\n{stack_frame:#?}");
}

extern "x86-interrupt" fn machine_check_handler(stack_frame: InterruptStackFrame) -> ! {
    panic!("EXCEPTION: Machine Check\n{stack_frame:#?}");
}

extern "x86-interrupt" fn simd_floating_point_handler(stack_frame: InterruptStackFrame) {
    log::info!("EXCEPTION: SIMD Floating-Point\n{stack_frame:#?}");
}

extern "x86-interrupt" fn virtualization_handler(stack_frame: InterruptStackFrame) {
    log::info!("EXCEPTION: Virtualization\n{stack_frame:#?}");
}

extern "x86-interrupt" fn vmm_communication_exception_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    log::info!("EXCEPTION: VMM Cummunication (error={error_code})\n{stack_frame:#?}");
}

extern "x86-interrupt" fn security_exception_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    log::info!("EXCEPTION: Security Exception (error={error_code})\n{stack_frame:#?}");
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    log::debug!(".");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn unhandled_interrupt_handler(stack_frame: InterruptStackFrame) {
    log::error!("Unhandled Interrupt\n{stack_frame:#?}")
}
