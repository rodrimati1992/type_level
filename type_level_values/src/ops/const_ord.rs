use crate_::ops::fn_adaptors::*;
use crate_::ops::fn_types::*;
use crate_::ops::{ConstEq, Contains, TypeFn, VariantAsTList, VariantAsTList_};
use crate_::std_types::cmp_ordering::{Equal_, Greater_, Less_, OrderingTrait, OrderingType};
use prelude::*;

use std_::cmp::Ordering;

/// Compares Self with Rhs,returning whether Self is Less_/Equal_/Greater_ than Rhs
pub trait ConstOrd_<Rhs> {
    type Output: OrderingTrait;
}

/// Compares L to R,returning an Ordering_.
///
/// Equivalent to ::std::cmp::Ord::cmp.
pub type ConstOrd<L, R> = TypeFn<ConstOrdOp, (L, R)>;

/// Returns whether L is less than R.
///
/// Equivalent to L < R.
pub type ConstLt<L, R> = TypeFn<ConstLtOp, (L, R)>;

/// Returns whether L is less than or equal to R.
///
/// Equivalent to L <= R.
pub type ConstLE<L, R> = TypeFn<ConstLEOp, (L, R)>;

/// Returns whether L is greater than R.
///
/// Equivalent to L > R.
pub type ConstGt<L, R> = TypeFn<ConstGtOp, (L, R)>;

/// Returns whether L is greater than or equal to R.
///
/// Equivalent to L >= R.
pub type ConstGE<L, R> = TypeFn<ConstGEOp, (L, R)>;

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
