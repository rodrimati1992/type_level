/**
A macro for defining a ConstMethod.

ConstMethod is a type which is used to implement an operation on a type's Const-parameter.

# Kinds of ConstMethod

There are 3 different kinds of ConstMethod:

- Built-in:can be called on any type,declared only on the type_level_values crate.

- Extension:can be called only on types that enable extension ConstMethods.

- Inherent:can be called on a type that derived ConstConstructor ,
 where the ConstMethod was declared in the same crate,
 and where the ConstConstructor of the type is the one passed to the const_method macro.


# Syntax

`$( ... )*` means repeated 0 or more times.

`$( ... )+` means repeated 1 or more times.

`$( ... )?` means that this is optional.

 `< ... >` is a variable,replaced with whatever it refers to.

```text

// The ConstConstructor of a type,obtainable with GetConstConstructor.
type ConstConstructor[ <generic_parameters> ]=( <ConstConstructor> )

// the where clause of the ConstConstructor struct itself.
$( where[ <where_predicates> ] )?

// An instance of user_traits::allowed_conversions::ConstAllowedConversions,
// generally from one of the aliases in the user_traits::allowed_conversions module.
type AllowedConversions=( <ConstAllowedConversions> );

// Whether this ConstMethod is an extension-method.
// Note that extension-methods have to be enabled (as a whole)
// by a ConstConstructor to be usable.
// Defaults to false.
$(extension_method= true/false ;)?

// Whether this is a safe ConstMethod or not.
// Defaults to safe.
// The safety of the ConstMethod is reflected in its constructor.
$(safety= safe/unsafe ;)?

// attributes for the struct representing the ConstMethod
$( #[<attribute>] )*

// visibility of the struct representing the ConstMethod
$( <visibility_specifier> )?
fn
$(
    <function_name> $( [ <generic_params> ] )? ( <current_constant_type> , <Msg_type> )
    $( where [ <where_predicates> ] )
    {
        /// The Variables declared here are generic types declared in the impl header.
        $( let <variable_name> $( = <type> )? ; )*
        <returned_type>
    }
)+

```

# Example of an extension method

This ConstMethod simply returns back the input constant `I`.

```
# #[macro_use]
# extern crate type_level_values;
# use type_level_values::prelude::*;


const_method!{
    type ConstConstructor[]=( T )
    type AllowedConversions=( allowed_conversions::All )

    extension_method=true;

    pub fn MutConstIdentity[I](I,()){ I }
}

# fn main(){}

```

# Example of a setter method from the example `_05_capabilities`.

This ConstMethod disables a capability,making is impossible to call the methods
related to that capability on the returned value.

```ignore

# #[macro_use]
# extern crate type_level_values;
# use type_level_values::prelude::*;


const_method!{
    type ConstConstructor[FS,EC]=( SideEffectfulCC<FS,EC> )
    type AllowedConversions=( allowed_conversions::All )

    pub fn DisableCapability[Caps,Field](Caps,Field)
    where [ Caps:SetField_<Field,DisabledCap> ]
    {Caps::Output}
}

# fn main(){}

```

# Example of a more complex method from the example `_06_channel`.

This ConstMethod counts down the number of remaining values to send over the channel
and when it reaches 0 it closes the channel.

```ignore

# #[macro_use]
# extern crate type_level_values;
# use type_level_values::prelude::*;


const_method!{
    type ConstConstructor[T]=( ChannelEndCC<T> )
    type AllowedConversions=( allowed_conversions::ByVal )

    fn TransferValue[I](I,())
    where [
        I:OpenTrait,
        I::remaining:Sub<U1,Output=var0>+ConstEq_<U1>,
        IfEager<ConstEq<I::remaining,U1>,
            Closed,
            Open<var0>
        >:TypeFn_<(),Output=var1>
    ]
    {
        let var0;let var1;
        var1
    }
}

# fn main(){}

```

*/
#[macro_export]
macro_rules! const_method {
    (
        type ConstConstructor[$($self_params:tt)*]=($this_cc:ty)
        $(where[ $($self_swhere:tt)* ])*

        type AllowedConversions=($conversions:ty)

        $(extension_method=$is_extension_method:tt;)*
        $(safety=$safety:tt;)*

        $(#[$attr:meta])*
        $(pub $(($($visibility:tt)*))*)*
        fn $op_name:ident $($rest:tt)+

    )=>{

        const_method!{inner_struct;
            safety=[$($safety)*]
            $(#[$attr])*
            #[derive(Debug)]
            [ $(pub $(($($visibility)*))*)* ]
            fn $op_name
        }

        const_method!{inner_extension_method;
            extension_method=[$($is_extension_method)*]
            op_name=[$op_name]
            this_cc=[$this_cc]
            self_params=[$($self_params)*]
            self_swhere=[$($($self_swhere)*)*]
        }

        impl $crate::user_traits::const_methods::OpAttrs for $op_name{
            type Conversions=$conversions;
        }

        const_method!{inner-fn;
            fn $op_name $($rest)*
        }

    };
    (inner_extension_method;extension_method=[true]$($rest:tt)*)=>{
        const_method!{inner_extension_method;extension_method=[True]$($rest)*}
    };
    (inner_extension_method;
        extension_method=[True]
        op_name=[$op_name:ident]
        $($rest:tt)*
    )=>{
        impl $crate::user_traits::const_methods::ConstMethod for $op_name{}

        impl $crate::user_traits::const_methods::ConstMethod_RegularExt for $op_name{}

        impl $crate::user_traits::const_methods::ExtensionConstMethod for $op_name{
            type MethodKind=
                $crate::user_traits::const_methods::type_level_ExtensionMethodKind::RegularExt;
        }
    };
    (inner_extension_method;
        extension_method=[internal_blanket_impl]
        op_name=[$op_name:ident]
        $($rest:tt)*
    )=>{
        impl $crate::user_traits::Sealed for $op_name{}

        impl $crate::user_traits::const_methods::ConstMethod for $op_name{}

        impl $crate::user_traits::const_methods::ExtensionConstMethod for $op_name{
            type MethodKind=
                $crate::user_traits::const_methods::type_level_ExtensionMethodKind::Builtin;
        }

        impl $crate::user_traits::const_methods::ConstMethod_BuiltinExt for $op_name{}
    };
    (inner_extension_method;extension_method=[False]$($rest:tt)*)=>{
        const_method!{inner_extension_method;extension_method=[]$($rest)*}
    };
    (inner_extension_method;extension_method=[false]$($rest:tt)*)=>{
        const_method!{inner_extension_method;extension_method=[]$($rest)*}
    };
    (inner_extension_method;
        extension_method=[]
        op_name=[$op_name:ident]
        this_cc=[$this_cc:ty]
        self_params=[ $($self_params:tt)* ]
        self_swhere=[ $($self_swhere:tt)* ]
    )=>{
        impl $crate::user_traits::const_methods::ConstMethod for $op_name{}

        impl<$($self_params)*>
            $crate::user_traits::const_traits::AllowOp <$op_name>
        for $this_cc
        where $($self_swhere)*
        {}
    };
    (inner_extension_method;extension_method=[$($tt:tt)*] $($rest:tt)*)=>{
        compile_error!{" extension_method= <value> ; <value> must be one of True/true/False/False"}
    };
    (struct_decl;
        safety[unsafe] privacy[$($privacy:tt)*]$(#[$attr:meta])* struct $op_name:ident;
    )=>{
        $(#[$attr])*
        pub struct $op_name{
            _dummy:()
        }
    };
    (struct_decl;safety[$(safe)*] privacy[pub] $(#[$attr:meta])* struct $op_name:ident;)=>{
        $(#[$attr])*
        pub struct $op_name;
    };
    (struct_decl;
        safety[$(safe)*] privacy[$($privacy:tt)*] $(#[$attr:meta])* struct $op_name:ident;
    )=>{
        $(#[$attr])*
        pub struct $op_name{
            _dummy:()
        }
    };
    (constructor_with;safety[unsafe] privacy[$($privacy:tt)*] )=>{
        $($privacy)* unsafe fn new()->Self {
            Self{ _dummy:() }
        }
    };
    (constructor_with;safety[$(safe)*] privacy[pub] )=>{
        pub unsafe fn new()->Self {
            Self{}
        }
    };
    (constructor_with;safety[$(safe)*] privacy[$($privacy:tt)*] )=>{
        $($privacy)* fn new()->Self {
            Self{ _dummy:() }
        }
    };
    (inner_struct;
        safety=[$($safety:tt)*] $(#[$attr:meta])* [$($visibility:tt)*] fn $op_name:ident
    )=>{
        const_method!{struct_decl;
            safety[$($safety)*]
            privacy[ $($visibility)* ]
            $(#[$attr])*
            struct $op_name;
        }

        impl $op_name{
            const_method!{constructor_with;
                safety[$($safety)*]
                privacy[ $($visibility)* ]
            }
        }
    };
    (inner-fn;
        fn
        $(
            $op_name:ident $([$($param_type:tt)*])* ($const_:ty,$msg:ty)
            $(where[$($bound:tt)*])*
            {
                $(let $variable:ident $(=$value:ty)*;)*
                $ret:ty
            }
        )+
    )=>{
        $(
            #[allow(non_camel_case_types)]
            impl<$($($param_type)* , )* $($variable,)* >
                $crate::user_traits::const_methods::ComputeConstParam_ <$const_,$msg>
            for $op_name
            where
                $($($value : $crate::reexports::TypeIdentity<Type=$variable> ,)*)*
                $($($bound)*)*
            {
                type Output=$ret;
            }

        )*
    };
}
