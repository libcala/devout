//! # DevOut
//! A simple cross-platform logging library.
//!
//! ## Example
//! ```rust
//! use devout::{dev, out};
//!
//! const INFO: &str = "Info";
//!
//! out!(INFO, "Result: {}", 4.4);
//! ```

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://libcala.github.io/logo.svg",
    html_favicon_url = "https://libcala.github.io/icon.svg"
)]

/// Use for messages to be journaled during both production and debugging.
#[macro_export]
macro_rules! out {
    ($tag:ident $(,)?) => {{
        $crate::out!($tag, "");
    }};
    ($tag:ident, $fmt:expr $(,)?) => {{
        $crate::_journal_hidden($tag, format_args!($fmt));
    }};
    [$tag:ident, $fmt:expr, $($args:tt)*] => {{
        $crate::_journal_hidden($tag, format_args!($fmt, $($args)*));
    }};
}

/// Use for messages to be journaled only during debugging.
#[macro_export]
macro_rules! dev {
    ($tag:ident $(,)?) => {{
        $crate::out!($tag, "");
    }};
    ($tag:ident, $fmt:expr $(,)?) => {{
        $crate::_journal_hidden($tag, format_args!($fmt));
    }};
    [$tag:ident, $fmt:expr, $($args:tt)* $(,)?] => {{
        $crate::_journal_hidden($tag, format_args!($fmt, $($args)*));
    }};
}

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use self::web::*;
#[cfg(not(target_arch = "wasm32"))]
mod std;
#[cfg(not(target_arch = "wasm32"))]
pub use self::std::*;
