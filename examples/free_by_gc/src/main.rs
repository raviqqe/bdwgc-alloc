use bdwgc_alloc::Allocator;
use std::alloc::Layout;

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

fn main() {
    unsafe { Allocator::initialize() }

    loop {
        let _ = unsafe { std::alloc::alloc(Layout::from_size_align(2 ^ 8, 8).unwrap()) };
    }
}
