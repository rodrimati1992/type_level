/*!
A ranged unsigned integer type which stores the number compressed and 
requires `.value()` to recover the uncompressed number.

*/

use core_extensions::{TryFrom, TryInto};
use crate_::ops::fn_types::ConstLEOp;
use crate_::prelude::*;

use std::cmp;
use std::fmt::Debug;
use std::mem::size_of;
use std::ops::{Add, Range, Shr, Sub};
use std::str::FromStr;

use num_traits::cast::AsPrimitive;

pub type RangedUInt<Start, End> = RangedUIntR<ConstRange<Start, End>>;

/// Ranged unsigned integer type,
/// using a ConstRange to determine the range it is limited to.
///
/// The ConstRange also determines the integer type stored.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, ConstConstructor)]
#[cconstructor(Type = "RangedUIntR", ConstParam = "R")]
pub struct RangedUIntInner<R>
where
    R: WrapperTrait,
    GetConstValue<R>: RangeTypes,
{
    range: PhantomWrapper<R>,
    value: <GetConstValue<R> as RangeTypes>::Compressed,
}

impl<R> RangedUIntR<R>
where
    R: RangeTypes,
{
    pub fn new(n: R::Decompressed) -> Option<Self> {
        let range_ = R::get_runt();
        if range_.start <= n && n < range_.end {
            Some(Self {
                value: (n - range_.start).as_(),
                range: PhantomWrapper::NEW,
            })
        } else {
            None
        }
    }

    pub fn with_range(n: R::Decompressed, _range: R) -> Option<Self> {
        Self::new(n)
    }

    pub fn value(self) -> R::Decompressed {
        self.value.into() + R::get_runt().start
    }

    pub fn range(self) -> Range<R::Decompressed> {
        R::get_runt()
    }
}

impl<R> TryFrom<R::Decompressed> for RangedUIntR<R>
where
    R: RangeTypes,
{
    type Error = UIntOutsideRange<R::Decompressed>;
    fn try_from(value: R::Decompressed) -> Result<Self, Self::Error> {
        let range_ = R::get_runt();
        if range_.start <= value && value < range_.end {
            Ok(Self {
                value: (value - range_.start).as_(),
                range: PhantomWrapper::NEW,
            })
        } else {
            Err(UIntOutsideRange {
                value,
                range: range_,
            })
        }
    }
}

impl<R> FromStr for RangedUIntR<R>
where
    R: RangeTypes,
    R::Decompressed: FromStr,
{
    type Err = RangedUIntParseError<R::Decompressed>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<R::Decompressed>()
            .map_err(|_| RangedUIntParseError::InvalidUInt {
                str_: s.into(),
                range: R::get_runt(),
            })?
            .try_into()
            .map_err(RangedUIntParseError::OutsideRange)
    }
}

#[derive(Debug, Clone)]
pub struct UIntOutsideRange<N> {
    pub range: Range<N>,
    pub value: N,
}

#[derive(Debug, Clone)]
pub enum RangedUIntParseError<N> {
    InvalidUInt { str_: String, range: Range<N> },
    OutsideRange(UIntOutsideRange<N>),
}

/// Trait for ConstRange<Start,End>.
///
/// Used to determine the integer type stored in the RangedUIntR ,
/// and to access the Range<Self::Decompressed> at runtime.
///
pub trait RangeTypes {
    /// The integer type stored in the RangedUIntR.
    type Compressed: Copy
        + 'static
        + Into<Self::Decompressed>
        + Debug
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
        + PartialEq
        + PartialOrd
        + Eq
        + Ord;

    fn get_runt() -> Range<Self::Decompressed>;
}

impl<R, SI, RI> RangeTypes for R
where
    R: IntoRuntime<Range<RI>>,
    IntTypeOfRange: TypeFn_<R, Output = SI>,
    IntTypeOfEnd: TypeFn_<R, Output = RI>,
    SI: Copy + 'static + Into<RI> + Debug + PartialEq + PartialOrd + Eq + Ord,
    RI: Sub<RI, Output = RI>
        + Add<RI, Output = RI>
        + Copy
        + Ord
        + 'static
        + AsPrimitive<SI>
        + Debug
        + PartialEq
        + PartialOrd
        + Eq
        + Ord,
{
    type Compressed = SI;
    type Decompressed = RI;

    fn get_runt() -> Range<Self::Decompressed> {
        R::to_runtime()
    }
}

type_fn!{
    /// The Integer type that would fit the distance between R::start and R::end.
    pub fn IntTypeOfRange[R](R)
    where[
        R     :RangeTrait,
        R::end:Sub<R::start,Output=N>,
        N:ConstEq_<U0,Output=is_0>,
        is_0:Boolean,
        IntTypeOfHelper:TypeFn_<(is_0,N),Output=out>
    ]{
        let N;let out;let is_0;
        out
    }
}

type_fn!{
    /// The Integer type that would fit between 0 and the R::end.
    pub fn IntTypeOfEnd[R](R)
    where[
        R     :RangeTrait,
        R::end:ConstEq_<U0,Output=is_0>,
        is_0:Boolean,
        IntTypeOfHelper:TypeFn_<(is_0,R::end),Output=out>
    ]{
        let out;let is_0;out
    }
}

type_fn!{
    pub fn IntTypeOfHelper(True,U0){u8}
    IntTypeOfHelper[End](False,End)
    where[
        End:Sub<U1,Output=n>,
        IntTypeOf:TypeFn_<n,Output=out>
    ]{
        let n;let out;
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

        DivU8 :ConstEq_<U0,Output=IsU16>,
        DivU16:ConstEq_<U0,Output=IsU32>,
        DivU32:ConstEq_<U0,Output=IsU64>,
        IntTypeHelper:TypeFn_<(IsU16,IsU32,IsU64),Output=out>,
        IsU16:Boolean,
        IsU32:Boolean,
        IsU64:Boolean,
    ]{
        let DivU8;let DivU16;let DivU32;
        let IsU16;let IsU32 ;let IsU64;
        let out;
        out
    }
}

type_fn!{
    pub fn IntTypeHelper(True,True,True){ u8 }
       IntTypeHelper(False,True,True){ u16 }
       IntTypeHelper(False,False,True){ u32 }
       IntTypeHelper(False,False,False){ u64 }
}
