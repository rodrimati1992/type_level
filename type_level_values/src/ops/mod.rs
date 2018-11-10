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
    AssertEq_,AssertEqOp, AssertEqMt, 
    AssertEqConstType_,AssertEqConstType,AssertEqConstTypeOp,AssertEqConstTypeMt,
    AssertFunc_,AssertFuncOp,AssertFuncMt,
    AssertThat_,AssertThatOp,AssertThatMt,
    AssertFnRet_,AssertFnRetOp,AssertFnRetMt,
    AssertConstType_,AssertConstTypeOp,AssertConstTypeMt,
};
pub use self::as_tlist::{
    AsTListOp, AsTList_, 
    VariantAsTListOp, VariantAsTList_,
};

pub use self::const_eq::{
    ConstEq_,ConstEqOp,ConstEqMt, 
    ConstNE_,ConstNEOp,ConstNEMt,
};
pub use self::const_ord::{
    ConstOrd, ConstOrd_,ConstOrdOp,
    ConstLtOp,ConstLEOp,ConstGtOp,ConstGEOp,
    ConstLtMt,ConstLEMt,ConstGtMt,ConstGEMt,

    Min,Min_,MinOp,MinMt,
    Max,Max_,MaxOp,MaxMt,
    MinMax,MinMax_,MinMaxOp,MinMaxMt,
};

pub use self::const_from::{
    ConstFrom_, ConstFromOp,
    ConstInto_, ConstIntoOp,ConstIntoMt,
};

pub use self::wrapper_ops::{
    UnwrapOp, Unwrap_,
    UnwrapOrOp, UnwrapOr_,UnwrapOrMt,
    UnwrapOrElseOp, UnwrapOrElse_,UnwrapOrElseMt,
    IntoInnerOp,IntoInner_,
    AndThenOp, AndThen_,AndThenMt,
    OrElseOp, OrElse_,OrElseMt,
};

pub use self::control_flow::{If,  Lazy,Panic};

pub use self::integers::{
    IntegerConsts,
    IsOneOp,
    Get0Op,Get1Op,
    Add1,Add1Op,
    Sub1,Sub1Op,
    AbsVal_ ,AbsVal ,AbsValOp ,
    SatSub_ ,SatSub ,SatSubOp ,SatSubMt ,
    SafeDiv_,SafeDiv,SafeDivOp,SafeDivMt,
    SafeSub_,SafeSub,SafeSubOp,SafeSubMt,
    SatSub1_,SatSub1,SatSub1Op,
    IsZero_ ,IsZero ,IsZeroOp ,
};


pub(crate) mod type_aliases{
    pub use crate_::ops::assertions::{
        AssertEq,AssertFunc,AssertThat,AssertFnRet,AssertConstType,
    };
    pub use crate_::ops::as_tlist::{
        AsTList,VariantAsTList,
    };

    pub use crate_::ops::const_eq::{
        ConstEq,ConstNE,
    };
    pub use crate_::ops::const_ord::{
        ConstLt  ,ConstLE  ,ConstGt  ,ConstGE  ,
    };

    pub use crate_::ops::const_from::{
        ConstFrom,ConstInto,
    };

    pub use crate_::ops::wrapper_ops::{
        Unwrap,UnwrapOr,UnwrapOrElse,IntoInner,AndThen,OrElse,
    };

    pub use crate_::ops::control_flow::{If,  Lazy};

    pub use crate_::ops::integers::{
        IsOne,IsZero,Get0,Get1,SafeDiv,SafeSub,Add1,Sub1,SatSub1,SatSub,
    };

}

pub use self::type_aliases::*;