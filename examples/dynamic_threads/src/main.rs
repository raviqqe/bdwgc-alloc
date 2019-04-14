extern crate bdwgc_allocator;

use bdwgc_allocator::Allocator;
use std::alloc::{GlobalAlloc, Layout};

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

fn main() {
    unsafe { Allocator::initialize() }

    loop {
        let handle = std::thread::spawn(move || {
            Allocator::register_current_thread().unwrap();

            let mut _n =
                unsafe { GLOBAL_ALLOCATOR.alloc(Layout::from_size_align(2 ^ 8, 8).unwrap()) };

            for _ in 0..100 {
                _n = unsafe { GLOBAL_ALLOCATOR.alloc(Layout::from_size_align(2 ^ 8, 8).unwrap()) }
            }

            Allocator::unregister_current_thread();
        });

        handle.join().unwrap();
    }
}
