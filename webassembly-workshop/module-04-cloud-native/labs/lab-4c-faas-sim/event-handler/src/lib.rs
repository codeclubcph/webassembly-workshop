// Event handler WASM guest for the FaaS simulator
// Receives JSON event payloads and returns a processed result

#[no_mangle]
pub extern "C" fn alloc(size: i32) -> i32 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr() as i32;
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn handle_event(ptr: i32, len: i32) -> i64 {
    let input = unsafe {
        let slice = std::slice::from_raw_parts(ptr as *const u8, len as usize);
        std::str::from_utf8_unchecked(slice)
    };

    // Simple event routing without external dependencies
    let event_type = if input.contains("order.placed") {
        "ORDER_ACCEPTED"
    } else if input.contains("user.signup") {
        "WELCOME_EMAIL_QUEUED"
    } else if input.contains("payment.failed") {
        "RETRY_SCHEDULED"
    } else {
        "UNKNOWN_EVENT"
    };

    let result = format!(r#"{{"status":"processed","action":"{}","input_len":{}}}"#,
        event_type, len);

    let result_bytes = result.into_bytes();
    let result_ptr = result_bytes.as_ptr() as i32;
    let result_len = result_bytes.len() as i32;
    std::mem::forget(result_bytes);

    ((result_ptr as i64) << 32) | (result_len as i64)
}
