/*!
Some example TypeFn_,alongside examples of how to use them with MutConstParam
*/

/**
Replaces the current constant with the one passed as a parameter,
using `this.mutparam(ReplaceConstValueOp::NEW, NewConstant::T )`
*/

use prelude::*;

use fn_adaptors::{ApplySelf,GetRhs};


mutator_fn!{
    type AllowedSelf=(allowed_self_constructors::All)

    /**
    A function which ignores the current ConstValue parameter,
    returning the second parameter.

    Equivalent to `|current,new| new`.
    */
    pub fn ReplaceWithParamFn=GetRhs;
}


mutator_fn!{
    type AllowedSelf=(allowed_self_constructors::All)

    /**
    Adapts a unary function to take the current ConstValue as a parameter.

    Binary functions don't need to be adapted.

    For adapting functions with 3 or more parameters use AdaptFn.
    */
    captures(Func)
    pub fn AdaptUnary[I,_ignored](I,_ignored)
    where[ Func:TypeFn_<I,Output=Out> ]
    { let Out;Out }
}


mutator_fn!{
    type AllowedSelf=(allowed_self_constructors::All)

    /**
    Adapts functions taking 3 or more parameters to take 
    the current ConstValue and a tuple containing the remaining parameters.

    For adapting unary functions use AdaptUnary.

    Binary functions don't need to be adapted.
    */
    captures(Func)
    pub fn AdaptFn[I,Rem](I,Rem)
    where[ ApplySelf<Func,I>:TypeFn_<Rem,Output=Out> ]
    { let Out;Out }
}

