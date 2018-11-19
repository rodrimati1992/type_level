use crate_::fn_adaptors::*;
use crate_::std_ops::*;
use crate_::ops::{ConstEq, VariantAsTList, VariantAsTList_,AssertEq};
use crate_::std_types::cmp_ordering::{Equal_, Greater_, Less_, OrderingTrait, OrderingType};
use prelude::*;

use std_::cmp::Ordering;

type_fn!{define_trait
    /// Compares Self with R,returning whether Self is Less_/Equal_/Greater_ than R
    trait=ConstOrd_ [R]
    /// Compares Self with R,returning whether Self is Less_/Equal_/Greater_ than R
    type=ConstOrd
    /// Compares Self with R,returning whether Self is Less_/Equal_/Greater_ than R
    fn_type=ConstOrdOp
    /// Compares Self with R,returning whether Self is Less_/Equal_/Greater_ than R
    method_like=ConstOrdMt
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

pub type ConstLtMt<R>=
    ApplyRhs<ConstLtOp,R>;


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

pub type ConstLEMt<R>=
    ApplyRhs<ConstLEOp,R>;


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

pub type ConstGtMt<R>=
    ApplyRhs<ConstGtOp,R>;


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

pub type ConstGEMt<R>=
    ApplyRhs<ConstGEOp,R>;



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


type_fn!{define_trait
    /// Returns the minimum value between Self and R.
    trait=Min_ [R]
    /// Returns the minimum value between Self and R.
    type=Min
    /// Returns the minimum value between Self and R.
    fn_type=MinOp
    /// Returns the minimum value between Self and R.
    method_like=MinMt
}

impl<L,R,_0,Out> Min_<R> for L 
where
    L:MinMax_<R,Output=(Out,_0)>
{
    type Output=Out;
}

type_fn!{define_trait
    /// Returns the maximum value between Self and R.
    trait=Max_ [R]
    /// Returns the maximum value between Self and R.
    type=Max
    /// Returns the maximum value between Self and R.
    fn_type=MaxOp
    /// Returns the maximum value between Self and R.
    method_like=MaxMt
}

impl<L,R,_0,Out> Max_<R> for L 
where
    L:MinMax_<R,Output=(_0,Out)>
{
    type Output=Out;
}


type_fn!{define_trait
    /// Returns a (mimumum_value,maximum_value) tuple by comparing Self and R.
    trait=MinMax_ [R]
    /// Returns a (mimumum_value,maximum_value) tuple by comparing Self and R.
    type=MinMax
    /// Returns a (mimumum_value,maximum_value) tuple by comparing Self and R.
    fn_type=MinMaxOp
    /// Returns a (mimumum_value,maximum_value) tuple by comparing Self and R.
    method_like=MinMaxMt
}

impl<L,R,order,Out> MinMax_<R> for L
where 
    L:ConstOrd_<R,Output=order>,
    MinMaxHelper<L,R>:TypeFn_<order,Output=Out>,
{
    type Output=Out;
}

type_fn!{
    captures(L,R)
    fn 
    MinMaxHelper(Less_){ (L,R) }
    MinMaxHelper(Equal_){ (L,R) }
    MinMaxHelper(Greater_){ (R,L) }
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

#[cfg(all(test,feature="passed_tests"))]
// #[cfg(test)]
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

    #[test]
    pub fn test_typenum() {
        let _: False = ConstLt::<U0, U0>::MTVAL;
        let _: True = ConstLt::<U0, U1>::MTVAL;
        let _: False = ConstLt::<U1, U1>::MTVAL;
        let _: False = ConstLt::<U2, U0>::MTVAL;
        let _: False = ConstLt::<U2, U1>::MTVAL;
        let _: False = ConstLt::<U2, U2>::MTVAL;
        let _: True = ConstLt::<U2, U3>::MTVAL;

        let _: True = ConstLE::<U0, U1>::MTVAL;
        let _: True = ConstLE::<U1, U1>::MTVAL;
        let _: False = ConstLE::<U2, U0>::MTVAL;
        let _: False = ConstLE::<U2, U1>::MTVAL;
        let _: True = ConstLE::<U2, U2>::MTVAL;
        let _: True = ConstLE::<U2, U3>::MTVAL;

        let _: False = ConstGt::<U0, U1>::MTVAL;
        let _: False = ConstGt::<U1, U1>::MTVAL;
        let _: True = ConstGt::<U2, U1>::MTVAL;

        let _: False = ConstGE::<U0, U1>::MTVAL;
        let _: True = ConstGE::<U1, U1>::MTVAL;
        let _: True = ConstGE::<U2, U0>::MTVAL;
        let _: True = ConstGE::<U2, U1>::MTVAL;
        let _: True = ConstGE::<U2, U2>::MTVAL;
        let _: False = ConstGE::<U2, U3>::MTVAL;
        let _: False = ConstGE::<U2, U4>::MTVAL;

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

    #[test]
    pub fn test_derived(){
        type Test<ordering,L,R>=
            AssEqTy<ordering,ConstOrd<L,R>>;

        let _:Test<Less_,Some_<U0>,None_>;
        let _:Test<Less_,Some_<U1>,None_>;
        let _:Test<Less_,Some_<U2>,None_>;
        let _:Test<Greater_,None_    ,Some_<U0>>;
        let _:Test<Greater_,None_    ,Some_<U1>>;
        let _:Test<Greater_,None_    ,Some_<U2>>;
        let _:Test<Equal_,Some_<U0>,Some_<U0>>;
        let _:Test<Equal_,Some_<U1>,Some_<U1>>;
        let _:Test<Equal_,Some_<U2>,Some_<U2>>;
        let _:Test<Equal_,None_    ,None_>;
        let _:Test<Less_,Some_<U0>,Some_<U1>>;
        let _:Test<Equal_,Some_<U1>,Some_<U1>>;
        let _:Test<Greater_,Some_<U2>,Some_<U1>>;
        

        let _:Test<Less_,Ok_<U0>,Err_<U0>>;
        let _:Test<Less_,Ok_<U1>,Err_<U0>>;
        let _:Test<Less_,Ok_<U2>,Err_<U0>>;
        
        let _:Test<Equal_,Ok_<U0>,Ok_<U0>>;
        let _:Test<Equal_,Ok_<U1>,Ok_<U1>>;
        let _:Test<Equal_,Ok_<U2>,Ok_<U2>>;
        
        let _:Test<Less_,Ok_<U0>,Ok_<U1>>;
        let _:Test<Equal_,Ok_<U1>,Ok_<U1>>;
        let _:Test<Greater_,Ok_<U2>,Ok_<U1>>;

        let _:Test<Less_,Err_<U0>,Err_<U1>>;
        let _:Test<Equal_,Err_<U1>,Err_<U1>>;
        let _:Test<Greater_,Err_<U2>,Err_<U1>>;

        let _:Test<Greater_,Err_<U0>,Ok_<U0>>;
        let _:Test<Greater_,Err_<U0>,Ok_<U1>>;
        let _:Test<Greater_,Err_<U0>,Ok_<U2>>;

        let _:Test<Greater_,Err_<U0>,Ok_<U0>>;
        let _:Test<Greater_,Err_<U0>,Ok_<U1>>;
        let _:Test<Greater_,Err_<U0>,Ok_<U2>>;

        let _:Test<Equal_,ConstPoint<U0,U0>,ConstPoint<U0,U0>>;
        let _:Test<Equal_,ConstPoint<U1,U1>,ConstPoint<U1,U1>>;
        let _:Test<Equal_,ConstPoint<U1,U2>,ConstPoint<U1,U2>>;
        let _:Test<Equal_,ConstPoint<U2,U2>,ConstPoint<U2,U2>>;

        let _:Test<Less_,ConstPoint<U0,U0>,ConstPoint<U1,U0>>;
        let _:Test<Less_,ConstPoint<U0,U0>,ConstPoint<U2,U0>>;
        let _:Test<Less_,ConstPoint<U0,U0>,ConstPoint<U3,U0>>;
        let _:Test<Less_,ConstPoint<U0,U0>,ConstPoint<U0,U1>>;
        let _:Test<Less_,ConstPoint<U0,U0>,ConstPoint<U0,U2>>;
        let _:Test<Less_,ConstPoint<U0,U0>,ConstPoint<U0,U3>>;

        let _:Test<Greater_,ConstPoint<U1,U0>,ConstPoint<U0,U0>>;
        let _:Test<Greater_,ConstPoint<U2,U0>,ConstPoint<U0,U0>>;
        let _:Test<Greater_,ConstPoint<U3,U0>,ConstPoint<U0,U0>>;
        let _:Test<Greater_,ConstPoint<U0,U1>,ConstPoint<U0,U0>>;
        let _:Test<Greater_,ConstPoint<U0,U2>,ConstPoint<U0,U0>>;
        let _:Test<Greater_,ConstPoint<U0,U3>,ConstPoint<U0,U0>>;

    }

    #[test]
    fn min_max(){
        type Test<L,R>=(
            AssertEq< Min<L,R> , L >,
            AssertEq< Min<R,L> , L >,
            AssertEq< Max<L,R> , R >,
            AssertEq< Max<R,L> , R >,
            AssertEq< MinMax<L,R> , (L,R) >,
            AssertEq< MinMax<R,L> , (L,R) >,
        );

        let _:Test<N2,N2>;
        let _:Test<N1,N1>;
        let _:Test<Z0,Z0>;
        let _:Test<P1,P1>;
        let _:Test<P2,P2>;

        let _:Test<N2,N1>;
        let _:Test<N2,Z0>;
        let _:Test<N2,P1>;

        let _:Test<N1,Z0>;
        let _:Test<N1,P1>;
        let _:Test<N1,P2>;
        
        let _:Test<Z0,P1>;
        let _:Test<Z0,P2>;
        let _:Test<Z0,P3>;
        
        let _:Test<P1,P2>;
        let _:Test<P1,P3>;
        let _:Test<P1,P4>;
        
        let _:Test<P2,P3>;
        let _:Test<P2,P4>;
        let _:Test<P2,P5>;


        let _:Test<U0,U0>;
        let _:Test<U1,U1>;
        let _:Test<U2,U2>;

        let _:Test<U0,U1>;
        let _:Test<U0,U2>;
        let _:Test<U0,U3>;
        
        let _:Test<U1,U2>;
        let _:Test<U1,U3>;
        let _:Test<U1,U4>;
        
        let _:Test<U2,U3>;
        let _:Test<U2,U4>;
        let _:Test<U2,U5>;
        


    }
}
