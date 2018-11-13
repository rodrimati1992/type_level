/*!
Operator adaptors for TypeFn_ implementors.



*/

use prelude::*;

use crate_::field_traits::{MapFieldOp};
use crate_::collection_ops::{Insert_,PushFrontMt,PushBackMt};

pub use crate_::type_fn::{
    TypeFn_,
    TypeFn,
    TypeFnMt,
    Piped_,
    Piped,
    PipedOp,
};

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
    /// Applies a parameter of a function (3+ params).
    /// 
    /// For unary functions use ops::Lazy.
    /// For binary functions use either ApplyRhs or ApplyLhs
    captures(Op,Nth,Value)
    pub fn ApplyNth[Input](Input)
    where[
        Input:Insert_<Nth,Value>,
        Op:TypeFn_< Input::Output >
    ]{ Op::Output }
}

type_fn!{
    /// Applies every parameter to a TypeFn_ (with 3 or more parameters)
    /// except for the nth,creating a unary function
    /// that takes that parameter and evaluates Op.
    /// 
    /// For unary functions use ops::Lazy.
    /// For binary functions use either ApplyRhs or ApplyLhs
    captures(Op,Nth,Value)
    pub fn ApplyNonNth[Input](Input)
    where[
        Value:Insert_<Nth,Input>,
        Op:TypeFn_< Value::Output >,
    ]{ Op::Output }
}



type_fn!{
    /**
    Applies every parameter of a function (3+ params) except the self parameter,
    which is by convention the first.

    This only works with functions that take at least 2 parameters other than Self.

    For functions taking 1 parameter other than Self use ApplyRhs.


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
    captures(Op, Params)
    pub fn ApplyNonSelf[self_](self_)
    where[ (PushFrontMt<self_>,Op):TypeFn_< Params,Output=Out > ]
    {  
        let Out;
        Out
    }
}

type_fn!{
    /**
    Applies the Self parameter of a function (3+ params),which is by convention the first.

    This only works with functions that take at least 2 parameters other than Self.

    For functions taking 1 parameter other than Self use ApplyLhs.
    */
    captures(Op, self_)
    pub fn ApplySelf[Params](Params)
    where[ (PushFrontMt<self_>,Op):TypeFn_< Params,Output=Out > ]
    {  
        let Out;
        Out
    }
}


type_fn!{
    /**
    Applies the last parameter of a function (3+ params).

    This only works with functions that take at least 3 parameters.

    For unary functions use ops::Lazy.
    For functions taking 2 parameter use ApplyRhs.
    */
    captures(Op, last)
    pub fn ApplyLast[Params](Params)
    where[ (PushBackMt<last>,Op):TypeFn_< Params,Output=Out > ]
    {  
        let Out;
        Out
    }
}


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
    /// Maps the nth parameter of a function(2+ params) using Mapper and then passes it to Op.
    ///
    /// For unary functions use `(MapOperation,TheUnaryFunction)` instead.
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



#[cfg(test)]
mod tests{
    use super::*;
    use crate_::ops::{AssertFnRet,AssertEq};
    use crate_::field_traits::{SetFieldOp};
    use crate_::std_ops::{AddOp,DivOp,SubOp};

    macro_rules! define_apply_test {
        ( $test_ident:ident , $test_type_alias:item ) => (
            $test_type_alias
            
            let _:$test_ident<(U0,U1,U2,U3),U0,False,(False,U1,U2,U3)>;
            let _:$test_ident<(U0,U1,U2,U3),U1,False,(U0,False,U2,U3)>;
            let _:$test_ident<(U0,U1,U2,U3),U2,False,(U0,U1,False,U3)>;
            let _:$test_ident<(U0,U1,U2,U3),U3,False,(U0,U1,U2,False)>;
        )
    }

    #[test]
    fn flip(){
        let _:AssertFnRet<(U3,U10),Flip<SubOp>,U7>;
        let _:AssertFnRet<(U2,U10),Flip<DivOp>,U5>;
    }

    #[test]
    fn apply_non_self(){
        define_apply_test!{
            Test,
            type Test<This,Field,Value,Expected>=(
                AssertEq<TypeFn< ApplyNonSelf<SetFieldOp,(Field,Value)> , This >, Expected >,
            );
        }
    }

    #[test]
    fn apply_nth(){
        define_apply_test!{
            Test,
            type Test<This,Field,Value,Expected>=(
                AssertEq<TypeFn< ApplyNth<SetFieldOp,U0,This > , (Field,Value) >, Expected >,
                AssertEq<TypeFn< ApplyNth<SetFieldOp,U1,Field> , (This,Value) >, Expected >,
                AssertEq<TypeFn< ApplyNth<SetFieldOp,U2,Value> , (This,Field) >, Expected >,
            );
        }
    }

    #[test]
    fn apply_self(){
        define_apply_test!{
            Test,
            type Test<This,Field,Value,Expected>=(
                AssertEq<TypeFn< ApplySelf<SetFieldOp,This> , (Field,Value) >, Expected >,
            );
        }
    }

    #[test]
    fn apply_last(){
        define_apply_test!{
            Test,
            type Test<This,Field,Value,Expected>=(
                AssertEq<TypeFn< ApplyLast<SetFieldOp,Value> , (This,Field) >, Expected >,
            );
        }
    }

    #[test]
    fn map_param(){
        type Test<Func,Mapper,Params,ExpectedMapLhs,ExpectedMapRhs>=(
            AssertFnRet<Params,MapLhs<Func,Mapper> , ExpectedMapLhs >,
            AssertFnRet<Params,MapRhs<Func,Mapper> , ExpectedMapRhs >,
            
            AssertFnRet<Params,MapNth<Func,U0,Mapper> , ExpectedMapLhs >,
            AssertFnRet<Params,MapNth<Func,U1,Mapper> , ExpectedMapRhs >,
        );

        let _:Test<DivOp,Const<U1  >,(U10,U10),U0,U10>;
        let _:Test<DivOp,Const<U8 >,(U16,U4 ),U2,U2 >;
        let _:Test<DivOp,Const<U100>,(U200,U200 ),U0,U2 >;
    }
    
}