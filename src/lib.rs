// DevOut
//
// Copyright (c) 2019-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Getting Started
//! Add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies.devout]
//! version = "0.1.0"
//! ```
//!
//! ```rust
//! use devout::{dev, out};
//!
//! const INFO: &str = "Info";
//!
//! // Prints twice when "dev" feature is enabled, once otherwise
//! dev!(INFO, "Result: {}", 4.4);
//! out!(INFO, "Result: {}", 4.4);
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/libcala/devout/master/res/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/libcala/devout/master/res/logo.svg",
    html_root_url = "https://docs.rs/devout"
)]
#![forbid(unsafe_code)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use self::web::*;
#[cfg(not(target_arch = "wasm32"))]
mod std;
#[cfg(not(target_arch = "wasm32"))]
pub use self::std::*;

/// Use for messages to be journaled during both production and development.
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

/// Use for messages to be journaled only during development.
#[macro_export]
macro_rules! dev {
    ($tag:ident $(,)?) => {{
        $crate::dev!($tag, "");
    }};
    ($tag:ident, $fmt:expr $(,)?) => {{
        $crate::_journal_hidden_dev($tag, format_args!($fmt));
    }};
    [$tag:ident, $fmt:expr, $($args:tt)* $(,)?] => {{
        $crate::_journal_hidden_dev($tag, format_args!($fmt, $($args)*));
    }};
}
