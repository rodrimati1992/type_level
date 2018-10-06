use crate_::ops::{VariantAsTList, VariantAsTList_};
use prelude::*;

type_fn!{define_trait
    /// Compares Self with R for equality,returning True/False.
    trait=ConstEq_ [R]
    /// Compares Self with R for equality,returning True/False.
    type=ConstEq
    /// Compares Self with R for equality,returning True/False.
    fn_type=ConstEqOp
}

type_fn!{define_trait
    /// Compares Self with R for inequality,returning True/False.
    trait=ConstNE_ [R]
    /// Compares Self with R for inequality,returning True/False.
    type=ConstNE
    /// Compares Self with R for inequality,returning True/False.
    fn_type=ConstNEOp
}

impl<Lhs, Rhs,is_eq> ConstNE_<Rhs> for Lhs
where
    Lhs: ConstEq_<Rhs,Output=is_eq>,
    is_eq:Boolean,
{
    type Output = is_eq::Not;
}

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
