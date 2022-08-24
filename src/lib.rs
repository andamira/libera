// libera::lib
//!
//

#![warn(clippy::all)]
#![allow(
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::needless_return,
    clippy::blanket_clippy_restriction_lints,
    clippy::pattern_type_mismatch
)]
#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

mod macros;

#[cfg(feature = "std")]
mod project;
#[cfg(feature = "std")]
pub use project::{project_root_path, project_root_path_string};

mod string;
pub use string::counter_string;
