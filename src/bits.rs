use std::marker::PhantomData;

use seq_macro::seq;



#[cfg(not(feature = "generic_const_exprs"))]
mod const_bounds {
    use super ::*;

    pub trait ConstMin<const N: usize, const MIN: usize> {}

    macro_rules! impl_const_min {
        ($min:expr, $n:expr) => {
            impl ConstMin<$n, $min> for () {}
        };

        ($min:expr ) => {
            seq!(N in $min..=128 {
                #(
                    impl_const_min!($min, N);
                )*
            });
        };

        () => {
            seq!(MIN in 1..=128 {
                #(
                    impl_const_min!(MIN);
                )*
            });
        }
    }

    pub trait ConstMax<const N: usize, const MAX: usize> {}

    macro_rules! impl_const_max {
        ($max:expr, $n:expr) => {
            impl ConstMax<$n, $max> for () {}
        };

        ($max:expr ) => {
            seq!(N in 1..=$max {
                #(
                    impl_const_max!($max, N);
                )*
            });
        };

        () => {
            seq!(MAX in 1..=128 {
                #(
                    impl_const_max!(MAX);
                )*
            });
        }
    }

    // yes, together these macros generate 16k impls. not sure if compiler is happy about that

    impl_const_min!();
    impl_const_max!();

}

#[cfg(not(feature = "generic_const_exprs"))]
pub use const_bounds::*;

#[derive(Debug, Default)]
pub struct ConstBounded<const N: usize, const MIN: usize, const MAX: usize>(PhantomData<()>);

