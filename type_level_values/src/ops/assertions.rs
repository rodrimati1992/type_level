use prelude::*;


//////////////////////////////////////////////////////////////


type_fn!{define_trait
    /**
    Asserts that Self is the same type as R.
    
    # Failing assertions.

    Assertions that fail produce a compile-time error.

    ```compile_fail
    use type_level_values::prelude::*;
    use type_level_values::ops::*;

    let _:AssertEq< U0,U1 >;
    ```
    
    */
    trait=AssertEq_ [R]
    /// Asserts that Self is the same type as R.
    type=AssertEq
    /// Asserts that Self is the same type as R.
    fn_type=AssertEqOp
    /// Asserts that This is the same type as Rhs.
    method_like=AssertEqMt
}


impl<L,R> AssertEq_<R> for L
where
    L:TypeIdentity<Type=R>
{
    type Output=L;
}


//////////////////////////////////////////////////////////////


type_fn!{define_trait
    /**
    Asserts that the ConstType of Self is the same as R.
    
    # Failing assertions.

    Assertions that fail produce a compile-time error.

    ```compile_fail
    use type_level_values::prelude::*;
    use type_level_values::ops::*;

    let _:AssertEqConstType< U0,False >;
    ```
    
    */
    trait=AssertEqConstType_ [R]
    /// Asserts that the ConstType of Self is the same as R.
    type=AssertEqConstType
    /// Asserts that the ConstType of Self is the same as R.
    fn_type=AssertEqConstTypeOp
    /// Asserts that the ConstType of This is the same as Rhs.
    method_like=AssertEqConstTypeMt
}


impl<L,R,TL,TR> AssertEqConstType_<R> for L
where
    ConstTypeOfOp:TypeFn_<L,Output=TL>,
    ConstTypeOfOp:TypeFn_<R,Output=TR>,
    TL:TypeIdentity<Type=TR>,
{
    type Output=L;
}


//////////////////////////////////////////////////////////////

type_fn!{define_trait
    /**
    Asserts that Func implements `TypeFn_<Val>`,returning Val unmodified.

    # Failing assertions.

    Assertions that fail produce a compile-time error.

    ```compile_fail
    use type_level_values::prelude::*;
    use type_level_values::ops::*;

    struct NewStruct;

    let _:AssertFunc< NewStruct,IsZeroOp >;
    ```

    */
    trait=AssertFunc_ [Func]
    /// Asserts that Func implements `TypeFn_<Val>`,returning Val unmodified.
    type=AssertFunc
    /// Asserts that Func implements `TypeFn_<Val>`,returning Val unmodified.
    fn_type=AssertFuncOp
    /// Asserts that Func implements `TypeFn_<Val>`,returning Val unmodified.
    method_like=AssertFuncMt
}

impl<This,Func> AssertFunc_<Func> for This
where 
    Func:TypeFn_<This>,
{
    type Output=This;
}


//////////////////////////////////////////////////////////////



type_fn!{define_trait
    /**
    Asserts that `Func:TypeFn_<Self>` evaluates to Ret,returning Self unmodified.

    # Failing assertions.

    Assertions that fail produce a compile-time error.

    ```compile_fail
    use type_level_values::prelude::*;
    use type_level_values::ops::*;

    let _:AssertFnRet< U0,IsZeroOp,False >;
    ```

    */
    trait=AssertFnRet_ [Func,Ret]
    /// Asserts that `Func:TypeFn_<This>` evaluates to Ret,returning This unmodified.
    type=AssertFnRet
    /// Asserts that `Func:TypeFn_<This>` evaluates to Ret,returning This unmodified.
    fn_type=AssertFnRetOp
    /// Asserts that `Func:TypeFn_<This>` evaluates to Ret,returning This unmodified.
    method_like=AssertFnRetMt
}


impl<This,Func,Ret> AssertFnRet_<Func,Ret> for This
where
    Func:TypeFn_<This,Output=Ret>
{
    type Output=This;
}


//////////////////////////////////////////////////////////////

type_fn!{define_trait
    /** 
    Asserts that `Self` is of `ConstTypeFor` ConstType
    
    # Failing assertions.

    Assertions that fail produce a compile-time error which mentions the 
    Self,Pred,and Msg values.

    ```compile_fail
    use type_level_values::prelude::*;
    use type_level_values::ops::*;

    let _:AssertConstType< U1,BooleanType >;

    ```

    */
    trait=AssertConstType_ [ConstTypeFor]
    /// Asserts that `Self` is of `ConstTypeFor` ConstType
    type=AssertConstType
    /// Asserts that `Self` is of `ConstTypeFor` ConstType
    fn_type=AssertConstTypeOp
    /// Asserts that `This` is of `ConstTypeFor` ConstType
    method_like=AssertConstTypeMt
}


impl<This,ConstTypeFor> AssertConstType_<ConstTypeFor> for This
where
    ConstTypeOfOp:TypeFn_<This,Output=ConstTypeFor>,
{
    type Output=Self;
}


//////////////////////////////////////////////////////////////



