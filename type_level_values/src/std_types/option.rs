use core_extensions::type_level_bool::{False, True};
use core_extensions::Void;

use crate_::fn_adaptors::{ApplyRhs,Const};
use crate_::std_ops::{BitAndOp, BitOrOp, DivOp,MulMt, NotOp};
use crate_::ops::{
    Unwrap_,Unwrap,UnwrapOr,UnwrapOrElse_,
    IntoInner_,
    If,
    AssertEq,
    ConstEqMt,
    ConstIntoMt,
    SafeDivOp,
};
use crate_::collection_ops::{
    FoldL, FoldL_, FoldR, FoldR_, 
    TryFoldL, TryFoldL_, TryFoldR, TryFoldR_, 
    Len_, 
    Map, Map_,
    Filter_,Filter,
    TryFoldType,TFVal,TFBreak,
};
use prelude::*;

use std_::fmt::Debug;
use std_::ops::{BitAnd, BitOr};
use std_::option::Option as StdOption;

#[derive(TypeLevel)]
#[typelevel(
    reexport = "pub",
    derive(ConstEq, ConstOrd),
    items(runtime_conv(Internal = "StdOption")),
)]
#[allow(dead_code)]
#[doc(hidden)]
pub enum Option<T> {
    #[typelevel(rename = "Some_")]
    Some(T),
    #[typelevel(rename = "None_")]
    None,
}

///////////////////////////////////////////////////////////////////////////////////////

type_fn!{
    pub fn NewSome[v](v){ Some_<v> }
}
pub type NewNone=Const<None_>;

///////////////////////////////////////////////////////////////////////////////////////

impl<Func,Params> TypeFn_<Params> for Some_<Func>
where
    Func: TypeFn_<Params>,
{
    type Output = Func::Output;
}
impl<Params> TypeFn_<Params> for None_ {
    type Output = Params;
}

///////////////////////////////////////////////////////////////////////////////////////

impl<Op, T> Map_<Op> for Some_<T>
where
    Op: TypeFn_<T>,
{
    type Output = Some_<Op::Output>;
}
impl<Op> Map_<Op> for None_ {
    type Output = None_;
}
///////////////////////////////////////////////////////////////////////////////////////

impl<Pred,Out, T> Filter_<Pred> for Some_<T>
where
    If<Pred,NewSome,NewNone>: TypeFn_<T,Output=Out>,
{
    type Output = Out;
}
impl<Pred> Filter_<Pred> for None_ {
    type Output = None_;
}
///////////////////////////////////////////////////////////////////////////////////////

impl<DefaultValue, Op, T> FoldR_<DefaultValue, Op> for Some_<T>
where
    Op: TypeFn_<(DefaultValue, T)>,
{
    type Output = Op::Output;
}
impl<DefaultValue, Op> FoldR_<DefaultValue, Op> for None_ {
    type Output = DefaultValue;
}

///////////////////////////////////////////////////////////////////////////////////////

impl<DefaultValue, Op, T> FoldL_<DefaultValue, Op> for Some_<T>
where
    Op: TypeFn_<(DefaultValue, T)>,
{
    type Output = Op::Output;
}
impl<DefaultValue, Op> FoldL_<DefaultValue, Op> for None_ {
    type Output = DefaultValue;
}

///////////////////////////////////////////////////////////////////////////////////////

impl<DefaultValue, Op, T,Out> TryFoldR_<DefaultValue, Op> for Some_<T>
where
    (Op,ConstIntoMt<TryFoldType>): TypeFn_<(DefaultValue, T),Output=Out>,
{
    type Output = Out;
}
impl<DefaultValue, Op> TryFoldR_<DefaultValue, Op> for None_ {
    type Output = TFVal<DefaultValue>;
}

///////////////////////////////////////////////////////////////////////////////////////

impl<DefaultValue, Op, T,Out> TryFoldL_<DefaultValue, Op> for Some_<T>
where
    (Op,ConstIntoMt<TryFoldType>): TypeFn_<(DefaultValue, T),Output=Out>,
{
    type Output = Out;
}
impl<DefaultValue, Op> TryFoldL_<DefaultValue, Op> for None_ {
    type Output = TFVal<DefaultValue>;
}

///////////////////////////////////////////////////////////////////////////////////////

impl<T> Len_ for Some_<T> {
    type Output = U1;
}
impl Len_ for None_ {
    type Output = U0;
}

///////////////////////////////////////////////////////////////////////////////////////

impl<V, O> BitOr<O> for Some_<V> {
    type Output = Self;
    fn bitor(self, _: O) -> Self {
        self
    }
}

impl<O> BitOr<O> for None_
where
    O: OptionTrait,
{
    type Output = O;
    fn bitor(self, v: O) -> O {
        v
    }
}

