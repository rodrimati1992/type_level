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

type_fn!{alias AddOp      [A,B]=Add}
type_fn!{alias BitAndOp   [A,B]=BitAnd}
type_fn!{alias BitXorOp   [A,B]=BitXor}
type_fn!{alias BitOrOp    [A,B]=BitOr}
type_fn!{alias DivOp      [A,B]=Div}
type_fn!{alias IndexOp    [A,B]=Index}
type_fn!{alias IndexMutOp [A,B]=IndexMut}
type_fn!{alias MulOp      [A,B]=Mul}
type_fn!{alias NegOp      [A]  =Neg}
type_fn!{alias NotOp      [A]  =Not}
type_fn!{alias RemOp      [A,B]=Rem}
type_fn!{alias ShlOp      [A,B]=Shl}
type_fn!{alias ShrOp      [A,B]=Shr}
type_fn!{alias SubOp      [A,B]=Sub}

pub use crate_::ops::const_ord::{
    ConstOrdOp,
    ConstLtOp,
    ConstLEOp,
    ConstGtOp,
    ConstGEOp,
};
pub use crate_::ops::const_from::{
    ConstFromOp,
    ConstIntoOp,
};
pub use crate_::ops::const_eq::{
    ConstEqOp,
    ConstNEOp,
};