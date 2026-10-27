//
#![cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#![cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
//
//! Internal scaffolding and documentation support.

crate::mods_in! {
    mod fragments; // Common documentation fragments.
}
crate::mods_out! {
    _crate_internals {
        pub(crate) use super::fragments::*;
    }
}
