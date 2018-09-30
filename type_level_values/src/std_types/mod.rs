pub mod cmp_ordering;
pub mod option;
pub mod phantomdata;
pub mod result;
pub mod tuples;

mod range_types;

pub use self::cmp_ordering::{Equal_, Greater_, Less_, OrderingTrait, OrderingType};

pub use self::range_types::{range, range_from, range_full, range_to};
#[cfg(rust_1_26)]
pub use self::range_types::{range_inclusive, range_to_inclusive};

pub use self::option::{None_, OptionTrait, OptionType, Some_};

pub use self::result::{Err_, Ok_, ResultTrait, ResultType};

pub use self::phantomdata::PhantomDataType;
