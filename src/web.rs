// DevOut
//
// Copyright (c) 2019-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cala_core::os::web::{JsFn, JsString};
use std::cell::RefCell;
use std::{fmt::Write, mem::MaybeUninit, sync::Once};

static mut CONSOLE_INFO: MaybeUninit<JsFn> = MaybeUninit::uninit();
static INIT: Once = Once::new();

// Log a message to the console.
#[allow(unsafe_code)]
fn console_info(a: &str) {
    unsafe {
        INIT.call_once(|| {
            CONSOLE_INFO = MaybeUninit::new(JsFn::new("console.info(param_a);"));
        });
        let console_info = &*CONSOLE_INFO.as_ptr();
        let r = console_info.call(Some(JsString::new(a).as_var()), None);
        debug_assert!(r.is_none());
    }
}

#[cfg(feature = "dev")]
static mut CONSOLE_DEBUG: MaybeUninit<JsFn> = MaybeUninit::uninit();
#[cfg(feature = "dev")]
static INIT_DEV: Once = Once::new();

// Log a message to the console.
#[cfg(feature = "dev")]
#[allow(unsafe_code)]
fn console_debug(a: &str) {
    unsafe {
        INIT_DEV.call_once(|| {
            CONSOLE_DEBUG = MaybeUninit::new(JsFn::new("console.debug(param_a);"));
        });
        let console_debug = &*CONSOLE_DEBUG.as_ptr();
        let r = console_debug.call(Some(JsString::new(a).as_var()), None);
        debug_assert!(r.is_none());
    }
}

thread_local!(static MESSAGE: RefCell<String> = RefCell::new(String::new()));

#[doc(hidden)]
pub fn _journal_hidden(tag: &str, args: std::fmt::Arguments<'_>) {
    MESSAGE.with(|message| {
        let message = &mut *message.borrow_mut();
        // Errors can't happen on `String`, so ignore the `Result`.
        let _ = message.write_fmt(format_args!("[{}]: {}", tag, args));
        // Send the message by calling JavaScript
        console_info(message);
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
            let _ = message.write_fmt(format_args!("[{}]: {}", _tag, _args));
            // Send the message by calling JavaScript
            console_debug(message);
            // Always clear after send, in case of sensitive info printing.
            message.clear();
        });
    }
}
