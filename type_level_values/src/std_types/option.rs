use core_extensions::type_level_bool::{False, True};
use core_extensions::Void;

use crate_::ops::fn_adaptors::ApplyRhs;
use crate_::ops::fn_types::{BitAndOp, BitOrOp, DivOp, MulOp, NotOp};
use crate_::ops::{FoldL, FoldL_, FoldR, FoldR_, Len_, Map, Map_, TypeFn, TypeFn_, Unwrap_};
use prelude::*;

use std_::fmt::Debug;
use std_::ops::{BitAnd, BitOr};
use std_::option::Option as StdOption;

use typenum::consts::{U0, U1, U2, U3, U4, U5, U6, U7, U8};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_option() {
        let _: Some_<U0> = Some_(U0::PW);
        let _: None_ = None_;

        assert_eq!(Some_(U0::PW).into_runtime(), Some(0));
        assert_eq!(None_.into_runtime(), None::<Void>);
    }

    #[test]
    fn option_iteration() {
        let _: U1 = <FoldR<None_, U1, DivOp>>::MTVAL;
        let _: U2 = <FoldR<Some_<U2>, U4, DivOp>>::MTVAL;

        let _: U1 = <FoldL<None_, U1, DivOp>>::MTVAL;
        let _: U2 = <FoldL<Some_<U2>, U4, DivOp>>::MTVAL;

        let _: Some_<U6> = <Map<Some_<U3>, ApplyRhs<MulOp, U2>>>::MTVAL;
        let _: None_ = <Map<None_, ApplyRhs<MulOp, U2>>>::MTVAL;
    }

    #[test]
    fn option_operators() {
        let _: None_ = None_ | None_;
        let _: Some_<U1> = None_ | Some_(U1::PW);
        let _: Some_<U2> = Some_(U2::PW) | None_;
        let _: Some_<U2> = Some_(U2::PW) | Some_(U1::PW);

        let _: None_ = None_ & None_;
        let _: None_ = None_ & Some_(U1::PW);
        let _: None_ = Some_(U2::PW) & None_;
        let _: Some_<U1> = Some_(U2::PW) & Some_(U1::PW);
    }

    #[test]
    fn option_functions() {
        let _: False = <TypeFn<IsSome, None_>>::MTVAL;
        let _: True = <TypeFn<IsSome, Some_<U1>>>::MTVAL;

        let _: True = <TypeFn<IsNone, None_>>::MTVAL;
        let _: False = <TypeFn<IsNone, Some_<U1>>>::MTVAL;

        let _: U0 = <<Some_<U0> as Unwrap_>::Output>::MTVAL;
    }

}
