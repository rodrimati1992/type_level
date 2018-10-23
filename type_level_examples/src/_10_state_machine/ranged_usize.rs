use type_level_values::core_extensions::{TryFrom, TryInto};
use type_level_values::ops::ConstLEOp;
use type_level_values::prelude::*;


use std::cmp;
use std::ops::Range;
use std::str::FromStr;

pub type RangedUsize<Start, End> = RangedUsizeR<ConstRange<Start, End>>;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, ConstConstructor)]
#[cconstructor(Type = "RangedUsizeR", ConstParam = "R")]
pub struct RangedUsizeInner<R> {
    value: usize,
    range: ConstWrapper<R>,
}

pub trait RangedUsizeBounds {
    fn get_range() -> Range<usize>;
}

impl<R> RangedUsizeBounds for RangedUsizeR<R>
where
    R: RangeTrait,
    R: IntoRuntime<Range<usize>>,
    ConstLEOp: TypeFn_<(R::start, R::end), Output = True>,
{
    fn get_range() -> Range<usize> {
        R::to_runtime()
    }
}

impl<R> RangedUsizeR<R>
where
    Self: RangedUsizeBounds,
{
    pub fn new(value: usize) -> Result<Self, UsizeOutsideRange> {
        Self::try_from(value)
    }

    pub fn with_range(value: usize, _range: R) -> Result<Self, UsizeOutsideRange> {
        Self::try_from(value)
    }
    pub fn saturating_from(value: usize) -> Self {
        let range_ = Self::get_range();
        let value = cmp::max(range_.start, value);
        let value = cmp::min(range_.end - 1, value);

        Self {
            value,
            range: ConstWrapper::NEW,
        }
    }
}

impl<R> RangedUsizeR<R> {
    pub fn value(self) -> usize {
        self.value
    }
}

impl<R> TryFrom<usize> for RangedUsizeR<R>
where
    Self: RangedUsizeBounds,
{
    type Error = UsizeOutsideRange;
    fn try_from(value: usize) -> Result<Self, UsizeOutsideRange> {
        let range_ = Self::get_range();
        if range_.start <= value && value < range_.end {
            Ok(Self {
                value,
                range: ConstWrapper::NEW,
            })
        } else {
            Err(UsizeOutsideRange {
                value,
                range: range_,
            })
        }
    }
}

impl<R> FromStr for RangedUsizeR<R>
where
    Self: RangedUsizeBounds,
{
    type Err = RangedUsizeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<usize>()
            .map_err(|_| RangedUsizeParseError::InvalidUsize(s.into()))?
            .try_into()
            .map_err(RangedUsizeParseError::OutsideRange)
    }
}

#[derive(Debug, Clone)]
pub struct UsizeOutsideRange {
    pub range: Range<usize>,
    pub value: usize,
}

#[derive(Debug, Clone)]
pub enum RangedUsizeParseError {
    InvalidUsize(String),
    OutsideRange(UsizeOutsideRange),
}
