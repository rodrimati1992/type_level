/**
A macro for defining a ConstMethod.

ConstMethod is a type which is used to implement an operation which mutates 
a type's ConstValue-parameter.

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

# Example of a Const-method.

This ConstMethod is used to freeze the constents of the wrapper.

```

# #[macro_use]
# extern crate derive_type_level;
# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;

fn main(){
    let mut wrapper:MutWrapper<String,Mutable>=
        MutWrapper::new("what".to_string(),Mutable);

    {
        let inner:&mut MutWrapper<String,Immutable>=
            wrapper.mutparam_mut(Freeze,Default::default());
        
        // The next line doesn't compile.
        // inner.push_str(" is");

        assert_eq!(&**inner,"what");
    }

    wrapper.push_str(" is");
    assert_eq!(&*wrapper,"what is");

}

use std::ops::{Deref,DerefMut};

#[derive(TypeLevel)]
#[typelevel(reexport(Variants))]
pub enum Mutability{
    Mutable,
    Immutable,
}

#[derive(ConstConstructor)]
#[cconstructor( Type="MutWrapper", ConstParam="Mut" )]
pub struct MutWrapperInner<T,Mut>{
    _mutability:ConstWrapper<Mut>,
    value:T,
}

impl<T,Mut> MutWrapper<T,Mut>{
    pub fn new(value:T,_mutability:Mut)->Self{
        Self{ value , _mutability:ConstWrapper::NEW }
    }
}

impl<T,Mut> Deref for MutWrapper<T,Mut>{
    type Target=T;
    fn deref(&self)->&T{
        &self.value
    }
}

impl<T> DerefMut for MutWrapper<T,Mutable>{
    fn deref_mut(&mut self)->&mut T{
        &mut self.value
    }
}

const_method!{
    type ConstConstructor[T]=( MutWrapperCC<T> )
    type AllowedConversions=( allowed_conversions::All )

    pub fn Freeze[I](I,()){ Immutable }
}

```



# Example of a Const-method.

Here we implement a channel which can only be called a limited ammount of times 
before it is closed,producing a compile-time error if one tries to call recv/send again.

This is also an example about how a ConstMethod can be private.

```


# #[macro_use]
# extern crate type_level_values;
# #[macro_use]
# extern crate derive_type_level;

# use type_level_values::prelude::*;
use type_level_values::ops::IfEager;

use std::ops::Sub;
use std::sync::mpsc::{self, Receiver as MPSCReceiver, RecvError, SendError, Sender as MPSCSender};

fn main () {
    let (tx, rx) = channel::<&'static str, U2>();

    #[allow(unused_variables)]
    let tx: Sender<_, Closed> = tx.send("hello").send("hello");

    let (rx, val) = rx.recv();
    println!("{}", val);
    let (rx, val) = rx.recv();
    println!("{}", val);
    let _:Receiver<_,Closed>=rx;
}


use self::bounded_channel::*;
pub mod bounded_channel{
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, TypeLevel)]
    #[typelevel(reexport(Variants,Traits))]
    pub enum State {
        Open { remaining: u64 },
        Closed,
    }


    #[derive(ConstConstructor)]
    #[cconstructor(Type = "ChannelEnd", ConstParam = "S")]
    pub struct ChannelEndInner<Chan, S: WrapperTrait> {
        channel: Chan,
        #[allow(dead_code)]
        state: ConstWrapperFromTrait<S>,
    }

    pub type Sender<T, S> = ChannelEnd<MPSCSender<T>, S>;
    pub type Receiver<T, S> = ChannelEnd<MPSCReceiver<T>, S>;

    pub type SenderCC<T> = ChannelEndCC<MPSCSender<T>>;
    pub type ReceiverCC<T> = ChannelEndCC<MPSCReceiver<T>>;

    pub fn channel<T: Send, L>() -> (Sender<T, Open<L>>, Receiver<T, Open<L>>){
        let (tx, rx) = mpsc::channel();
        (
            Sender   { channel: tx, state: ConstWrapper::NEW },
            Receiver { channel: rx, state: ConstWrapper::NEW },
        )
    }

    impl<T: Send, L> Sender<T, L> {
        pub fn send<__NextSelf>(self, value: T) -> __NextSelf
        where Self: MCPBounds<TransferValue, (), NextSelf = __NextSelf>,
        {
            self.channel.send(value).unwrap();
            self.mutparam(TransferValue::new(), Default::default())
        }
    }

    impl<T: Send, L> Receiver<T, L> {
        pub fn recv<__NextSelf>(self) -> (__NextSelf, T)
        where Self: MCPBounds<TransferValue, (), NextSelf = __NextSelf>,
        {
            let ret = self.channel.recv().unwrap();
            (self.mutparam(TransferValue::new(), Default::default()), ret)
        }
    }

    const_method!{
        type ConstConstructor[T]=( ChannelEndCC<T> )
        type AllowedConversions=( allowed_conversions::ByVal )

        fn TransferValue[I](I,())
        where [
            I:OpenTrait,
            I::remaining :Sub<U1,Output=var0>+ConstEq_<U1,Output=is_1>,
            is_1:Boolean,
            IfEager< is_1, Closed, Open<var0> >:TypeFn_<(),Output=var1>
        ]
        {
            let var0;let is_1;let var1;
            var1
        }
    }
}



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

        impl $crate::user_traits::const_methods::ConstMethod_Extension for $op_name{}

        impl $crate::user_traits::const_methods::ExtensionConstMethod for $op_name{
            type MethodKind=
                $crate::user_traits::const_methods::type_level_ExtensionMethodKind::Extension;
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
