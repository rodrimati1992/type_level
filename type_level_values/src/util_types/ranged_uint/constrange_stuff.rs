use std_::cmp;
use std_::fmt::{self,Debug};
use std_::ops::{Add, Range, Shr, Sub,};

use crate_::prelude::*;
use crate_::std_ops::{SubTA,ShlTA};
use crate_::ops::{ConstLEOp,ConstGEMt,SatSub1Op,Sub1,Add1,};

use super::RangedUIntR;

use num_traits::cast::AsPrimitive;

/// Trait for .
pub trait RangeTypes<N> {

    /// Whether the range is empty.
    fn is_empty()->bool;
    
    /// The start of the range.
    fn start() -> Self::Decompressed ;
    
    /// The end of the exclusive range.
    /// Returns None if the range covers all of the maximum integer size available.
    fn end() -> Option<Self::Decompressed> ;
    
    /// The end of the inclusive range.
    fn end_inclusive() -> Self::Decompressed ;
}


pub type Compressed<This>=
    <This as RangeTypes>::Compressed;

pub type Decompressed<This>=
    <This as RangeTypes>::Decompressed;


impl<R> RangeTypes for RangedUIntR<R>
where R:RangeTypes
{
    type Compressed=R::Compressed;
    type Decompressed=R::Decompressed;

    #[inline]
    fn is_empty()->bool{
        R::is_empty()
    }
    #[inline]
    fn start() -> Self::Decompressed {
        R::start()
    }
    #[inline]
    fn end() -> Option<Self::Decompressed> {
        R::end()
    }
    #[inline]
    fn end_inclusive() -> Self::Decompressed {
        R::end_inclusive()
    }
}

impl<N,Start,End,IsEmpty,EndSub1> RangeTypes<N> for ConstRange<Start,End>
where
    Start:IntoRuntime<N>,
    End:IntoRuntime<N>,
    
    ConstLEOp:TypeFn_<(End,Start),Output=IsEmpty>,
    IsEmpty:IntoRuntime<bool>,
    
    SatSub1Op:TypeFn_<End,Output=EndSub1>,
    EndSub1:IntoRuntime<N>,
    
    AssertEndFitsInType: TypeFn_<(N,End)>,



{

    #[inline]
    fn is_empty()->bool{
        IsEmpty::to_runtime()
    }
    #[inline]
    fn start() -> Self::Decompressed {
        Start::to_runtime()
    }
    #[inline]
    fn end() -> Option<Self::Decompressed> {
        let end=End::to_runtime();
        if end < EndSub1::to_runtime() { None }else{ Some(end) }
    }
    #[inline]
    fn end_inclusive() -> Self::Decompressed {
        EndSub1::to_runtime()
    }
}


type_fn!{
    fn AssertEndFitsInType[Type,End](Type,End)
    where[
        IntTypeOf:TypeFn_<End,Output=TypeOfEnd>
        AssertThatOp:TypeFn_<(
            (Type,End),
            AssertEndFitsInTypePred,
            TypeThatFitsEnd<TypeOfEnd>
        )>
    ]{
        let TypeOfEnd;
        ()
    }
}

struct TypeThatFitsEnd<End>(End);

type_fn!{
    fn AssertEndFitsInTypePred[Type,End](Type,End)
    where[ (Const<Type>,EndFromIntType,ConstGEMt<End>):TypeFn_<(),Output=Out> ]
    { let Out;Out }
}


type_fn!{
    pub fn 
        EndFromIntType(u8){ ShlTA<U1,U8> }
        EndFromIntType(u16){ ShlTA<U1,U16> }
        EndFromIntType(u32){ ShlTA<U1,U32> }
        EndFromIntType(u64){ ShlTA<U1,U64> }
        EndFromIntType(usize){ UWordEnd }

        EndFromIntType(i8){ ShlTA<U1,U7> }
        EndFromIntType(i16){ ShlTA<U1,U15> }
        EndFromIntType(i32){ ShlTA<U1,U31> }
        EndFromIntType(i64){ ShlTA<U1,U63> }
        EndFromIntType(isize){ ShrTA<UWordEnd,U1> }
}


#[cfg(target_pointer_width = "8")]
type UWord = u8;
#[cfg(target_pointer_width = "16")]
type UWord = u16;
#[cfg(target_pointer_width = "32")]
type UWord = u32;
#[cfg(target_pointer_width = "64")]
type UWord = u64;
#[cfg(target_pointer_width = "128")]
type UWord = u128;


