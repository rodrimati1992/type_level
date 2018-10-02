//! This example shows off a ranged unsigned integer type which is optimized for
//! smaller storage.
//!
//! The value of Start and End determines the integer types used,
//! there is no need to specify any integer types.
//!
//! ie: a RangedUInt<U100,U356> would be stored as a u8 and
//! returned as a u16 (between 100 and 356 exclussive) from the `value` method .
//!
//!

#[macro_use]
extern crate type_level_values;

#[macro_use]
extern crate derive_type_level;
extern crate num_traits;
extern crate take_mut;

use type_level_values::prelude::*;


use num_traits::cast::AsPrimitive;

use std::fmt::Debug;
use std::mem::size_of;
use std::ops::{Add, Range, Shr, Sub};

fn main() {
    type U65535 = <U65536 as Sub<U1>>::Output;

    assert_eq!(size_of::<u8>(), size_of::<RangedUInt<U0, U0>>());
    assert_eq!(size_of::<u8>(), size_of::<RangedUInt<U0, U256>>());
    assert_eq!(size_of::<u8>(), size_of::<RangedUInt<U100, U356>>());
    assert_eq!(size_of::<u16>(), size_of::<RangedUInt<U0, U65535>>());

    {
        type UsedRange = ConstRange<U0, U10>;
        let range = ConstRange {
            start: U0::PW,
            end: U10::PW,
        };
        let ranged_int = |n| RangedUIntR::with_range(n, range).unwrap().value();

        assert_eq!(ranged_int(0), 0);
        assert_eq!(ranged_int(5), 5);
        assert_eq!(ranged_int(9), 9);
        assert_eq!(RangedUIntR::new(10), None::<RangedUIntR<UsedRange>>);
    }

    {
        type UsedRange = ConstRange<U0, U100>;
        let range = ConstRange {
            start: U0::PW,
            end: U100::PW,
        };
        let ranged_int = |n| RangedUIntR::with_range(n, range).unwrap().value();

        assert_eq!(ranged_int(0), 0);
        assert_eq!(ranged_int(5), 5);
        assert_eq!(ranged_int(9), 9);
        assert_eq!(ranged_int(50), 50);
        assert_eq!(ranged_int(99), 99);
        assert_eq!(RangedUIntR::new(100), None::<RangedUIntR<UsedRange>>);
    }

    {
        type UsedRange = ConstRange<U10, U100>;
        let range = UsedRange::MTVAL;
        let ranged_int = |n| RangedUIntR::with_range(n, range).unwrap().value();

        assert_eq!(RangedUIntR::new(0), None::<RangedUIntR<UsedRange>>);
        assert_eq!(RangedUIntR::new(5), None::<RangedUIntR<UsedRange>>);
        assert_eq!(RangedUIntR::new(9), None::<RangedUIntR<UsedRange>>);
        assert_eq!(ranged_int(10), 10);
        assert_eq!(ranged_int(50), 50);
        assert_eq!(ranged_int(99), 99);
        assert_eq!(RangedUIntR::new(100), None::<RangedUIntR<UsedRange>>);
    }
}

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
    n: <GetConstValue<R> as RangeTypes>::Stored,
}

impl<R> RangedUIntR<R>
where
    R: RangeTypes,
{
    fn new(n: R::Returned) -> Option<Self> {
        let range_ = R::get_runt();
        if range_.start <= n && n < range_.end {
            Some(Self {
                n: (n - range_.start).as_(),
                range: PhantomWrapper::NEW,
            })
        } else {
            None
        }
    }

    fn with_range(n: R::Returned, _range: R) -> Option<Self> {
        Self::new(n)
    }

    fn value(self) -> R::Returned {
        self.n.into() + R::get_runt().start
    }

    #[allow(dead_code)]
    fn range(self) -> Range<R::Returned> {
        R::get_runt()
    }
}

/// Trait for ConstRange<Start,End>.
///
/// Used to determine the integer type stored in the RangedUIntR.
pub trait RangeTypes {
    /// The integer type stored in the RangedUIntR.
    type Stored: Copy + 'static + Into<Self::Returned> + Debug + PartialEq + PartialOrd + Eq + Ord;

    /// The integer taken by RangedUIntR::new and returned by RangedUIntR::value.
    type Returned: Sub<Self::Returned, Output = Self::Returned>
        + Add<Self::Returned, Output = Self::Returned>
        + Copy
        + 'static
        + AsPrimitive<Self::Stored>
        + Debug
        + PartialEq
        + PartialOrd
        + Eq
        + Ord;

    fn get_runt() -> Range<Self::Returned>;
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
    type Stored = SI;
    type Returned = RI;

    fn get_runt() -> Range<Self::Returned> {
        R::to_runtime()
    }
}

type_fn!{
    /// The Integer type of the distance between the ends of the range.
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
    /// The Integer type of the distance between the ends of the range.
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
    #[doc(hidden)]
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
    #[doc(hidden)]
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
    #[doc(hidden)]
    pub fn IntTypeHelper(True,True,True){ u8 }
       IntTypeHelper(False,True,True){ u16 }
       IntTypeHelper(False,False,True){ u32 }
       IntTypeHelper(False,False,False){ u64 }
}
