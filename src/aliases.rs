#![allow(non_camel_case_types)]
use seq_macro::seq;

// Define type aliases like u1, u63 and u80 using the smallest possible underlying data type.
// These are for convenience only - AInt<u32, 15> is still legal
macro_rules! type_alias {
    ($storage:ty, ($name:ident, $bits:expr)) => {
        pub type $name = crate::AInt<$storage, $bits>;
    }
}

seq!(BITS in 1..8 {
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


seq!(BITS in 65..128 {
    #(
        type_alias!(u128,(u~BITS, BITS));
        type_alias!(i128,(i~BITS, BITS));
    )*
});

