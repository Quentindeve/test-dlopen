#![feature(str_from_raw_parts)]

fn open_dl(path: &str) -> Result<*mut libc::c_void, String> {
    let path = format!("{path}\0");

    let path: *const libc::c_char = path.as_ptr().cast();
    let handle = unsafe { libc::dlopen(path, libc::RTLD_NOW) };

    if handle.is_null() {
        let dl_error: &str = unsafe {
            let ptr = libc::dlerror();
            let len = libc::strlen(ptr);
            std::str::from_raw_parts(ptr.cast(), len)
        };

        return Err(String::from(dl_error));
    }
    Ok(handle)
}

fn get_symbol<T>(handle: *mut libc::c_void, symbol: &str) -> Result<&T, String> {
    let symbol_name = format!("{symbol}\0");
    let symbol = unsafe { libc::dlsym(handle, symbol_name.as_ptr().cast()) };

    if symbol.is_null() {
        let dl_error: &str = unsafe {
            let ptr = libc::dlerror();
            let len = libc::strlen(ptr);
            std::str::from_raw_parts(ptr.cast(), len)
        };

        return Err(String::from(dl_error));
    }
    return unsafe { Ok(&*symbol.cast()) };
}

fn main() -> Result<(), String> {
    let handle = open_dl("./libfoo.so")?;

    // Tricky part, get the symbol with the correct signature and calling it
    let symbol: extern "C" fn() -> *const u8 = *get_symbol(handle, "hello_world")?;
    let val = symbol(); // Blows up
    let string = unsafe { std::str::from_raw_parts(val, 13) };

    println!("{string}");

    Ok(())
}
