#![doc(
    html_logo_url = "https://github.com/opensass/scroll-rs/blob/main/assets/logo.png",
    html_favicon_url = "https://github.com/opensass/scroll-rs/blob/main/assets/favicon.png"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub mod common;

#[cfg(feature = "yew")]
pub mod yew;

#[cfg(feature = "dio")]
pub mod dioxus;

pub use common::Behavior;
