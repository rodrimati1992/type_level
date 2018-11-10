/**
A macro for defining a Mutator Function.

Mutator Function is a TypeFn_ which can mutate a type's ConstValue-parameter.

# Kinds of Mutator Function

There are 2 different kinds of Mutator Function:

- Regular:
    can be called on the type specified in `type This[T,I]=(Foo<T,I>)` ,

- Extension:
    declared by not specifying `type This[..]=(..)`,
    can be called on types that allow it with `impl AllowMutatorFn<Func> for This {}`.



# Syntax

`$( ... )*` means repeated 0 or more times.

`$( ... )+` means repeated 1 or more times.

`$( ... )?` means that this is optional.

 `< ... >` is a variable,replaced with whatever it refers to.

```text

$(
    // The type whose ConstValue parameter we are mutating.
    type This[ <generic_parameters> ]=( <This_type> )

    // the where clause containing with the constraints for 
    // `impl AllowMutatorFn< <function_name> > for <This_type>`.
    $( where[ <where_predicates> ] )?
)?


/**
The classes of type containing self allowed in MutConstParam methods.

They are a combination of pass self by reference,mutable reference,and value.

The values of this associated type are `allowed_self_constructors::{All,ByRef,ByMut,ByVal}`

*/
type AllowedSelf=( <ConstAllowedConversions> );

<syntax_for_declaring_function>

```

`<syntax_for_declaring_function>` is defined
[in the `type_fn` macro](./macro.type_fn.html#syntax-for-declaring-a-new-typefn_)


# Example of a Mutator Function.

This Mutator Function is used to freeze the constents of the wrapper.

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
            wrapper.mutparam_mut(Freeze::NEW,().ty_());
        
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

#[derive(MutConstValue)]
#[mcv( Type="MutWrapper", ConstValue="Mut" )]
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

mutator_fn!{
    type This[T,I]=( MutWrapper<T,I> )
    type AllowedSelf=( allowed_self_constructors::All )

    pub fn Freeze[I](I,()){ Immutable }
}

```



### Example of a Mutator Function.

Here we implement a channel which can only be called a limited ammount of times 
before it is closed,producing a compile-time error if one tries to call recv/send again.

This is also an example about how a Mutator Function can be private.

```


# #[macro_use]
# extern crate type_level_values;
# #[macro_use]
# extern crate derive_type_level;

# use type_level_values::prelude::*;
use type_level_values::ops::{If,ConstLEMt,Sub1Op};
use type_level_values::std_ops::*;
use type_level_values::fn_adaptors::Const;

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

    #[derive(MutConstValue)]
    #[mcv(
        Type = "ChannelEnd", ConstValue= "S"
    )]
    pub struct ChannelEndInner<Chan, S: WrapperTrait> {
        channel: Chan,
        #[allow(dead_code)]
        state: ConstWrapperFromTrait<S>,
    }

    pub type Sender<T, S> = ChannelEnd<MPSCSender<T>, S>;
    pub type Receiver<T, S> = ChannelEnd<MPSCReceiver<T>, S>;


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
            self.mutparam(TransferValue::NEW, Default::default())
        }
    }

    impl<T: Send, L> Receiver<T, L> {
        pub fn recv<__NextSelf>(self) -> (__NextSelf, T)
        where Self: MCPBounds<TransferValue, (), NextSelf = __NextSelf>,
        {
            let ret = self.channel.recv().unwrap();
            (self.mutparam(TransferValue::NEW, Default::default()), ret)
        }
    }

    type_fn!{
        pub fn NewOpen[v](v){ Open<v> }
    }

    mutator_fn!{
        type This[T,S]=( ChannelEnd<T,S> )
        type AllowedSelf=( allowed_self_constructors::ByVal )

        fn TransferValue[I](I,())
        where [
            I:OpenTrait,
            I::remaining : Piped_<
                If<ConstLEMt<U1>,
                    Const<Closed>,
                    (Sub1Op,NewOpen),
                >,Output=Out
            >,
        ]
        {
            let Out;
            Out
        }
    }
}



```



### Example of an extension Mutator Function

This Mutator Function simply returns back the input constant `I`.

