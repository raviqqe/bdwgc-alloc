extern crate bdwgc_allocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: bdwgc_allocator::Allocator = bdwgc_allocator::Allocator;

fn main() {
    unsafe { bdwgc_allocator::Allocator::initialize() }

    let mut _n = bdwgc_allocator::Allocator::alloc(2 ^ 8);

    loop {
        _n = bdwgc_allocator::Allocator::alloc(2 ^ 8)
    }
}
