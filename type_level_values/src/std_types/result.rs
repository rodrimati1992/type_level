use core_extensions::type_level_bool::{False, True};
use core_extensions::Void;

use crate_::std_ops::{BitAndOp, BitOrOp, DivOp, MulOp, NotOp};
use crate_::ops::{
    AssertConstTypeMt,AssertEq,AssertFnRet,
    Unwrap_,Unwrap,UnwrapOrElse_,UnwrapOr,
    IntoInner_,
    Add1Op,
    AndThen_,OrElse_,AndThenMt,OrElseMt,
};
use crate_::collection_ops::{FoldL_, FoldR_, Len_, Map, Map_};
use prelude::*;

use std_::ops::{BitAnd, BitOr};
use std_::result::Result as StdResult;



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

type_fn!{
    pub fn NewOk[v](v){ Ok_<v> }
}
type_fn!{
    pub fn NewErr[v](v){ Err_<v> }
}

///////////////////////////////////////////////////////////////////////////////////////

impl<T,Func,Out> Map_<Func> for Ok_<T>
where Func:TypeFn_<T,Output=Out>,
{
    type Output=Ok_<Out>;
}

impl<T,Func> Map_<Func> for Err_<T>{
    type Output=Self;
}

///////////////////////////////////////////////////////////////////////////////////////

type_fn!{
    pub fn 
        MapErrOp[T,Func](Ok_<T>,Func){ Ok_<T>  }
        
        MapErrOp[T,Func](Err_<T>,Func)
        where[ Func:TypeFn_<T> ]
        { Err_<Func::Output> }
}

type_fn!{
    captures(Func)
    pub fn 
        MapErrMt[T](Ok_<T>){ Ok_<T>  }
        
        MapErrMt[T](Err_<T>)
        where[ Func:TypeFn_<T> ]
        { Err_<Func::Output> }
}

///////////////////////////////////////////////////////////////////////////////////////

impl<T,Func,Out> AndThen_<Func> for Ok_<T>
where
    (Func,AssertConstTypeMt<ResultType>):TypeFn_<T,Output=Out>
{
    type Output=Out;
}

impl<E,Func> AndThen_<Func> for Err_<E>{
    type Output=Self;
}

///////////////////////////////////////////////////////////////////////////////////////


impl<T,Func> OrElse_<Func> for Ok_<T>{
    type Output=Self;
}

impl<E,Func,Out> OrElse_<Func> for Err_<E>
where
    (Func,AssertConstTypeMt<ResultType>):TypeFn_<E,Output=Out>
{
    type Output=Out;
}

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

impl<T,Def> UnwrapOrElse_<Def> for Ok_<T> {
    type Output = T;
}

impl<E,Def> UnwrapOrElse_<Def> for Err_<E> 
where 
    Def:TypeFn_<E>
{
    type Output = Def::Output;
}

/////////////////////////////


impl<T> IntoInner_ for Ok_<T> {
    type Output = T;
}

impl<T> IntoInner_ for Err_<T> {
    type Output = T;
}


/////////////////////////////

// #[cfg(test)]
#[cfg(all(test,feature="passed_tests"))]
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
        let _: AssertEq<TypeFn<IsOk, Err_<False>>,False>;
        let _: AssertEq<TypeFn<IsOk, Ok_<U1>>,True>;

        let _: AssertEq<TypeFn<IsErr, Err_<False>>,True>;
        let _: AssertEq<TypeFn<IsErr, Ok_<U1>>,False>;

        let _: AssertEq<Unwrap<Ok_<U0>>,U0>;
        let _: AssertEq<Unwrap<Ok_<U1>>,U1>;

        let _: AssertEq<UnwrapOr<Ok_<U0>,False>,U0>;
        let _: AssertEq<UnwrapOr<Ok_<U1>,False>,U1>;
        let _: AssertEq<UnwrapOr<Err_<U100>,U400>,U400>;
        let _: AssertEq<UnwrapOr<Err_<U100>,U200>,U200>;

    }

    #[test]
    fn mapping(){
        type Test0<Val,Func,Expected>=
            AssertEq<Map<Val,Func>,Expected>;

        let _:Test0<Ok_<U0>,Add1Op,Ok_<U1>>;
        let _:Test0<Ok_<U1>,Add1Op,Ok_<U2>>;
        let _:Test0<Err_<U0>,Add1Op,Err_<U0>>;
        let _:Test0<Err_<U1>,Add1Op,Err_<U1>>;
        
        type Test1<Val,Func,Expected>=(
            AssertFnRet<(Val,Func),MapErrOp,Expected>,
            AssertFnRet<Val,MapErrMt<Func>,Expected>,
        );

        let _:Test1<Err_<U0>,Add1Op,Err_<U1>>;
        let _:Test1<Err_<U1>,Add1Op,Err_<U2>>;
        let _:Test1<Ok_<U0>,Add1Op,Ok_<U0>>;
        let _:Test1<Ok_<U1>,Add1Op,Ok_<U1>>;
        
    }

    #[test]
    fn and_then_or_else(){
        type TestAT<Val,Func,Expected>=
            AssertFnRet<Val,AndThenMt<Func>,Expected>;

        type TestOE<Val,Func,Expected>=
            AssertFnRet<Val,OrElseMt<Func>,Expected>;

        type AddOk=(Add1Op,NewOk);
        type AddErr=(Add1Op,NewErr);


        let _:TestAT<Ok_<U0>,AddOk,Ok_<U1>>;
        let _:TestAT<Ok_<U1>,AddOk,Ok_<U2>>;
        let _:TestAT<Ok_<U0>,AddErr,Err_<U1>>;
        let _:TestAT<Ok_<U1>,AddErr,Err_<U2>>;
        let _:TestAT<Ok_<U1>,(AddErr,AndThenMt<AddErr>,AndThenMt<AddErr>),Err_<U2>>;

        let _:TestOE<Err_<U0>,AddErr,Err_<U1>>;
        let _:TestOE<Err_<U1>,AddErr,Err_<U2>>;
        let _:TestOE<Err_<U0>,AddOk,Ok_<U1>>;
        let _:TestOE<Err_<U1>,AddOk,Ok_<U2>>;
        let _:TestOE<Err_<U1>,(AddOk,OrElseMt<AddOk>,OrElseMt<AddOk>),Ok_<U2>>;

    }
}
