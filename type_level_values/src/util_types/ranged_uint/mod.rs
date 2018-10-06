/*!
A ranged unsigned integer type which stores the number compressed and 
requires `.value()` to recover the uncompressed number.

*/

mod constrange_stuff;

#[cfg(test)]
mod tests;

use core_extensions::{TryFrom, TryInto,BoolExt};

use typenum::operator_aliases::Sum;
use num_traits::cast::AsPrimitive;

use crate_::prelude::*;

use std_::cmp::{self,Eq,Ord,PartialEq,PartialOrd};
use std_::mem::size_of;
use std_::ops::{Range};
#[cfg(rust_1_26)]
use std_::ops::RangeInclusive;
use std_::str::FromStr;


pub use self::constrange_stuff::RangeTypes;


pub type RangedUInt<Start, End> = 
    RangedUIntR<ConstRange<Start, End>>;

pub type RangedUIntL<Start,Len>=
    RangedUIntR<ConstRange<Start,Sum<Start,Len>>>;


/**
Ranged unsigned integer type,
using a ConstRange to determine the range it is limited to.

The R ConstRange determines the integer type stored.

# Type aliases

RangedUIntR<R>: where R must be a ConstRange<start,end> (a half-open range).

RangedUInt<Start,End>: Start .. End  is a half-open range.

RangedUIntL<Start,Len>: where Len is how long the range is.


# Storage

This integer type is optimized for smaller storage by representing the start of the range
as 0,and the end of the range as the distance between start and end.

```
# use type_level_values::prelude::*;
use type_level_values::typenum::operator_aliases::{Sum};
use type_level_values::util_types::ranged_uint::{RangedUInt,RangedUIntL};
use std::mem::size_of;

macro_rules! new_ranged{( $ty:ty, $num:expr )=>{
    <$ty>::new($num).expect(concat!("number: ",stringify!($num))).value()
}}

# {
    type IntFrom256=RangedUIntL<U0,U256>;
    assert_eq!(1  ,size_of::<IntFrom256>());
    assert_eq!(0  ,new_ranged!(IntFrom256,0));
    assert_eq!(255,new_ranged!(IntFrom256,255));
    // overflowing literal: assert_eq!(None,new_ranged!(IntFrom256,256));
# }
# {
    type Int1024_256=RangedUIntL<U1024,U256>;
    assert_eq!(1,size_of::<Int1024_256>(),"Int1024_256");
    
    assert_eq!(1024    ,new_ranged!(Int1024_256,1024));
    assert_eq!(1024+255,new_ranged!(Int1024_256,1024+255));
    // overflowing literal: assert_eq!(None,new_ranged!(Int1024_256,1024+256));
# }
# {
    type Int1024_257=RangedUIntL<U1024,U257>;
    assert_eq!(2,size_of::<Int1024_257>(),"Int1024_257");

    assert_eq!(1024    ,new_ranged!(Int1024_257,1024+0  ));
    assert_eq!(1024+255,new_ranged!(Int1024_257,1024+255));
    assert_eq!(1024+256,new_ranged!(Int1024_257,1024+256));
    assert_eq!(None    ,Int1024_257::new(1024+257).ok());
# }


```

*/
#[derive(Debug, Copy, Clone, ConstConstructor)]
#[cconstructor(Type = "RangedUIntR", ConstParam = "R")]
pub struct RangedUIntInner<R>
where
    R: WrapperTrait,
    UnwrapConst<R>: RangeTypes,
{
    range: ConstWrapper<R>,
    value: <UnwrapConst<R> as RangeTypes>::Compressed,
}

impl<R> RangedUIntR<R>
where
    R: RangeTypes,
{
    /// Constructs this ranged integer,inferring the range.
    pub fn new(n: R::Decompressed) -> Result<Self,UIntOutsideRange<R::Decompressed>> {
        if !R::is_empty() && R::start() <= n && n <= R::end_inclusive() {
            Ok(Self {
                value: (n - R::start()).as_(),
                range: ConstWrapper::NEW,
            })
        } else {
            Err(UIntOutsideRange {
                value:n,
                start: R::start(),
                end:R::end(),
                end_inclusive: R::end_inclusive(),
            })
        }
    }

    /// Constructs this ranged integer,passing the ConstRange by value.
    pub fn with_range(
        n: R::Decompressed, 
        _range: R
    ) -> Result<Self,UIntOutsideRange<R::Decompressed>> {
        Self::new(n)
    }

    /// The value of this integer.
    pub fn value(self) -> R::Decompressed {
        self.value.into() + R::start()
    }
    pub fn start()->R::Decompressed{
        R::start()
    }
    pub fn end()->R::Decompressed{
        R::end()
    }
    pub fn end_inclusive()->R::Decompressed{
        R::end_inclusive()
    }
    /// Returns the range of this integer.
    #[cfg(rust_1_26)]
    pub fn range_inclusive(&self)->RangeInclusive<R::Decompressed>{
        R::start()..=R::end_inclusive()
    }
    /// Returns the range of this integer.
    ///
    /// Returns None if the Range covers all of R::Decompressed.
    pub fn range(&self)->Option<Range<R::Decompressed>>{
        if !R::is_empty() && R::end()==R::Decompressed::from(0) {
            None
        }else{
            Some(R::start()..R::end())
        }
    }
}


impl<R1> Eq for RangedUIntR<R1>
where 
    R1:RangeTypes,
    RangedUIntR<R1>:PartialEq,
{}

impl<R1,R2> PartialEq<RangedUIntR<R2>> for RangedUIntR<R1>
where 
    R1:RangeTypes,
    R2:RangeTypes,
    R1::Decompressed:TypeIdentity<Type=R2::Decompressed>
{
    fn eq(&self, other: &RangedUIntR<R2>) -> bool{
        self.value().into_type_val()==other.value()
    }
}


impl<R1,R2> PartialOrd<RangedUIntR<R2>> for RangedUIntR<R1>
where 
    R1:RangeTypes,
    R2:RangeTypes,
    R1::Decompressed:TypeIdentity<Type=R2::Decompressed>
{
    fn partial_cmp(&self, other: &RangedUIntR<R2>) -> Option<cmp::Ordering>{
        self.value().into_type_val().partial_cmp(&other.value())
    }
}

impl<R1> Ord for RangedUIntR<R1>
where
    R1:RangeTypes,
{
    fn cmp(&self, _other: &Self) -> cmp::Ordering{
        cmp::Ordering::Equal
    }
}


impl<R> TryFrom<R::Decompressed> for RangedUIntR<R>
where
    R: RangeTypes,
{
    type Error = UIntOutsideRange<R::Decompressed>;
    fn try_from(value: R::Decompressed) -> Result<Self, Self::Error> {
        Self::new(value)
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
                #[cfg(feature="std")]
                str_: s.into(),
                start: R::start(),
                end_inclusive: R::end_inclusive(),
            })?
            .try_into()
            .map_err(RangedUIntParseError::OutsideRange)
    }
}

#[derive(Debug,Copy,PartialEq, Clone)]
pub struct UIntOutsideRange<N> {
    pub value: N,
    pub start: N,
    pub end_inclusive: N,
    end:N,
}

#[derive(Debug, Clone)]
pub enum RangedUIntParseError<N> {
    InvalidUInt { 
        #[cfg(feature="std")]
        str_: String, 
        start: N,
        end_inclusive: N,
    },
    OutsideRange(UIntOutsideRange<N>),
}

