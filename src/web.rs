extern {
    fn console_warn(data: *const u16, size: usize);
    fn console_info(data: *const u16, size: usize);
    fn console_debug(data: *const u16, size: usize);
}

fn string_to_utf16(string: &str) -> Vec<u16> {
    string.encode_utf16().collect()
}

#[doc(hidden)]
pub fn _out(string: &str) {
    let string = string_to_utf16(string);
    unsafe { console_info(string.as_ptr(), string.len()); }
}

#[doc(hidden)]
pub fn _dev(string: &str) {
    let string = string_to_utf16(string);
    unsafe { console_debug(string.as_ptr(), string.len()); }
}

#[doc(hidden)]
pub fn _fix(string: &str) {
    let string = string_to_utf16(string);
    unsafe { console_warn(string.as_ptr(), string.len()); }
}
