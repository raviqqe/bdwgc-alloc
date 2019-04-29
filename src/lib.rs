extern crate libc;

mod error;

use libc::{c_char, c_int, c_void, size_t};
use std::alloc::{GlobalAlloc, Layout};

const GC_SUCCESS: c_int = 0;

#[repr(C)]
struct GcStackBase {
    mem_base: *const c_void,
    reg_base: *const c_void,
}

#[link(name = "gc", kind = "static")]
extern "C" {
    fn GC_allow_register_threads() -> c_void;
    fn GC_disable() -> c_void;
    fn GC_enable() -> c_void;
    fn GC_free(ptr: *mut c_void);
    fn GC_get_stack_base(stack_base: *mut GcStackBase) -> c_int;
    fn GC_init() -> c_void;
    fn GC_malloc(size: size_t) -> *mut c_void;
    fn GC_register_my_thread(stack_base: *const GcStackBase) -> c_int;
    fn GC_set_stack_bottom(thread: *const c_void, stack_bottom: *const c_char);
    fn GC_unregister_my_thread();
}

pub struct Allocator;

impl Allocator {
    pub fn disable_gc() {
        unsafe { GC_disable() };
    }

    pub fn enable_gc() {
        unsafe { GC_enable() };
    }

    pub unsafe fn initialize() {
        GC_init();
        GC_allow_register_threads();
    }

    pub unsafe fn register_current_thread() -> Result<(), error::Error> {
        let mut base = GcStackBase {
            mem_base: std::ptr::null(),
            reg_base: std::ptr::null(),
        };

        if GC_get_stack_base(&mut base) != GC_SUCCESS {
            return Err(error::Error::new("failed to get stack base"));
        } else if GC_register_my_thread(&base) != GC_SUCCESS {
            return Err(error::Error::new("failed to register a thread for GC"));
        }

        Ok(())
    }

    pub unsafe fn set_stack_bottom(bottom: *const u8) {
        GC_set_stack_bottom(
            std::mem::transmute(nix::sys::pthread::pthread_self()),
            std::mem::transmute(bottom),
        );
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
}
