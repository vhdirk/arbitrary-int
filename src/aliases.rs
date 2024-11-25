#![allow(non_camel_case_types)]
use seq_macro::seq;

// Define type aliases like u1, u63 and u80 using the smallest possible underlying data type.
// These are for convenience only - AInt<u32, 15> is still legal
macro_rules! type_alias {
    ($storage:ty, ($name:ident, $bits:expr)) => {
        pub type $name = crate::AInt<$storage, $bits>;
    }
}

seq!(BITS in 0..8 {
    #(
        type_alias!(u8,(u~BITS, BITS));
        type_alias!(i8,(i~BITS, BITS));
    )*
});

seq!(BITS in 9..16 {
    #(
        type_alias!(u16,(u~BITS, BITS));
        type_alias!(i16,(i~BITS, BITS));
    )*
});

seq!(BITS in 17..32 {
    #(
        type_alias!(u32,(u~BITS, BITS));
        type_alias!(i32,(i~BITS, BITS));
    )*
});

seq!(BITS in 33..64 {
    #(
        type_alias!(u64,(u~BITS, BITS));
        type_alias!(i64,(i~BITS, BITS));
    )*
});

#[cfg(feature = "128")]
mod aliases_128 {
    use super::*;

    seq!(BITS in 65..128 {
        #(
            type_alias!(u128,(u~BITS, BITS));
            type_alias!(i128,(i~BITS, BITS));
        )*
    });
}

#[cfg(feature = "128")]
pub use aliases_128::*;

// We need to wrap this in a macro, currently: https://github.com/rust-lang/rust/issues/67792#issuecomment-1130369066
macro_rules! boolu1 {
    () => {
        impl From<bool> for u1 {
            #[inline]
            fn from(value: bool) -> Self {
                u1::new(value as u8)
            }
        }
        impl From<u1> for bool {
            #[inline]
            fn from(value: u1) -> Self {
                match value.value {
                    0 => false,
                    1 => true,
                    _ => panic!("arbitrary-int already validates that this is unreachable"), //TODO: unreachable!() is not const yet
                }
            }
        }
    };
}

boolu1!();