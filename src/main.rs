extern crate jemallocator;

#[cfg(unix)]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
fn main() {
    println!("Hello, world!");
}
