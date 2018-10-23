use prelude::*;

use crate_::field_traits::MapField;
use crate_::ops::*;
use crate_::fn_adaptors::*;
use crate_::std_ops::*;
use crate_::std_types::cmp_ordering::{Equal_, Greater_, Less_};

use std_::ops::Add;

/// Allows lazily evaluating a function,
///
/// Equivalent to creating an `impl Fn()->T`.
pub struct Lazy<Function, Params>(Function, Params);

impl<Function, Params> TypeFn_<()> for Lazy<Function, Params>
where
    Function: TypeFn_<Params>,
{
    type Output = Function::Output;
}

////////////////////////////////////////////////////////////////////////////////////////////

type_fn!{
    captures(state,Then,Else)
    #[doc(hidden)]
    pub fn IfHelper(True )where[Then:TypeFn_<state>,]{ Then::Output }
           IfHelper(False)where[Else:TypeFn_<state>,]{ Else::Output }
}

type_fn!{
    captures(Pred,Then,Else=IdentityFn)
    /**
    An if expression that takes lazily evaluated Then and Else branches,
    only evaluating the branch that was taken.

    Equivalent to
    ```text
    for<State> | state:State |{
        if Pred(state) {
            Then(state)
        }else{
            Else(state)
        }
    }
    ```
    */
    pub fn If[state](state)
    where [
        Pred:TypeFn_<state>,
        IfHelper<state,Then,Else>:TypeFn_<Pred::Output,Output=out>,
    ]{ let out;out }
}


////////////////////////////////////////////////////////////////////////////////////////////

type_fn!{
    captures(Then,Else)
    #[doc(hidden)]
    pub fn IfEagerHelper(True ){ Then }
           IfEagerHelper(False){ Else }
}

type_fn!{
    captures(Cond,Then,Else)
    /// An if expression eagerly evaluates both branches.
    ///
    /// Equivalent to `||{ if Cond { Then }else{ Else } }`
    pub fn IfEager(())
    where [
        IfEagerHelper<Then,Else>:TypeFn_<Cond,Output=out>,
    ]{ let out;out }
}

///////////////////////////////////////////////////////////////////////////////////////////

// #[cfg(all(test,feature="passed_tests"))]
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn if_(){
        type Cond0=If<ConstGEOp,SubOp,(GetLhs,Add1Op)>;
        let _:AssertFnRet<(U5,U20),Cond0,U6>;
        let _:AssertFnRet<(U20,U5),Cond0,U15>;

        struct Yep;
        struct Nope;
        type Cond1=If<ConstEqOp,Const<Yep>,Const<Nope>>;
        let _:AssertFnRet<(U5,U5),Cond1,Yep>;
        let _:AssertFnRet<(U20,U5),Cond1,Nope>;
    }

    #[test]
    fn if_eager(){
        type Cond0<L,R>=IfEager<ConstGE<L,R>,TypeFn<SatSubOp,(L,R)>,Add1<L>>;
        let _:AssertFnRet<(),Cond0<U5,U20>,U6>;
        let _:AssertFnRet<(),Cond0<U20,U5>,U15>;

        struct Yep;
        struct Nope;
        type Cond1<L,R>=IfEager<ConstEq<L,R>,Yep,Nope>;
        let _:AssertFnRet<(),Cond1<U5,U5>,Yep>;
        let _:AssertFnRet<(),Cond1<U20,U5>,Nope>;
    }

    #[test]
    fn lazy(){
        // This tests that the constraints of the function don't get 
        // evaluated when constructing Lazy.
        let _:Lazy<AddOp,()>;
        let _:Lazy<SubOp,()>;
        let _:Lazy<ConstEqOp,()>;



        type Test<Func,Params>=(
            AssertEq<
                TypeFn<Func,Params>,
                TypeFn<Lazy<Func,Params>,()>,
            >,
        );

        let _:Test<AddOp,(U10,U5)>;
        let _:Test<SubOp,(U10,U5)>;
        let _:Test<ConstEqOp,((),())>;
        let _:Test<ConstEqOp,(U0,U0)>;
        let _:Test<ConstEqOp,(U10,U10)>;

    }

}
