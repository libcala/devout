use std::cell::RefCell;
use std::fmt::Write;

// FIXME: Don't block
/*fn get_writer() -> (std::sync::mpsc::Sender, std::sync::mpsc::Receiver) {
    std::sync::mpsc::channel()
}*/

thread_local!(static MESSAGE: RefCell<String> = RefCell::new(String::new()));

#[doc(hidden)]
pub fn _journal_hidden(tag: &str, args: std::fmt::Arguments) {
    MESSAGE.with(|message| {
        let message = &mut *message.borrow_mut();
        // Errors can't happen on `String`, so ignore the `Result`.
        let _ = message.write_fmt(format_args!("[{}]: {}\n", tag, args));
        // Send the message by locking and writing to stdout.
        use std::io::Write;
        // Ignore errors, writing to stdout should work.
        let _ = std::io::stdout().lock().write_all(message.as_bytes());
        // Always clear after send, in case of sensitive information printing.
        message.clear();
    });
}

#[doc(hidden)]
pub fn _journal_hidden_devel(_tag: &str, _args: std::fmt::Arguments) {
    MESSAGE.with(|message| {
        let message = &mut *message.borrow_mut();
        // Errors can't happen on `String`, so ignore the `Result`.
        let _ = message.write_fmt(format_args!("[{}]: {}\n", _tag, _args));
        // Send the message by locking and writing to stdout.
        use std::io::Write;
        // Ignore errors, writing to stdout should work.
        let _ = std::io::stderr().lock().write_all(message.as_bytes());
        // Always clear after send, in case of sensitive information printing.
        message.clear();
    });
}
