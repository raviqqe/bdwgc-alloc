extern crate bdwgc_alloc;

use bdwgc_alloc::Allocator;
use std::alloc::{GlobalAlloc, Layout};

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

fn main() {
    unsafe { Allocator::initialize() }

    let handle = std::thread::spawn(move || {
        unsafe { Allocator::register_current_thread().unwrap() }

        loop {
            unsafe { GLOBAL_ALLOCATOR.alloc(Layout::from_size_align(2 ^ 8, 8).unwrap()) };
        }
    });

    handle.join().unwrap();
}
