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
pub use self::assertions::{
    AssertEq, AssertEqOp, AssertEq_,
    AssertFunc_,AssertFuncOp,AssertFunc ,AssertFuncMt,
    AssertFnRet_,AssertFnRetOp,AssertFnRet,AssertFnRetMt
};
pub use self::as_tlist::{
    AsTList, AsTListOp, AsTList_, VariantAsTList, VariantAsTListOp, VariantAsTList_,
};

pub use self::const_eq::{
    ConstEq, ConstEq_,ConstEqOp, 
    ConstNE, ConstNE_,ConstNEOp,
};
pub use self::const_ord::{
    ConstOrd, ConstOrd_,
    ConstLt  ,ConstLE  ,ConstGt  ,ConstGE  ,
    ConstLtOp,ConstLEOp,ConstGtOp,ConstGEOp,

};

pub use self::const_from::{
    ConstFrom, ConstFrom_, ConstFromOp,ConstFromMt,
    ConstInto, ConstInto_, ConstIntoOp,ConstIntoMt,
};

pub use self::wrapper_ops::{
    Unwrap, UnwrapOp, Unwrap_,
    UnwrapOr, UnwrapOrOp, UnwrapOr_,
    IntoInner_,IntoInner,IntoInnerOp,
};

pub use self::control_flow::{If, IfEager, Lazy, LogicalAnd, LogicalOr};

pub use self::integers::{
    IntegerConsts,
    IsOne,IsOneOp,
    IsZero,IsZeroOp,
    Get0,Get0Op,
    Get1,Get1Op,
    SafeDiv,SafeDivOp,
    SafeSub,SafeSubOp,
    Add1Op,Add1,
    Sub1Op,Sub1,
    SatSub1,SatSub1Op,
    SatSub,SatSubOp,
};
