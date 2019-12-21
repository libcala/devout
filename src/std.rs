use std::io::Write;

#[doc(hidden)]
pub fn _out(string: &str) {
    let out = std::io::stdout();
    let mut lock = out.lock();

    let _ = lock.write_all(string.as_bytes());
    let _ = lock.write_all(b"\n");
}

#[doc(hidden)]
pub fn _dev(string: &str) {
    let err = std::io::stderr();
    let mut lock = err.lock();

    let _ = lock.write_all(string.as_bytes());
    let _ = lock.write_all(b"\n");
}

#[doc(hidden)]
pub fn _fix(string: &str) {
    let err = std::io::stderr();
    let mut lock = err.lock();

    let _ = lock.write_all(string.as_bytes());
    let _ = lock.write_all(b"\n");
}
