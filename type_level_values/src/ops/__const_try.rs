use prelude::*;


/// Trait representing fallible type-level values.
///
/// This trait defines 2 kinds of values:
///
/// - ok values:which is the value of an operation that succeded.
///
/// - err values:which is the value of an operation that failed.
///
pub trait ConstTry{
    /// Whether this ConstValue is an ok value,as defined by the type.
    ///
    /// If this is `True` then `Self::ToResult` is Ok_<_>.
    type IsOk ;
    /// Whether this ConstValue is an error value,as defined by the type.
    ///
    /// If this is `True` then `Self::ToResult` is Err_<_>.
    type IsErr;

    /// The ConstResult of this value,either Ok_ or Err_.
    type ToResult:ConstTry<IsOk=Self::IsOk,IsErr=Self::IsErr>+ResultTrait;
}


pub type TryIsOk<This>=
    <This as ConstTry>::IsOk;

type_fn!{
    pub fn TryIsOkOp[T](T)
    where[ T:ConstTry ]
    { T::IsOk }
}


pub type TryIsErr<This>=
    <This as ConstTry>::IsErr;

type_fn!{
    pub fn TryIsErrOp[T](T)
    where[ T:ConstTry ]
    { T::IsErr }
}


pub type ToResult<This>=
    <This as ConstTry>::ToResult;

type_fn!{
    pub fn ToResultOp[T](T)
    where[ T:ConstTry ]
    { T::ToResult }
}




impl<T> ConstTry for Some_<T>{
    type IsOk =True ;
    type IsErr=False;

    type ToResult=Ok_<T>;
}

impl ConstTry for None_{
    type IsOk =False;
    type IsErr=True;

    type ToResult=Err_<()>;
}

impl<T> ConstTry for Ok_<T>{
    type IsOk=True;
    type IsErr=False;

    type ToResult=Self;
}

impl<T> ConstTry for Err_<T>{
    type IsOk=False;
    type IsErr=True;

    type ToResult=Self;
}


#[cfg(all(test,feature="passed_tests"))]
mod tests{
    use super::*;
    use prelude::*;
    use crate_::ops::AssertEq;

    type TestToResult<L,R>=AssertEq< ToResult<L>, R >;
    
    #[test]
    fn option_result(){
        let _:TestToResult< Ok_<U10>   , Ok_<U10> >;
        let _:TestToResult< Err_<U10>  , Err_<U10> >;

        let _:TestToResult< Some_<U10> , Ok_<U10> >;
        let _:TestToResult< None_      , Err_<()> >;
    }
}
