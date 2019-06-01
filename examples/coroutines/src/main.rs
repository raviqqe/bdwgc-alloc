extern crate bdwgc_alloc;
extern crate coroutine;

use bdwgc_alloc::Allocator;
use coroutine::asymmetric::Coroutine;
use std::alloc::Layout;

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

fn main() {
    unsafe { Allocator::initialize() }
    Allocator::lock();

    let handle = Coroutine::spawn(move |_, _| {
        let bottom: u8 = 0;
        unsafe { Allocator::set_stack_bottom(&bottom) }
        Allocator::unlock();

        loop {
            unsafe { std::alloc::alloc(Layout::from_size_align(2 ^ 8, 8).unwrap()) };
        }
    });

    for _ in handle {}
}
