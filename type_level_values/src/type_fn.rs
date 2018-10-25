/*!
Contains the TypeFn_ trait,for type-level-functions.

# Macros

[type_fn](../macro.type_fn.html):
    is a macro for declaring type-level-functions.



*/

////////////////////////////////////////////////////////////////////////////////////

/**

A macro for implementing TypeFn_ .

For more information please look at the 
[documentation for the TypeFn_ trait](./type_fn/trait.TypeFn_.html)


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


# Declaring a TypeFn_/method_like alias for a pre-existing trait

### Syntax for TypeFn_

```ignore

$(#[<attribute>])*
alias <function_name> 
[<self_identifier> $( ,<type_param> )*] $( ::$assoc_ty:ident )? =<trait_name> 
$( where[ <where_predicates> ] )?

```
### Syntax for Methodlike

```ignore

$(#[<attribute>])*
method_like_alias <function_name> 
[<self_identifier> $( ,<type_param> )*] $( ::$assoc_ty:ident )? =<trait_name> 
$( where[ <where_predicates> ] )?

```

method_like is a TypeFn_ which captures all parameters except for the first one,
allowing one to it in a list of functions,which act like a method chain.

Capturing parameters means that they are generic parameters to the struct representing the 
type-level function.

### Example

```
#[macro_use]
extern crate type_level_values;

use std::ops::{Add,Sub,Deref};
use type_level_values::runtime_value::ConstTypeOf_;
use type_level_values::ops::*;
use type_level_values::prelude::*;
use type_level_values::extern_types::typenum::UnsignedInteger;

type_fn!{alias AdditionOp[This,Rhs]=Add}
type_fn!{method_like_alias AdditionMt[This,Rhs]=Add}

type_fn!{alias SubsOp[This,Rhs]=Sub}
type_fn!{method_like_alias SubsMt[This,Rhs]=Sub}

type_fn!{alias DerefOp[This]::Target=Deref}

type_fn!{alias ConstTypeOfOp[This]::Type = ConstTypeOf_}

fn main(){
    let _:U10=TypeFn::<AdditionOp,(U2,U8)>::MTVAL;
    let _:U16=TypeFn::<AdditionOp,(U2,U14)>::MTVAL;
    
    let _:U10=TypeFn::<AdditionMt<U2>,U8>::MTVAL;
    let _:U16=TypeFn::<AdditionMt<U2>,U14>::MTVAL;
    

    let _:U6=TypeFn::<SubsOp,(U8,U2)>::MTVAL;
    let _:U12=TypeFn::<SubsOp,(U14,U2)>::MTVAL;
    
    let _:U6=TypeFn::<SubsMt<U2>,U8>::MTVAL;
    let _:U12=TypeFn::<SubsMt<U2>,U14>::MTVAL;
    

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

# Defining/Using a trait ,declaring a type alias and TypeFn_ for that trait 

define_trait:declares a trait,
use_trait   :does not declare a trait,

Both of them:

- Define a type alias for the trait,

- Define a TypeFn_ which delegates to the trait,

- Optionally define a method-like TypeFn_ for the trait.

### Syntax

```text

define_trait | use_trait

$( #[ <attribute_for_trait> ] )*
trait= <name_of_trait> [ $( <type_parameter_of_trait> ),* ]
$( where[ <where_predicates> ] )?

$( #[ <attribute_for_type_alias> ] )*
type= <name_of_type_alias> t
    
$( #[ <attribute_for_TypeFn_impl_block> ] )*
fn_type=$op_name:ident

$(
    $( #[ <attribute_for_method_like_TypeFn_impl_block> ] )*
    method_like=$method_like_name:ident
)?

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
    method_like=RotateMt
}

impl<This,By,Res0,Res1> Rotate_<By> for This
where
    This:Add<By,Output=Res0>,
    Res0:Rem<U16,Output=Res1>,
{
    type Output=Res1;
}



type_fn!{use_trait
    trait=Sub[Rhs]
    type=Subing
    fn_type=SubingOp
    method_like=SubingMt
}


fn main(){

    let _:U8=Rotate::<U2,U6>::MTVAL;
    let _:U1=Rotate::<U11,U6>::MTVAL;

    let _:U8=TypeFn::<RotateOp,(U2,U6)>::MTVAL;
    let _:U1=TypeFn::<RotateOp,(U11,U6)>::MTVAL;

    let _:U8=TypeFn::<RotateMt<U6>,U2>::MTVAL;
    let _:U1=TypeFn::<RotateMt<U6>,U11>::MTVAL;



    let _:U14=Subing::<U20,U6>::MTVAL;
    let _:U5=Subing::<U11,U6>::MTVAL;

    let _:U14=TypeFn::<SubingOp,(U20,U6)>::MTVAL;
    let _:U5=TypeFn::<SubingOp,(U11,U6)>::MTVAL;

    let _:U14=TypeFn::<SubingMt<U6>,U20>::MTVAL;
    let _:U5=TypeFn::<SubingMt<U6>,U11>::MTVAL;

}


```



*/
#[macro_export]
macro_rules! type_fn {
    (   $(#[$attr_op:meta])*
        alias $op_name:ident[$lhs:ident$(,$param:ident)* $(,)*] 
            $(::$assoc_ty:ident)* =$trait_name:ident
        $(where[$($bound:tt)*])*
    ) => {
        $(#[$attr_op])*
        /**
        A type-level function.Implements TypeFn<> for the trait of a similar name.
        
        To instantiate a runtime value of this function use `Type::CW`/`<Type>::CW`.
        */
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
    (   $(#[$attr_op:meta])*
        method_like_alias 
            $op_name:ident[$lhs:ident $(,$param:ident $( = $def_ty:ty )* )* $(,)* ] 
            $(::$assoc_ty:ident)* =$trait_name:ident
        $(where[$($bound:tt)*])*
    ) => {
        $(#[$attr_op])*
        /**
        A type-level function.
        
        Implements TypeFn<> for the trait of a similar name.
        
        This is defined to encourage function composition,emulating method chains.
        
        To instantiate a runtime value of this function use `Type::CW`/`<Type>::CW`.
        */
        #[allow(non_camel_case_types)]
        pub struct $op_name<$($param $(= $def_ty )* ),*>($($param),*);

        #[allow(non_camel_case_types)]
        impl<$lhs$(,$param)*> $crate::type_fn::TypeFn_<$lhs> for $op_name<$($param),*>
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
    (outer_alias_associated_type; [$($anything:tt)*] :: $assoc:ident )=>{
        $($anything)* ::$assoc
    };
    (outer_alias_associated_type; [$($anything:tt)*] )=>{
        $($anything)* ::Output
    };
    (define_trait;method_like;[$($anything:tt)*][]) => {};
    (define_trait;method_like;
        [
            params[$($params:tt)*]
            trait=$trait_name:ident
            $(::$assoc_ty:ident)*
            $(where[$($bound:tt)*])*
        ]
        [
            $(#[$attr_mt:meta])*
            method_like=$mt_name:ident
        ]
    ) => {
        type_fn!{
            $(#[$attr_mt])*
            method_like_alias $mt_name[__Self, $($params)* ] $(::$assoc_ty)* =$trait_name
            $(where[$($bound)*])*
        }
    };
    (define_trait;declaring_trait;

        $(#[$attr_trait:meta])*
        trait=$trait_name:ident[$($param:ident $(= $param_default:ty)* ),*]

        $($tokens:tt)*
    )=>{
        $(#[$attr_trait])*
        pub trait $trait_name< $($param $(= $param_default)* ),* >{
            type Output;
        }
    };
    (use_trait

        $(#[$attr_trait:meta])*
        trait=$trait_name:ident[$($param:ident $(= $param_default:ty)* ),*]$(::$assoc_ty:ident)*

        $(where[$($bound:tt)*])*

        $(#[$attr_type:meta])*
        type=$type_alias_name:ident

        $(#[$attr_op:meta])*
        fn_type=$op_name:ident
        
        $(
            $(#[$attr_mt:meta])*
            method_like=$mt_name:ident
        )*
    ) => {

        type_fn!{
            $(#[$attr_op])*
            alias $op_name[__Self $(,$param)*] $(::$assoc_ty)* =$trait_name
            $(where[$($bound)*])*
        }
        
        type_fn!{
            define_trait;method_like;
            [
                params[$($param $(= $param_default)* ),*]
                trait=$trait_name
                $(::$assoc_ty)*
                $(where[$($bound)*])*
            ]
            [$(
                $(#[$attr_mt])*
                method_like=$mt_name
            )*]
            
        }

        $(#[$attr_type])*
        ///
        /// A type alias for the trait of a similar name.
        #[allow(non_camel_case_types)]
        pub type $type_alias_name<__Self $(,$param $(= $param_default)* )* >=
            type_fn!(
                outer_alias_associated_type;
                [ <__Self as $trait_name<$($param),*>> ]
                $(::$assoc_ty)*
            );
    };
    (define_trait
        $($tokens:tt)*
    ) => {
        type_fn!{ define_trait;declaring_trait; $($tokens)* }
        type_fn!{ use_trait $($tokens)* }
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
        captures[$($bound_vars:tt)*]
        $(#[$attr:meta])*
        $(pub $(($($visibility:tt)*))*)*
        fn $op_name:ident $($rest:tt)*
    )=>{
        
        
        type_fn!{inner_struct_decl;
            captures[$($bound_vars)*]
            privacy[ $(pub $(($($visibility)*))*)* ]
            $(#[$attr])*
            #[allow(non_camel_case_types)]
            ///
            /// To instantiate a runtime value of this function use `Type::CW`/`<Type>::CW`.
            struct $op_name;
        }
    };
    (inner_struct_decl;
        captures[$($bound_vars:ident $(= $bound_def:ty )* ),*]
        privacy[pub] $(#[$attr:meta])* struct $op_name:ident;
    )=>{
        $(#[$attr])*
        #[allow(non_camel_case_types)]
        pub struct $op_name<$($bound_vars $(=$bound_def)* ,)*>(
            pub $crate::prelude::VariantPhantom<(
                $($crate::prelude::VariantPhantom<$bound_vars>,)*
            )>
        );
    };
    (inner_struct_decl;
        captures[$($bound_vars:ident $(= $bound_def:ty )* ),*]
        privacy[$($privacy:tt)*] $(#[$attr:meta])* struct $op_name:ident;
    )=>{
        #[doc(hidden)]
        $(#[$attr])*
        #[allow(non_camel_case_types)]
        pub struct $op_name<$($bound_vars $(=$bound_def)* ,)*>(
            $(pub $crate::prelude::VariantPhantom<$bound_vars>,)*
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















/////////////////////////////////////////////////////////////////////////////////////////////

/**
A type-level function.

Type-level functions is what this library calls every implementor of this trait.

# Implementations

The preferred way to implement this trait is using the type_fn macro.

# Parameters

This is the convention how TypeFn_ gets implemented 
depending on the ammount of parameters it takes:

- 0 parameters: `TypeFn_<()>`.
- 1 parameter : `TypeFn_<Param0>`.
- 2 parameters: `TypeFn_<(Param0,Param1)>`.
- 3 parameters: `TypeFn_<(Param0,Param1,Param2)>`.
- 4 parameters: `TypeFn_<(Param0,Param1,Param2,Param3)>`.
- etc

This has the slight downside that function adaptors have to account for 
functions that take 0 and 1 parameters.


### Example

Implementing a multiply add operation.

```
# #[macro_use]
# extern crate type_level_values;

use type_level_values::std_ops::{MulOp,AddOp};
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

use type_level_values::std_ops::{MulOp,AddMt};
use type_level_values::prelude::*;


type_fn!{
    pub fn MulAdd[L,R](L,R)
    where[ tlist![ MulOp, AddMt<R> ]:TypeFn_<(L,R),Output=out> ]
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
pub trait TypeFn_<Params:?Sized> {
    /// The return value of the function
    type Output;
}

type_fn!{use_trait
    trait=TypeFn_ [Params]
    /// Calls the TypeFn_ `__Self` with the `Params` function parameters.
    type=TypeFn
    /// Calls the TypeFn_ `__Self` with the `Params` function parameters.
    fn_type=TypeFnOp
    /// Calls the TypeFn_ `__Self` with the `Params` function parameters.
    method_like=TypeFnMt
}




/// Calls the TypeFn_ `Func` with `__Self` as the function parameters.
pub trait Piped_<Func>{
    type Output;
}

impl<__Self,Func> Piped_<Func> for __Self
where Func:TypeFn_<__Self>
{
    type Output=Func::Output;
}


/// Calls the TypeFn_ `Func` with `__Self` as the function parameters.
pub type Piped<__Self,Func>=TypeFn<Func,__Self>;


type_fn!{
    /// Calls the TypeFn_ `Func` with `__Self` as the function parameters.
    pub fn PipedOp[__Self,Func](__Self,Func)
    where[ Func:TypeFn_<__Self> ]
    { Func::Output }
}

