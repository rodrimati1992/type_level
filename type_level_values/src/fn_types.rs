/*!
These are TypeFn_ aliases of pre-existing trait.
*/

use prelude::*;

use std_::ops::{
    Add as Std_Add, BitAnd as Std_BitAnd, BitOr as Std_BitOr, BitXor as Std_BitXor, Div as Std_Div,
    Index as Std_Index, IndexMut as Std_IndexMut, Mul as Std_Mul, Neg as Std_Neg, Not as Std_Not,
    Rem as Std_Rem, Shl as Std_Shl, Shr as Std_Shr, Sub as Std_Sub,
};
use crate_::std_types::cmp_ordering::{Equal_, Greater_, Less_};
use crate_::ops::{ConstFrom_, ConstInto_,ConstEq_, ConstNE_};


use fn_adaptors::*;
use crate_::ops::iteration_ops::*;

type_fn!{alias AddOp      [A,B]=Std_Add}
type_fn!{alias BitAndOp   [A,B]=Std_BitAnd}
type_fn!{alias BitXorOp   [A,B]=Std_BitXor}
type_fn!{alias BitOrOp    [A,B]=Std_BitOr}
type_fn!{alias DivOp      [A,B]=Std_Div}
type_fn!{alias IndexOp    [A,B]=Std_Index}
type_fn!{alias IndexMutOp [A,B]=Std_IndexMut}
type_fn!{alias MulOp      [A,B]=Std_Mul}
type_fn!{alias NegOp      [A]  =Std_Neg}
type_fn!{alias NotOp      [A]  =Std_Not}
type_fn!{alias RemOp      [A,B]=Std_Rem}
type_fn!{alias ShlOp      [A,B]=Std_Shl}
type_fn!{alias ShrOp      [A,B]=Std_Shr}
type_fn!{alias SubOp      [A,B]=Std_Sub}
type_fn!{alias ConstOrdOp [A,B]=ConstOrd_}
type_fn!{alias ConstEqOp  [A,B]=ConstEq_}
type_fn!{alias ConstNEOp  [A,B]=ConstNE_}
type_fn!{alias ConstFromOp[A,B]=ConstFrom_}
type_fn!{alias ConstIntoOp[A,B]=ConstInto_}

type_fn!{
    /// Const less-than.
    pub fn ConstLtOp[L,R](L,R)
    where[
        L:ConstOrd_<R>,
        L::Output:ConstEq_<Less_,Output=Out>,
        Out:Boolean,
    ]{ let Out;Out }
}

type_fn!{
    /// Const less-than-or-equal.
    pub fn ConstLEOp[L,R](L,R)
    where[
        L:ConstOrd_<R>,
        IsLessOrEqual:TypeFn_<L::Output,Output=Out>,
    ]{ let Out;Out }
}

type_fn!{
    /// Const greater-than.
    pub fn ConstGtOp[L,R](L,R)
    where[
        L:ConstOrd_<R>,
        L::Output:ConstEq_<Greater_,Output=Out>,
        Out:Boolean,
    ]{ let Out;Out }
}

type_fn!{
    /// Const greater-than-or-equal.
    pub fn ConstGEOp[L,R](L,R)
    where[
        L:ConstOrd_<R>,
        IsGreaterOrEqual:TypeFn_<L::Output,Output=Out>,
    ]{ let Out;Out }
}

use self::_helper::*;

mod _helper {
    use super::*;
    type_fn!{
        pub fn IsLessOrEqual(Less_ ){True}
               IsLessOrEqual(Equal_){True}
               IsLessOrEqual(Greater_){False}
    }
    type_fn!{
        pub fn IsGreaterOrEqual(Less_ ){False}
               IsGreaterOrEqual(Equal_){True}
               IsGreaterOrEqual(Greater_){True}
    }
}
