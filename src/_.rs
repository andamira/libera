//
//! Experimental vocabulary for expressive composition.
//

// environment
#![cfg_attr(not(feature = "std"), no_std)]
// safety
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
// nightly
#![cfg_attr(nightly_doc, doc(test(attr(feature(doc_cfg)))))]
#![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]

/* imports */

extern crate self as libera;

use ::devela::__hidden::*;
use ::devela::all::*;

crate::mods_in! {
    // …

    // internal
    pub mod_ yard; // Scaffolding, taxonomy, and documentation support.
}

#[doc = crate::_DOC_ALL!()]
pub mod all {
    #[allow(unused_imports)]
    pub use crate::_all::*;
}

crate::mods_out! {
    _pub_mods {
    }
    _crate_internals {
        pub use super::yard::_crate_internals::*;
    }
}
