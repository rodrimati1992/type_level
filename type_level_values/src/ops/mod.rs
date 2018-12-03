/*!
Contains many operations on `ConstValue`s.
*/

pub mod as_tlist;
pub mod assertions;
pub mod const_eq;
pub mod const_from;
pub mod const_ord;
pub mod control_flow;
// Uncomment this one the API is certain.
// pub mod const_try;
pub mod integers;
pub mod wrapper_ops;

// pub use self::const_try::{
//     ConstTry,
//     TryIsOk,
//     TryIsOkOp,
//     TryIsErr,
//     TryIsErrOp,
//     ToResult,
//     ToResultOp,
// };
pub use self::as_tlist::{AsTListOp, AsTList_, VariantAsTListOp, VariantAsTList_};
pub use self::assertions::{
    AssertConstTypeMt, AssertConstTypeOp, AssertConstType_, AssertEqConstType, AssertEqConstTypeMt,
    AssertEqConstTypeOp, AssertEqConstType_, AssertEqMt, AssertEqOp, AssertEq_, AssertFuncMt,
    AssertFuncOp, AssertFunc_, AssertPipedRetMt, AssertPipedRetOp, AssertPipedRet_, AssertThatMt,
    AssertThatOp, AssertThat_,
};

pub use self::const_eq::{ConstEqMt, ConstEqOp, ConstEq_, ConstNEMt, ConstNEOp, ConstNE_};
pub use self::const_ord::{
    ConstGEMt, ConstGEOp, ConstGtMt, ConstGtOp, ConstLEMt, ConstLEOp, ConstLtMt, ConstLtOp,
    ConstOrd, ConstOrdOp, ConstOrd_, Max, MaxMt, MaxOp, Max_, Min, MinMax, MinMaxMt, MinMaxOp,
    MinMax_, MinMt, MinOp, Min_,
};

pub use self::const_from::{ConstFromOp, ConstFrom_, ConstIntoMt, ConstIntoOp, ConstInto_};

pub use self::wrapper_ops::{
    AndThenMt, AndThenOp, AndThen_, IntoInnerOp, IntoInner_, OrElseMt, OrElseOp, OrElse_, UnwrapOp,
    UnwrapOrElseMt, UnwrapOrElseOp, UnwrapOrElse_, UnwrapOrMt, UnwrapOrOp, UnwrapOr_, Unwrap_,
};

pub use self::control_flow::{If, Lazy, Panic};

pub use self::integers::{
    AbsVal, AbsValOp, AbsVal_, Add1, Add1Op, Get0Op, Get1Op, IntegerConsts, IsOneOp, IsZero,
    IsZeroOp, IsZero_, SafeDiv, SafeDivMt, SafeDivOp, SafeDiv_, SafeSub, SafeSubMt, SafeSubOp,
    SafeSub_, SatSub, SatSub1, SatSub1Op, SatSub1_, SatSubMt, SatSubOp, SatSub_, Sub1, Sub1Op,
};

pub(crate) mod type_aliases {
    pub use crate_::ops::as_tlist::{AsTList, VariantAsTList};
    pub use crate_::ops::assertions::{
        AssertConstType, AssertEq, AssertFunc, AssertPipedRet, AssertThat,
    };

    pub use crate_::ops::const_eq::{ConstEq, ConstNE};
    pub use crate_::ops::const_ord::{ConstGE, ConstGt, ConstLE, ConstLt};

    pub use crate_::ops::const_from::{ConstFrom, ConstInto};

    pub use crate_::ops::wrapper_ops::{
        AndThen, IntoInner, OrElse, Unwrap, UnwrapOr, UnwrapOrElse,
    };

    pub use crate_::ops::control_flow::{If, Lazy};

    pub use crate_::ops::integers::{
        Add1, Get0, Get1, IsOne, IsZero, SafeDiv, SafeSub, SatSub, SatSub1, Sub1,
    };

}

pub use self::type_aliases::*;
