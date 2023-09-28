use bdwgc_alloc::Allocator;
use std::alloc::Layout;

const ITERATION_COUNT: usize = 1000_000;

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

fn main() {
    unsafe { Allocator::initialize() }

    let handle = std::thread::spawn(move || {
        unsafe { Allocator::register_current_thread().unwrap() }

        for _ in 0..ITERATION_COUNT {
            let _ = unsafe { std::alloc::alloc(Layout::from_size_align(2 ^ 8, 8).unwrap()) };
        }
    });

    handle.join().unwrap();
}
