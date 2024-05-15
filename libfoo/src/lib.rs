#[no_mangle]
pub extern "C" fn hello_world() -> *const u8 {
    "Hello World !".as_ptr().cast()
}
