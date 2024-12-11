#![cfg_attr(
    feature = "generic_const_exprs",
    allow(incomplete_features),
    feature(generic_const_exprs),
)]
#![cfg_attr(
    feature = "inherent_associated_types",
    allow(incomplete_features),
    feature(inherent_associated_types),
)]

#![cfg_attr(
    feature = "step_trait",
    allow(incomplete_features),
    feature(step_trait)
)]


#[cfg(not(feature = "std"))]
extern crate alloc;

mod aint;
mod aliases;
mod traits;
mod convert;
mod container;

mod bits;
mod error;
mod macros;
mod util;

mod impl_core;

pub(crate) use container::AIntContainer;
pub use error::{AIntErrorKind, ParseAIntError, TryNewError};
pub use traits::{Number, SignedNumberType, UnsignedNumberType};

pub mod prelude {
    pub use crate::aint::AInt;
    pub use crate::aliases::*;
}

pub use prelude::*;

#[cfg(feature = "num-traits")]
mod impl_num_traits;

#[cfg(feature = "serde")]
mod impl_serde;

#[cfg(feature = "borsh")]
mod impl_borsh;

#[cfg(feature = "schemars")]
mod impl_schemars;

#[cfg(feature = "funty")]
mod impl_funty;

#[cfg(feature = "defmt")]
mod impl_defmt;

#[cfg(feature = "step_trait")]
mod impl_step_trait;

#[cfg(test)]
mod tests;
