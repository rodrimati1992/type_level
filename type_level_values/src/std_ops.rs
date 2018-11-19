/*!
These define TypeFn_ aliases of standard library traits,type aliases for them,
and reversed versions of non-commutative binary operators.
*/

use prelude::*;

use std_::ops::{
    Add, BitAnd, BitOr, BitXor, Deref,Div,
    Index, IndexMut, Mul, Neg, Not,
    Rem, Shl, Shr, Sub,
};
// use crate_::std_types::cmp_ordering::{Equal_, Greater_, Less_};
use fn_adaptors::*;
// use crate_::collection_ops::*;

macro_rules! declare_rev_and_method_like {
    (  
        operator=$op:ty,

        $(#[$type_meta:meta])*
        type=$type_ident:ident,

        $(#[$rev_meta:meta])*
        rev=$rev_ident:ident,

        $(#[$method_like_meta:meta])*
        rev_method_like=$mt_ident:ident,
    ) => (
        $(#[$type_meta])*
        pub type $type_ident<L,R>=TypeFn<$op,(R,L)>;

        $(#[$rev_meta])*
        pub type $rev_ident=Flip<$op>;

        $(#[$method_like_meta])*
        pub type $mt_ident<L>=ApplyLhs<$op,L>;
    )
}

type_fn!{use_trait 
    trait=Add [Rhs]
    type=AddTA
    fn_type=AddOp
    method_like=AddMt
}

type_fn!{use_trait 
    trait=BitAnd [Rhs]
    type=BitAndTA
    fn_type=BitAndOp
    method_like=BitAndMt
}

type_fn!{use_trait 
    trait=BitXor [Rhs]
    type=BitXorTA
    fn_type=BitXorOp
    method_like=BitXorMt
}

type_fn!{use_trait 
    trait=BitOr [Rhs]
    type=BitOrTA
    fn_type=BitOrOp
    method_like=BitOrMt
}

type_fn!{use_trait 
    trait=Div [Rhs]
    type=DivTA
    fn_type=DivOp
    method_like=DivMt
}

declare_rev_and_method_like!{
    operator=DivOp,

    /// Division with the operands reversed
    type=DivRev,
    /// Division with the operands reversed
    rev=DivRevOp,
    /// Division with the operands reversed
    rev_method_like=DivRevMt,
}

type_fn!{use_trait 
    trait=Index [Rhs]
    type=IndexTA
    fn_type=IndexOp
    method_like=IndexMt
}
type_fn!{use_trait 
    trait=Deref []::Target
    type=DerefTA
    fn_type=DerefOp
    method_like=DerefMt
}

type_fn!{use_trait 
    trait=Mul [Rhs]
    type=MulTA
    fn_type=MulOp
    method_like=MulMt
}

type_fn!{use_trait 
    trait=Neg []
    type=NegTA
    fn_type=NegOp
    method_like=NegMt
}

type_fn!{use_trait 
    trait=Not []
    type=NotTA
    fn_type=NotOp
    method_like=NotMt
}

type_fn!{use_trait 
    trait=Rem [Rhs]
    type=RemTA
    fn_type=RemOp
    method_like=RemMt
}

declare_rev_and_method_like!{
    operator=RemOp,

    /// Remainder with the operands reversed
    type=RemRev,
    /// Remainder with the operands reversed
    rev=RemRevOp,
    /// Remainder with the operands reversed
    rev_method_like=RemRevMt,
}


type_fn!{use_trait 
    trait=Shl [Rhs]
    type=ShlTA
    fn_type=ShlOp
    method_like=ShlMt
}

declare_rev_and_method_like!{
    operator=ShlOp,

    /// Shift left with the operands reversed
    type=ShlRev,
    /// Shift left with the operands reversed
    rev=ShlRevOp,
    /// Shift left with the operands reversed
    rev_method_like=ShlRevMt,
}

type_fn!{use_trait 
    trait=Shr [Rhs]
    type=ShrTA
    fn_type=ShrOp
    method_like=ShrMt
}

declare_rev_and_method_like!{
    operator=ShrOp,

    /// Shift right with the operands reversed
    type=ShrRev,
    /// Shift right with the operands reversed
    rev=ShrRevOp,
    /// Shift right with the operands reversed
    rev_method_like=ShrRevMt,
}


type_fn!{use_trait 
    trait=Sub [Rhs]
    type=SubTA
    fn_type=SubOp
    method_like=SubMt
}

declare_rev_and_method_like!{
    operator=SubOp,

    /// Subtraction with the operands reversed
    type=SubRev,
    /// Subtraction with the operands reversed
    rev=SubRevOp,
    /// Subtraction with the operands reversed
    rev_method_like=SubRevMt,
}

