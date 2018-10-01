/**
A type-level function.

Type-level functions is what this library calls every implementor of this trait.

# Implementations

The preferred way to implement this trait is using the type_fn macro.

### Example

Implementing a multiply add operation.

```
# #[macro_use]
# extern crate type_level_values;

use type_level_values::fn_types::{MulOp,AddOp};
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

use type_level_values::fn_adaptors::ApplyRhs;
use type_level_values::fn_types::{MulOp,AddOp};
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


### Example

Emulating type constructors with a TypeFn_.

```
# #[macro_use]
# extern crate type_level_values;

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

### Example 

Implementing a function which unwraps a type-level Option<_>
or else returns the value from the `default` function.

The `default` capture gets translated to the type parameter of `UnwrapOrElse`.

```
# #[macro_use]
# extern crate type_level_values;

use type_level_values::fn_adaptors::{Const};
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

////////////////////////////////////////////////////////////////////////////////////

/**

A macro for implementing TypeFn_ .

For usage examples of declaring a new TypeFn_  please look at the 
[documentation for the TypeFn_ trait](./ops/trait.TypeFn_.html)


# Syntax for declaring a new TypeFn_

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


# Declaring a TypeFn_ alias for a pre-existing trait

# Syntax

```ignore

$(#[<attribute>])*
alias <function_name> 
[<self_identifier> $( ,<type_param> )*] $( ::$assoc_ty:ident )? =<trait_name> 
$( where[ <where_predicates> ] )?

```

### Example

```
#[macro_use]
extern crate type_level_values;

use std::ops::{Add,Deref};
use type_level_values::runtime_value::ConstTypeOf_;
use type_level_values::ops::*;
use type_level_values::prelude::*;
use type_level_values::extern_types::typenum::UnsignedInteger;

type_fn!{alias AdditionOp[This,Rhs]=Add}

type_fn!{alias DerefOp[This]::Target=Deref}

type_fn!{alias ConstTypeOfOp[This]::Type = ConstTypeOf_}

fn main(){
    let _:U10=TypeFn::<AdditionOp,(U2,U8)>::MTVAL;
    let _:U16=TypeFn::<AdditionOp,(U2,U14)>::MTVAL;
    
    let _:VariantPhantom<usize> =
        TypeFn::<DerefOp,Box<usize>>::T;
    let _:VariantPhantom<String>=
        TypeFn::<DerefOp,&'static String>::T;

    let _:VariantPhantom<BooleanType>=
        TypeFn::<ConstTypeOfOp, True >::T;
    let _:VariantPhantom<BooleanType>=
        TypeFn::<ConstTypeOfOp, False >::T;
    let _:VariantPhantom<UnsignedInteger>=
        TypeFn::<ConstTypeOfOp, U0 >::T;

}

```

# Defining a new trait ,type alias and TypeFn_ for that trait 

This is the way to define a trait for type-level values,
defining a type alias for the trait,
and defining a TypeFn_ which delegates to the trait.

### Syntax

```text

define_trait

$( #[ <attribute_for_trait> ] )*
trait= <name_of_trait> [ $( <type_parameter_of_trait> ),* ]
$( where[ <where_predicates> ] )?

$( #[ <attribute_for_type_alias> ] )*
type= <name_of_type_alias> t
    
$( #[ <attribute_for_TypeFn_impl_block> ] )*
fn_type=$op_name:ident

```

### Example

```
#[macro_use]
extern crate type_level_values;

use type_level_values::prelude::*;
use type_level_values::ops::*;
use std::ops::*;

type_fn!{define_trait
    trait=Rotate_[By]
    type=Rotate
    fn_type=RotateOp
}

impl<This,By,Res0,Res1> Rotate_<By> for This
where
    This:Add<By,Output=Res0>,
    Res0:Rem<U16,Output=Res1>,
{
    type Output=Res1;
}

fn main(){

    let _:U8=Rotate::<U2,U6>::MTVAL;
    let _:U1=Rotate::<U11,U6>::MTVAL;

    let _:U8=TypeFn::<RotateOp,(U2,U6)>::MTVAL;
    let _:U1=TypeFn::<RotateOp,(U11,U6)>::MTVAL;

}


```



*/
#[macro_export]
macro_rules! type_fn {
    (   $(#[$attr_op:meta])*
        alias $op_name:ident[$lhs:ident$(,$param:ident)*] $(::$assoc_ty:ident)* =$trait_name:ident
        $(where[$($bound:tt)*])*
    ) => {
        ///
        /// A type-level function.Implements TypeFn<> for the trait of a similar name.
        #[allow(non_camel_case_types)]
        pub struct $op_name;

        #[allow(non_camel_case_types)]
        impl<$lhs$(,$param)*> $crate::type_fn::TypeFn_<($lhs $(,$param)*)> for $op_name
        where
            $lhs:$trait_name< $($param),* >,
            type_fn!( inner_alias_associated_type; $lhs $(::$assoc_ty)* ):Sized,
            $($($bound)*)*
        {
            type Output=type_fn!( inner_alias_associated_type; $lhs $(::$assoc_ty)* );
        }
    };
    (inner_alias_associated_type; $lhs:ident :: $assoc:ident )=>{
        $lhs::$assoc
    };
    (inner_alias_associated_type; $lhs:ident )=>{
        $lhs::Output
    };
    (define_trait

        $(#[$attr_trait:meta])*
        trait=$trait_name:ident[$($param:ident),*]
        $(where[$($bound:tt)*])*

        $(#[$attr_type:meta])*
        type=$type_alias_name:ident

        $(#[$attr_op:meta])*
        fn_type=$op_name:ident
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
            $crate::type_fn::TypeFn_<($($param)*)>
        for $op_name<$($bound_vars,)*>
        where
            $($($value : $crate::reexports::TypeIdentity<Type=$variable> ,)*)*
            $($($bound)*)*
        {
            type Output=$ret;
        }
    }
}
