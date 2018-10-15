use std_::cmp;
use std_::fmt::{self,Debug};
use std_::ops::{Add, Range, Shr, Sub};

use crate_::prelude::*;
use crate_::fn_types::ConstLEOp;

#[allow(unused_imports)]
use typenum::operator_aliases::{Sub1,Add1,Diff as Sub_,Sum,Shleft,Shright};

use num_traits::cast::AsPrimitive;

/// Trait for ConstRange<Start,End>.
///
/// Used to determine the integer type stored in RangedUIntR (Compressed),
/// and the type returned by `RangedUIntR::value` (Decompressed).
///
pub trait RangeTypes {
    /// The integer type stored in the RangedUIntR.
    type Compressed: Copy
        + 'static
        + Into<Self::Decompressed>
        + Debug
        + fmt::Display
        + PartialEq
        + PartialOrd
        + Eq
        + Ord;

    /// The integer taken by RangedUIntR::new and returned by RangedUIntR::value.
    type Decompressed: Sub<Self::Decompressed, Output = Self::Decompressed>
        + Add<Self::Decompressed, Output = Self::Decompressed>
        + Copy
        + 'static
        + AsPrimitive<Self::Compressed>
        + Debug
        + fmt::Display
        + PartialEq
        + PartialOrd
        + Eq
        + Ord
        + From<u8>;

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

impl<R, SI, RI,Start,End,IsEmpty,EndSub1> RangeTypes for R
where
    R: TypeIdentity<Type=ConstRange<Start,End>>,
    Start:IntoRuntime<RI>,
    End:IntoRuntime<RI>,
    
    ConstLEOp:TypeFn_<(End,Start),Output=IsEmpty>,
    IsEmpty:IntoRuntime<bool>,
    
    SaturatingSub1:TypeFn_<End,Output=EndSub1>,
    EndSub1:IntoRuntime<RI>,
    
    IntTypeOfRange: TypeFn_<R, Output = SI>,
    IntTypeOf: TypeFn_<End, Output = RI>,

    SI: Copy + 'static + Into<RI> + Debug + fmt::Display + PartialEq + PartialOrd + Eq + Ord,
    RI: Sub<RI, Output = RI>
        + Add<RI, Output = RI>
        + Copy
        + Ord
        + 'static
        + AsPrimitive<SI>
        + Debug
        + fmt::Display
        + PartialEq
        + PartialOrd
        + Eq
        + Ord
        + From<u8>,
{
    type Compressed = SI;
    type Decompressed = RI;

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
    /// The Integer type that would fit the distance between R::start and R::end.
    pub fn IntTypeOfRange[R](R)
    where[
        R     :RangeTrait,
        R::end:Sub<R::start,Output=N>,
        SaturatingSub1:TypeFn_<N,Output=substart>,
        IntTypeOf:TypeFn_<substart,Output=out>
    ]{
        let N;let out;let substart;
        out
    }
}

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


type_fn!{
    /// Subtracts 1 from an unsigned integer.Stopping at 0.
    pub fn SaturatingSub1[L](L)
    where[
        L:ConstEq_<U0>,
        SaturatingSub1Helper:TypeFn_<(L::Output,L) ,Output=Out>
    ]{
        let Out;Out
    }
}

type_fn!{
    fn 
        SaturatingSub1Helper(True,U0){U0}

        SaturatingSub1Helper[L](False,L)
        where[ L:Sub<U1> ]
        { L::Output }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn saturating_sub(){
        let _:U0=TypeFn::<SaturatingSub1,U0>::MTVAL;
        let _:U0=TypeFn::<SaturatingSub1,U1>::MTVAL;
        let _:U1=TypeFn::<SaturatingSub1,U2>::MTVAL;
        let _:U2=TypeFn::<SaturatingSub1,U3>::MTVAL;
        let _:U3=TypeFn::<SaturatingSub1,U4>::MTVAL;
    }

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
        test_int_type!( (u8,Shleft<U1,U0>) );
        test_int_type!( (u8,Shleft<U1,U1>) );
        test_int_type!( (u8,Shleft<U1,U2>) );
        test_int_type!( (u8,Shleft<U1,U3>) );
        test_int_type!( (u8,Shleft<U1,U4>) );
        test_int_type!( (u8,Shleft<U1,U5>) );
        test_int_type!( (u8,Shleft<U1,U6>) );
        test_int_type!( (u8,Shleft<U1,U7>) );
        test_int_type!( (u8,Sub1<Shleft<U1,U8>>) );
        test_int_type!( (u16,Shleft<U1,U8>) );
        test_int_type!( (u16,Shleft<U1,U9>) );
        test_int_type!( (u16,Shleft<U1,U10>) );
        test_int_type!( (u16,Shleft<U1,U13>) );
        test_int_type!( (u16,Shleft<U1,U14>) );
        test_int_type!( (u16,Shleft<U1,U15>) );
        test_int_type!( (u16,Sub1<Shleft<U1,U16>>) );
        test_int_type!( (u32,Shleft<U1,U16>) );
        test_int_type!( (u32,Shleft<U1,U17>) );
        test_int_type!( (u32,Shleft<U1,U18>) );
        test_int_type!( (u32,Shleft<U1,U29>) );
        test_int_type!( (u32,Shleft<U1,U30>) );
        test_int_type!( (u32,Shleft<U1,U31>) );
        test_int_type!( (u32,Sub1<Shleft<U1,U32>>) );
        test_int_type!( (u64,Shleft<U1,U32>) );
        test_int_type!( (u64,Shleft<U1,U33>) );
        test_int_type!( (u64,Shleft<U1,U34>) );
        test_int_type!( (u64,Shleft<U1,U60>) );
        test_int_type!( (u64,Shleft<U1,U61>) );
        test_int_type!( (u64,Shleft<U1,U62>) );
        test_int_type!( (u64,Shleft<U1,U63>) );
        test_int_type!( (u64,Sub1<Shleft<U1,U64>>) );
        
    }

}