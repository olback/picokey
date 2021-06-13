use {
    aes_gcm_siv::{
        aead::{Aead, NewAead},
        Aes256GcmSiv, Key, Nonce,
    },
    core::slice,
};

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
