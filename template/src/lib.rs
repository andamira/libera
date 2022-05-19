// ::
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
// #![cfg(features = "std")]
#![no_std]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq![1, true.into()];
    }
}