type_fn!{define_trait
    /** 
    Asserts that `Self` satisfies the `Pred` predicate

    # Failing assertions.

    Assertions that fail produce a compile-time error which mentions the 
    Self,Pred,and Msg values.

    ```compile_fail
    use type_level_values::prelude::*;
    use type_level_values::ops::*;

    let _:AssertThat< U1,IsZeroOp >;

    ```

    */
    trait=AssertThat_ [Pred,Msg=()]
    /// Asserts that `Self` satisfies the `Pred` predicate
    type=AssertThat
    /// Asserts that `Self` satisfies the `Pred` predicate
    fn_type=AssertThatOp
    /// Asserts that `This` satisfies the `Pred` predicate
    method_like=AssertThatMt
}

impl<This,Pred,Out> TypeFn_<(This,Pred)> for AssertThatOp
where 
    This:AssertThat_<Pred,Output=Out>
{
    type Output=Out;
}

// type_fn!{
//     captures(Pred,Msg=())
//     pub fn AssertThatMt[This](This)
//     where[ This:AssertThat_<Pred,Msg> ]
//     { This }
// }

impl<This,Pred,Msg> AssertThat_<Pred,Msg> for This
where
    (
        Pred,
        AssertThatHelper<Pred,This,Msg>,
        AssertEqMt<True>,
    ):TypeFn_<This>,
{
    type Output=This;
}


/// Marker type used to print that an assertion failed.
pub struct AssertionFailure<Cond,Msg>(VariantPhantom<(Cond,Msg)>);

/// The message of an assertion that failed.
pub struct Message<Msg>(VariantPhantom<Msg>);

/// The predicate and the value passed to it,
/// to evaluate the condition of  an assertion for AssertThat*.
pub struct PredicateAndValue<Pred,Val>(VariantPhantom<(Pred,Val)>);


type_fn!{
    captures(Pred,This,Msg)
    pub fn 
        AssertThatHelper(False)
        where [
            Msg:TypeFn_<()>,
        ]{ 
            AssertionFailure<
                Message<Msg::Output>,
                PredicateAndValue<Pred,This>
            >
        }
        AssertThatHelper(True){ True }
}


// #[cfg(test)]
#[cfg(all(test,feature="passed_tests"))]
mod tests{
    use super::*;
    use crate_::ops::*;
    use crate_::std_ops::*;
    use crate_::fn_adaptors::*;

    #[test]
    fn assert_eq(){
        fn check< L >()
        where
            AssertEq<L,L>:TypeIdentity<Type=L>,
            L:AssertEq_<L,Output=L>,
            AssertEqOp:TypeFn_<(L,L),Output=L>,
            AssertEqMt<L>:TypeFn_<L,Output=L>,
        {}

        check::<U0>();
        check::<U1>();
        check::<U2>();
        check::<U3>();
        check::<()>();
        check::<False>();
        check::<True>();
    }

    #[test]
    fn assert_eq_consttype(){
        fn check< L >()
        where
            AssertEqConstType<L,L>:TypeIdentity<Type=L>,
            L:AssertEqConstType_<L,Output=L>,
            AssertEqConstTypeOp:TypeFn_<(L,L),Output=L>,
            AssertEqConstTypeMt<L>:TypeFn_<L,Output=L>,
        {}

        check::<U0>();
        check::<U1>();
        check::<U2>();
        check::<U3>();
        check::<()>();
        check::<False>();
        check::<True>();
    }

    #[test]
    fn assert_func(){
        fn check< L,Func >()
        where
            AssertFunc<L,Func>:TypeIdentity<Type=L>,
            L:AssertFunc_<Func,Output=L>,
            AssertFuncOp:TypeFn_<(L,Func),Output=L>,
            AssertFuncMt<Func>:TypeFn_<L,Output=L>,
        {}

        check::<U0,Add1Op>();
        check::<U1,Add1Op>();
        check::<U2,Add1Op>();
        check::<U3,Add1Op>();
        check::<(),IdentityFn>();
        check::<False,NotOp>();
        check::<True,NotOp>();
    }

    #[test]
    fn assert_fn_ret(){
        fn check< L,Func,Ret >()
        where
            AssertFnRet<L,Func,Ret>:TypeIdentity<Type=L>,
            L:AssertFnRet_<Func,Ret,Output=L>,
            AssertFnRetOp:TypeFn_<(L,Func,Ret),Output=L>,
            AssertFnRetMt<Func,Ret>:TypeFn_<L,Output=L>,
        {}

        check::<U0,Add1Op,U1>();
        check::<U1,Add1Op,U2>();
        check::<U2,Add1Op,U3>();
        check::<U3,Add1Op,U4>();
        check::<(),IdentityFn,()>();
        check::<False,NotOp,True>();
        check::<True,NotOp,False>();
    }

    #[test]
    fn assert_that(){
        fn check< L,Pred >()
        where
            AssertThat<L,Pred>:TypeIdentity<Type=L>,
            L:AssertThat_<Pred,Output=L>,
            AssertThatOp:TypeFn_<(L,Pred),Output=L>,
            AssertThatMt<Pred>:TypeFn_<L,Output=L>,
        {}

        check::<U0,IsZeroOp>();
        check::<U1,IsOneOp >();
        check::<U2,ConstEqMt<U2>>();
        check::<U3,ConstEqMt<U3>>();
        check::<(),ConstEqMt<()>>();
        check::<False,NotOp>();
        check::<True,IdentityFn>();
    }

}
