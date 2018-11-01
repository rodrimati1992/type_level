use crate_::ops::{VariantAsTList, VariantAsTList_};
use prelude::*;

type_fn!{define_trait
    /// Compares Self with R for equality,returning True/False.
    trait=ConstEq_ [R]
    /// Compares Self with R for equality,returning True/False.
    type=ConstEq
    /// Compares Self with R for equality,returning True/False.
    fn_type=ConstEqOp
    /// Compares Self with R for equality,returning True/False.
    method_like=ConstEqMt
}

type_fn!{define_trait
    /// Compares Self with R for inequality,returning True/False.
    trait=ConstNE_ [R]
    /// Compares Self with R for inequality,returning True/False.
    type=ConstNE
    /// Compares Self with R for inequality,returning True/False.
    fn_type=ConstNEOp
    method_like=ConstNEMt
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

#[cfg(all(test,feature="passed_tests"))]
mod tests {
    use super::*;

    use typenum::consts::{U0, U1, U2};


    #[derive(TypeLevel)]
    #[typelevel(
        reexport(Struct),
        derive(ConstEq,ConstOrd),
    )]
    #[allow(dead_code)]
    struct Point{
        x:u32,
        y:u32,
    }

    type Test<equality,L,R>=
        AssEqTy<equality,ConstEq<L,R>>;

    #[test]
    pub fn test_typenum() {
        let _:Test<False,U1, U2>;
        let _:Test<True,U1, U1>;
        let _:Test<False,U1, U0>;

        let _:Test<False,(U1), (U2)>;
        let _:Test<True,(U1), (U1)>;
        let _:Test<False,(U1), (U0)>;

        let _:Test<False,(U1, U1), (U1, U2)>;
        let _:Test<True,(U1, U1), (U1, U1)>;
        let _:Test<False,(U1, U1), (U1, U0)>;

        let _:Test<False,(U1, U1, U1), (U1, U1, U2)>;
        let _:Test<True,(U1, U1, U1), (U1, U1, U1)>;
        let _:Test<False,(U1, U1, U1), (U1, U1, U0)>;

        let _:Test<False,(U1, U1, U1, U1), (U1, U1, U1, U2)>;
        let _:Test<True,(U1, U1, U1, U1), (U1, U1, U1, U1)>;
        let _:Test<False,(U1, U1, U1, U1), (U1, U1, U1, U0)>;
    }


    #[test]
    pub fn test_derived(){
        let _:Test<False,Some_<U0>,None_>;
        let _:Test<False,Some_<U1>,None_>;
        let _:Test<False,Some_<U2>,None_>;
        let _:Test<False,None_    ,Some_<U0>>;
        let _:Test<False,None_    ,Some_<U1>>;
        let _:Test<False,None_    ,Some_<U2>>;
        let _:Test<True,Some_<U0>,Some_<U0>>;
        let _:Test<True,Some_<U1>,Some_<U1>>;
        let _:Test<True,Some_<U2>,Some_<U2>>;
        let _:Test<True,None_    ,None_>;
        let _:Test<False,Some_<U0>,Some_<U1>>;
        let _:Test<True,Some_<U1>,Some_<U1>>;
        let _:Test<False,Some_<U2>,Some_<U1>>;
        

        let _:Test<False,Ok_<U0>,Err_<U0>>;
        let _:Test<False,Ok_<U1>,Err_<U0>>;
        let _:Test<False,Ok_<U2>,Err_<U0>>;
        
        let _:Test<True,Ok_<U0>,Ok_<U0>>;
        let _:Test<True,Ok_<U1>,Ok_<U1>>;
        let _:Test<True,Ok_<U2>,Ok_<U2>>;
        
        let _:Test<False,Ok_<U0>,Ok_<U1>>;
        let _:Test<True,Ok_<U1>,Ok_<U1>>;
        let _:Test<False,Ok_<U2>,Ok_<U1>>;

        let _:Test<False,Err_<U0>,Err_<U1>>;
        let _:Test<True,Err_<U1>,Err_<U1>>;
        let _:Test<False,Err_<U2>,Err_<U1>>;

        let _:Test<False,Err_<U0>,Ok_<U0>>;
        let _:Test<False,Err_<U0>,Ok_<U1>>;
        let _:Test<False,Err_<U0>,Ok_<U2>>;

        let _:Test<False,Err_<U0>,Ok_<U0>>;
        let _:Test<False,Err_<U0>,Ok_<U1>>;
        let _:Test<False,Err_<U0>,Ok_<U2>>;

        let _:Test<True,ConstPoint<U0,U0>,ConstPoint<U0,U0>>;
        let _:Test<True,ConstPoint<U1,U1>,ConstPoint<U1,U1>>;
        let _:Test<True,ConstPoint<U1,U2>,ConstPoint<U1,U2>>;
        let _:Test<True,ConstPoint<U2,U2>,ConstPoint<U2,U2>>;

        let _:Test<False,ConstPoint<U0,U0>,ConstPoint<U1,U0>>;
        let _:Test<False,ConstPoint<U0,U0>,ConstPoint<U2,U0>>;
        let _:Test<False,ConstPoint<U0,U0>,ConstPoint<U3,U0>>;
        let _:Test<False,ConstPoint<U0,U0>,ConstPoint<U0,U1>>;
        let _:Test<False,ConstPoint<U0,U0>,ConstPoint<U0,U2>>;
        let _:Test<False,ConstPoint<U0,U0>,ConstPoint<U0,U3>>;

        let _:Test<False,ConstPoint<U1,U0>,ConstPoint<U0,U0>>;
        let _:Test<False,ConstPoint<U2,U0>,ConstPoint<U0,U0>>;
        let _:Test<False,ConstPoint<U3,U0>,ConstPoint<U0,U0>>;
        let _:Test<False,ConstPoint<U0,U1>,ConstPoint<U0,U0>>;
        let _:Test<False,ConstPoint<U0,U2>,ConstPoint<U0,U0>>;
        let _:Test<False,ConstPoint<U0,U3>,ConstPoint<U0,U0>>;

    }

}
