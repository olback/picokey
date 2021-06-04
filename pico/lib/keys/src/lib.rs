#![no_std]

const PRIVATE_KEY: &[u8] = include_bytes!("../../../../keys/tx_private.pem");
const PUBLIC_KEY: &[u8] = include_bytes!("../../../../keys/rx_public.pem");

#[repr(C)]
pub struct Key {
    pub data: *const u8,
    pub len: usize,
}

#[no_mangle]
pub extern "C" fn get_private_key() -> Key {
    Key {
        data: PRIVATE_KEY.as_ptr(),
        len: PRIVATE_KEY.len(),
    }
}

#[no_mangle]
pub extern "C" fn get_public_key() -> Key {
    Key {
        data: PUBLIC_KEY.as_ptr(),
        len: PUBLIC_KEY.len(),
    }
}

#[no_mangle]
pub extern "C" fn what() -> i32 {
    10
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
