use prelude::*;
use std_::ops::{
    Add as Std_Add, BitAnd as Std_BitAnd, BitOr as Std_BitOr, BitXor as Std_BitXor, Div as Std_Div,
    Index as Std_Index, IndexMut as Std_IndexMut, Mul as Std_Mul, Neg as Std_Neg, Not as Std_Not,
    Rem as Std_Rem, Shl as Std_Shl, Shr as Std_Shr, Sub as Std_Sub,
};

use crate_::field_traits::{MapFieldOp, SetField_};
use crate_::ops::{ConstFrom_, ConstInto_, Insert_,ConstNE_};

use crate_::std_types::cmp_ordering::{Equal_, Greater_, Less_};

use typenum::consts::U0;

/**
A type-level function.

Type-level functions is what this library calls every implementor of this trait.

# Implementations

The preferred way to implement this trait is using the type_fn macro.

# Example

Implementing a multiply add operation.

```
# #[macro_use]
# extern crate type_level_values;

use type_level_values::ops::{TypeFn,TypeFn_};
use type_level_values::ops::fn_types::{MulOp,AddOp};
use type_level_values::prelude::*;


use std::ops::{Mul,Add};

type_fn!{
    pub fn MulAdd[L,R](L,R)
    where[
        L:Mul<R,Output=res0>,
        res0:Add<R,Output=res1>,
    ]{
        let res0;
        let res1;
        res1
    }
}

fn main(){
    let _:U20=TypeFn::<MulAdd,(U4,U4)>::MTVAL;
    let _:U50=TypeFn::<MulAdd,(U4,U10)>::MTVAL;
}


```

A more advanced version,using function composition

```

# #[macro_use]
# extern crate type_level_values;

use type_level_values::ops::{TypeFn,TypeFn_};
use type_level_values::ops::fn_adaptors::ApplyRhs;
use type_level_values::ops::fn_types::{MulOp,AddOp};
use type_level_values::prelude::*;


type_fn!{
    pub fn MulAdd[L,R](L,R)
    where[ tlist![ MulOp, ApplyRhs<AddOp,R> ]:TypeFn_<(L,R),Output=out> ]
    { let out;out }
}

fn main(){
    let _:U20=TypeFn::<MulAdd,(U4,U4)>::MTVAL;
    let _:U50=TypeFn::<MulAdd,(U4,U10)>::MTVAL;
}

```


# Example

Emulating type constructors with a TypeFn_.

```
# #[macro_use]
# extern crate type_level_values;

use type_level_values::ops::{TypeFn,TypeFn_};
use type_level_values::prelude::*;


use std::collections::{BTreeSet,BTreeMap};

type_fn!{pub fn VecFn[T](T){ Vec<T> }}

type_fn!{pub fn BTreeSetFn[T](T){ BTreeSet<T> }}

type_fn!{pub fn BTreeMapFn[K,V](K,V){ BTreeMap<K,V> }}

fn main(){
    let _:TypeFn<VecFn,usize> =
        Vec::<usize>::new();

    let _:TypeFn<BTreeSetFn,usize> =
        BTreeSet::<usize>::new();

    let _:TypeFn<BTreeMapFn,(String,usize)> =
        BTreeMap::<String,usize>::new();
}

```

# Example 

Implementing a function which unwraps a type-level Option<_>
or else returns the value from the `default` function.

The `default` capture gets translated to the type parameter of `UnwrapOrElse`.

```
# #[macro_use]
# extern crate type_level_values;

use type_level_values::ops::{TypeFn,TypeFn_};
use type_level_values::ops::fn_adaptors::{Const};
use type_level_values::prelude::*;



type_fn!{
    captures(default)

    pub fn
        UnwrapOrElse[T](Some_<T>)
        { T }
        
        UnwrapOrElse(None_)
        where[ default:TypeFn_<()> ]
        { default::Output }

}

fn main(){
    let _:U10=TypeFn::<UnwrapOrElse<Const<U0>>, Some_<U10>>::MTVAL;
    let _:U5 =TypeFn::<UnwrapOrElse<Const<U5>>, None_     >::MTVAL;
}

```



*/
pub trait TypeFn_<Params> {
    /// The return value of the function
    type Output;
}

/// Calls the TypeFn_ `This` with the `Params` function parameters.
pub type TypeFn<This, Params> = <This as TypeFn_<Params>>::Output;

/// The ConstType of a TypeFn_
#[derive(Debug, Copy, Clone, Default)]
pub struct TypeFnType;

impl ConstType for TypeFnType {}

////////////////////////////////////////////////////////////////////////////////////

