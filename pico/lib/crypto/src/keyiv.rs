const KEY_IV: &[u8] = include_bytes!("../../../../keyiv");

pub const KEY_LENGTH: usize = 32;
pub const IV_LENGTH: usize = 12;

#[no_mangle]
pub extern "C" fn get_key_ptr() -> *const u8 {
    let range = 0..KEY_LENGTH;
    return KEY_IV[range].as_ptr();
}

#[no_mangle]
pub extern "C" fn get_iv_ptr() -> *const u8 {
    let range = KEY_LENGTH..(KEY_LENGTH + IV_LENGTH);
    return KEY_IV[range].as_ptr();
}
