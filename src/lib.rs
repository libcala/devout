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
//! use devout::{log, Tag};
//!
//! const INFO: Tag = Tag::new("Info").show(true);
//!
//! log!(INFO, "Result: {}", 4.4);
//! ```

#![doc(
    html_logo_url = "https://libcala.github.io/logo.svg",
    html_favicon_url = "https://libcala.github.io/icon.svg",
    html_root_url = "https://docs.rs/devout"
)]
#![deny(unsafe_code)]
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

/// A tag to identify a log.
#[derive(Copy, Clone, Debug)]
pub struct Tag(Option<&'static str>);

impl Tag {
    /// Create a new tag by passing a textual identifier.
    #[inline(always)]
    pub const fn new(ident: &'static str) -> Self {
        Tag(Some(ident))
    }

    /// Hide logs using this tag.
    #[inline(always)]
    pub const fn hide(self) -> Self {
        Tag(None)
    }

    /// Choose whether or not to show this tag based on a boolean value.
    /// `true` is show, and `false` is hide.
    #[inline(always)]
    pub const fn show(self, log: bool) -> Self {
        if log {
            self
        } else {
            self.hide()
        }
    }

    /// Returns true if logs using this tag are shown.
    #[inline(always)]
    pub const fn is_shown(self) -> bool {
        self.0.is_some()
    }

    /// Get tag as optional identifier.
    #[inline(always)]
    const fn as_option(&self) -> Option<&'static str> {
        self.0
    }

    /// Print out a log message with this tag.  Prefer `log!()` instead.
    #[inline(always)]
    pub fn log(&self, args: std::fmt::Arguments<'_>) {
        if let Some(tag) = self.as_option() {
            #[cfg(not(target_arch = "wasm32"))]
            let _ = <std::io::Stdout as std::io::Write>::write_fmt(
                &mut std::io::stdout(),
                format_args!("[{}] {}\n", tag, args),
            );

            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
                "[{}] {}",
                tag, args
            )));
        }
    }
}

/// Write a message to the log.
#[macro_export]
macro_rules! log {
    ($tag:ident) => {{
        $tag.log(format_args!(""));
    }};
    ($tag:ident, $($arg:tt)*) => {{
        $tag.log(format_args!($($arg)*));
    }};
}
