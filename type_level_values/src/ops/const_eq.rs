use crate_::ops::{VariantAsTList, VariantAsTList_};
use prelude::*;

/// Compares Self with Rhs for equality,returning True/False.
pub trait ConstEq_<Rhs> {
    type Output: Boolean;
}

/// Compares L with R for equality,returning True/False.
pub type ConstEq<L, R> = <L as ConstEq_<R>>::Output;

pub use crate_::ops::fn_types::ConstEqOp;

pub trait ConstNE_<Rhs> {
    type Output: Boolean;
}

impl<Lhs, Rhs> ConstNE_<Rhs> for Lhs
where
    Lhs: ConstEq_<Rhs>,
{
    type Output = <ConstEq<Lhs, Rhs> as Boolean>::Not;
}

/// Compares L with R for inequality,returning True/False.
pub type ConstNE<L, R> = <L as ConstNE_<R>>::Output;

mod ord_for_numtype {
    use super::*;

    use crate_::ops::ConstInto_;
    use crate_::std_types::cmp_ordering::{Equal_, Greater_, Less_, OrderingTrait};

    use typenum::bit::{B0, B1};
    use typenum::marker_traits::{Bit, Integer, NonZero, Unsigned};
    use typenum::{NInt, PInt, UInt, UTerm, Z0};

    use typenum::type_operators::IsEqual as TNIsEqual;

    // delegates the ConstEq impl to `typenum::type_operators::TNIsEqual`
    macro_rules! typenum_ord_impl {
        (   $This:ident
            $( vars=[$($prec_ty:ident),*] )*
            $( where [$($predicates:tt)*] )*
        ) => {
            impl< $($($prec_ty,)*)* Rhs,TNO,O> ConstEq_<Rhs> for $This<$($($prec_ty,)*)*>
            where
                $($($predicates)*)*
                Self:TNIsEqual<Rhs,Output=TNO>,
                TNO:Bit+ConstInto_<BooleanType,Output=O>,
                O:Boolean,
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
        let _: False = ConstEq::<U1, U2>::MTVAL;
        let _: True = ConstEq::<U1, U1>::MTVAL;
        let _: False = ConstEq::<U1, U0>::MTVAL;

        let _: False = ConstEq::<(U1), (U2)>::MTVAL;
        let _: True = ConstEq::<(U1), (U1)>::MTVAL;
        let _: False = ConstEq::<(U1), (U0)>::MTVAL;

        let _: False = ConstEq::<(U1, U1), (U1, U2)>::MTVAL;
        let _: True = ConstEq::<(U1, U1), (U1, U1)>::MTVAL;
        let _: False = ConstEq::<(U1, U1), (U1, U0)>::MTVAL;

        let _: False = ConstEq::<(U1, U1, U1), (U1, U1, U2)>::MTVAL;
        let _: True = ConstEq::<(U1, U1, U1), (U1, U1, U1)>::MTVAL;
        let _: False = ConstEq::<(U1, U1, U1), (U1, U1, U0)>::MTVAL;

        let _: False = ConstEq::<(U1, U1, U1, U1), (U1, U1, U1, U2)>::MTVAL;
        let _: True = ConstEq::<(U1, U1, U1, U1), (U1, U1, U1, U1)>::MTVAL;
        let _: False = ConstEq::<(U1, U1, U1, U1), (U1, U1, U1, U0)>::MTVAL;
    }

}
