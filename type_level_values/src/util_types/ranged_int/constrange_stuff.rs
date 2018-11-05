use std_::cmp;
use std_::fmt::{self,Debug};
use std_::ops::{Add, Range, Shr, Sub,Neg,};
#[cfg(rust_1_27)]
use std_::ops::RangeInclusive;

use crate_::prelude::*;
use crate_::fn_adaptors::{
    Const,
    IdentityFn,
    GetLhs,
};
use crate_::extern_types::typenum::{UnsignedInteger,SignedInteger};
use crate_::std_types::range::{RangeTrait,fields as range_f};
use crate_::collection_ops::{Map_,MapMt,AllMt};
use crate_::std_ops::{SubTA,SubRevOp,ShlTA,ShlMt,ShrTA,ShrMt,NegOp,NegTA,BitAndOp};
use crate_::field_traits::GetFieldMt;
use crate_::ops::{
    ConstLEOp,ConstLtOp,ConstGEMt,ConstEqMt,ConstLtMt,ConstLEMt,
    Sub1Op,SatSubMt,Sub1,
    Add1,Add1Op,
    AssertThatOp,AssertFnRet,
    If,
    ConstIntoMt,ConstInto_,
    MinMaxOp,
};

use super::RangedIntR;

use num_traits::cast::AsPrimitive;

/// Trait for RangedIntR  .
pub trait RangedTrait {
    type Integer;
    type IntRange;

    /// Whether the range is empty.
    fn is_empty()->bool;
    
    /// The span of the range.None if it covers the entire integer type.
    fn len() -> Option<Self::Integer> ;
    
    /// The start of the range.
    fn start() -> Self::Integer ;
    
    /// The end of the exclusive range.
    /// Returns None if the range covers all of the maximum integer size available.
    fn end() -> Option<Self::Integer> ;
    
    /// Returns whether the value is within the range.
    fn is_in_range(value:&Self::Integer)->bool;
}


mod ranged_vars{
    #[derive(TypeLevel)]
    #[typelevel(reexport(Struct,Traits))]
    #[typelevel(items(runtime_conv(NoImpls)))]
    pub struct RangedVars{
        pub is_empty:(),
        pub opt_end:(),
        pub rlen:(),
    }

    pub use self::type_level_RangedVars::fields as fields_rtv;
}

use self::ranged_vars::*;


type_fn!{
    captures(N,R,IntRange,Start,End)
    fn RangedTraitHelper()
    where[
        IntRange:RangeTrait,
        ConstLEOp:TypeFn_<(End,Start),Output=IsEmpty>,
        End:Piped_< If<ConstLtMt<IntRange::end>,NewSome,NewNone>, Output=OptEnd >,
        OptEnd:Map_<SatSubMt<Start>,Output=RLen>,
        AssertRangeFitsInType: TypeFn_<(N,R)>,
    ]{
        let IsEmpty;let OptEnd;let RLen;

        Construct<RangedVars_Uninit,(
            (fields_rtv::is_empty,IsEmpty),
            (fields_rtv::opt_end,OptEnd),
            (fields_rtv::rlen,RLen),
        )>
    }
}


impl<N,R,Start,End,vars,IntRange>
    RangedTrait  for RangedIntR<N,R>
where
    R:TypeIdentity<Type=ConstRange<Start,End>>,
    RangeFromIntType:TypeFn_<N,Output=IntRange>,
    (Start,End):Piped_<(
        MapMt<ConstTypeOfOp>,
        ValidateConstTypes,
        MapMt<RangedTraitHelper<N,R,IntRange,Start,End>>,
    ),Output=Some_<vars>>,

    vars:RangedVarsTrait,
    vars::is_empty:IntoRuntime<bool>,
    vars::rlen    :IntoRuntime<Option<N>>,
    vars::opt_end :IntoRuntime<Option<N>>,

    Start:IntoRuntime<N>,

    N:Ord,
{
    type Integer=N;
    type IntRange=IntRange;

    #[inline]
    fn is_empty()->bool{
        vars::is_empty::to_runtime()
    }
    #[inline]
    fn len() -> Option<N> {
        vars::rlen::to_runtime()
    }
    #[inline]
    fn start() -> N {
        Start::to_runtime()
    }
    #[inline]
    fn end() -> Option<N> {
        vars::opt_end::to_runtime()
    }
    fn is_in_range(value:&N)->bool{
        Self::start() <= *value &&
        Self::end().map_or(true,|end| *value < end )
    }
}

type_fn!{
    /// Used to check that the ConstTypes of start and end in ConstRange<start,end>  
    /// are the same.
    pub fn 
    ValidateConstTypes(UnsignedInteger,UnsignedInteger){ Some_<()> }
    ValidateConstTypes(SignedInteger  ,SignedInteger  ){ Some_<()> }
}


