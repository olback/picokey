#![no_std]
#![feature(default_alloc_error_handler)]
use {
    aes_gcm_siv::{
        aead::{Aead, NewAead},
        Aes256GcmSiv, Key, Nonce,
    },
    core::slice,
    mallocator::Mallocator,
};

macro_rules! assert_len {
    ($input:expr, $required:expr) => {
        if $input != $required {
            return -3;
        }
    };
}

#[global_allocator]
static A: Mallocator = Mallocator;

/// cbindgen:ignore
extern "C" {
    fn panic_handler() -> !;
}

#[no_mangle]
pub extern "C" fn base64_encode(
    data_in: *const u8,
    data_in_len: u32,
    data_out: *mut u8,
    data_out_len: *mut u32,
) -> i32 {
    let data_in_slice = unsafe { slice::from_raw_parts(data_in, data_in_len as usize) };
    let data_out_slice = unsafe { slice::from_raw_parts_mut(data_out, (*data_out_len) as usize) };

    let encoded = base64::encode(data_in_slice);
    let encoded_bytes = encoded.as_bytes();

    let len = encoded_bytes.len();
    if len > data_out_slice.len() {
        // ERROR: Output larger than output buffer
        return -1;
    }

    // memcpy from our result to passed slice
    data_out_slice[0..len].clone_from_slice(&encoded_bytes[0..len]);
    unsafe { (*data_out_len) = len as u32 }

    0
}

#[no_mangle]
pub extern "C" fn base64_decode(
    data_in: *const u8,
    data_in_len: u32,
    data_out: *mut u8,
    data_out_len: *mut u32,
) -> i32 {
    let data_in_slice = unsafe { slice::from_raw_parts(data_in, data_in_len as usize) };
    let data_out_slice = unsafe { slice::from_raw_parts_mut(data_out, (*data_out_len) as usize) };

    let decoded = match base64::decode(data_in_slice) {
        Ok(d) => d,
        Err(_) => return -1,
    };
    let decoded_bytes = decoded.as_slice();

    let len = decoded_bytes.len();
    if len > data_out_slice.len() {
        // ERROR: Output larger than output buffer
        return -2;
    }

    // memcpy from our result to passed slice
    data_out_slice[0..len].clone_from_slice(&decoded_bytes[0..len]);
    unsafe { (*data_out_len) = len as u32 }

    0
}

#[no_mangle]
pub extern "C" fn aes_gcm_siv_encrypt(
    key: *const u8,
    key_len: u32,
    iv: *const u8,
    iv_len: u32,
    data_in: *const u8,
    data_in_len: u32,
    data_out: *mut u8,
    data_out_len: *mut u32,
) -> i32 {
    assert_len!(key_len, 32);
    assert_len!(iv_len, 12);
    let key_slice = unsafe { slice::from_raw_parts(key, key_len as usize) };
    let iv_slice = unsafe { slice::from_raw_parts(iv, iv_len as usize) };
    let data_in_slice = unsafe { slice::from_raw_parts(data_in, data_in_len as usize) };
    let data_out_slice = unsafe { slice::from_raw_parts_mut(data_out, (*data_out_len) as usize) };

    let e_key = Key::from_slice(key_slice);
    let cipher = Aes256GcmSiv::new(e_key);
    let nonce = Nonce::from_slice(iv_slice);

    let res = match cipher.encrypt(nonce, data_in_slice) {
        Ok(e) => e,
        Err(_) => return -1,
    };

    if res.len() > data_out_slice.len() {
        // ERROR, result larger than output buffer
        return -2;
    }

    // memcpy from our result to passed slice
    data_out_slice[0..res.len()].clone_from_slice(&res[0..res.len()]);
    unsafe { (*data_out_len) = res.len() as u32 };

    0
}

#[no_mangle]
pub extern "C" fn aes_gcm_siv_decrypt(
    key: *const u8,
    key_len: u32,
    iv: *const u8,
    iv_len: u32,
    data_in: *const u8,
    data_in_len: u32,
    data_out: *mut u8,
    data_out_len: *mut u32,
) -> i32 {
    assert_len!(key_len, 32);
    assert_len!(iv_len, 12);
    let key_slice = unsafe { slice::from_raw_parts(key, key_len as usize) };
    let iv_slice = unsafe { slice::from_raw_parts(iv, iv_len as usize) };
    let data_in_slice = unsafe { slice::from_raw_parts(data_in, data_in_len as usize) };
    let data_out_slice = unsafe { slice::from_raw_parts_mut(data_out, (*data_out_len) as usize) };

    let e_key = Key::from_slice(key_slice);
    let cipher = Aes256GcmSiv::new(e_key);
    let nonce = Nonce::from_slice(iv_slice);

    let res = match cipher.decrypt(nonce, data_in_slice) {
        Ok(e) => e,
        Err(_) => return -1,
    };

    if res.len() > data_out_slice.len() {
        // ERROR, result larger than output buffer
        return -2;
    }

    // memcpy from our result to passed slice
    data_out_slice[0..res.len()].clone_from_slice(&res[0..res.len()]);
    unsafe { (*data_out_len) = res.len() as u32 };

    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { panic_handler() }
}
