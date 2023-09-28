use bdwgc_alloc::Allocator;

const ITERATION_COUNT: usize = 1000_000;

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

fn main() {
    unsafe { Allocator::initialize() }

    let mut _n: Box<[u8; 2 ^ 8]> = Box::new([0; 2 ^ 8]);

    for _ in 0..ITERATION_COUNT {
        _n = Box::new([0; 2 ^ 8]);
    }
}
