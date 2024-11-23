#![cfg_attr(feature = "step_trait", feature(step_trait))]
#![cfg_attr(feature = "generic_const_exprs", feature(generic_const_exprs), allow(incomplete_features))]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod aint;
mod traits;
mod aliases;

mod error;
mod util;
mod macros;

mod impl_core;

pub use aint::{AInt, UnsignedNumberType};
pub use error::{NumberErrorKind, ParseNumberError, TryNewError};
pub use traits::{Number, NumberType};
pub use aliases::*;

#[cfg(feature = "num-traits")]
mod impl_num_traits;

#[cfg(feature = "serde")]
mod impl_serde;

#[cfg(feature = "borsh")]
mod impl_borsh;

#[cfg(feature = "step_trait")]
mod impl_step_trait;

#[cfg(feature = "schemars")]
mod impl_schemars;

#[cfg(feature = "funty")]
mod impl_funty;

#[cfg(feature = "defmt")]
mod impl_defmt;

#[cfg(test)]
mod tests;
