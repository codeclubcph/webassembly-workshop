// WASM guest for the memory isolation demo
// Exposes read/write functions on its own linear memory

#[no_mangle]
pub extern "C" fn write_i32(addr: i32, val: i32) {
    unsafe {
        let ptr = addr as *mut i32;
        *ptr = val;
    }
}

#[no_mangle]
pub extern "C" fn read_i32(addr: i32) -> i32 {
    unsafe {
        let ptr = addr as *const i32;
        *ptr
    }
}
