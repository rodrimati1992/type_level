#[macro_use]
pub mod type_fn;

pub mod as_tlist;
pub mod const_eq;
pub mod const_from;
pub mod const_ord;
pub mod control_flow;
pub mod iteration_ops;
pub mod tuple_impls;
pub mod wrapper_ops;

#[doc(inline)]
pub use self::type_fn::{
    fn_adaptors, fn_types,TypeFn, TypeFn_,
};

#[doc(inline)]
pub use self::as_tlist::{
    AsTList, AsTListOp, AsTList_, VariantAsTList, VariantAsTListOp, VariantAsTList_,
};

pub use self::iteration_ops::{
    ContainsOp,AllOp,AnyOp,All, Any, Contains, 
    Filter, FilterOp, Filter_, FoldL, FoldLOp, FoldL_, FoldR,
    FoldROp, FoldR_, Insert, InsertOp, Insert_, Len, LenOp, Len_, Map, MapOp, Map_, Pop, PopBack,
    PopBackOp, PopBack_, PopOp, Pop_, Push, PushBack, PushBackOp, PushBack_, PushOp, Push_,
    PopFront,PopFrontOp, PopFront_,PushFront, PushFrontOp, PushFront_,
    ReduceL, ReduceLOp, ReduceL_, ReduceR, ReduceROp, ReduceR_, Remove, RemoveOp, Remove_, Repeat,
    RepeatOp, Repeat_, Reverse, ReverseOp, Reverse_,
};

pub use self::const_eq::{ConstEq, ConstEq_,ConstNE_,ConstNE};
pub use self::const_ord::{ConstOrd, ConstOrd_};

pub use self::const_from::{ConstFrom, ConstFrom_};
pub use self::const_from::{ConstInto, ConstInto_};

pub use self::wrapper_ops::{Unwrap, UnwrapOp, Unwrap_};

pub use self::control_flow::{If, IfEager, Lazy, LogicalAnd, LogicalOr};