///////////////////////////////////////////////////////////////////////////////////////

impl<V, O> BitAnd<O> for Some_<V>
where
    O: OptionTrait,
{
    type Output = O;
    fn bitand(self, v: O) -> O {
        v
    }
}

impl<O> BitAnd<O> for None_ {
    type Output = None_;
    fn bitand(self, _: O) -> None_ {
        None_
    }
}

///////////////////////////////////////////////////////////////////////////////////////

type_fn!{
    pub fn IsSome(None_){False}
           IsSome[T](Some_<T>){True}
}

pub type IsNone = (IsSome, NotOp);

///////////////////////////////////////////////////////////////////////////////////////

impl<T> Unwrap_ for Some_<T> {
    type Output = T;
}

///////////////////////////////////////////////////////////////////////////////////////

impl<DefFunc> UnwrapOrElse_<DefFunc> for None_ 
where 
    DefFunc:TypeFn_<()>
{
    type Output = DefFunc::Output;
}

impl<T,DefFunc> UnwrapOrElse_<DefFunc> for Some_<T> {
    type Output = T;
}

///////////////////////////////////////////////////////////////////////////////////////


impl<> IntoInner_ for None_ {
    type Output = ();
}

impl<T> IntoInner_ for Some_<T> {
    type Output = T;
}

///////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
// #[cfg(all(test,feature="passed_tests"))]
mod tests {
    use super::*;

    #[test]
    fn construct_option() {
        let _: Some_<U0> = Some_(U0::CW);
        let _: None_ = None_;

        assert_eq!(Some_(U0::CW).into_runtime(), Some(0));
        assert_eq!(None_.into_runtime(), None::<Void>);
    }

    #[test]
    fn option_iteration() {
        type TestFold<This,DefaultValue,Func,Res>=(
            AssertEq<FoldL<This, DefaultValue, Func>,Res>,
            AssertEq<FoldR<This, DefaultValue, Func>,Res>,
        );
        let _: TestFold<None_, U1, DivOp,U1>;
        let _: TestFold<Some_<U2>, U4, DivOp,U2>;


        type TestTryFold<This,DefaultValue,Func,Res>=(
            AssertEq<TryFoldL<This, DefaultValue, Func>,Res>,
            AssertEq<TryFoldR<This, DefaultValue, Func>,Res>,
        );
        let _: TestTryFold<None_    , U8, SafeDivOp,TFVal<U8>>;
        let _: TestTryFold<None_    , U0, SafeDivOp,TFVal<U0>>;
        let _: TestTryFold<Some_<U2>, U8, SafeDivOp,TFVal<U4>>;
        let _: TestTryFold<Some_<U0>, U8, SafeDivOp,TFBreak<None_>>;

        let _: AssertEq<Some_<U6>,Map<Some_<U3>, MulMt<U2>>>;
        let _: AssertEq<None_,Map<None_, MulMt<U2>>>;

        let _: AssertEq< Some_<U3> , Filter<Some_<U3>, ConstEqMt<U3>>>;
        let _: AssertEq< None_ , Filter<Some_<U3>, ConstEqMt<U0>>>;
        let _: AssertEq< None_ , Filter<None_,Const<True>>>;
        let _: AssertEq< None_ , Filter<None_,Const<False>>>;
    }

    #[test]
    fn option_operators() {
        let _: None_ = None_ | None_;
        let _: Some_<U1> = None_ | Some_(U1::CW);
        let _: Some_<U2> = Some_(U2::CW) | None_;
        let _: Some_<U2> = Some_(U2::CW) | Some_(U1::CW);

        let _: None_ = None_ & None_;
        let _: None_ = None_ & Some_(U1::CW);
        let _: None_ = Some_(U2::CW) & None_;
        let _: Some_<U1> = Some_(U2::CW) & Some_(U1::CW);
    }

    #[test]
    fn option_functions() {
        let _:AssEqTy<TypeFn<IsSome, None_>,False>;
        let _:AssEqTy<TypeFn<IsSome, Some_<U1>>,True>;

        let _:AssEqTy<TypeFn<IsNone, None_>,True>;
        let _:AssEqTy<TypeFn<IsNone, Some_<U1>>,False>;

        let _:AssEqTy<Unwrap<Some_<U0>>,U0>;
        let _:AssEqTy<Unwrap<Some_<U1>>,U1>;

        let _:AssEqTy<UnwrapOr<Some_<U0>,U100>,U0>;
        let _:AssEqTy<UnwrapOr<Some_<U1>,U100>,U1>;
        let _:AssEqTy<UnwrapOr<None_    ,U200>,U200>;
    }

}
