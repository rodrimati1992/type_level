use prelude::*;

use crate_::field_traits::MapField;
use crate_::fn_adaptors::*;
use crate_::ops::*;
use crate_::std_ops::*;
use crate_::std_types::cmp_ordering::{Equal_, Greater_, Less_};

use std_::ops::Add;

/// Allows lazily evaluating a function,
///
/// Equivalent to `|_| function(params) `,where params can be any ammount of parameters.
pub struct Lazy<Function, Params>(Function, Params);

impl<Function, Params, _0> TypeFn_<_0> for Lazy<Function, Params>
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

    # Example

    Implementing a safe division function.

    ```
    # #[macro_use]
    # extern crate type_level_values;

    # use type_level_values::prelude::*;
    use type_level_values::ops::*;
    use type_level_values::std_ops::*;
    use type_level_values::fn_adaptors::*;

    type SafeDiv=If<(GetRhs,IsZeroOp),GetLhs,DivOp>;

    fn main(){
        let _:AssertEq<TypeFn<SafeDiv,(U10,U0)>,U10>;
        let _:AssertEq<TypeFn<SafeDiv,(U10,U1)>,U10>;
        let _:AssertEq<TypeFn<SafeDiv,(U10,U2)>,U5>;
        let _:AssertEq<TypeFn<SafeDiv,(U10,U3)>,U3>;
        let _:AssertEq<TypeFn<SafeDiv,(U10,U4)>,U2>;
        let _:AssertEq<TypeFn<SafeDiv,(U10,U5)>,U2>;
        let _:AssertEq<TypeFn<SafeDiv,(U10,U6)>,U1>;
    }

    ```
    */
    pub fn If[state](state)
    where [
        Pred:TypeFn_<state>,
        IfHelper<state,Then,Else>:TypeFn_<Pred::Output,Output=out>,
    ]{ let out;out }
}

///////////////////////////////////////////////////////////////////////////////////////////

type_fn!{
    captures(Msg)
    /**
    Immediately causes a compile-time error with the `Msg` message.

    # Example

    Implementing a division function,which panics if the denominator is 0.

    ```
    # #[macro_use]
    # extern crate type_level_values;

    # use type_level_values::prelude::*;
    use type_level_values::ops::*;
    use type_level_values::std_ops::*;
    use type_level_values::fn_adaptors::*;

    struct attempted_to_divide_by_zero;

    type PanicDiv=If<(GetRhs,IsZeroOp),(GetLhs,Panic<attempted_to_divide_by_zero>),DivOp>;

    fn main(){
        // This causes a compile_time error
        //let _:AssertEq<TypeFn<PanicDiv,(U10,U0)>,U10>;
        let _:AssertEq<TypeFn<PanicDiv,(U10,U1)>,U10>;
        let _:AssertEq<TypeFn<PanicDiv,(U10,U2)>,U5>;
        let _:AssertEq<TypeFn<PanicDiv,(U10,U3)>,U3>;
        let _:AssertEq<TypeFn<PanicDiv,(U10,U4)>,U2>;
        let _:AssertEq<TypeFn<PanicDiv,(U10,U5)>,U2>;
        let _:AssertEq<TypeFn<PanicDiv,(U10,U6)>,U1>;
    }

    ```

    # Example of unconditioal panic

    ```compile_fail
    # #[macro_use]
    # extern crate type_level_values;

    # use type_level_values::prelude::*;
    use type_level_values::ops::*;

    # fn main(){

    struct explicit_panic;

    let _:TypeFn<Panic<explicit_panic>,()>;

    # }

    */
    pub fn Panic[_0](_0)
    where[ Panicking<Msg>:TypeIdentity<Type= IsPanicking > ]
    { () }
}

#[doc(hidden)]
pub struct Panicking<T>(T);
#[doc(hidden)]
pub struct IsPanicking;

///////////////////////////////////////////////////////////////////////////////////////////

#[cfg(all(test, feature = "passed_tests"))]
// #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_() {
        type Cond0 = If<ConstGEOp, SubOp, (GetLhs, Add1Op)>;
        let _: AssertPipedRet<(U5, U20), Cond0, U6>;
        let _: AssertPipedRet<(U20, U5), Cond0, U15>;

        struct Yep;
        struct Nope;
        type Cond1 = If<ConstEqOp, Const<Yep>, Const<Nope>>;
        let _: AssertPipedRet<(U5, U5), Cond1, Yep>;
        let _: AssertPipedRet<(U20, U5), Cond1, Nope>;
    }

    #[test]
    fn lazy() {
        // This tests that the constraints of the function don't get
        // evaluated when constructing Lazy.
        let _: Lazy<AddOp, ()>;
        let _: Lazy<SubOp, ()>;
        let _: Lazy<ConstEqOp, ()>;

        type Test<Func, Params> = (AssertEq<TypeFn<Func, Params>, TypeFn<Lazy<Func, Params>, ()>>,);

        let _: Test<AddOp, (U10, U5)>;
        let _: Test<SubOp, (U10, U5)>;
        let _: Test<ConstEqOp, ((), ())>;
        let _: Test<ConstEqOp, (U0, U0)>;
        let _: Test<ConstEqOp, (U10, U10)>;
    }

}
