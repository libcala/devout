//! # Devout
//! A simple cross-platform logging library.
//!
//! ## Example
//! ```rust
//! use devout::*;
//!
//! fix!("FIXME: {}", 12);
//! dev!("Test: {}", 40);
//! out!("Result: {}", 4.4);
//! ```

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://libcala.github.io/logo.svg",
    html_favicon_url = "https://libcala.github.io/icon.svg"
)]

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use self::web::*;
#[cfg(not(target_arch = "wasm32"))]
mod std;
#[cfg(not(target_arch = "wasm32"))]
pub use self::std::*;

/// Print line to stdout.  Use for messages that will print in production.
#[macro_export] macro_rules! out {
    () => {{
        $crate::_out("");
    }};
    ($($arg:tt)*) => {{
        $crate::_out(&format!($($arg)*));
    }};
}

/// Print line to stderr.  Use for messages that will print in development, but
/// not in production.
#[macro_export] macro_rules! dev {
    () => {{
        $crate::_dev("");
    }};
    ($($arg:tt)*) => {{
        $crate::_dev(&format!($($arg)*));
    }};
}

/// Print line to stderr.  Use for runtime cases where something should be
/// fixed.  It does not have to be detremental.
#[macro_export] macro_rules! fix {
    () => {{
        $crate::_fix("");
    }};
    ($($arg:tt)*) => {{
        $crate::_fix(&format!($($arg)*));
    }};
}
