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

$( #[ <attribute> ] )*

$( captures( $( <captured_variable> ,)* ) )?

$( #[ <attribute> ] )*

$( <visibility_specifier> )? fn
$(
    <function_name> $( [ <generic_params> ] )? ( $( <function_parameter:ty> ,)* )
    $( where [ <where_predicates> ] )?
    {
        $( let <variable_name> $( = <type> )? ; )*
        <returned_type>
    }
)+

```

All the attributes get applied to the generated `struct function_name`.

All the captures inside `captures(...)` get translated to type parameters on the 
generated struct.

The generated struct is always `pub`,with a #[doc(hidden)] attribute if 
\<visibility_specifier\> is not `pub`.
The reason it is always `pub` is because it
causes a compiletime error to call a private TypeFn_ in a public function.

### Example

This is an example of declaring a TypeFn_ that uses all the syntax in the macro.

```

#[macro_use]
extern crate type_level_values;

use type_level_values::ops::AssertEq;
use type_level_values::std_ops::*;
use type_level_values::prelude::*;

type_fn!{
    /// This doc-comment gets applied to the generated `struct MulThenAdd<L,R>(...)`,
    /// like every other attribute.
    #[derive(Debug)] // This derive attribute gets applied to the MulThenAdd struct.
    captures( L,R )
    /// This doc-comment also gets applied to the generated `struct MulThenAdd<L,R>(...)`,
    /// after the one above captures.
    pub(crate) fn MulThenAdd[Added](Added)
    where[
        (L,R):Piped_<PipedFunction,Output=Out>,
    ]{
        let PipedFunction=(
            MulOp,
            AddMt<Added>,
        );
        let Out;
        Out
    }

}


fn main(){
    let _:AssertEq< TypeFn< MulThenAdd<U2,U0> , U10 >,U10 >;
    let _:AssertEq< TypeFn< MulThenAdd<U2,U1> , U10 >,U12 >;
    let _:AssertEq< TypeFn< MulThenAdd<U2,U2> , U10 >,U14 >;
    let _:AssertEq< TypeFn< MulThenAdd<U2,U3> , U10 >,U16 >;
    let _:AssertEq< TypeFn< MulThenAdd<U2,U4> , U10 >,U18 >;
    let _:AssertEq< TypeFn< MulThenAdd<U2,U5> , U10 >,U20 >;

    let _:AssertEq< TypeFn< MulThenAdd<U3,U0> , U20 >,U20 >;
    let _:AssertEq< TypeFn< MulThenAdd<U3,U1> , U20 >,U23 >;
    let _:AssertEq< TypeFn< MulThenAdd<U3,U2> , U20 >,U26 >;
    let _:AssertEq< TypeFn< MulThenAdd<U3,U3> , U20 >,U29 >;
    let _:AssertEq< TypeFn< MulThenAdd<U3,U4> , U20 >,U32 >;
}


```

# Declaring an function delegating to another TypeFn_

### Syntax

```text
$( #[ <attribute> ] )*

$( captures( $( <captured_variable> ,)* ) )?

$( #[ <attribute> ] )*

$( <visibility_specifier> )? fn`<function_name> = <function_type>
```

Where \<function_type\> can be any type that implements TypeFn_

### Example

```

#[macro_use]
extern crate type_level_values;

use type_level_values::ops::*;
use type_level_values::std_ops::*;
use type_level_values::fn_adaptors::*;
use type_level_values::prelude::*;

type_fn!{
    /// This function is equivalent to calling AddOp.
    pub fn MyAdd1 = AddOp;
}

type_fn!{
    /// This function only takes 1 parameter.
    captures(L)
    pub fn MyAdd2 = ApplyLhs<AddOp,L>;
}

type_fn!{
    /// This function ignores all function parameters.
    captures(L,R)
    pub fn MyAdd3 = Lazy<AddOp,(L,R)>;
}


fn main(){
    let _:AssertEq< TypeFn<MyAdd1,(U0,U2)>,U2 >;
    let _:AssertEq< TypeFn<MyAdd1,(U2,U2)>,U4 >;


    let _:AssertEq< TypeFn<MyAdd2<U2>,U0>,U2 >;
    let _:AssertEq< TypeFn<MyAdd2<U2>,U2>,U4 >;

    
    //  It ignores the parameters 
    let _:AssertEq< TypeFn<MyAdd3<U0,U0>,U2 >,U0 >;
    let _:AssertEq< TypeFn<MyAdd3<U0,U0>,U10>,U0 >;
    
    let _:AssertEq< TypeFn<MyAdd3<U2,U0>,U2 >,U2 >;
    let _:AssertEq< TypeFn<MyAdd3<U2,U0>,U10>,U2 >;

    let _:AssertEq< TypeFn<MyAdd3<U2,U2>,U2 >,U4 >;
    let _:AssertEq< TypeFn<MyAdd3<U2,U2>,U10>,U4 >;



}


```


# Declaring a TypeFn_/method_like alias for a pre-existing trait

### Syntax for creating a TypeFn_ alias of a pre-existing trait

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
allowing one to use it in a list of functions,acting like a method chain.

Captures are generic parameters to the struct representing the type-level function.

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

# Defining/Using a trait ,declaring a type alias and a TypeFn_ for that trait 

define_trait:declares a trait.

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
#[macro_export(local_inner_macros)]
macro_rules! type_fn {
    (   $(#[$attr_op:meta])*
        alias $op_name:ident[$lhs:ident$(,$param:ident)* $(,)*]
            $(::$assoc_ty:ident)* =$trait_name:ident
        $(where[$($bound:tt)*])*
    ) => {
        $(#[$attr_op])*
        /**
A type-level function.

Implements TypeFn<> for the trait of a similar name.
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
        */
        #[allow(non_camel_case_types)]
        pub struct $op_name<$($param:?Sized $(= $def_ty )* ),*>(
            pub $crate::prelude::VariantPhantom<(
                $(
                    $crate::prelude::VariantPhantom<$param>,
                )*
            )>
        );

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
    (// declares a new function,aliasing for a pre-existing function
        $(#[$attr_above:meta])*
        $(captures($($bound_vars:ident $(= $bound_def:ty )* ),*)
            $(#[$attr_bellow:meta])*
        )*
        $(pub $(($($visibility:tt)*))*)*
        fn $fn_name:ident=$equals:ty;
    )=>{
        type_fn!{
            $(#[$attr_above])*
            $(
                captures($($bound_vars $(= $bound_def )* ),*)
                $(#[$attr_bellow])*
            )*

            $(pub $(($($visibility:tt)*))*)*
            fn $fn_name[Params](Params)
            where[ $equals:$crate::type_fn::TypeFn_<Params,Output=Out> ]
            { let Out; Out }
        }
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
            struct $op_name;
        }


    };
    (inner_struct_decl;
        captures[$($bound_vars:ident $(= $bound_def:ty )* ),*]
        privacy[pub] $(#[$attr:meta])* struct $op_name:ident;
    )=>{
        type_fn!{
            inner_struct_decl;shared;
            captures[$($bound_vars $(=$bound_def)* ,)*]
            privacy[pub]
            $(#[$attr])*
            #[allow(non_camel_case_types)]
            struct $op_name;
        }

        impl<$($bound_vars,)*> Default for $op_name<$($bound_vars,)*> {
            #[inline]
            fn default()->Self{
                $op_name($crate::std_::marker::PhantomData)
            }
        }
    };
    (inner_struct_decl;
        captures[$($bound_vars:ident $(= $bound_def:ty )* ),*]
        privacy[$($privacy:tt)*] $(#[$attr:meta])* struct $op_name:ident;
    )=>{
        type_fn!{
            inner_struct_decl;shared;
            captures[$($bound_vars $(=$bound_def)* ),*]
            privacy[$($privacy)*]
            $(#[$attr])*
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            struct $op_name;
        }
    };
    (inner_struct_decl;shared;
        captures[$($bound_vars:ident $(= $bound_def:ty )* ),* $(,)* ]
        privacy[$($privacy:tt)*] $(#[$attr:meta])* struct $op_name:ident;
    )=>{
        $(#[$attr])*
        pub struct $op_name<$($bound_vars:?Sized $(=$bound_def)* ,)*>(
            pub $crate::prelude::VariantPhantom<(
                $(
                    $crate::prelude::VariantPhantom<$bound_vars>,
                )*
            )>

        );

        impl<$($bound_vars,)*> $op_name<$($bound_vars,)*> {
            #[allow(dead_code)]
            $($privacy)* const NEW:Self=$op_name($crate::std_::marker::PhantomData);
        }
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