#[cfg(not(feature = "generic_const_exprs"))]
impl<const N: usize, const MIN: usize, const MAX: usize> ConstBounded<N, MIN, MAX>
where
    (): ConstMin<N, MIN> + ConstMax<N, MAX>,
{
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

#[cfg(feature = "generic_const_exprs")]
impl<const N: usize, const MIN: usize, const MAX: usize> ConstBounded<N, MIN, MAX>
where
    [(); (N >= MIN) as usize]:,
    [(); (N <= MAX) as usize]:,
{
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}


// pub enum _0 {}
// pub enum _1 {}

// pub trait Bit {
//     const VALUE: u32;
//     type Not: Bit;
//     type And<Rhs: Bit>: Bit;
//     type Or<Rhs: Bit>: Bit;
//     type Xor<Rhs: Bit>: Bit;
//     type Eq<Rhs: Bit>: Bit;
//     type Ne<Rhs: Bit>: Bit;
//     type Lt<Rhs: Bit>: Bit;
//     type Gt<Rhs: Bit>: Bit;
//     type Le<Rhs: Bit>: Bit;
//     type Ge<Rhs: Bit>: Bit;
//     type Add<Rhs: Bit, Carry: Bit>: Bit;
//     type AddCarry<Rhs: Bit, Carry: Bit>: Bit;
// }

// pub trait BitAssert: Bit {}

// impl Bit for _0 {
//     const VALUE: u32 = 0;
//     type Not = _1;
//     type And<Rhs: Bit> = <_0 as BitAnd<Rhs>>::Res;
//     type Or<Rhs: Bit> = <_0 as BitOr<Rhs>>::Res;
//     type Xor<Rhs: Bit> = <_0 as BitXor<Rhs>>::Res;
//     type Eq<Rhs: Bit> = <_0 as BitEq<Rhs>>::Res;
//     type Ne<Rhs: Bit> = <_0 as BitNe<Rhs>>::Res;
//     type Lt<Rhs: Bit> = <_0 as BitLt<Rhs>>::Res;
//     type Gt<Rhs: Bit> = <_0 as BitGt<Rhs>>::Res;
//     type Le<Rhs: Bit> = <_0 as BitLe<Rhs>>::Res;
//     type Ge<Rhs: Bit> = <_0 as BitGe<Rhs>>::Res;
//     type Add<Rhs: Bit, Carry: Bit> = <_0 as BitAdd<Rhs, Carry>>::Res;
//     type AddCarry<Rhs: Bit, Carry: Bit> = <_0 as BitAdd<Rhs, Carry>>::Carry;
// }

// impl Bit for _1 {
//     const VALUE: u32 = 1;
//     type Not = _0;
//     type And<Rhs: Bit> = <_1 as BitAnd<Rhs>>::Res;
//     type Or<Rhs: Bit> = <_1 as BitOr<Rhs>>::Res;
//     type Xor<Rhs: Bit> = <_1 as BitXor<Rhs>>::Res;
//     type Eq<Rhs: Bit> = <_1 as BitEq<Rhs>>::Res;
//     type Ne<Rhs: Bit> = <_1 as BitNe<Rhs>>::Res;
//     type Lt<Rhs: Bit> = <_1 as BitLt<Rhs>>::Res;
//     type Gt<Rhs: Bit> = <_1 as BitGt<Rhs>>::Res;
//     type Le<Rhs: Bit> = <_1 as BitLe<Rhs>>::Res;
//     type Ge<Rhs: Bit> = <_1 as BitGe<Rhs>>::Res;
//     type Add<Rhs: Bit, Carry: Bit> = <_1 as BitAdd<Rhs, Carry>>::Res;
//     type AddCarry<Rhs: Bit, Carry: Bit> = <_1 as BitAdd<Rhs, Carry>>::Carry;
// }

// impl BitAssert for _1 {}

// pub type Not<B> = <B as Bit>::Not;
// pub type And<Lhs, Rhs> = <Lhs as Bit>::And<Rhs>;
// pub type Or<Lhs, Rhs> = <Lhs as Bit>::Or<Rhs>;
// pub type Xor<Lhs, Rhs> = <Lhs as Bit>::Xor<Rhs>;
// pub type Eq<Lhs, Rhs> = <Lhs as Bit>::Eq<Rhs>;
// pub type Ne<Lhs, Rhs> = <Lhs as Bit>::Ne<Rhs>;
// pub type Lt<Lhs, Rhs> = <Lhs as Bit>::Lt<Rhs>;
// pub type Gt<Lhs, Rhs> = <Lhs as Bit>::Gt<Rhs>;
// pub type Le<Lhs, Rhs> = <Lhs as Bit>::Le<Rhs>;
// pub type Ge<Lhs, Rhs> = <Lhs as Bit>::Ge<Rhs>;
// pub type Add<Lhs, Rhs, CarryIn = _0> = <Lhs as Bit>::Add<Rhs, CarryIn>;
// pub type AddCarry<Lhs, Rhs, CarryIn = _0> = <Lhs as Bit>::AddCarry<Rhs, CarryIn>;

// pub trait BitAnd<Rhs: Bit>: Bit {
//     type Res: Bit;
// }

// pub trait BitOr<Rhs: Bit>: Bit {
//     type Res: Bit;
// }

// pub trait BitXor<Rhs: Bit>: Bit {
//     type Res: Bit;
// }

// pub trait BitEq<Rhs: Bit>: Bit {
//     type Res: Bit;
// }

// pub trait BitNe<Rhs: Bit>: Bit {
//     type Res: Bit;
// }

// pub trait BitLt<Rhs: Bit>: Bit {
//     type Res: Bit;
// }

// pub trait BitGt<Rhs: Bit>: Bit {
//     type Res: Bit;
// }

// pub trait BitLe<Rhs: Bit>: Bit {
//     type Res: Bit;
// }

// pub trait BitGe<Rhs: Bit>: Bit {
//     type Res: Bit;
// }

// pub trait BitAdd<Rhs: Bit, CarryIn: Bit>: Bit {
//     type Res: Bit;
//     type Carry: Bit;
// }

// impl<Rhs: Bit> BitAnd<Rhs> for _0 {
//     type Res = _0;
// }

// impl<Rhs: Bit> BitAnd<Rhs> for _1 {
//     type Res = Rhs;
// }

// impl<Rhs: Bit> BitOr<Rhs> for _0 {
//     type Res = Rhs;
// }

// impl<Rhs: Bit> BitOr<Rhs> for _1 {
//     type Res = _1;
// }

// impl<Rhs: Bit> BitXor<Rhs> for _0 {
//     type Res = Rhs;
// }

// impl<Rhs: Bit> BitXor<Rhs> for _1 {
//     type Res = Rhs::Not;
// }

// impl<Rhs: Bit> BitEq<Rhs> for _0 {
//     type Res = Rhs::Not;
// }

// impl<Rhs: Bit> BitEq<Rhs> for _1 {
//     type Res = Rhs;
// }

// impl<Rhs: Bit> BitNe<Rhs> for _0 {
//     type Res = Rhs;
// }

// impl<Rhs: Bit> BitNe<Rhs> for _1 {
//     type Res = Rhs::Not;
// }

// impl<Rhs: Bit> BitLt<Rhs> for _0 {
//     type Res = Rhs;
// }

// impl<Rhs: Bit> BitLt<Rhs> for _1 {
//     type Res = _0;
// }

// impl<Rhs: Bit> BitGt<Rhs> for _0 {
//     type Res = _0;
// }

// impl<Rhs: Bit> BitGt<Rhs> for _1 {
//     type Res = Rhs::Not;
// }

// impl<Rhs: Bit> BitLe<Rhs> for _0 {
//     type Res = _1;
// }

// impl<Rhs: Bit> BitLe<Rhs> for _1 {
//     type Res = Rhs;
// }

// impl<Rhs: Bit> BitGe<Rhs> for _0 {
//     type Res = Rhs::Not;
// }

// impl<Rhs: Bit> BitGe<Rhs> for _1 {
//     type Res = _1;
// }

// impl<Rhs: Bit, CarryIn: Bit> BitAdd<Rhs, CarryIn> for _0 {
//     type Res = Rhs::Xor<CarryIn>;
//     type Carry = Rhs::And<CarryIn>;
// }

// impl<Rhs: Bit, CarryIn: Bit> BitAdd<Rhs, CarryIn> for _1 {
//     type Res = <Rhs::Not as Bit>::Xor<CarryIn>;
//     type Carry = Rhs::Or<<Rhs::Not as Bit>::And<CarryIn>>;
// }

// pub trait Bits {
//     const BITS: u32;
//     type B7: Bit;
//     type B6: Bit;
//     type B5: Bit;
//     type B4: Bit;
//     type B3: Bit;
//     type B2: Bit;
//     type B1: Bit;
//     type B0: Bit;

//     type Eq<Rhs: Bits>: Bit;
//     type Ne<Rhs: Bits>: Bit;
//     type Lt<Rhs: Bits>: Bit;
//     type Gt<Rhs: Bits>: Bit;
//     type Le<Rhs: Bits>: Bit;
//     type Ge<Rhs: Bits>: Bit;
//     type Add<Rhs: Bits>: Bits;
// }

// pub struct BitsVal<B7: Bit, B6: Bit, B5: Bit, B4: Bit, B3: Bit, B2: Bit, B1: Bit, B0: Bit>(
//     core::marker::PhantomData<(B7, B6, B5, B4, B3, B2, B1, B0)>,
// );

// impl<B7: Bit, B6: Bit, B5: Bit, B4: Bit, B3: Bit, B2: Bit, B1: Bit, B0: Bit> Bits
//     for BitsVal<B7, B6, B5, B4, B3, B2, B1, B0>
// {
//     const BITS: u32 = (B7::VALUE << 7)
//         | (B6::VALUE << 6)
//         | (B5::VALUE << 5)
//         | (B4::VALUE << 4)
//         | (B3::VALUE << 3)
//         | (B2::VALUE << 2)
//         | (B1::VALUE << 1)
//         | B0::VALUE;
//     type B7 = B7;
//     type B6 = B6;
//     type B5 = B5;
//     type B4 = B4;
//     type B3 = B3;
//     type B2 = B2;
//     type B1 = B1;
//     type B0 = B0;

//     type Eq<Rhs: Bits> = And<
//         And<And<Eq<B7, Rhs::B7>, Eq<B6, Rhs::B6>>, And<Eq<B5, Rhs::B5>, Eq<B4, Rhs::B4>>>,
//         And<And<Eq<B3, Rhs::B3>, Eq<B2, Rhs::B2>>, And<Eq<B1, Rhs::B1>, Eq<B0, Rhs::B0>>>,
//     >;

//     type Ne<Rhs: Bits> = Not<Self::Eq<Rhs>>;

//     type Lt<Rhs: Bits> = Or<
//         Lt<B7, Rhs::B7>,
//         And<
//             Eq<B7, Rhs::B7>,
//             Or<
//                 Lt<B6, Rhs::B6>,
//                 And<
//                     Eq<B6, Rhs::B6>,
//                     Or<
//                         Lt<B5, Rhs::B5>,
//                         And<
//                             Eq<B5, Rhs::B5>,
//                             Or<
//                                 Lt<B4, Rhs::B4>,
//                                 And<
//                                     Eq<B4, Rhs::B4>,
//                                     Or<
//                                         Lt<B3, Rhs::B3>,
//                                         And<
//                                             Eq<B3, Rhs::B3>,
//                                             Or<
//                                                 Lt<B2, Rhs::B2>,
//                                                 And<
//                                                     Eq<B2, Rhs::B2>,
//                                                     Or<
//                                                         Lt<B1, Rhs::B1>,
//                                                         And<Eq<B1, Rhs::B1>, Lt<B0, Rhs::B0>>,
//                                                     >,
//                                                 >,
//                                             >,
//                                         >,
//                                     >,
//                                 >,
//                             >,
//                         >,
//                     >,
//                 >,
//             >,
//         >,
//     >;

//     type Gt<Rhs: Bits> = Rhs::Lt<Self>;

//     type Le<Rhs: Bits> = Not<Self::Gt<Rhs>>;

//     type Ge<Rhs: Bits> = Not<Self::Lt<Rhs>>;

//     type Add<Rhs: Bits> = BitsVal<
//         Add<
//             B7,
//             Rhs::B7,
//             AddCarry<
//                 B6,
//                 Rhs::B6,
//                 AddCarry<
//                     B5,
//                     Rhs::B5,
//                     AddCarry<
//                         B4,
//                         Rhs::B4,
//                         AddCarry<
//                             B3,
//                             Rhs::B3,
//                             AddCarry<B2, Rhs::B1, AddCarry<B1, Rhs::B1, AddCarry<B0, Rhs::B0>>>,
//                         >,
//                     >,
//                 >,
//             >,
//         >,
//         Add<
//             B6,
//             Rhs::B6,
//             AddCarry<
//                 B5,
//                 Rhs::B5,
//                 AddCarry<
//                     B4,
//                     Rhs::B4,
//                     AddCarry<
//                         B3,
//                         Rhs::B3,
//                         AddCarry<B2, Rhs::B1, AddCarry<B1, Rhs::B1, AddCarry<B0, Rhs::B0>>>,
//                     >,
//                 >,
//             >,
//         >,
//         Add<
//             B5,
//             Rhs::B5,
//             AddCarry<
//                 B4,
//                 Rhs::B4,
//                 AddCarry<
//                     B3,
//                     Rhs::B3,
//                     AddCarry<B2, Rhs::B1, AddCarry<B1, Rhs::B1, AddCarry<B0, Rhs::B0>>>,
//                 >,
//             >,
//         >,
//         Add<
//             B4,
//             Rhs::B4,
//             AddCarry<
//                 B3,
//                 Rhs::B3,
//                 AddCarry<B2, Rhs::B1, AddCarry<B1, Rhs::B1, AddCarry<B0, Rhs::B0>>>,
//             >,
//         >,
//         Add<B3, Rhs::B3, AddCarry<B2, Rhs::B1, AddCarry<B1, Rhs::B1, AddCarry<B0, Rhs::B0>>>>,
//         Add<B2, Rhs::B2, AddCarry<B1, Rhs::B1, AddCarry<B0, Rhs::B0>>>,
//         Add<B1, Rhs::B1, AddCarry<B0, Rhs::B0>>,
//         Add<B0, Rhs::B0>,
//     >;
// }

// pub trait BitsEq<Rhs: Bits>: Bits {}
// pub trait BitsNe<Rhs: Bits>: Bits {}
// pub trait BitsLt<Rhs: Bits>: Bits {}
// pub trait BitsGt<Rhs: Bits>: Bits {}
// pub trait BitsLe<Rhs: Bits>: Bits {}
// pub trait BitsGe<Rhs: Bits>: Bits {}

// impl<L: Bits, R: Bits> BitsEq<R> for L where L::Eq<R>: BitAssert {}

// impl<L: Bits, R: Bits> BitsNe<R> for L where L::Ne<R>: BitAssert {}

// impl<L: Bits, R: Bits> BitsLt<R> for L where L::Lt<R>: BitAssert {}

// impl<L: Bits, R: Bits> BitsGt<R> for L where L::Gt<R>: BitAssert {}

// impl<L: Bits, R: Bits> BitsLe<R> for L where L::Le<R>: BitAssert {}

// impl<L: Bits, R: Bits> BitsGe<R> for L where L::Ge<R>: BitAssert {}



// pub struct B<const BITS: usize>;

// macro_rules! impl_bits {
//     ($bits:literal, $b7:ident, $b6:ident, $b5:ident, $b4:ident, $b3:ident, $b2:ident, $b1:ident, $b0:ident) => {
//         impl Bits for B<$bits> {
//             const BITS: u32 = $bits as u32;
//             type B7 = $b7;
//             type B6 = $b6;
//             type B5 = $b5;
//             type B4 = $b4;
//             type B3 = $b3;
//             type B2 = $b2;
//             type B1 = $b1;
//             type B0 = $b0;

//             type Eq<Rhs: Bits> = <BitsVal<$b7, $b6, $b5, $b4, $b3, $b2, $b1, $b0> as Bits>::Eq<Rhs>;

//             type Ne<Rhs: Bits> = <BitsVal<$b7, $b6, $b5, $b4, $b3, $b2, $b1, $b0> as Bits>::Ne<Rhs>;

//             type Lt<Rhs: Bits> = <BitsVal<$b7, $b6, $b5, $b4, $b3, $b2, $b1, $b0> as Bits>::Lt<Rhs>;

//             type Gt<Rhs: Bits> = <BitsVal<$b7, $b6, $b5, $b4, $b3, $b2, $b1, $b0> as Bits>::Gt<Rhs>;

//             type Le<Rhs: Bits> = <BitsVal<$b7, $b6, $b5, $b4, $b3, $b2, $b1, $b0> as Bits>::Le<Rhs>;

//             type Ge<Rhs: Bits> = <BitsVal<$b7, $b6, $b5, $b4, $b3, $b2, $b1, $b0> as Bits>::Ge<Rhs>;

//             type Add<Rhs: Bits> =
//                 <BitsVal<$b7, $b6, $b5, $b4, $b3, $b2, $b1, $b0> as Bits>::Add<Rhs>;
//         }
//     };
// }

// impl_bits!(0, _0, _0, _0, _0, _0, _0, _0, _0);
// impl_bits!(1, _0, _0, _0, _0, _0, _0, _0, _1);
// impl_bits!(2, _0, _0, _0, _0, _0, _0, _1, _0);
// impl_bits!(3, _0, _0, _0, _0, _0, _0, _1, _1);
// impl_bits!(4, _0, _0, _0, _0, _0, _1, _0, _0);
// impl_bits!(5, _0, _0, _0, _0, _0, _1, _0, _1);
// impl_bits!(6, _0, _0, _0, _0, _0, _1, _1, _0);
// impl_bits!(7, _0, _0, _0, _0, _0, _1, _1, _1);
// impl_bits!(8, _0, _0, _0, _0, _1, _0, _0, _0);
// impl_bits!(9, _0, _0, _0, _0, _1, _0, _0, _1);
// impl_bits!(10, _0, _0, _0, _0, _1, _0, _1, _0);
// impl_bits!(11, _0, _0, _0, _0, _1, _0, _1, _1);
// impl_bits!(12, _0, _0, _0, _0, _1, _1, _0, _0);
// impl_bits!(13, _0, _0, _0, _0, _1, _1, _0, _1);
// impl_bits!(14, _0, _0, _0, _0, _1, _1, _1, _0);
// impl_bits!(15, _0, _0, _0, _0, _1, _1, _1, _1);
// impl_bits!(16, _0, _0, _0, _1, _0, _0, _0, _0);
// impl_bits!(17, _0, _0, _0, _1, _0, _0, _0, _1);
// impl_bits!(18, _0, _0, _0, _1, _0, _0, _1, _0);
// impl_bits!(19, _0, _0, _0, _1, _0, _0, _1, _1);
// impl_bits!(20, _0, _0, _0, _1, _0, _1, _0, _0);
// impl_bits!(21, _0, _0, _0, _1, _0, _1, _0, _1);
// impl_bits!(22, _0, _0, _0, _1, _0, _1, _1, _0);
// impl_bits!(23, _0, _0, _0, _1, _0, _1, _1, _1);
// impl_bits!(24, _0, _0, _0, _1, _1, _0, _0, _0);
// impl_bits!(25, _0, _0, _0, _1, _1, _0, _0, _1);
// impl_bits!(26, _0, _0, _0, _1, _1, _0, _1, _0);
// impl_bits!(27, _0, _0, _0, _1, _1, _0, _1, _1);
// impl_bits!(28, _0, _0, _0, _1, _1, _1, _0, _0);
// impl_bits!(29, _0, _0, _0, _1, _1, _1, _0, _1);
// impl_bits!(30, _0, _0, _0, _1, _1, _1, _1, _0);
// impl_bits!(31, _0, _0, _0, _1, _1, _1, _1, _1);
// impl_bits!(32, _0, _0, _1, _0, _0, _0, _0, _0);
// impl_bits!(33, _0, _0, _1, _0, _0, _0, _0, _1);
// impl_bits!(34, _0, _0, _1, _0, _0, _0, _1, _0);
// impl_bits!(35, _0, _0, _1, _0, _0, _0, _1, _1);
// impl_bits!(36, _0, _0, _1, _0, _0, _1, _0, _0);
// impl_bits!(37, _0, _0, _1, _0, _0, _1, _0, _1);
// impl_bits!(38, _0, _0, _1, _0, _0, _1, _1, _0);
// impl_bits!(39, _0, _0, _1, _0, _0, _1, _1, _1);
// impl_bits!(40, _0, _0, _1, _0, _1, _0, _0, _0);
// impl_bits!(41, _0, _0, _1, _0, _1, _0, _0, _1);
// impl_bits!(42, _0, _0, _1, _0, _1, _0, _1, _0);
// impl_bits!(43, _0, _0, _1, _0, _1, _0, _1, _1);
// impl_bits!(44, _0, _0, _1, _0, _1, _1, _0, _0);
// impl_bits!(45, _0, _0, _1, _0, _1, _1, _0, _1);
// impl_bits!(46, _0, _0, _1, _0, _1, _1, _1, _0);
// impl_bits!(47, _0, _0, _1, _0, _1, _1, _1, _1);
// impl_bits!(48, _0, _0, _1, _1, _0, _0, _0, _0);
// impl_bits!(49, _0, _0, _1, _1, _0, _0, _0, _1);
// impl_bits!(50, _0, _0, _1, _1, _0, _0, _1, _0);
// impl_bits!(51, _0, _0, _1, _1, _0, _0, _1, _1);
// impl_bits!(52, _0, _0, _1, _1, _0, _1, _0, _0);
// impl_bits!(53, _0, _0, _1, _1, _0, _1, _0, _1);
// impl_bits!(54, _0, _0, _1, _1, _0, _1, _1, _0);
// impl_bits!(55, _0, _0, _1, _1, _0, _1, _1, _1);
// impl_bits!(56, _0, _0, _1, _1, _1, _0, _0, _0);
// impl_bits!(57, _0, _0, _1, _1, _1, _0, _0, _1);
// impl_bits!(58, _0, _0, _1, _1, _1, _0, _1, _0);
// impl_bits!(59, _0, _0, _1, _1, _1, _0, _1, _1);
// impl_bits!(60, _0, _0, _1, _1, _1, _1, _0, _0);
// impl_bits!(61, _0, _0, _1, _1, _1, _1, _0, _1);
// impl_bits!(62, _0, _0, _1, _1, _1, _1, _1, _0);
// impl_bits!(63, _0, _0, _1, _1, _1, _1, _1, _1);
// impl_bits!(64, _0, _1, _0, _0, _0, _0, _0, _0);
// impl_bits!(65, _0, _1, _0, _0, _0, _0, _0, _1);
// impl_bits!(66, _0, _1, _0, _0, _0, _0, _1, _0);
// impl_bits!(67, _0, _1, _0, _0, _0, _0, _1, _1);
// impl_bits!(68, _0, _1, _0, _0, _0, _1, _0, _0);
// impl_bits!(69, _0, _1, _0, _0, _0, _1, _0, _1);
// impl_bits!(70, _0, _1, _0, _0, _0, _1, _1, _0);
// impl_bits!(71, _0, _1, _0, _0, _0, _1, _1, _1);
// impl_bits!(72, _0, _1, _0, _0, _1, _0, _0, _0);
// impl_bits!(73, _0, _1, _0, _0, _1, _0, _0, _1);
// impl_bits!(74, _0, _1, _0, _0, _1, _0, _1, _0);
// impl_bits!(75, _0, _1, _0, _0, _1, _0, _1, _1);
// impl_bits!(76, _0, _1, _0, _0, _1, _1, _0, _0);
// impl_bits!(77, _0, _1, _0, _0, _1, _1, _0, _1);
// impl_bits!(78, _0, _1, _0, _0, _1, _1, _1, _0);
// impl_bits!(79, _0, _1, _0, _0, _1, _1, _1, _1);
// impl_bits!(80, _0, _1, _0, _1, _0, _0, _0, _0);
// impl_bits!(81, _0, _1, _0, _1, _0, _0, _0, _1);
// impl_bits!(82, _0, _1, _0, _1, _0, _0, _1, _0);
// impl_bits!(83, _0, _1, _0, _1, _0, _0, _1, _1);
// impl_bits!(84, _0, _1, _0, _1, _0, _1, _0, _0);
// impl_bits!(85, _0, _1, _0, _1, _0, _1, _0, _1);
// impl_bits!(86, _0, _1, _0, _1, _0, _1, _1, _0);
// impl_bits!(87, _0, _1, _0, _1, _0, _1, _1, _1);
// impl_bits!(88, _0, _1, _0, _1, _1, _0, _0, _0);
// impl_bits!(89, _0, _1, _0, _1, _1, _0, _0, _1);
// impl_bits!(90, _0, _1, _0, _1, _1, _0, _1, _0);
// impl_bits!(91, _0, _1, _0, _1, _1, _0, _1, _1);
// impl_bits!(92, _0, _1, _0, _1, _1, _1, _0, _0);
// impl_bits!(93, _0, _1, _0, _1, _1, _1, _0, _1);
// impl_bits!(94, _0, _1, _0, _1, _1, _1, _1, _0);
// impl_bits!(95, _0, _1, _0, _1, _1, _1, _1, _1);
// impl_bits!(96, _0, _1, _1, _0, _0, _0, _0, _0);
// impl_bits!(97, _0, _1, _1, _0, _0, _0, _0, _1);
// impl_bits!(98, _0, _1, _1, _0, _0, _0, _1, _0);
// impl_bits!(99, _0, _1, _1, _0, _0, _0, _1, _1);
// impl_bits!(100, _0, _1, _1, _0, _0, _1, _0, _0);
// impl_bits!(101, _0, _1, _1, _0, _0, _1, _0, _1);
// impl_bits!(102, _0, _1, _1, _0, _0, _1, _1, _0);
// impl_bits!(103, _0, _1, _1, _0, _0, _1, _1, _1);
// impl_bits!(104, _0, _1, _1, _0, _1, _0, _0, _0);
// impl_bits!(105, _0, _1, _1, _0, _1, _0, _0, _1);
// impl_bits!(106, _0, _1, _1, _0, _1, _0, _1, _0);
// impl_bits!(107, _0, _1, _1, _0, _1, _0, _1, _1);
// impl_bits!(108, _0, _1, _1, _0, _1, _1, _0, _0);
// impl_bits!(109, _0, _1, _1, _0, _1, _1, _0, _1);
// impl_bits!(110, _0, _1, _1, _0, _1, _1, _1, _0);
// impl_bits!(111, _0, _1, _1, _0, _1, _1, _1, _1);
// impl_bits!(112, _0, _1, _1, _1, _0, _0, _0, _0);
// impl_bits!(113, _0, _1, _1, _1, _0, _0, _0, _1);
// impl_bits!(114, _0, _1, _1, _1, _0, _0, _1, _0);
// impl_bits!(115, _0, _1, _1, _1, _0, _0, _1, _1);
// impl_bits!(116, _0, _1, _1, _1, _0, _1, _0, _0);
// impl_bits!(117, _0, _1, _1, _1, _0, _1, _0, _1);
// impl_bits!(118, _0, _1, _1, _1, _0, _1, _1, _0);
// impl_bits!(119, _0, _1, _1, _1, _0, _1, _1, _1);
// impl_bits!(120, _0, _1, _1, _1, _1, _0, _0, _0);
// impl_bits!(121, _0, _1, _1, _1, _1, _0, _0, _1);
// impl_bits!(122, _0, _1, _1, _1, _1, _0, _1, _0);
// impl_bits!(123, _0, _1, _1, _1, _1, _0, _1, _1);
// impl_bits!(124, _0, _1, _1, _1, _1, _1, _0, _0);
// impl_bits!(125, _0, _1, _1, _1, _1, _1, _0, _1);
// impl_bits!(126, _0, _1, _1, _1, _1, _1, _1, _0);
// impl_bits!(127, _0, _1, _1, _1, _1, _1, _1, _1);
// impl_bits!(128, _1, _0, _0, _0, _0, _0, _0, _0);
