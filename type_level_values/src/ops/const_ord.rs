use crate_::fn_adaptors::*;
use crate_::fn_types::*;
use crate_::ops::{ConstEq, Contains, VariantAsTList, VariantAsTList_};
use crate_::std_types::cmp_ordering::{Equal_, Greater_, Less_, OrderingTrait, OrderingType};
use prelude::*;

use std_::cmp::Ordering;

/// Compares Self with Rhs,returning whether Self is Less_/Equal_/Greater_ than Rhs
pub trait ConstOrd_<Rhs> {
    type Output;
}

/// Compares L to R,returning an Ordering_.
///
pub type ConstOrd<L, R> = TypeFn<ConstOrdOp, (L, R)>;

type_fn!{
    /// Compares L to R,returning an Ordering_.
    ///
    alias ConstOrdOp[L,R]=ConstOrd_
}


/// Returns whether L < R.
///
pub type ConstLt<L, R> = TypeFn<ConstLtOp, (L, R)>;

type_fn!{
    /// Returns whether L < R.
    ///
    pub fn ConstLtOp[L,R](L,R)
    where[
        L:ConstOrd_<R>,
        L::Output:ConstEq_<Less_,Output=Out>,
    ]{ let Out;Out }
}


/// Returns whether L <= R.
///
pub type ConstLE<L, R> = TypeFn<ConstLEOp, (L, R)>;

type_fn!{
    /// Returns whether L <= R.
    ///
    pub fn ConstLEOp[L,R](L,R)
    where[
        L:ConstOrd_<R>,
        _IsLessOrEqual:TypeFn_<L::Output,Output=Out>,
    ]{ let Out;Out }
}


/// Returns whether L > R.
///
pub type ConstGt<L, R> = TypeFn<ConstGtOp, (L, R)>;

type_fn!{
    /// Returns whether L > R.
    ///
    pub fn ConstGtOp[L,R](L,R)
    where[
        L:ConstOrd_<R>,
        L::Output:ConstEq_<Greater_,Output=Out>,
    ]{ let Out;Out }
}


/// Returns whether L >= R.
///
pub type ConstGE<L, R> = TypeFn<ConstGEOp, (L, R)>;

type_fn!{
    /// Returns whether L >= R.
    ///
    pub fn ConstGEOp[L,R](L,R)
    where[
        L:ConstOrd_<R>,
        _IsGreaterOrEqual:TypeFn_<L::Output,Output=Out>,
    ]{ let Out;Out }
}


type_fn!{
    fn _IsLessOrEqual(Less_ ){True}
       _IsLessOrEqual(Equal_){True}
       _IsLessOrEqual(Greater_){False}
}
type_fn!{
    fn _IsGreaterOrEqual(Less_ ){False}
       _IsGreaterOrEqual(Equal_){True}
       _IsGreaterOrEqual(Greater_){True}
}


mod numtype_impls {
    use super::*;

    use crate_::ops::ConstInto_;

    use typenum::bit::{B0, B1};
    use typenum::marker_traits::{Bit, Integer, NonZero, Unsigned};
    use typenum::{NInt, PInt, UInt, UTerm, Z0};

    use typenum::type_operators::Cmp as TNCmp;

    // delegates the ConstOrd impl to `typenum::type_operators::Cmp`
    macro_rules! typenum_ord_impl {
        (   $This:ident
            $( vars=[$($prec_ty:ident),*] )*
            $( where [$($predicates:tt)*] )*
        ) => {
            impl< $($($prec_ty,)*)* Rhs,TNO,O> ConstOrd_<Rhs> for $This<$($($prec_ty,)*)*>
            where
                $($($predicates)*)*
                Self:TNCmp<Rhs,Output=TNO>,
                TNO:ConstInto_<OrderingType,Output=O>,
                O:OrderingTrait,
            {
                type Output=O;
            }
        }
    }

    typenum_ord_impl!{ B0 }
    typenum_ord_impl!{ B1 }
    typenum_ord_impl!{ UInt vars=[U,B] }
    typenum_ord_impl!{ PInt vars=[U] where [U:NonZero+Unsigned,] }
    typenum_ord_impl!{ NInt vars=[U] where [U:NonZero+Unsigned,] }
    typenum_ord_impl!{ Z0 }
    typenum_ord_impl!{ UTerm }

}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::consts::{U0, U1, U2};

    #[test]
    pub fn test_typenum() {
        let _: True = ConstLt::<U0, U1>::MTVAL;
        let _: False = ConstLt::<U1, U1>::MTVAL;

        let _: True = ConstLE::<U0, U1>::MTVAL;
        let _: True = ConstLE::<U1, U1>::MTVAL;
        let _: False = ConstLE::<U2, U1>::MTVAL;

        let _: False = ConstGt::<U0, U1>::MTVAL;
        let _: False = ConstGt::<U1, U1>::MTVAL;
        let _: True = ConstGt::<U2, U1>::MTVAL;

        let _: False = ConstGE::<U0, U1>::MTVAL;
        let _: True = ConstGE::<U1, U1>::MTVAL;
        let _: True = ConstGE::<U2, U1>::MTVAL;

        let _: Less_ = ConstOrd::<U1, U2>::MTVAL;
        let _: Equal_ = ConstOrd::<U1, U1>::MTVAL;
        let _: Greater_ = ConstOrd::<U1, U0>::MTVAL;

        let _: Less_ = ConstOrd::<(U1), (U2)>::MTVAL;
        let _: Equal_ = ConstOrd::<(U1), (U1)>::MTVAL;
        let _: Greater_ = ConstOrd::<(U1), (U0)>::MTVAL;

        let _: Less_ = ConstOrd::<(U1, U1), (U1, U2)>::MTVAL;
        let _: Equal_ = ConstOrd::<(U1, U1), (U1, U1)>::MTVAL;
        let _: Greater_ = ConstOrd::<(U1, U1), (U1, U0)>::MTVAL;

        let _: Less_ = ConstOrd::<(U1, U1, U1), (U1, U1, U2)>::MTVAL;
        let _: Equal_ = ConstOrd::<(U1, U1, U1), (U1, U1, U1)>::MTVAL;
        let _: Greater_ = ConstOrd::<(U1, U1, U1), (U1, U1, U0)>::MTVAL;

        let _: Less_ = ConstOrd::<(U1, U1, U1, U1), (U1, U1, U1, U2)>::MTVAL;
        let _: Equal_ = ConstOrd::<(U1, U1, U1, U1), (U1, U1, U1, U1)>::MTVAL;
        let _: Greater_ = ConstOrd::<(U1, U1, U1, U1), (U1, U1, U1, U0)>::MTVAL;
    }
}
