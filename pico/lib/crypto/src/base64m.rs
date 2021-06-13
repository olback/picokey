use core::slice;

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
