#[repr(C, packed)]
pub struct InterruptDescriptorTableDescriptor {
    size: u16,
    offset: u64,
}

impl InterruptDescriptorTableDescriptor {
    pub fn new(size: u16, offset: u64) -> Self {
        Self {
            size: size - 1,
            offset,
        }
    }
}
