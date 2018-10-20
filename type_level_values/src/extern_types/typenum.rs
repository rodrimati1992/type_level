use prelude::*;

use crate_::ops::{IntegerConsts,AssertEq,AssertFnRet};

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

macro_rules! from_const {
    (signed $([ $(#[$attr:meta])* $type:ty,$constant:ident ])* ) => {
        $(
            $(#[$attr])*
            impl<U: Unsigned +Default + NonZero+Copy> IntoRuntime<$type> for PInt<U>{
                fn to_runtime()->$type{
                    <PInt<U> as Integer>::$constant
                }
            }
            $(#[$attr])*
            impl<U: Unsigned +Default + NonZero+Copy> IntoRuntime<$type> for NInt<U>{
                fn to_runtime()->$type{
                    <NInt<U> as Integer>::$constant
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
                const VALUE:$type=<NInt<U> as Integer>::$constant;
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
    [i8   ,I8]
    [i16  ,I16]
    [i32  ,I32]
    [isize,ISIZE]
    [i64  ,I64]
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
pub struct TNOrderingType;

impl ConstType for TNOrderingType {}

/////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, Copy, Clone)]
pub struct SignedInteger;

impl ConstType for SignedInteger {}

#[derive(Debug, Default, Copy, Clone)]
pub struct UnsignedInteger;

impl ConstType for UnsignedInteger {}

#[derive(Debug, Default, Copy, Clone)]
pub struct BitType;

impl ConstType for BitType {}

/////////////////////////////////////////////////////////////////////////////////////////////////


impl IntegerConsts for SignedInteger{
    type Zero=Z0;
    type One=P1;
}

impl IntegerConsts for UnsignedInteger{
    type Zero=U0;
    type One=U1;
}

/////////////////////////////////////////////////////////////////////////////////////////////////


/////////////////////////////////////////////////////////////////////////////////////////////////


/////////////////////////////////////////////////////////////////////////////////////////////////
