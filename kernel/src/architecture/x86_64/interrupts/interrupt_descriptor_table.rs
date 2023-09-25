pub struct InterruptDescriptorTable;

impl InterruptDescriptorTable {
    pub fn new() -> Self {
        // TODO: This struct should be allocated on the heap, using `Box::pin`.
        // However, we don't yet have a global allocator available for use.
        // Update this when a global allocator is implemented.
        Self
    }
}
