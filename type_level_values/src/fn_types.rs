/*!
These are TypeFn_ aliases of pre-existing trait.
*/

use prelude::*;

use std_::ops::{
    Add as Add, BitAnd as BitAnd, BitOr as BitOr, BitXor as BitXor, Div as Div,
    Index as Index, IndexMut as IndexMut, Mul as Mul, Neg as Neg, Not as Not,
    Rem as Rem, Shl as Shl, Shr as Shr, Sub as Sub,
};
use crate_::std_types::cmp_ordering::{Equal_, Greater_, Less_};
use crate_::ops::{ConstEq_, ConstNE_};


use fn_adaptors::*;
use crate_::collection_ops::*;

macro_rules! declare_rev_and_method_like {
    (  
        operator=$op:ty,

        $(#[$rev_meta:meta])*
        rev=$rev_ident:ident,

        $(#[$method_like_meta:meta])*
        rev_method_like=$mt_ident:ident,
    ) => (
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
    rev=DivRevOp,
    rev_method_like=DivRevMt,
}

type_fn!{use_trait 
    trait=Index [Rhs]
    type=IndexTA
    fn_type=IndexOp
    method_like=IndexMt
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
    rev=RemRevOp,
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
    rev=ShlRevOp,
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
    rev=ShrRevOp,
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
    rev=SubRevOp,
    rev_method_like=SubRevMt,
}


pub use crate_::ops::const_ord::{
    ConstOrdOp,
    ConstLtOp,ConstLtMt,
    ConstLEOp,ConstLEMt,
    ConstGtOp,ConstGtMt,
    ConstGEOp,ConstGEMt,
};
pub use crate_::ops::const_eq::{
    ConstEqOp,ConstEqMt,
    ConstNEOp,ConstNEMt,
};
pub use crate_::ops::const_from::{
    ConstFromOp,
    ConstIntoOp,
    ConstIntoMt,
};