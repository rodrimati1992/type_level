/*!
Contains many operations on `ConstValue`s.
*/

pub mod as_tlist;
pub mod const_eq;
pub mod const_from;
pub mod const_ord;
pub mod control_flow;
pub mod wrapper_ops;

#[doc(inline)]
pub use self::as_tlist::{
    AsTList, AsTListOp, AsTList_, VariantAsTList, VariantAsTListOp, VariantAsTList_,
};

pub use self::const_eq::{ConstEq, ConstEq_, ConstNE, ConstNE_};
pub use self::const_ord::{ConstOrd, ConstOrd_};

pub use self::const_from::{ConstFrom, ConstFrom_};
pub use self::const_from::{ConstInto, ConstInto_};

pub use self::wrapper_ops::{
    Unwrap, UnwrapOp, Unwrap_,
    UnwrapOr, UnwrapOrOp, UnwrapOr_,
};

pub use self::control_flow::{If, IfEager, Lazy, LogicalAnd, LogicalOr};