#[cfg(target_pointer_width = "8")]
type UWordEnd = ShlTA<1,U8>;
#[cfg(target_pointer_width = "16")]
type UWordEnd = ShlTA<1,U16>;
#[cfg(target_pointer_width = "32")]
type UWordEnd = ShlTA<1,U32>;
#[cfg(target_pointer_width = "64")]
type UWordEnd = ShlTA<1,U64>;
#[cfg(target_pointer_width = "128")]
type UWordEnd = ShlTA<1,U128>;




type_fn!{
    /// The Integer type of a type-level integer,one of u8,u16,u32,u64.
    pub fn IntTypeOf[N](N)
    where[
        N     :Shr<U8 ,Output=DivU8 >,
        DivU8 :Shr<U8 ,Output=DivU16>,
        DivU16:Shr<U16,Output=DivU32>,
        DivU32:Shr<U32,Output=DivU64>,

        DivU8 :ConstEq_<U0,Output=IsU16>,
        DivU16:ConstEq_<U0,Output=IsU32>,
        DivU32:ConstEq_<U0,Output=IsU64>,
        DivU64:ConstEq_<U0,Output=IsU128>,
        IntTypeHelper:TypeFn_<(IsU16,IsU32,IsU64,IsU128),Output=out>,
    ]{
        let DivU8;let DivU16;let DivU32;let DivU64;
        let IsU16;let IsU32 ;let IsU64; let IsU128;
        let out;
        out
    }
}

// #[cfg(not(rust_1_26))]
type MaxUInt=u64;

// Re-enable once typenum does not require nightly to compile with the i128 feature.
// #[cfg(rust_1_26)]
// type MaxUInt=u128;

type_fn!{
    fn IntTypeHelper(True,True,True,True){ u8 }
       IntTypeHelper(False,True,True,True){ u16 }
       IntTypeHelper(False,False,True,True){ u32 }
       IntTypeHelper(False,False,False,True){ u64 }
       IntTypeHelper(False,False,False,False){ MaxUInt }
}

#[cfg(all(test,feature="passed_tests"))]
mod test{
    use super::*;


    #[test]
    fn int_type(){
        macro_rules! test_int_type {
            (
                ( $int_ty:ty,$int_val:ty )
            ) => (
                let _:VariantPhantom<$int_ty>=
                    TypeFn::<IntTypeOf,$int_val>::T;
            )
        }
        
        test_int_type!( (u8,U0) );
        test_int_type!( (u8,ShlTA<U1,U0>) );
        test_int_type!( (u8,ShlTA<U1,U1>) );
        test_int_type!( (u8,ShlTA<U1,U2>) );
        test_int_type!( (u8,ShlTA<U1,U3>) );
        test_int_type!( (u8,ShlTA<U1,U4>) );
        test_int_type!( (u8,ShlTA<U1,U5>) );
        test_int_type!( (u8,ShlTA<U1,U6>) );
        test_int_type!( (u8,ShlTA<U1,U7>) );
        test_int_type!( (u8,Sub1<ShlTA<U1,U8>>) );
        test_int_type!( (u16,ShlTA<U1,U8>) );
        test_int_type!( (u16,ShlTA<U1,U9>) );
        test_int_type!( (u16,ShlTA<U1,U10>) );
        test_int_type!( (u16,ShlTA<U1,U13>) );
        test_int_type!( (u16,ShlTA<U1,U14>) );
        test_int_type!( (u16,ShlTA<U1,U15>) );
        test_int_type!( (u16,Sub1<ShlTA<U1,U16>>) );
        test_int_type!( (u32,ShlTA<U1,U16>) );
        test_int_type!( (u32,ShlTA<U1,U17>) );
        test_int_type!( (u32,ShlTA<U1,U18>) );
        test_int_type!( (u32,ShlTA<U1,U29>) );
        test_int_type!( (u32,ShlTA<U1,U30>) );
        test_int_type!( (u32,ShlTA<U1,U31>) );
        test_int_type!( (u32,Sub1<ShlTA<U1,U32>>) );
        test_int_type!( (u64,ShlTA<U1,U32>) );
        test_int_type!( (u64,ShlTA<U1,U33>) );
        test_int_type!( (u64,ShlTA<U1,U34>) );
        test_int_type!( (u64,ShlTA<U1,U60>) );
        test_int_type!( (u64,ShlTA<U1,U61>) );
        test_int_type!( (u64,ShlTA<U1,U62>) );
        test_int_type!( (u64,ShlTA<U1,U63>) );
        test_int_type!( (u64,Sub1<ShlTA<U1,U64>>) );
        
    }

}