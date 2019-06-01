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
    let mut handle = Coroutine::spawn(move |me, _| {
        let bottom: u8 = 0;
        unsafe { Allocator::set_stack_bottom(&bottom) }
        Allocator::unlock();

        me.yield_with(42);
        unsafe { Allocator::set_stack_bottom(&bottom) }
        Allocator::unlock();

        loop {
            unsafe { std::alloc::alloc(Layout::from_size_align(2 ^ 8, 8).unwrap()) };
        }
    });

    Allocator::lock();
    assert_eq!(handle.resume(0).unwrap(), 42);

    handle.resume(0).unwrap();
}
