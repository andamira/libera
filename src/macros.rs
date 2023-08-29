// libera::macros
//
//!
//

// reexporting libraries
macro_rules! reexport {
    // reexport ::$name
    ($($name:ident),+ $(,)?) => { $( reexport![@ $name]; )+ };
    (@ $name:ident) => { devela::paste! {
        #[doc = "reexported from [" $name "](https://crates.io/crates/" $name ")."]
        #[doc(inline)]
        pub use ::$name;
    }};
    // reexport ::$name (optional feature)
    ( $($name:ident $feat:literal ,)+ $(,)? ) => {
        $( reexport![@ $name $feat]; )+
    };
    (@ $name:ident $feat:literal) => { devela::paste! {
        #[doc = "reexported from [" $name "](https://crates.io/crates/" $name ")."]
        #[doc(inline)]
        #[cfg(feature = $feat)]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = $feat)))]
        pub use ::$name;
    }};

    // reexport ::$name::all::*
    (@all $($name:ident ,)+ $(,)? ) => {
        $( reexport![@all $name $feat]; )+
    };
    (@all $name:ident ) => { devela::paste! {
        #[doc = "<small>[`" $name "`](https://crates.io/crates/" $name ")</small>"]
        pub use ::$name::all::*;
    }};
    // reexport ::$name::all::* (optional feature)
    (@all $($name:ident $feat:literal ,)+ $(,)? ) => {
        $( reexport![@all $name $feat]; )+
    };
    (@all $name:ident $feat:literal ) => { devela::paste! {
        #[doc = "<small>[`" $name "`](https://crates.io/crates/" $name ")</small>"]
        #[cfg(feature = $feat)]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = $feat)))]
        pub use ::$name::all::*;
    }};
}
pub(crate) use reexport;
