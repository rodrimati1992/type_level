/*!
Operator adaptors for TypeFn_ implementors.



*/

use prelude::*;

use crate_::field_traits::{MapFieldOp};
use crate_::collection_ops::{Insert_};

type_fn!{
    captures(Op,Rhs)
    /// Type-level version of "|x|Op(x,Rhs)"
    pub fn ApplyRhs[Lhs](Lhs)
    where [ Op: TypeFn_<(Lhs, Rhs)> ]
    { Op::Output }
}

type_fn!{
    captures(Op,Lhs)
    /// Type-level version of "|x|Op(Lhs,x)"
    pub fn ApplyLhs[Rhs](Rhs)
    where [ Op: TypeFn_<(Lhs, Rhs)> ]
    { Op::Output }
}

type_fn!{
    /// Type-level version of |l,r| func(r,l)
    captures(Func)
    pub fn Flip[L,R](L,R)
    where[ Func:TypeFn_<(R,L)> ]
    {
        Func::Output
    }
}

type_fn!{
    /// Applies a parameter of a TypeFn_< SomeTuple > ,
    /// reducing the arity of the resulting TypeFn_<> by 1.
    /// 
    /// This only works with functions that take at least 3 parameters.
    /// 
    captures(Op,Nth,Value)
    pub fn ApplyNth[Input](Input)
    where[
        Input:Insert_<Nth,Value>,
        Op:TypeFn_< Input::Output >
    ]{ Op::Output }
}

type_fn!{
    /// Applies every parameter to Op except for the nth,creating a unary function
    /// that takes that parameter and evaluates Op.
    /// 
    /// This only works with functions that take at least 3 parameters.
    /// 
    captures(Op,Nth,Value)
    pub fn ApplyNonNth[Input](Input)
    where[
        Value:Insert_<Nth,Input>,
        Op:TypeFn_< Value::Output >,
    ]{ Op::Output }
}

/**
Applies every parameter except the self parameter,which is by convention the first.

This only works with functions that take at least 2 parameters other than Self.

# Example

```
# #[macro_use]
# extern crate derive_type_level;

# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;

use type_level_values::field_traits::{SetField,SetFieldOp};
use type_level_values::fn_adaptors::ApplyNonSelf;

#[derive(TypeLevel)]
#[typelevel(reexport(Struct,Traits))]
pub struct Rectangle{
    pub x:u32,
    pub y:u32,
    pub w:u32,
    pub h:u32,
}
use self::type_level_Rectangle::fields;

type InitialRectangle=SetField<
    Rectangle_Uninit,
    fields::All,
    U0
>;


type SetX<X>=ApplyNonSelf<SetFieldOp,(fields::x,X)>;

type SetY<Y>=ApplyNonSelf<SetFieldOp,(fields::y,Y)>;

type SetW<W>=ApplyNonSelf<SetFieldOp,(fields::w,W)>;

type SetH<H>=ApplyNonSelf<SetFieldOp,(fields::h,H)>;


fn main(){
    let _:ConstRectangle<U0,U0,U0,U0>=InitialRectangle::MTVAL;

    let _:ConstRectangle<U5,U10,U20,U0>=TypeFn::<
        (SetX<U5>,SetY<U10>,SetW<U20>),
        InitialRectangle
    >::MTVAL;

    let _:ConstRectangle<U0,U0,U1024,U128>=TypeFn::<
        (SetW<U1024>,SetH<U128>),
        InitialRectangle,
    >::MTVAL;
}

```


*/
pub type ApplyNonSelf<Op, Params> = ApplyNonNth<Op, U0, Params>;

/**
Applies the Self parameter for a function,which is by convention the first.

This only works with functions that take at least 2 parameters other than Self.
*/
pub type ApplySelf<Op, This> = ApplyNth<Op, U0, This>;

type_fn!{
    captures(Op, Mapper)
    /// Type-level version of "|l,r|Op(Mapper(l),r)"
    pub fn MapLhs[Lhs, Rhs](Lhs, Rhs)
    where[
        Mapper: TypeFn_<Lhs, Output = Res0>,
        Op: TypeFn_<(Res0, Rhs)>,
    ]{
        let Res0;
        Op::Output
    }
}

type_fn!{
    captures(Op, Mapper)
    /// Type-level version of "|l,r|Op(l,Mapper(r))"
    pub fn MapRhs[Lhs, Rhs](Lhs, Rhs)
    where[
        Mapper: TypeFn_<Rhs, Output = Res0>,
        Op: TypeFn_<(Lhs, Res0)>,
    ]{
        let Res0;
        Op::Output
    }
}

type_fn!{
    captures(Op, Nth, Mapper)
    /// Maps the nth parameter using Mapper and then passes it to Op.
    ///
    /// Note:This does not work with unary functions because they don't use tuples.
    pub fn MapNth[Params](Params)
    where[
        MapFieldOp: TypeFn_<(Params, Nth, Mapper), Output = Res0>,
        Op: TypeFn_<Res0>,
    ]{
        let Res0;
        Op::Output
    }
}

type_fn!{
    /// Type-level version of "|l,r| r(l) "
    pub fn EvalRhsOp[Lhs, Rhs](Lhs, Rhs)
    where[ Rhs: TypeFn_<Lhs> ]
    { Rhs::Output }
}

type_fn!{
    /// Type-level version of "|l,r| l(r) "
    pub fn EvalLhsOp[Lhs, Rhs](Lhs, Rhs)
    where[ Lhs: TypeFn_<Rhs> ]
    { Lhs::Output }
}

type_fn!{
    /// Type-level version of "|l,_|l"
    pub fn GetLhs[L,R](L,R){ L }
}
type_fn!{
    /// Type-level version of "|_,r|r"
    pub fn GetRhs[L,R](L,R){ R }
}

type_fn!{
    captures(Value)
    /// Type-level version of "|_| Value ".
    pub fn Const[Params](Params){ Value }
}

type_fn!{
    captures(T)

    /// Ignores the captured variable,acting like an TypeFn_ identity function.
    pub fn Ignoring[Params](Params){ Params }
}

/// Ignores `First`,returning `Second` .
pub type IgnoreFirst<First, Second> = TypeFn<Ignoring<First>, Second>;

type_fn!{
    /// Type-level version of "|x| x ".
    pub fn IdentityFn[P](P){P}
}
