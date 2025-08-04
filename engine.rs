#![allow(dead_code, unused_variables)]

// Rexports
use crate::modpub;

modpub!(traits);
modpub!(scene);
modpub!(window);

/// Macro for reexporting
#[macro_export]
macro_rules! modpub {
    ($($name:ident),*) => {
        $(pub mod $name; pub use $name::*;)*
    };
}
