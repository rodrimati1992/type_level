use crate_::ops::control_flow::If;
use crate_::fn_adaptors::*;
use crate_::fn_types::{AddOp, ConstEqOp, ConstLEOp, ConstNEOp, SubOp};
use crate_::ops::{ConstEq};
use crate_::collection_ops::{FoldL, InsertOp};
use crate_::std_types::cmp_ordering::{Equal_, Greater_, Less_, OrderingTrait};
use prelude::*;

use core_extensions::type_level_bool::{Boolean, False, True};
use core_extensions::TypeIdentity;

use typenum::consts::{
    U0, U1, U10, U11, U12, U13, U14, U15, U16, U2, U21, U24, U28, U3, U32, U4, U48, U480, U5, U6,
    U64, U7, U8, U9,
};

use std_::ops::{
    Add, Range as StdRange, RangeFrom as StdRangeFrom, RangeFull as StdRangeFull,
    RangeTo as StdRangeTo, Sub,
};
#[cfg(rust_1_26)]
use std_::ops::{RangeInclusive as StdRangeInclusive, RangeToInclusive as StdRangeToInclusive};

pub mod range {
    use super::*;
    #[derive(TypeLevel)]
    #[typelevel(
        // print_derive,
        // skip_derive,
        reexport = "pub",
        derive(ConstEq, ConstOrd),
        items(runtime_conv(Internal="StdRange")),
    )]
    #[allow(dead_code)]
    #[doc(hidden)]
    pub struct Range<T> {
        pub start: T,
        pub end: T,
    }
}


//////////////////////////////////////////////////////////////////////////////////

pub mod range_from {
    use super::*;

    #[derive(TypeLevel)]
    #[typelevel(
        // print_derive,
        // skip_derive,
        reexport = "pub",
        derive(ConstEq, ConstOrd),
        items(IntoRuntime(Internal (Type="StdRangeFrom",Manual))),
        items(IntoConstType(Internal = "StdRangeFrom")),
    )]
    #[allow(dead_code)]
    #[doc(hidden)]
    pub struct RangeFrom<Start> {
        pub start: Start,
    }

    impl<St, T> IntoRuntime<StdRangeFrom<T>> for ConstRangeFrom<St>
    where
        St: IntoRuntime<T>,
    {
        fn to_runtime() -> StdRangeFrom<T> {
            St::to_runtime()..
        }
    }

    #[cfg(rust_1_22)]
    impl<St, T> IntoConstant<StdRangeFrom<T>> for ConstRangeFrom<St>
    where
        St: IntoConstant<T>,
    {
        const VALUE: StdRangeFrom<T> = St::VALUE..;
    }

}

/////////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////////

pub mod range_full {
    use super::*;

    #[derive(TypeLevel)]
    #[typelevel(
        reexport = "pub",
        derive(ConstEq, ConstOrd),
        items(IntoRuntime(Internal(Type = "StdRangeFull", Manual))),
        items(IntoConstType(Internal = "StdRangeFull")),
    )]
    #[allow(dead_code)]
    #[doc(hidden)]
    pub struct RangeFull;

    impl IntoRuntime<StdRangeFull> for ConstRangeFull {
        fn to_runtime() -> StdRangeFull {
            ..
        }
    }

    #[cfg(rust_1_22)]
    impl IntoConstant<StdRangeFull> for ConstRangeFull {
        const VALUE: StdRangeFull = ..;
    }

}

pub mod range_to {
    use super::*;

    #[derive(TypeLevel)]
    #[typelevel(
        reexport = "pub",
        derive(ConstEq, ConstOrd),
        items(IntoRuntime(Internal(Type = "StdRangeTo", Manual))),
        items(IntoConstType(Internal = "StdRangeTo")),
    )]
    #[allow(dead_code)]
    #[doc(hidden)]
    pub struct RangeTo<End> {
        pub end: End,
    }

    impl<End, T> IntoRuntime<StdRangeTo<T>> for ConstRangeTo<End>
    where
        End: IntoRuntime<T>,
    {
        fn to_runtime() -> StdRangeTo<T> {
            ..End::to_runtime()
        }
    }

    #[cfg(rust_1_22)]
    impl<End, T> IntoConstant<StdRangeTo<T>> for ConstRangeTo<End>
    where
        End: IntoConstant<T>,
    {
        const VALUE: StdRangeTo<T> = ..End::VALUE;
    }

}

#[cfg(rust_1_26)]
pub mod range_inclusive {
    use super::*;

