//! Rust bindings to Apple’s `libsecurity_translocate`, part of the Security
//! framework, which facilitates the creation and destruction of app
//! translocation points. These bindings are based on the
//! [`security-translocate-sys` crate](https://docs.rs/security-translocate-sys/0.1.1).

mod bindings;
pub use bindings::*;

mod option;

mod result;
