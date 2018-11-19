use prelude::*;

use crate_::fn_adaptors::{Const,GetRhs};
use crate_::ops::*;
use crate_::std_ops::*;

use typenum::bit::{B0, B1};
use typenum::marker_traits::{Bit, Integer, NonZero, Unsigned};
use typenum::{Equal, Greater, Less, NInt, PInt, U0, UInt, UTerm, Z0};


////////////////////////////////////////////////////////////////////////////////////////////////

impl ConstTypeOf_ for B0 {
    type Type = BitType;
}
impl ConstTypeOf_ for B1 {
    type Type = BitType;
}
impl<U: Unsigned + Default + Copy, B: Bit + Default + Copy> ConstTypeOf_ for UInt<U, B> {
    type Type = UnsignedInteger;
}
impl<U: Unsigned + Default + NonZero + Copy> ConstTypeOf_ for PInt<U> {
    type Type = SignedInteger;
}
impl<U: Unsigned + Default + NonZero + Copy> ConstTypeOf_ for NInt<U> {
    type Type = SignedInteger;
}
impl ConstTypeOf_ for Z0 {
    type Type = SignedInteger;
}
impl ConstTypeOf_ for UTerm {
    type Type = UnsignedInteger;
}


/////////////////////////////////////////////////////////////////////////////////////////////////

trait Dummy00<T>{
    const VALUE_00:T;
}

