use seq_macro::seq;

#[allow(unused_imports)]
use crate::*;

macro_rules! native_macro {
    ($($type:ident),+) => {
        $(
            #[doc=core::concat!("Returns a [`", core::stringify!($type), "`](type@", core::stringify!($type), ") checked at compile time.")]
            #[doc=""]
            #[doc="# Example"]
            #[doc="```"]
            #[doc="# use arbitrary_int::*;"]
            #[doc=core::concat!("let x = ", core::stringify!($type), "!(0);")]
            #[doc="assert_eq!(x.value(), 0);"]
            #[doc="```"]
            #[macro_export]
            macro_rules! $type {
                ($val:literal) => {{
                    const __AINT_LITERAL_VALUE: $type = $val;
                    __AINT_LITERAL_VALUE
                }};
            }
        )*
    };
}

native_macro!(u8, u16, u32, u64, u128);
native_macro!(i8, i16, i32, i64, i128);


macro_rules! lit_macro {
    ($type:ident) => {
        #[doc=core::concat!("Returns a [`", core::stringify!($type), "`](type@", core::stringify!($type), ") checked at compile time.")]
        #[doc=""]
        #[doc="# Example"]
        #[doc="```"]
        #[doc="# use arbitrary_int::*;"]
        #[doc=core::concat!("let x = ", core::stringify!($type), "!(0);")]
        #[doc="assert_eq!(x.value(), 0);"]
        #[doc="```"]
        #[macro_export]
        macro_rules! $type {
            ($val:literal) => {{
                const __AINT_LITERAL_VALUE: $type = match $crate::$type::try_new($val) {
                    Ok(__aint_literal_value) => __aint_literal_value,
                    Err(_) => ::core::panic!(::core::concat!("Invalid value for ", ::core::stringify!($type))),
                };
                __AINT_LITERAL_VALUE
            }};
        }
    };
}


seq!(BITS in 1..8 {
    #(
        lit_macro!(u~BITS);
        lit_macro!(i~BITS);
    )*
});

seq!(BITS in 9..16 {
    #(
        lit_macro!(u~BITS);
        lit_macro!(i~BITS);
    )*
});

seq!(BITS in 17..32 {
    #(
        lit_macro!(u~BITS);
        lit_macro!(i~BITS);
    )*
});

seq!(BITS in 33..64 {
    #(
        lit_macro!(u~BITS);
        lit_macro!(i~BITS);
    )*
});

seq!(BITS in 65..128 {
    #(
        lit_macro!(u~BITS);
        lit_macro!(i~BITS);
    )*
});
