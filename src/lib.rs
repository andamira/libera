// libera
//
//! A suite of modular, multidisciplinary and mutually coherent libraries.
//

// warnings
#![warn(clippy::all)]
// environment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#[cfg(feature = "alloc")]
extern crate alloc;

// safeguards
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
#[cfg(all(feature = "safe", feature = "unsafe"))]
compile_error!("You can't enable the `safe` and `unsafe*` features at the same time.");

mod macros;
use macros::reexport;

reexport![devela];
reexport![
    acolor "acolor",
    alazar "alazar",
    depura "depura",
    espera "espera",
    ladata "ladata",
    numera "numera",
    revela "revela",
    textos "textos",
];

/// All items are reexported here.
pub mod all {
    use super::reexport;

    reexport![@all devela];
    reexport![@all
        acolor "acolor",
        alazar "alazar",
        depura "depura",
        espera "espera",
        ladata "ladata",
        numera "numera",
        revela "revela",
        textos "textos",
    ];
}
