
pub mod constrange_stuff;

#[cfg(all(test,feature="passed_tests"))]
mod tests;

use core_extensions::{TryFrom, TryInto,BoolExt,OptionExt};

use num_traits::cast::AsPrimitive;

use crate_::prelude::*;
use crate_::std_ops::{AddTA};

use std_::cmp::{self,Eq,Ord,PartialEq,PartialOrd};
use std_::mem::size_of;
use std_::ops::{Range};
#[cfg(rust_1_27)]
use std_::ops::RangeInclusive;
use std_::str::FromStr;


pub use self::constrange_stuff::{
    RangedTrait,
    Compressed,
    Decompressed,
};


pub type RangedUInt<Start, End> = 
    RangedUIntR<ConstRange<Start, End>>;

pub type RangedUIntL<Start,Len>=
    RangedUIntR<ConstRange<Start,AddTA<Start,Len>>>;


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
pub struct RangedUIntInner<N,R>
where
    R: WrapperTrait,
{
    range: ConstWrapper<R>,
    value: N,
}

impl<N,R> RangedUIntR<N,R>
where
    Self: RangedTrait<N>,
{
    /// Constructs this ranged integer,inferring the range.
    pub fn new(n: N) -> Result<Self,UIntOutsideRange<N>> {
        if !Self::is_empty() && Self::start() <= n && n <= Self::end_inclusive() {
            Ok(Self {
                value: n,
                range: ConstWrapper::NEW,
            })
        } else {
            let start=Self::start();
            Err(UIntOutsideRange {
                len:Self::end().map(|end| if end < start { 0.into() }else{end-start} ),
                value:n,
                start: Self::start(),
                end:Self::end(),
                end_inclusive: Self::end_inclusive(),
            })
        }
    }

    /// Constructs this ranged integer,passing the ConstRange by value.
    pub fn with_range(n: N, _range: R) -> Result<Self,UIntOutsideRange<N>> {
        Self::new(n)
    }

    /// The value of this integer.
    pub fn value(self) -> N {
        self.value.into()
    }
    pub fn start()->N{
        Self::start_()
    }
    pub fn end()->Option<N>{
        Self::end_()
    }
    pub fn end_inclusive()->N{
        Self::end_inclusive_()
    }
    /// Returns the range of this integer.
    #[cfg(rust_1_27)]
    pub fn range_inclusive(&self)->RangeInclusive<N>{
        RangeInclusive::new(
            Self::start(),
            Self::end_inclusive()
        )
    }
    /// Returns the range of this integer.
    ///
    /// Returns None if the range covers all of the maximum integer size available.
    pub fn range(&self)->Option<Range<N>>{
        Self::end().map(|end| Self::start()..end )
    }
}


impl<N,R> Eq for RangedUIntR<N,R>
where N:Eq
{}

impl<N,R1,R2> PartialEq<RangedUIntR<N,R2>> for RangedUIntR<N,R1>
where
    N:PartialEq,
{
    fn eq(&self, other: &RangedUIntR<N,R2>) -> bool{
        self.value()==other.value()
    }
}


impl<N,R1,R2> PartialOrd<RangedUIntR<N,R2>> for RangedUIntR<N,R1>
where
    N:PartialOrd,
{
    fn partial_cmp(&self, other: &RangedUIntR<N,R2>) -> Option<cmp::Ordering>{
        self.value() < other.value()
    }
}

impl<N,R> Ord for RangedUIntR<N,R>
where N:Ord,
{
    fn cmp(&self, _other: &Self) -> cmp::Ordering{
        cmp::Ordering::Equal
    }
}


impl<N,R> TryFrom<N> for RangedUIntR<N,R>
where
    Self: RangedTrait<N>,
{
    type Error = UIntOutsideRange<N>;
    fn try_from(value: N) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<N,R> FromStr for RangedUIntR<N,R>
where
    Self: RangedTrait<N>,
    N: FromStr,
{
    type Err = RangedUIntParseError<N>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<N>()
            .map_err(|_| RangedUIntParseError::InvalidUInt {
                #[cfg(feature="std")]
                str_: s.into(),
                start: Self::start(),
                end_inclusive: Self::end_inclusive(),
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
    pub len:Option<N>,
    end:Option<N>,
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

