bitflags::bitflags! {
    pub struct InterruptDescriptorTypeAttributes: u8 {
        const GATE_TYPE_INTERRUPT = 0xE;
        const GATE_TYPE_TRAP = 0xF;
        const DPL_RING_0 = 0 << 5;
        const DPL_RING_1 = 1 << 5;
        const DPL_RING_2 = 2 << 5;
        const DPL_RING_3 = 3 << 5;
        const PRESENT = 1 << 7;
    }
}
