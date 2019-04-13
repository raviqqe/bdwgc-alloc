extern crate libc;

mod error;

use libc::{c_int, c_void, size_t};
use std::alloc::{GlobalAlloc, Layout};

const GC_SUCCESS: c_int = 0;

#[repr(C)]
struct GCStackBase {
    mem_base: *const c_void,
    reg_base: *const c_void,
}

#[link(name = "gc")]
extern "C" {
    fn GC_allow_register_threads() -> c_void;
    fn GC_free(ptr: *mut c_void);
    fn GC_get_stack_base(stack_base: *mut GCStackBase) -> c_int;
    fn GC_init() -> c_void;
    fn GC_malloc(size: size_t) -> *mut c_void;
    fn GC_malloc_uncollectable(size: size_t) -> *mut c_void;
    fn GC_register_my_thread(stack_base: *const GCStackBase) -> c_int;
    fn GC_unregister_my_thread();
}

static mut GC_STARTED: bool = false;
pub struct Allocator;

impl Allocator {
    pub unsafe fn initialize() {
        GC_init();
        GC_allow_register_threads();
    }

    pub unsafe fn start_gc() {
        GC_STARTED = true
    }

    pub fn register_current_thread() -> Result<(), error::Error> {
        let mut base = GCStackBase {
            mem_base: std::ptr::null(),
            reg_base: std::ptr::null(),
        };

        if unsafe { GC_get_stack_base(&mut base) } != GC_SUCCESS {
            return Err(error::Error::new("failed to get stack base"));
        }

        if unsafe { GC_register_my_thread(&base) } != GC_SUCCESS {
            return Err(error::Error::new("failed to register a thread for GC"));
        }

        Ok(())
    }

    pub fn unregister_current_thread() {
        unsafe { GC_unregister_my_thread() }
    }

    pub fn alloc(size: usize) -> *mut u8 {
        unsafe { Allocator.alloc(Layout::from_size_align_unchecked(size, 8)) }
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        return if GC_STARTED {
            GC_malloc(layout.size())
        } else {
            GC_malloc_uncollectable(layout.size())
        } as *mut u8;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        GC_free(ptr as *mut c_void)
    }
}
