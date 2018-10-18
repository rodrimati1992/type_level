use prelude::*;

use crate_::field_traits::MapField;
use crate_::fn_adaptors::*;
use crate_::fn_types::*;
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

////////////////////////////////////////////////////////////////////////////////////////////

type_fn!{
    /**
    Logical or,which only evaluates the second parameter if the first parameter is False.

    Equivalent to
    ```ignore
    |lhs:impl()->bool, rhs:bool|{
        if lhs { true }else{ rhs() }
    }
    ```
    */
    pub fn LogicalOr[Rhs](True ,Rhs){ True }

    LogicalOr[Rhs](False,Rhs)
    where[
        Rhs:TypeFn_<()>,
        Rhs::Output:Boolean,
    ]{ Rhs::Output }
}

type_fn!{
    /**
    Logical and,which only evaluates the second parameter if the first parameter is True.

    Equivalent to
    ```ignore
    |lhs:bool, rhs:impl()->bool|{
        if lhs { rhs() }else{ false }
    }
    ```
    */
    pub fn LogicalAnd[Rhs](False ,Rhs){ False }

    LogicalAnd[Rhs](True,Rhs)
    where[
        Rhs:TypeFn_<()>,
        Rhs::Output:Boolean,
    ]{ Rhs::Output }
}

///////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

}
