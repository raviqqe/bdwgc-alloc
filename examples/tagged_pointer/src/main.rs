use bdwgc_alloc::Allocator;
use std::alloc::{alloc, Layout};

const BITS: usize = (1 << 8 - 1) << 48;

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

fn main() {
    unsafe { Allocator::initialize() }

    let x = allocate();

    unsafe { *x = 42 };

    let x = x as usize | BITS;
    let mut xs = vec![];

    loop {
        assert_eq!(x & BITS, BITS);
        assert_eq!(unsafe { *((x & !BITS) as *mut usize) }, 42);

        let ptr = allocate();
        unsafe { *ptr = 0 };
        xs.push(ptr);

        Allocator::force_collect();
    }
}

fn allocate() -> *mut usize {
    (unsafe { alloc(Layout::from_size_align(1 << 8, 8).unwrap()) }) as *mut usize
}
