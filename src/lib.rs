extern crate libc;

mod error;

use libc::{c_int, c_void, size_t};
use std::alloc::{GlobalAlloc, Layout};

const GC_SUCCESS: c_int = 0;

#[repr(C)]
struct GcStackBase {
    mem_base: *const c_void,
    // TODO: Add reg_base field to support IA64.
}

#[link(name = "gc", kind = "static")]
extern "C" {
    fn GC_allow_register_threads();
    fn GC_alloc_lock();
    fn GC_alloc_unlock();
    fn GC_free(ptr: *mut c_void);
    fn GC_get_stack_base(stack_base: *mut GcStackBase) -> c_int;
    fn GC_init();
    fn GC_malloc(size: size_t) -> *mut c_void;
    fn GC_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn GC_register_my_thread(stack_base: *const GcStackBase) -> c_int;
    fn GC_set_stackbottom(thread: *const c_void, stack_bottom: *const GcStackBase);
    fn GC_unregister_my_thread();
}

pub struct Allocator;

impl Allocator {
    pub fn lock() {
        unsafe { GC_alloc_lock() }
    }

    pub fn unlock() {
        unsafe { GC_alloc_unlock() }
    }

    pub unsafe fn initialize() {
        GC_init();
        GC_allow_register_threads();
    }

    pub unsafe fn register_current_thread() -> Result<(), error::Error> {
        let mut base = GcStackBase {
            mem_base: std::ptr::null(),
        };

        if GC_get_stack_base(&mut base) != GC_SUCCESS {
            return Err(error::Error::new("failed to get stack base"));
        } else if GC_register_my_thread(&base) != GC_SUCCESS {
            return Err(error::Error::new("failed to register a thread for GC"));
        }

        Ok(())
    }

    pub unsafe fn set_stack_bottom(bottom: *const u8) {
        GC_set_stackbottom(
            std::ptr::null(),
            &GcStackBase {
                mem_base: bottom as *const libc::c_void,
            },
        )
    }

    pub unsafe fn unregister_current_thread() {
        GC_unregister_my_thread()
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        GC_malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        GC_free(ptr as *mut c_void)
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, size: usize) -> *mut u8 {
        GC_realloc(ptr as *mut c_void, size) as *mut u8
    }
}
