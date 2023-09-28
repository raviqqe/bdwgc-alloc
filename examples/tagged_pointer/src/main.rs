use bdwgc_alloc::Allocator;
use std::alloc::{alloc, Layout};

const BITS: usize = 7 << 48;

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

fn main() {
    unsafe { Allocator::initialize() }

    let mut xs = vec![];

    for i in 0..10000000 {
        let ptr = unsafe { alloc(Layout::new::<usize>()) } as *mut usize;

        unsafe { *ptr = i };

        xs.push(ptr as usize | BITS);
    }

    for x in xs {
        println!("{}", unsafe { *((x & !BITS) as *mut usize) });
    }
}
