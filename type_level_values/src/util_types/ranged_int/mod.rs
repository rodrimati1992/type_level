
pub mod constrange_stuff;

#[cfg(test)]
// #[cfg(all(test,feature="passed_tests"))]
mod tests;

use core_extensions::{TryFrom, TryInto,BoolExt,OptionExt};

use num_traits::cast::AsPrimitive;

use crate_::prelude::*;
use crate_::std_ops::{AddTA};

use std_::cmp::{self,Eq,Ord,PartialEq,PartialOrd};
use std_::mem::size_of;
use std_::ops::{Range};
use std_::str::FromStr;


pub use self::constrange_stuff::{
    RangedTrait,
};


pub type RangedInt<Integer,Start, End> = 
    RangedIntR<Integer,ConstRange<Start, End>>;

pub type RangedIntL<Integer,Start,Len>=
    RangedIntR<Integer,ConstRange<Start,AddTA<Start,Len>>>;


/**
Ranged unsigned integer type,
using a ConstRange to determine the range it is limited to.

# Type aliases

RangedIntR<IntegerType,R>: where R must be a ConstRange<start,end> (a half-open range).

RangedInt<IntegerType,Start,End>: Start .. End  is a half-open range.

RangedIntL<IntegerType,Start,Len>: where Len is how long the range is.

# Guarantees 

This type guarantees that:

- The range cannot be outside the bounds of the integer type.

- The integer is within the (half-open) range.

-

# Example 

```
# use type_level_values::prelude::*;
use type_level_values::util_types::ranged_int::{RangedInt,RangedIntL};
use std::mem::size_of;

macro_rules! new_ranged{( $ty:ty, $num:expr )=>{
    <$ty>::new($num).expect(concat!("number: ",stringify!($num))).value()
}}

# {
    type IntFrom256=RangedIntL<u8,U0,U256>;
    assert_eq!(1  ,size_of::<IntFrom256>());
    assert_eq!(0  ,new_ranged!(IntFrom256,0));
    assert_eq!(255,new_ranged!(IntFrom256,255));
    // overflowing literal: assert_eq!(None,new_ranged!(IntFrom256,256));
# }
# {
    type Int1024_256=RangedIntL<u16,U1024,U256>;
    assert_eq!(2,size_of::<Int1024_256>(),"Int1024_256");
    
    assert_eq!(1024    ,new_ranged!(Int1024_256,1024));
    assert_eq!(1024+255,new_ranged!(Int1024_256,1024+255));
    assert_eq!(None,Int1024_256::new(1024+256).ok());
# }
# {
    type Int1024_257=RangedIntL<u16,U1024,U257>;
    assert_eq!(2,size_of::<Int1024_257>(),"Int1024_257");

    assert_eq!(1024    ,new_ranged!(Int1024_257,1024+0  ));
    assert_eq!(1024+255,new_ranged!(Int1024_257,1024+255));
    assert_eq!(1024+256,new_ranged!(Int1024_257,1024+256));
    assert_eq!(None    ,Int1024_257::new(1024+257).ok());
# }


```

*/
#[derive(Debug, Copy, Clone, ConstConstructor)]
#[cconstructor(Type = "RangedIntR", ConstParam = "R")]
pub struct RangedIntInner<N,R>
where
    R: WrapperTrait,
{
    range: ConstWrapper<R>,
    value: N,
}

impl<N,R> RangedIntR<N,R>{
    /// Constructs this ranged integer,inferring the range.
    pub fn new(n: N) -> Result<Self,UIntOutsideRange<N>> 
    where
        Self: RangedTrait<Integer=N>,
    {
        if Self::is_in_range(&n) {
            Ok(Self {
                value: n,
                range: ConstWrapper::NEW,
            })
        } else {
            let start=Self::start();
            Err(UIntOutsideRange {
                value:n,
                len:Self::len(),
                start: Self::start(),
                end:Self::end(),
            })
        }
    }

    /// Constructs this ranged integer,passing the ConstRange by value.
    pub fn with_range(n: N, _range: R) -> Result<Self,UIntOutsideRange<N>> 
    where
        Self: RangedTrait<Integer=N>,
    {
        Self::new(n)
    }

    /// Converts self to cover the entire integer type's range.
    pub fn into_full_range(self)->RangedIntR<N,<Self as RangedTrait>::IntRange>
    where
        Self: RangedTrait<Integer=N>,
        RangedIntR<N,<Self as RangedTrait>::IntRange>: RangedTrait<Integer=N>,
    {
        RangedIntR::new(self.into_inner()).ok().expect("\
            bug: RangedIntR::into_full_range should never \
                fail to broaden the range to cover the entire integer \
        ")
    }
}

impl<N,R> RangedIntR<N,R>{

    /// The value of this integer.
    pub fn into_inner(self) -> N {
        self.value
    }
    /// The value of this integer.
    pub fn value(self) -> N {
        self.value
    }
    /// The value of this integer.
    pub fn inner(&self) -> &N {
        &self.value
    }
}

impl<N,R> Eq for RangedIntR<N,R>
where N:Eq
{}

impl<N,R1,R2> PartialEq<RangedIntR<N,R2>> for RangedIntR<N,R1>
where
    N:PartialEq,
{
    fn eq(&self, other: &RangedIntR<N,R2>) -> bool{
        self.inner()==other.inner()
    }
}


impl<N,R1,R2> PartialOrd<RangedIntR<N,R2>> for RangedIntR<N,R1>
where
    N:PartialOrd,
{
    fn partial_cmp(&self, other: &RangedIntR<N,R2>) -> Option<cmp::Ordering>{
        self.inner().partial_cmp(other.inner())
    }
}

impl<N,R> Ord for RangedIntR<N,R>
where N:Ord,
{
    fn cmp(&self, _other: &Self) -> cmp::Ordering{
        cmp::Ordering::Equal
    }
}


impl<N,R> TryFrom<N> for RangedIntR<N,R>
where
    Self: RangedTrait<Integer=N>,
{
    type Error = UIntOutsideRange<N>;
    fn try_from(value: N) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<N,R> FromStr for RangedIntR<N,R>
where
    Self: RangedTrait<Integer=N>,
    N: FromStr,
{
    type Err = RangedIntParseError<N>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<N>()
            .map_err(|_| RangedIntParseError::InvalidUInt {
                #[cfg(feature="std")]
                str_: s.into(),
                start: Self::start(),
            })?
            .try_into()
            .map_err(RangedIntParseError::OutsideRange)
    }
}

#[derive(Debug,Copy, Clone,PartialEq)]
pub struct UIntOutsideRange<N> {
    pub value: N,
    pub start: N,
    pub len:Option<N>,
    pub end:Option<N>,
}


#[derive(Debug, Clone)]
pub enum RangedIntParseError<N> {
    InvalidUInt { 
        #[cfg(feature="std")]
        str_: String, 
        start: N,
    },
    OutsideRange(UIntOutsideRange<N>),
}




fn what(){
    let _=RangedInt::<i8,Z0,N10>::new(0);
}