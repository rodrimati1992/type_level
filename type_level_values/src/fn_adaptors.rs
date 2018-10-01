/*!
Operator adaptors for TypeFn_ implementors.

Contains types for converting operators to a different arity(amount of parameters),
transforming one input to an operator before applying it,etc.
*/

use prelude::*;

use crate_::field_traits::{MapFieldOp};
use crate_::ops::{Insert_};

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
    /// Applies a parameter of a TypeFn_< SomeTuple > ,
    /// reducing the arity of the resulting TypeFn_<> by 1.
    ///
    /// # Example
    ///
    /// FoldLOp is a TypeFn<(Collection,Default,Op)>
    ///
    /// type PartialA=ApplyNth<FoldLOp,U2,IgnoreFirst>;
    ///
    /// PartialA impls TypeFn<(Collection,Default)>
    ///
    ///
    /// type PartialB=ApplyNth<FoldLOp,U1,True>;
    ///
    /// PartialB impls TypeFn<(Collection,Op)>
    ///
    ///
    /// type PartialC=ApplyNth<FoldLOp,U0,(U100,U30,U50)>;
    ///
    /// PartialC impls TypeFn<(Default,Op)>
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
    captures(Op,Nth,Value)
    pub fn ApplyNonNth[Input](Input)
    where[
        Value:Insert_<Nth,Input>,
        Op:TypeFn_< Value::Output >,
    ]{ Op::Output }
}

/// Applies every parameter except the self parameter,which is by convention the first.
pub type ApplyNonSelf<Op, Params> = ApplyNonNth<Op, U0, Params>;

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
    pub fn ReturnLhs[L,R](L,R){ L }
}
type_fn!{
    /// Type-level version of "|_,r|r"
    pub fn ReturnRhs[L,R](L,R){ R }
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

type_fn!{
    captures(F)
    /// A type-level version of "|x| f((x,)) "
    pub fn TupledIn[Input](Input)
    where[ F: TypeFn_<(Input,)>, ]
    { F::Output }
}