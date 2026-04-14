// Plugin: uppercase – transforms input string to UPPERCASE

#[no_mangle]
pub extern "C" fn alloc(size: i32) -> i32 {
    let mut buf = Vec::<u8>::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr() as i32;
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn transform(ptr: i32, len: i32) -> i64 {
    let input = unsafe {
        let slice = std::slice::from_raw_parts(ptr as *const u8, len as usize);
        std::str::from_utf8_unchecked(slice)
    };

    let result = input.to_uppercase();
    let result_bytes = result.into_bytes();
    let result_ptr = result_bytes.as_ptr() as i32;
    let result_len = result_bytes.len() as i32;
    std::mem::forget(result_bytes);

    // Pack pointer and length into a single i64
    ((result_ptr as i64) << 32) | (result_len as i64)
}