/**

A macro for declaring a struct which implements TypeFn_ .

For usage examples please look at the 
[documentation for the TypeFn_ trait](./ops/trait.TypeFn_.html)

# Syntax

`$( ... )*` means repeated 0 or more times.

`$( ... )+` means repeated 1 or more times.

`$( ... )?` means that this is optional.

 `< ... >` is a syntactic variable,replaced with whatever it refers to.

```text


$( captures( $( <captured_variable> ,)* ) )?

$( <visibility_specifier> )? fn
$(
    <function_name> $( [ <generic_params> ] )? ( $( <function_parameter:ty> ,)* )
    $( where [ <where_predicates> ] )
    {
        $( let <variable_name> $( = <type> )? ; )*
        <returned_type>
    }
)+

```

All the captures inside `captures(...)` get translated to type parameters on the 
generated struct.

The `<visibility_specifier>` gets translated to the visibility of the constructor for the 
generated struct.

*/
#[macro_export]
macro_rules! type_fn {
    (   $(#[$attr_op:meta])*
        alias $op_name:ident[$lhs:ident$(,$param:ident)*]=$trait_name:ident
        $(where[$($bound:tt)*])*
    ) => {
        ///
        /// A type-level function.Implements TypeFn<> for the trait of a similar name.
        #[allow(non_camel_case_types)]
        pub struct $op_name;

        #[allow(non_camel_case_types)]
        impl<$lhs$(,$param)*> $crate::ops::TypeFn_<($lhs $(,$param)*)> for $op_name
        where $lhs:$trait_name< $($param),* >,
              $lhs::Output:Sized,
              $($($bound)*)*
        {
            type Output=$lhs::Output;
        }
    };
    (define-trait
        $(#[$attr_op:meta])*
        fn_type=$op_name:ident
        $(#[$attr_trait:meta])*
        trait=$trait_name:ident[$($param:ident),*]
        $(where[$($bound:tt)*])*
        $(#[$attr_type:meta])*
        type=$type_alias_name:ident
    ) => {

        type_fn!{
            $(#[$attr_op])*
            alias $op_name[__Self $(,$param)*]=$trait_name
            $(where[$($bound)*])*
        }

        $(#[$attr_trait])*
        ///
        /// A type-level function.
        pub trait $trait_name< $($param),* >{
            type Output;
        }

        $(#[$attr_type])*
        ///
        /// A type-level function.Type alias for the trait of a similar name.
        #[allow(non_camel_case_types)]
        pub type $type_alias_name<__Self $(,$param)* >=
            <__Self as $trait_name<$($param),*>>::Output;
    };
    (
        $(#[$attr_above:meta])*
        $(captures($($bound_vars:ident $(= $bound_def:ty )* ),*)
            $(#[$attr_bellow:meta])*
        )*
        $(pub $(($($visibility:tt)*))*)*
        fn $($rest:tt)+
    )=>{
        type_fn!{inner-function-decl-struct;
            captures[$($($bound_vars $(= $bound_def )* ),*)*]
            $(#[$attr_above])*
            $($(#[$attr_bellow])*)*
            $(pub $(($($visibility)*))*)*
            fn $($rest)+
        }

        type_fn!{inner_function_decl0;
            captures[ ($($($bound_vars),*)*) ]
            fn $($rest)+
        }

    };
    (inner-function-decl-struct;
        captures[$($bound_vars:ident $(= $bound_def:ty )* ),*]
        $(#[$attr:meta])*
        $(pub $(($($visibility:tt)*))*)*
        fn $op_name:ident $($rest:tt)*
    )=>{
        $(#[$attr])*
        #[allow(non_camel_case_types)]
        $(pub $(($($visibility)*))*)*
        struct $op_name<$($bound_vars $(=$bound_def)* ,)*>(
            $(pub $bound_vars,)*
        );
    };
    (inner_function_decl0;
        captures[ $bound_vars:tt ]
        fn
        $(
            $op_name:ident
            $([$($param_type:tt)*])*
            ($($param:tt)*)
            $(where[$($bound:tt)*])*
            {
                $(let $variable:ident $(=$value:ty)*;)*
                $ret:ty
            }
        )+
    )=>{
        $(
            type_fn!{inner-function-decl1;
                captures[$bound_vars]
                $op_name
                [$($($param_type)*)*]
                ($($param)*)
                $(where[$($bound)*])*
                {
                    $(let $variable $(=$value)*;)*
                    $ret
                }
            }
        )+
    };

    (inner-function-decl1;
        captures[ $bound_vars:tt ]
        $op_name:ident
        []
        $($rest:tt)*
    )=>{
        type_fn!{inner-function-decl2;
            captures[$bound_vars]
            $op_name
            []
            $($rest)*
        }
    };
    (inner-function-decl1;
        captures[ $bound_vars:tt ]
        $op_name:ident
        [$($param_type:tt)*]
        $($rest:tt)*
    )=>{
        type_fn!{inner-function-decl2;
            captures[$bound_vars]
            $op_name
            [$($param_type)* , ]
            $($rest)*
        }
    };
    (inner-function-decl2;
        captures[ ($($bound_vars:ident),*) ]
        $op_name:ident
        [$($param_type:tt)*]
        ($($param:tt)*)
        $(where[$($bound:tt)*])*
        {
            $(let $variable:ident $(=$value:ty)*;)*
            $ret:ty
        }
    )=>{
        #[allow(non_camel_case_types)]
        impl< $($param_type)* $($bound_vars,)* $($variable,)*  >
            $crate::ops::TypeFn_<($($param)*)>
        for $op_name<$($bound_vars,)*>
        where
            $($($value : $crate::reexports::TypeIdentity<Type=$variable> ,)*)*
            $($($bound)*)*
        {
            type Output=$ret;
        }
    }
}

/// These are types that represent operators,unary and binary.
///
/// Every type here also implements the TypeFn_ trait so as to be usable in contexts
/// where the operator is generic.
pub mod fn_types {
    use super::fn_adaptors::*;
    use super::*;
    use crate_::ops::iteration_ops::*;

    type_fn!{alias AddOp      [A,B]=Std_Add}
    type_fn!{alias BitAndOp   [A,B]=Std_BitAnd}
    type_fn!{alias BitXorOp   [A,B]=Std_BitXor}
    type_fn!{alias BitOrOp    [A,B]=Std_BitOr}
    type_fn!{alias DivOp      [A,B]=Std_Div}
    type_fn!{alias IndexOp    [A,B]=Std_Index}
    type_fn!{alias IndexMutOp [A,B]=Std_IndexMut}
    type_fn!{alias MulOp      [A,B]=Std_Mul}
    type_fn!{alias NegOp      [A]  =Std_Neg}
    type_fn!{alias NotOp      [A]  =Std_Not}
    type_fn!{alias RemOp      [A,B]=Std_Rem}
    type_fn!{alias ShlOp      [A,B]=Std_Shl}
    type_fn!{alias ShrOp      [A,B]=Std_Shr}
    type_fn!{alias SubOp      [A,B]=Std_Sub}
    type_fn!{alias ConstOrdOp [A,B]=ConstOrd_}
    type_fn!{alias ConstEqOp  [A,B]=ConstEq_}
    type_fn!{alias ConstNEOp  [A,B]=ConstNE_}
    type_fn!{alias ConstFromOp[A,B]=ConstFrom_}
    type_fn!{alias ConstIntoOp[A,B]=ConstInto_}

    type_fn!{
        /// Const less-than.
        pub fn ConstLtOp[L,R](L,R)
        where[
            L:ConstOrd_<R>,
            L::Output:ConstEq_<Less_,Output=Out>,
            Out:Boolean,
        ]{ let Out;Out }
    }

    type_fn!{
        /// Const less-than-or-equal.
        pub fn ConstLEOp[L,R](L,R)
        where[
            L:ConstOrd_<R>,
            IsLessOrEqual:TypeFn_<L::Output,Output=Out>,
        ]{ let Out;Out }
    }

    type_fn!{
        /// Const greater-than.
        pub fn ConstGtOp[L,R](L,R)
        where[
            L:ConstOrd_<R>,
            L::Output:ConstEq_<Greater_,Output=Out>,
            Out:Boolean,
        ]{ let Out;Out }
    }

    type_fn!{
        /// Const greater-than-or-equal.
        pub fn ConstGEOp[L,R](L,R)
        where[
            L:ConstOrd_<R>,
            IsGreaterOrEqual:TypeFn_<L::Output,Output=Out>,
        ]{ let Out;Out }
    }

    use self::_helper::*;

    mod _helper {
        use super::*;
        type_fn!{
            pub fn IsLessOrEqual(Less_ ){True}
                   IsLessOrEqual(Equal_){True}
                   IsLessOrEqual(Greater_){False}
        }
        type_fn!{
            pub fn IsGreaterOrEqual(Less_ ){False}
                   IsGreaterOrEqual(Equal_){True}
                   IsGreaterOrEqual(Greater_){True}
        }
    }

}

/// Operator adaptors for TypeFn_ implementors.
///
/// Contains types for converting operators to a different arity(amount of parameters),
/// transforming one input to an operator before applying it,etc.
pub mod fn_adaptors {
    use super::*;


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

}
