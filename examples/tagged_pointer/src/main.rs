use bdwgc_alloc::Allocator;
use std::{
    alloc::{alloc, Layout},
    thread::spawn,
};

const BITS: usize = 7 << 48;

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

fn main() {
    unsafe { Allocator::initialize() }

    let t1 = spawn(|| {
        unsafe { Allocator::register_current_thread().unwrap() }

        loop {
            let ptr = allocate();
            unsafe { *ptr = 0 };
        }
    });

    let t2 = spawn(|| {
        unsafe { Allocator::register_current_thread().unwrap() }

        let x = allocate();

        unsafe { *x = 42 };

        loop {
            assert_eq!(unsafe { *((x as usize & !BITS) as *mut usize) }, 42);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

fn allocate() -> *mut usize {
    (unsafe { alloc(Layout::new::<usize>()) }) as *mut usize
}
