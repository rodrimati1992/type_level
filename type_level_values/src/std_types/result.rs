use core_extensions::type_level_bool::{False, True};
use core_extensions::Void;

use crate_::fn_types::{BitAndOp, BitOrOp, DivOp, MulOp, NotOp};
use crate_::ops::{FoldL_, FoldR_, Map_, Unwrap_};
use prelude::*;

use std_::ops::{BitAnd, BitOr};
use std_::result::Result as StdResult;

use typenum::consts::{U0, U1, U2, U3, U4, U5, U6, U7, U8};

#[derive(TypeLevel)]
#[typelevel(
    reexport = "pub",
    derive(ConstEq, ConstOrd),
    items(runtime_conv(Internal = "StdResult")),
)]
#[allow(dead_code)]
#[doc(hidden)]
pub enum Result<T, E> {
    #[typelevel(rename = "Ok_")]
    Ok(T),
    #[typelevel(rename = "Err_")]
    Err(E),
}

#[doc(inline)]
pub use self::type_level_Result::*;

/////////////////////////////

impl<V, O> BitOr<O> for Ok_<V> {
    type Output = Self;
    fn bitor(self, _: O) -> Self {
        self
    }
}

impl<E, O> BitOr<O> for Err_<E>
where
    O: ResultTrait,
{
    type Output = O;
    fn bitor(self, v: O) -> O {
        v
    }
}

/////////////////////////////

impl<V, O> BitAnd<O> for Ok_<V>
where
    O: ResultTrait,
{
    type Output = O;
    fn bitand(self, v: O) -> O {
        v
    }
}

impl<E, O> BitAnd<O> for Err_<E> {
    type Output = Self;
    fn bitand(self, _: O) -> Self {
        self
    }
}

/////////////////////////////

type_fn!{
    pub fn IsOk[V](Ok_<V>){True}
           IsOk[V](Err_<V>){False}
}

pub type IsErr = (IsOk, NotOp);

/////////////////////////////

impl<T> Unwrap_ for Ok_<T> {
    type Output = T;
}

/////////////////////////////

#[allow(dead_code)]
fn tests() {
    use typenum::consts::{U0, U1};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_option() {
        let _: Ok_<U0> = Ok_(U0::CW);
        let _: Err_<U1> = Err_(U1::CW);

        assert_eq!(Ok_(U0::CW).into_runtime(), Ok::<_, ()>(0));
        assert_eq!(Err_(False.into()).into_runtime(), Err::<(), _>(false));
    }

    #[test]
    fn result_operators() {
        let _: Ok_<U0> = Ok_(U0::CW) | Err_(U1::CW);
        let _: Ok_<U0> = Ok_(U0::CW) | Ok_(U1::CW);
        let _: Err_<U1> = Err_(U0::CW) | Err_(U1::CW);
        let _: Ok_<U1> = Err_(U0::CW) | Ok_(U1::CW);

        let _: Err_<U1> = Ok_(U0::CW) & Err_(U1::CW);
        let _: Ok_<U1> = Ok_(U0::CW) & Ok_(U1::CW);
        let _: Err_<U0> = Err_(U0::CW) & Err_(U1::CW);
        let _: Err_<U0> = Err_(U0::CW) & Ok_(U1::CW);
    }

    #[test]
    fn result_functions() {
        let _: False = <TypeFn<IsOk, Err_<False>>>::MTVAL;
        let _: True = <TypeFn<IsOk, Ok_<U1>>>::MTVAL;

        let _: True = <TypeFn<IsErr, Err_<False>>>::MTVAL;
        let _: False = <TypeFn<IsErr, Ok_<U1>>>::MTVAL;

        let _: U0 = <<Ok_<U0> as Unwrap_>::Output>::MTVAL;
    }

}
