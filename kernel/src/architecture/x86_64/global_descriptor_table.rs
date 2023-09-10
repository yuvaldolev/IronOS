use x86_64::instructions::segmentation::{Segment, CS};
use x86_64::instructions::tables;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub const DOUBLE_FAULT_INTERRUPT_STACK_TABLE_INDEX: u16 = 0;

const STACK_SIZE: usize = 4096 * 5;

// WARN: This stack doesn't have a guard page. Use with caution!
// TODO: This should be replaced with a proper stack allocation when one is implemented.
static mut DOUBLE_FAULT_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

lazy_static::lazy_static! {
    static ref TASK_STATE_SEGMENT: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_INTERRUPT_STACK_TABLE_INDEX as usize] =
            VirtAddr::from_ptr(unsafe { &DOUBLE_FAULT_STACK }) + STACK_SIZE;
        tss
    };
}

lazy_static::lazy_static! {
    static ref GLOBAL_DESCRIPTOR_TABLE: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TASK_STATE_SEGMENT));
        let selectors = Selectors {
            code_selector,
            tss_selector,
        };
        (gdt, selectors)
    };
}

pub fn initialize() {
    // Load the Global Descriptor Table.
    GLOBAL_DESCRIPTOR_TABLE.0.load();

    // Reload the CS register.
    unsafe { CS::set_reg(GLOBAL_DESCRIPTOR_TABLE.1.code_selector) };

    // Load the Task State Segment.
    unsafe { tables::load_tss(GLOBAL_DESCRIPTOR_TABLE.1.tss_selector) };
}
