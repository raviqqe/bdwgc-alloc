extern crate bdwgc_allocator;

use bdwgc_allocator::Allocator;
use std::alloc::{GlobalAlloc, Layout};

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

fn main() {
    unsafe { Allocator::initialize() }

    let mut _n = unsafe { GLOBAL_ALLOCATOR.alloc(Layout::from_size_align(2 ^ 8, 8).unwrap()) };

    loop {
        _n = unsafe { GLOBAL_ALLOCATOR.alloc(Layout::from_size_align(2 ^ 8, 8).unwrap()) };
    }
}
