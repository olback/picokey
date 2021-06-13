#![no_std]
#![feature(default_alloc_error_handler)]
use mallocator::Mallocator;

macro_rules! assert_len {
    ($input:expr, $required:expr) => {
        if $input != $required {
            return -3;
        }
    };
}

mod aes_gcm_sivm;
mod base64m;
mod keyiv;

#[global_allocator]
static A: Mallocator = Mallocator;

/// cbindgen:ignore
extern "C" {
    fn panic_handler() -> !;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { panic_handler() }
}
