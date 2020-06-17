use cala_core::os::web::{JsFn, JsString};
use std::cell::RefCell;
use std::{fmt::Write, mem::MaybeUninit, sync::Once};

static mut CONSOLE_LOG: MaybeUninit<JsFn> = MaybeUninit::uninit();
static INIT: Once = Once::new();

// Log a message to the console.
fn console_log(a: &str) {
    unsafe {
        INIT.call_once(|| {
            CONSOLE_LOG = MaybeUninit::new(JsFn::new("console.log(param_a); return 4294967295;"));
        });
        let console_log = &*CONSOLE_LOG.as_ptr();
        let r = console_log.call(Some(JsString::new(a).as_var()), None);
        debug_assert!(r.is_none());
    }
}

thread_local!(static MESSAGE: RefCell<String> = RefCell::new(String::new()));

#[doc(hidden)]
pub fn _journal_hidden(tag: &str, args: std::fmt::Arguments) {
    MESSAGE.with(|message| {
        let message = &mut *message.borrow_mut();
        // Errors can't happen on `String`, so ignore the `Result`.
        let _ = message.write_fmt(format_args!("[{}]: {}", tag, args));
        // Send the message by calling JavaScript
        console_log(message);
        // Always clear after send, in case of sensitive information printing.
        message.clear();
    });
}