type_fn!{
    fn AssertRangeFitsInType[Type,R](Type,R)
    where[
        R:RangeTrait,
        RangeFromIntType:TypeFn_<Type,Output=TypeRange>,
        TypeRange:RangeTrait,
        ConstLEOp:TypeFn_<(TypeRange::start,R::start),Output=start_fits_a>,
        ConstLtOp:TypeFn_<(R::start,TypeRange::end),Output=start_fits_b>,
        ConstLEOp:TypeFn_<(TypeRange::start,R::end),Output=end_fits_a>,
        ConstLEOp:TypeFn_<(R::end,TypeRange::end),Output=end_fits_b>,
        
        AssertThatOp:TypeFn_<(
            (
                (start_fits_a,start_fits_b,end_fits_a,end_fits_b),
                R,
            ),
            (GetLhs,AllMt<IdentityFn>),
            Const<ConstRangeOutsideIntegerType>
        )>,
    ]{
        let TypeRange;
        let start_fits_a;
        let start_fits_b;
        let end_fits_a;
        let end_fits_b;
        ()
    }
}



pub struct ConstRangeOutsideIntegerType;


type_fn!{
    pub fn 
        RangeFromIntType(u8)   { ConstRange< U0 , ShlTA<U1,U8> > }
        RangeFromIntType(u16)  { ConstRange< U0 , ShlTA<U1,U16> > }
        RangeFromIntType(u32)  { ConstRange< U0 , ShlTA<U1,U32> > }
        RangeFromIntType(u64)  { ConstRange< U0 , ShlTA<U1,U64> > }
        RangeFromIntType(usize){ ConstRange< U0 , UWordEnd > }

        RangeFromIntType(i8)   { TypeFn<RFITHelper,ShlTA<U1,U7> > }
        RangeFromIntType(i16)  { TypeFn<RFITHelper,ShlTA<U1,U15> > }
        RangeFromIntType(i32)  { TypeFn<RFITHelper,ShlTA<U1,U31> > }
        RangeFromIntType(i64)  { TypeFn<RFITHelper,ShlTA<U1,U63> > }
        RangeFromIntType(isize){ TypeFn<RFITHelper,IWordEnd > }
}