```
# #[macro_use]
# extern crate type_level_values;
# use type_level_values::prelude::*;


mutator_fn!{
    type AllowedSelf=( allowed_self_constructors::All )

    pub fn MutConstIdentity[I](I,()){ I }
}

# fn main(){}

```

# Adapting a pre-existing fucntion

To adapt a pre-existing function to be a mutator function you must declare a mutator_fn 
using type_fn's delegation syntax and use an adaptor function from user_traits as appropriate.

The adaptor functions by function arity:

- If the function takes 1 parameter:use user_traits::AdaptUnary.

- If the function takes 2 parameter there is no need to adapt it.

- If the function takes 3 or more parameters:use user_traits::AdaptFn.


All of the examples here are for extension Mutator Functions,
it would be the same if the This type was specified.

### Example of delegating to a unary function

```
# #[macro_use]
# extern crate type_level_values;
# use type_level_values::prelude::*;

use type_level_values::std_ops::*;
use type_level_values::user_traits::AdaptUnary;

mutator_fn!{
    type AllowedSelf=( allowed_self_constructors::All )

    pub fn MyAdd1=AdaptUnary<Add1Op>;
}

# fn main(){}

```

### Example of delegating to a binary function

```
# #[macro_use]
# extern crate type_level_values;
# use type_level_values::prelude::*;

use type_level_values::std_ops::*;
use type_level_values::user_traits::AdaptUnary;

mutator_fn!{
    type AllowedSelf=( allowed_self_constructors::All )

    pub fn MySub=SubOp;
}

# fn main(){}

```

### Example of delegating to a ternary(and beyond) function.

```
# #[macro_use]
# extern crate type_level_values;
# use type_level_values::prelude::*;

use type_level_values::std_ops::*;
use type_level_values::user_traits::AdaptUnary;

type_fn!{
    pub fn MulThenAdd[A,B,C](A,B,C)
    where[ A:Piped<(AddMt<B>,MulMt<C>),Output=Out> ]
    {let Out;Out}
}

mutator_fn!{
    type AllowedSelf=( allowed_self_constructors::All )

    pub fn My=AdaptFn<MulThenAdd>;
}

# fn main(){}

```


*/
#[macro_export]
macro_rules! mutator_fn {
    (
        $(
            type This[ $($self_gens:tt)* ]=( $this:ty )
            $(where [$($self_where:tt)*] )*
        )*

        type AllowedSelf=( $allowed_self:ty )

        $(#[$attr_above:meta])*
        $(
            captures($($bound_vars:ident $(= $bound_def:ty )* ),*)
            $(#[$attr_bellow:meta])*
        )*
        $(pub $(($($visibility:tt)*))*)*
        fn $fn_ident:ident $($rest:tt)+
    )=>{
        type_fn!{
            $( #[$attr_above] )*
            $(
                captures($($bound_vars $(= $bound_def )* ),*)
                $(#[$attr_bellow])*
            )*
            $(pub $(($($visibility)*))*)*
            fn $fn_ident $($rest)+
        }
        impl<$($($bound_vars,)*)*> 
            $crate::user_traits::MutatorFnAttrs 
        for $fn_ident< $($($bound_vars,)*)* > 
        {
            type AllowedSelf=$allowed_self;
        }

        mutator_fn!{
            inner-1;
            type This[$($($self_gens)*)*]=( $($this)* )
            $($(where [$($self_where)*] )*)*
            captures( $($($bound_vars),*)* )
            fn $fn_ident
        }

    };
    (inner-1;
        type This[]=()
        captures( $($bound_vars:ident)* )
        fn $fn_ident:ident
    )=>{};
    (inner-1;
        type This[ $($self_gens:tt)* ]=( $this:ty )
        $(where [$($self_where:tt)*] )*
        captures( $($bound_vars:ident),* )
        fn $fn_ident:ident
    )=>{
        impl< $($self_gens)* , $($bound_vars,)*> 
            $crate::user_traits::AllowMutatorFn<$fn_ident< $($($bound_vars,)*)* >> 
        for $this
        where $( $($self_where)* )*
        {}
    };
}

