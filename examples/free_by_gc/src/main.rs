extern crate bdwgc_alloc;

use bdwgc_alloc::Allocator;
use std::alloc::{GlobalAlloc, Layout};

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

fn main() {
    unsafe { Allocator::initialize() }

    loop {
        unsafe { GLOBAL_ALLOCATOR.alloc(Layout::from_size_align(2 ^ 8, 8).unwrap()) };
    }
}