    #[derive(TypeLevel)]
    #[typelevel(
        reexport = "pub",
        derive(ConstEq, ConstOrd),
        items(IntoRuntime(Internal(Type = "StdRangeInclusive", Manual))),
        items(IntoConstType(Internal = "StdRangeInclusive")),
    )]
    #[allow(dead_code)]
    #[doc(hidden)]
    pub struct RangeInclusive<T> {
        pub start: T,
        pub end: T,
    }

    #[cfg(rust_1_27)]
    impl<St, End, T> IntoRuntime<StdRangeInclusive<T>> for ConstRangeInclusive<St, End>
    where
        St: IntoRuntime<T>,
        End: IntoRuntime<T>,
    {
        fn to_runtime() -> StdRangeInclusive<T> {
            StdRangeInclusive::new(St::to_runtime(), End::to_runtime())
        }
    }

    #[cfg(rust_1_27)]
    impl<St, End, T> IntoConstant<StdRangeInclusive<T>> for ConstRangeInclusive<St, End>
    where
        St: IntoConstant<T>,
        End: IntoConstant<T>,
    {
        const VALUE: StdRangeInclusive<T> = StdRangeInclusive::new(St::VALUE, End::VALUE);
    }

}

#[cfg(rust_1_26)]
pub mod range_to_inclusive {
    use super::*;

    #[derive(TypeLevel)]
    #[typelevel(
        reexport = "pub",
        derive(ConstEq, ConstOrd),
        items(IntoRuntime(Internal(Type = "StdRangeToInclusive", Manual))),
        items(IntoConstType(Internal = "StdRangeToInclusive")),
    )]
    #[allow(dead_code)]
    #[doc(hidden)]
    pub struct RangeToInclusive<End> {
        pub end: End,
    }

    #[cfg(rust_1_27)]
    impl<End, T> IntoRuntime<StdRangeToInclusive<T>> for ConstRangeToInclusive<End>
    where
        End: IntoRuntime<T>,
    {
        fn to_runtime() -> StdRangeToInclusive<T> {
            StdRangeToInclusive {
                end: End::to_runtime(),
            }
        }
    }

    #[cfg(rust_1_27)]
    impl<End, T> IntoConstant<StdRangeToInclusive<T>> for ConstRangeToInclusive<End>
    where
        End: IntoConstant<T>,
    {
        const VALUE: StdRangeToInclusive<T> = StdRangeToInclusive { end: End::VALUE };
    }

}

//////////////////////////////////////////////////////////////////////////////////
////////////                Constructors
//////////////////////////////////////////////////////////////////////////////////


#[allow(unused_macros)]
macro_rules! rangei_ {
    (value,$($rest:tt)*)=>{ <rangei_!($($rest)*) as $crate::RuntimeValue>::MTVAL };
    ($start:ty => $end:ty) =>($crate::std_types::range_inclusive::ConstRangeInclusive<$start,$end>);
    (          => $end:ty) =>($crate::std_types::range_to_inclusive::ConstRangeToInclusive<$end> );
    ($start:ty =>        ) =>($crate::std_types::range_from::ConstRangeFrom<$start> );
    (                    ) =>($crate::std_types::range_full::ConstRangeFull );
}

#[allow(unused_macros)]
macro_rules! range_ {
    (value,$($rest:tt)*)=>{ <range_!($($rest)*) as $crate::RuntimeValue>::MTVAL };
    ($start:ty => $end:ty) => ( $crate::std_types::range::ConstRange<$start,$end> );
    (          => $end:ty) => ( $crate::std_types::range_to::ConstRangeTo<$end> );
    ($start:ty =>        ) => ( $crate::std_types::range_from::ConstRangeFrom<$start> );
    (                    ) => ( $crate::std_types::range_full::ConstRangeFull );
}

#[cfg(all(test,feature="passed_tests"))]
mod test_eq {
    use super::*;

    #[test]
    #[cfg(rust_1_26)]
    fn test_eq_range() {
        let _: True = ConstEq::<range_!(U0=>U0), range_!(U0=>U0)>::MTVAL;
        let _: True = ConstEq::<rangei_!(U0=>U0), rangei_!(U0=>U0)>::MTVAL;
        let _: False = ConstEq::<range_!(U0=>U1), range_!(U0=>U0)>::MTVAL;
        let _: False = ConstEq::<rangei_!(U0=>U1), rangei_!(U0=>U0)>::MTVAL;

        let _: True = ConstEq::<range_!(=>U0), range_!(=>U0)>::MTVAL;
        let _: True = ConstEq::<rangei_!(=>U0), rangei_!(=>U0)>::MTVAL;
        let _: False = ConstEq::<range_!(=>U1), range_!(=>U0)>::MTVAL;
        let _: False = ConstEq::<rangei_!(=>U1), rangei_!(=>U0)>::MTVAL;

        let _: True = ConstEq::<range_!(U0=>), range_!(U0=>)>::MTVAL;
        let _: True = ConstEq::<rangei_!(U0=>), rangei_!(U0=>)>::MTVAL;
        let _: False = ConstEq::<range_!(U1=>), range_!(U0=>)>::MTVAL;
        let _: False = ConstEq::<rangei_!(U1=>), rangei_!(U0=>)>::MTVAL;

        let _: True = ConstEq::<range_!(), range_!()>::MTVAL;
        let _: True = ConstEq::<rangei_!(), rangei_!()>::MTVAL;
    }
}
