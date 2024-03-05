#![warn(rustdoc::broken_intra_doc_links)]
//!
//! A simple Google Bigtable client.
//!
//! See [`bigtable`] package for more info.
//!
//! [[github repo]]
//!
//! [`bigtable`]: mod@crate::bigtable
//! [github repo]: https://github.com/liufuyang/bigtable_rs
mod auth_service;
pub mod bigtable;
pub mod google;
mod root_ca_certificate;
mod util;

/// Reexported dependencies for convenience.
pub mod reexports {
    pub use tonic;
}
