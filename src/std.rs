// DevOut
//
// Copyright (c) 2019-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::cell::RefCell;
use std::fmt::Write;

// FIXME: Don't block
/*fn get_writer() -> (std::sync::mpsc::Sender, std::sync::mpsc::Receiver) {
    std::sync::mpsc::channel()
}*/

thread_local!(static MESSAGE: RefCell<String> = RefCell::new(String::new()));

#[doc(hidden)]
pub fn _journal_hidden(tag: &str, args: std::fmt::Arguments<'_>) {
    MESSAGE.with(|message| {
        let message = &mut *message.borrow_mut();
        // Errors can't happen on `String`, so ignore the `Result`.
        let _ = message.write_fmt(format_args!("[{}]: {}\n", tag, args));
        // Send the message by locking and writing to stdout.
        use std::io::Write;
        // Ignore errors, writing to stdout should work.
        let _ = { std::io::stdout().lock().write_all(message.as_bytes()) };
        // Always clear after send, in case of sensitive information printing.
        message.clear();
    });
}

#[doc(hidden)]
pub fn _journal_hidden_dev(_tag: &str, _args: std::fmt::Arguments<'_>) {
    #[cfg(feature = "dev")] {
        MESSAGE.with(|message| {
            let message = &mut *message.borrow_mut();
            // Errors can't happen on `String`, so ignore the `Result`.
            let _ = message.write_fmt(format_args!("[{}]: {}\n", _tag, _args));
            // Send the message by locking and writing to stdout.
            use std::io::Write;
            // Ignore errors, writing to stdout should work.
            let _ = { std::io::stderr().lock().write_all(message.as_bytes()) };
            // Always clear after send, in case of sensitive information printing.
            message.clear();
        });
    }
}