type_fn!{
    fn RFITHelper[N](N)
    where[
        N:ConstInto_<SignedInteger,Output=SN>,
        SN:Neg<Output=NegSN>
    ]{
        let SN;let NegSN;
        ConstRange< NegSN ,SN >
    }
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
type UWordEnd = ShlTA<U1,U8>;
#[cfg(target_pointer_width = "16")]
type UWordEnd = ShlTA<U1,U16>;
#[cfg(target_pointer_width = "32")]
type UWordEnd = ShlTA<U1,U32>;
#[cfg(target_pointer_width = "64")]
type UWordEnd = ShlTA<U1,U64>;
#[cfg(target_pointer_width = "128")]
type UWordEnd = ShlTA<U1,U128>;

type IWordEnd = Piped<UWordEnd,(
    ShrMt<U1>,
    ConstIntoMt<SignedInteger>,
)>;


type_fn!{
    /// The Integer type of a type-level integer,one of u8,u16,u32,u64.
    pub fn IntTypeOf[N](N)
    where[
        ConstTypeOfOp:TypeFn_<N,Output=CType>,
        IntTypeHelper1:TypeFn_<(CType,N),Output=N2>,
        N2    :Shr<U8 ,Output=DivU8 >,
        DivU8 :Shr<U8 ,Output=DivU16>,
        DivU16:Shr<U16,Output=DivU32>,
        DivU32:Shr<U32,Output=DivU64>,

        DivU8 :ConstEq_<U0,Output=IsU16>,
        DivU16:ConstEq_<U0,Output=IsU32>,
        DivU32:ConstEq_<U0,Output=IsU64>,
        DivU64:ConstEq_<U0,Output=IsU128>,
        IntTypeHelper2:TypeFn_<(CType,IsU16,IsU32,IsU64,IsU128),Output=out>,
    ]{
        let CType;let N2;
        let DivU8;let DivU16;let DivU32;let DivU64;
        let IsU16;let IsU32 ;let IsU64; let IsU128;
        let out;
        out
    }
}

// #[cfg(not(rust_1_26))]
type MaxUInt=u64;
type MaxInt=i64;

// Re-enable once typenum does not require nightly to compile with the i128 feature.
// #[cfg(rust_1_26)]
// type MaxUInt=u128;

type_fn!{
    fn 
    IntTypeHelper1[N](UnsignedInteger,N){ N }
    IntTypeHelper1[N](SignedInteger,N)
    where[
        (
            If<ConstLtMt<Z0>,NegOp>,
            ConstIntoMt<UnsignedInteger>,
            ShlMt<U1>,
        ):TypeFn_<N,Output=Out>,
    ]{ let Out;Out }
}

type_fn!{
    fn 
    IntTypeHelper2(UnsignedInteger,True,True,True,True){ u8 }
    IntTypeHelper2(UnsignedInteger,False,True,True,True){ u16 }
    IntTypeHelper2(UnsignedInteger,False,False,True,True){ u32 }
    IntTypeHelper2(UnsignedInteger,False,False,False,True){ u64 }
    IntTypeHelper2(UnsignedInteger,False,False,False,False){ MaxUInt }
    
    IntTypeHelper2(SignedInteger,True,True,True,True){ i8 }
    IntTypeHelper2(SignedInteger,False,True,True,True){ i16 }
    IntTypeHelper2(SignedInteger,False,False,True,True){ i32 }
    IntTypeHelper2(SignedInteger,False,False,False,True){ i64 }
    IntTypeHelper2(SignedInteger,False,False,False,False){ MaxInt }
}





#[cfg(all(test,feature="passed_tests"))]
// #[cfg(test)]
mod test{
    use super::*;


    #[test]
    fn int_type(){
        type Test<IntegerType,End,After=IdentityFn>=
            AssertFnRet<
                End,
                (After,IntTypeOf),
                IntegerType
            >;
        
        let _:Test<u8,U0>;
        let _:Test<u8,ShlTA<U1,U0>>;
        let _:Test<u8,ShlTA<U1,U1>>;
        let _:Test<u8,ShlTA<U1,U2>>;
        let _:Test<u8,ShlTA<U1,U3>>;
        let _:Test<u8,ShlTA<U1,U4>>;
        let _:Test<u8,ShlTA<U1,U5>>;
        let _:Test<u8,ShlTA<U1,U6>>;
        let _:Test<u8,ShlTA<U1,U7>>;
        let _:Test<u8,ShlTA<U1,U8>,Sub1Op>;
        let _:Test<u16,ShlTA<U1,U8>>;
        let _:Test<u16,ShlTA<U1,U9>>;
        let _:Test<u16,ShlTA<U1,U10>>;
        let _:Test<u16,ShlTA<U1,U13>>;
        let _:Test<u16,ShlTA<U1,U14>>;
        let _:Test<u16,ShlTA<U1,U15>>;
        let _:Test<u16,ShlTA<U1,U16>,Sub1Op>;
        let _:Test<u32,ShlTA<U1,U16>>;
        let _:Test<u32,ShlTA<U1,U17>>;
        let _:Test<u32,ShlTA<U1,U18>>;
        let _:Test<u32,ShlTA<U1,U29>>;
        let _:Test<u32,ShlTA<U1,U30>>;
        let _:Test<u32,ShlTA<U1,U31>>;
        let _:Test<u32,ShlTA<U1,U32>,Sub1Op>;
        let _:Test<u64,ShlTA<U1,U32>>;
        let _:Test<u64,ShlTA<U1,U33>>;
        let _:Test<u64,ShlTA<U1,U34>>;
        let _:Test<u64,ShlTA<U1,U60>>;
        let _:Test<u64,ShlTA<U1,U61>>;
        let _:Test<u64,ShlTA<U1,U62>>;
        let _:Test<u64,ShlTA<U1,U63>>;
        let _:Test<u64,ShlTA<U1,U64>,Sub1Op>;

        type GetAfter<Power,After>=(
            ShlMt<Power>,
            ConstIntoMt<SignedInteger>,
            After,
        );

        type Pow2Signed<Type,Power,After=IdentityFn>=(
            Test<Type,U1,GetAfter<Power,After>>,
            Test<Type,U1,GetAfter<Power,(After,NegOp)>>,
        );

        let _:Pow2Signed<i8,U0>;
        let _:Pow2Signed<i8,U1>;
        let _:Pow2Signed<i8,U2>;
        let _:Pow2Signed<i8,U3>;
        let _:Pow2Signed<i8,U4>;
        let _:Pow2Signed<i8,U5>;
        let _:Pow2Signed<i8,U6>;
        let _:Pow2Signed<i8,U7,Sub1Op>;
        let _:Pow2Signed<i16,U8>;
        let _:Pow2Signed<i16,U9>;
        let _:Pow2Signed<i16,U10>;
        let _:Pow2Signed<i16,U11>;
        let _:Pow2Signed<i16,U12>;
        let _:Pow2Signed<i16,U13>;
        let _:Pow2Signed<i16,U14>;
        let _:Pow2Signed<i16,U15,Sub1Op>;
        let _:Pow2Signed<i32,U16>;
        let _:Pow2Signed<i32,U17>;
        let _:Pow2Signed<i32,U18>;
        let _:Pow2Signed<i32,U29>;
        let _:Pow2Signed<i32,U30>;
        let _:Pow2Signed<i32,U31,Sub1Op>;
        let _:Pow2Signed<i64,U32>;
        let _:Pow2Signed<i64,U33>;
        let _:Pow2Signed<i64,U34>;
        let _:Pow2Signed<i64,U60>;
        let _:Pow2Signed<i64,U61>;
        let _:Pow2Signed<i64,U62>;
        let _:Pow2Signed<i64,U63,Sub1Op>;
        

        
    }

}