macro_rules! from_const {
    (signed $([ $(#[$attr:meta])* $type:ty,$constant:ident,$unsigned_const:ident ])* ) => {
        $(
            $(#[$attr])*
            impl<U: Unsigned +Default + NonZero+Copy> IntoRuntime<$type> for PInt<U>{
                fn to_runtime()->$type{
                    <PInt<U> as Integer>::$constant
                }
            }


            $(#[$attr])*
            impl<U: Unsigned +Default + NonZero+Copy> Dummy00<$type> for NInt<U>{
                // Emulating 2's complement negation to avoid typenum bug 
                // with the minimum signed integers
                const VALUE_00:$type=(!<U as Unsigned>::$unsigned_const + 1)as $type;
            }

            $(#[$attr])*
            impl<U: Unsigned +Default + NonZero+Copy> IntoRuntime<$type> for NInt<U>{
                fn to_runtime()->$type{
                    <Self as Dummy00<$type>>::VALUE_00
                }
            }
            $(#[$attr])*
            impl IntoRuntime<$type> for Z0{
                fn to_runtime()->$type{
                    0
                }
            }

            $(#[$attr])*
            #[cfg(rust_1_22)]
            impl<U: Unsigned +Default + NonZero+Copy> IntoConstant<$type> for PInt<U>{
                const VALUE:$type=<PInt<U> as Integer>::$constant;
            }

            $(#[$attr])*
            #[cfg(rust_1_22)]
            impl<U: Unsigned +Default + NonZero+Copy> IntoConstant<$type> for NInt<U>{
                const VALUE:$type=<Self as Dummy00<$type>>::VALUE_00;
            }

            $(#[$attr])*
            #[cfg(rust_1_22)]
            impl IntoConstant<$type> for Z0{
                const VALUE:$type=0;
            }


        )*
    };
    (unsigned $([ $(#[$attr:meta])* $type:ty,$constant:ident ])* ) => {
        $(
            $(#[$attr])*
            impl<U: Unsigned +Default +Copy,B:Bit+Copy+Default>
                IntoRuntime<$type> for UInt<U,B>
            {
                fn to_runtime()->$type{
                    <UInt<U,B> as Unsigned>::$constant
                }
            }

            $(#[$attr])*
            #[cfg(rust_1_22)]
            impl<U: Unsigned +Default +Copy,B:Bit+Copy+Default> IntoConstant<$type> for UInt<U,B>{
                const VALUE:$type= <UInt<U,B> as Unsigned>::$constant ;
            }

            $(#[$attr])*
            impl IntoRuntime<$type> for UTerm{
                fn to_runtime()->$type{
                    0
                }
            }

            $(#[$attr])*
            #[cfg(rust_1_22)]
            impl IntoConstant<$type> for UTerm{
                const VALUE:$type=0;
            }
        )*
    };
}

from_const!{signed
    [i8   ,I8   ,U8]
    [i16  ,I16  ,U16]
    [i32  ,I32  ,U32]
    [isize,ISIZE,USIZE]
    [i64  ,I64  ,U64]
    // Re-enable one typenum does not require nightly to compile with the i128 feature.
    //[#[cfg(feature="i128")] i128 ,I128]
}

from_const!{unsigned
    [u8   ,U8]
    [i8   ,I8]
    [u16  ,U16]
    [i16  ,I16]
    [u32  ,U32]
    [i32  ,I32]
    [usize,USIZE]
    [isize,ISIZE]
    [i64,I64]
    [u64,U64]
    // Re-enable one typenum does not require nightly to compile with the i128 feature.
    //[#[cfg(feature="i128")] u128,U128] 
    //[#[cfg(feature="i128")] i128,I128]
}

#[cfg(rust_1_26)]
impl IntoConstType_ for u128{ type ToConst=UnsignedInteger; }

#[cfg(rust_1_26)]
impl IntoConstType_ for i128{ type ToConst=SignedInteger; }

macro_rules! compiletime_equiv {
    ( $comp:ty [$($runt:ty),*] ) => {
        $( impl IntoConstType_ for  $runt{ type ToConst=$comp; } )*
    }
}

compiletime_equiv!{UnsignedInteger [u8,u16,u32,usize,u64] }
compiletime_equiv!{SignedInteger   [i8,i16,i32,isize,i64] }

/////////////////////////////////////////////////////////////////////////////////////////////////

mod sealed {
    pub trait Sealed {}
}
use self::sealed::Sealed;

/////////////////////////////////////////////////////////////////////////////////////////////////

pub trait TNOrdering_: Sealed {}

impl Sealed for Less {}
impl TNOrdering_ for Less {}
impl Sealed for Equal {}
impl TNOrdering_ for Equal {}
impl Sealed for Greater {}
impl TNOrdering_ for Greater {}

#[derive(Debug, Default, Copy, Clone)]
/// The ConstType of typenum::{Equal, Greater, Less}
pub struct TNOrderingType;

impl ConstType for TNOrderingType {}

impl ConstTypeOf_ for Equal{
    type Type=TNOrderingType;
}
impl ConstTypeOf_ for Greater{
    type Type=TNOrderingType;
}
impl ConstTypeOf_ for Less{
    type Type=TNOrderingType;
}



/////////////////////////////////////////////////////////////////////////////////////////////////

/// The ConstType of signed typenum integers.
#[derive(Debug, Default, Copy, Clone)]
pub struct SignedInteger;

impl ConstType for SignedInteger {}

/// The ConstType of unsigned typenum integers.
#[derive(Debug, Default, Copy, Clone)]
pub struct UnsignedInteger;

impl ConstType for UnsignedInteger {}

/// The ConstType of typenum::bit::{B0,B1}
#[derive(Debug, Default, Copy, Clone)]
pub struct BitType;

impl ConstType for BitType {}

/////////////////////////////////////////////////////////////////////////////////////////////////


impl ConstFrom_<UTerm> for SignedInteger {
    type Output=Z0;
}

impl ConstFrom_<Z0> for UnsignedInteger {
    type Output=UTerm;
}

impl<U,B> ConstFrom_<UInt<U, B>> for SignedInteger
where
    U: Unsigned + Default + Copy, 
    B: Bit + Default + Copy,
{
    type Output = PInt<UInt<U, B>> ;
}
impl<U> ConstFrom_<PInt<U>> for UnsignedInteger
where
    U: Unsigned + Default + NonZero + Copy,
{
    type Output = U;
}

/////////////////////////////////////////////////////////////////////////////////////////////////


impl IntegerConsts for SignedInteger{
    type Zero=Z0;
    type One=P1;

    type Min=None_;
    type Max=None_;
}

impl IntegerConsts for UnsignedInteger{
    type Zero=U0;
    type One=U1;

    type Min=Some_<U0>;
    type Max=None_;
}

/////////////////////////////////////////////////////////////////////////////////////////////////


macro_rules! with_function {
    (unsigned; $bin_op:ident[ $($params:ident),* $(,)* ] , $function:ty ) => (
        impl<$($params,)* U,B,Out> $bin_op<$($params,)*> for UInt<U, B> 
        where
            U: Unsigned + Default + Copy, 
            B: Bit + Default + Copy,
            $function:TypeFn_<(Self$(,$params)*),Output=Out>,
        {
            type Output = Out;
        }
        impl<$($params,)* Out> $bin_op<$($params,)*> for UTerm 
        where 
            $function:TypeFn_<(Self$(,$params)*),Output=Out>,
        {
            type Output = Out;
        }
    );
    (signed; $bin_op:ident [ $($params:ident),* $(,)* ] , $function:ty ) => (
        impl<$($params,)* U,Out> $bin_op<$($params,)*> for NInt<U> 
        where
            U: Unsigned + Default + NonZero + Copy,
            $function:TypeFn_<(Self $(,$params)*),Output=Out>,
        {
            type Output = Out;
        }
        impl<$($params,)* U,Out> $bin_op<$($params,)*> for PInt<U> 
        where
            U: Unsigned + Default + NonZero + Copy,
            $function:TypeFn_<(Self $(,$params)*),Output=Out>,
        {
            type Output = Out;
        }
        impl<$($params,)* Out> $bin_op<$($params,)*> for Z0 
        where 
            $function:TypeFn_<(Self $(,$params)*),Output=Out>,
        {
            type Output = Out;
        }

    )
}


impl<R,U,B,Out> SatSub_<R> for UInt<U, B> 
where
    U: Unsigned + Default + Copy, 
    B: Bit + Default + Copy,
    If<ConstGEOp, SubOp,Const<U0> >:TypeFn_<(Self,R),Output=Out>
{
    type Output = Out;
}
impl<R> SatSub_<R> for UTerm {
    type Output = UTerm;
}

with_function!{ signed; SatSub_[R] , SubOp }


/////////////////////////////////////////////////////////////////////////////////////////////////


impl<U,B,Out> SatSub1_ for UInt<U, B> 
where
    U: Unsigned + Default + Copy, 
    B: Bit + Default + Copy,
    Sub1Op:TypeFn_<Self,Output=Out>
{
    type Output = Out;
}
impl SatSub1_ for UTerm {
    type Output = UTerm;
}

with_function!{ signed; SatSub1_[] , Sub1Op }


/////////////////////////////////////////////////////////////////////////////////////////////////


with_function!{ signed; SafeSub_[R] , (SubOp,NewSome) }
with_function!{ unsigned; SafeSub_[R] , SafeSubHelper }

type SafeSubHelper=
    If<ConstGEOp,(SubOp,NewSome),NewNone>;

/////////////////////////////////////////////////////////////////////////////////////////////////

with_function!{ unsigned; SafeDiv_[R] , SafeDivHelper }
with_function!{ signed; SafeDiv_[R] , SafeDivHelper }

type SafeDivHelper=
    If<(GetRhs,IsZeroOp),NewNone,(DivOp,NewSome)>;


/////////////////////////////////////////////////////////////////////////////////////////////////

impl< U,B> IsZero_ for UInt<U, B> 
where
    U: Unsigned + Default + Copy, 
    B: Bit + Default + Copy,
{
    type Output = False;
}
impl<> IsZero_ for UTerm {
    type Output = True;
}
impl< U> IsZero_ for NInt<U> 
where
    U: Unsigned + Default + NonZero + Copy,
{
    type Output = False;
}
impl< U> IsZero_ for PInt<U> 
where
    U: Unsigned + Default + NonZero + Copy,
{
    type Output = False;
}
impl<> IsZero_ for Z0 {
    type Output = True;
}


/////////////////////////////////////////////////////////////////////////////////////////////////

impl< U,B> AbsVal_ for UInt<U, B> 
where
    U: Unsigned + Default + Copy, 
    B: Bit + Default + Copy,
{
    type Output = Self;
}
impl<> AbsVal_ for UTerm {
    type Output = Self;
}
impl< U> AbsVal_ for NInt<U> 
where
    U: Unsigned + Default + NonZero + Copy,
{
    type Output = PInt<U>;
}
impl< U> AbsVal_ for PInt<U> 
where
    U: Unsigned + Default + NonZero + Copy,
{
    type Output = Self;
}
impl<> AbsVal_ for Z0 {
    type Output = Self;
}

/////////////////////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////////////////////


/////////////////////////////////////////////////////////////////////////////////////////////////
