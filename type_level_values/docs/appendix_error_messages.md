/*!

This appendix is about how to read compiler error messages.

# Guidelines

The recommended approach to reading error messages is to read the error message in this order of importance:

- Read the "main" error message,if the types in the error message are not too long.

- Read the "help" if it's printed.

- Read the first note.

# Example

Say that we make the mistake of attempting to construct a Point with the wrong field accessor.

```ignore

#[macro_use]
extern crate type_level_values;
#[macro_use]
extern crate derive_type_level;

use type_level_values::prelude::*;
use type_level_values::values_prelude::*;

#[derive(TypeLevel)]
#[typelevel(reexport(Struct))]
struct Point {
    x:u32,
    y:u32,
}

use self::type_level_Point::fields;

fn main(){
    let _:construct!{Point_Uninit=>
        fields::x = U0,
        fields::y = U1,
        ()=U2,
    }=MarkerType::MTVAL;
}

```

This is the error message produced by the previous code example:

```text
error[E0277]: the trait bound `type_level_Point::ConstPoint<type_level_values::typenum::UTerm, type_level_values::typenum::UInt<type_level_values::typenum::UTerm, type_level_values::values_prelude::B1>>: type_level_values::field_traits::SetField_<(), type_level_values::typenum::UInt<type_level_values::typenum::UInt<type_level_values::typenum::UTerm, type_level_values::values_prelude::B1>, type_level_values::values_prelude::B0>>` is not satisfied
  --> src/../docs/reading_error_messages.md:33:11
   |
21 |       let _:construct!{Point_Uninit=>
   |  ___________^
22 | |         fields::x = U0,
23 | |         fields::y = U1,
24 | |         ()=U2,
25 | |     }=MarkerType::MTVAL;
   | |_____^ the trait `type_level_values::field_traits::SetField_<(), type_level_values::typenum::UInt<type_level_values::typenum::UInt<type_level_values::typenum::UTerm, type_level_values::values_prelude::B1>, type_level_values::values_prelude::B0>>` is not implemented for `type_level_Point::ConstPoint<type_level_values::typenum::UTerm, type_level_values::typenum::UInt<type_level_values::typenum::UTerm, type_level_values::values_prelude::B1>>`
   |
   = help: the following implementations were found:
             <type_level_Point::ConstPoint<x, y> as type_level_values::field_traits::SetField_<type_level_Point::__fields::x, NewValue>>
             <type_level_Point::ConstPoint<x, y> as type_level_values::field_traits::SetField_<type_level_Point::__fields::y, NewValue>>
             <type_level_Point::ConstPoint<x, y> as type_level_values::field_traits::SetField_<type_level_Point::__fields::All, NewValue>>
   = note: required because of the requirements on the impl of `type_level_values::TypeFn_<(type_level_Point::ConstPoint<type_level_values::typenum::UTerm, type_level_values::typenum::UInt<type_level_values::typenum::UTerm, type_level_values::values_prelude::B1>>, ((), type_level_values::typenum::UInt<type_level_values::typenum::UInt<type_level_values::typenum::UTerm, type_level_values::values_prelude::B1>, type_level_values::values_prelude::B0>))>` for `type_level_values::field_traits::SetFieldValuePair`
   = note: required because of the requirements on the impl of `type_level_values::ops::FoldL_<type_level_Point::ConstPoint<type_level_values::field_traits::UninitField<type_level_Point::__fields::x>, type_level_values::field_traits::UninitField<type_level_Point::__fields::y>>, type_level_values::field_traits::SetFieldValuePair>` for `type_level_values::new_types::TList<(type_level_Point::__fields::x, type_level_values::typenum::UTerm), type_level_values::new_types::TList<(type_level_Point::__fields::y, type_level_values::typenum::UInt<type_level_values::typenum::UTerm, type_level_values::values_prelude::B1>), type_level_values::new_types::TList<((), type_level_values::typenum::UInt<type_level_values::typenum::UInt<type_level_values::typenum::UTerm, type_level_values::values_prelude::B1>, type_level_values::values_prelude::B0>), type_level_values::new_types::TNil>>>`

```


Note in the main error message:

```text
^ the trait
`type_level_values::field_traits::SetField_<(), ... >` 
is not implemented for 
`type_level_Point::ConstPoint<..>`
```

In the help it also says that ConstPoint<..> implements 
SetField_\<fields::x>,SetField_\<fields::y>,and SetField_\<fields::All>.


# Example

Say that we have a channel type with a specified sequence of `send` and `recv` method calls.



```ignore

#[macro_use]
extern crate type_level_values;
#[macro_use]
extern crate derive_type_level;

use std::any::Any;


use type_level_values::prelude::*;
use type_level_values::values_prelude::*;


fn main(){
    let (channel0,channel1)=Channel::new::<String,usize>();

    // This produces a compile-time error.
    let (channel0,value)=channel0.recv().unwrap();
    println!("channel0 received:{}",value);
    
    let channel0=channel0.send("hello".into()).unwrap();
    
    let (channel1,value)=channel1.recv().unwrap();
    println!("channel1 received:{}",value);
    
    channel1.send(100);

}


#[derive(TypeLevel)]
#[typelevel(reexport(Variants,Traits))]
pub enum State{
    MustSend{
        #[typelevel(bound="Send+'static")]
        type_:(),
    },
    MustReceive{
        #[typelevel(bound="Send+'static")]
        type_:(),
    },
}

use self::type_level_State::fields;

use std::sync::mpsc::{self,Sender,Receiver};

type AnyBox=Box<Any+Send+'static>;

#[derive(ConstConstructor)]
#[cconstructor(Type="Channel",ConstParam="S")]
pub struct ChannelInner<S>{
    sender  :Sender<AnyBox>,
    receiver:Receiver<AnyBox>,
    state:ConstWrapper<S>,
}


impl Channel<()>{
    pub fn new<T,T2>()->(
        Channel<tlist![MustSend<T>,MustReceive<T2>]>,
        Channel<tlist![MustReceive<T>,MustSend<T2>]>,
    ){
        let (c_tx, s_rx) = mpsc::channel();
        let (s_tx, c_rx) = mpsc::channel();
        (Channel::<()>::new_inner(c_tx,c_rx),Channel::<()>::new_inner(s_tx,s_rx))
    }
}

impl<S> Channel<S>{
    fn new_inner<S2>(sender:Sender<AnyBox>,receiver:Receiver<AnyBox>)->Channel<S2>{
        Channel{ sender, receiver, state:ConstWrapper::NEW }
    }
    fn change_list<S2>(self)->Channel<S2>{
        Self::new_inner(self.sender,self.receiver)
    }
}

impl<S,Rem> Channel<tlist![S,..Rem]>
where S:MustSendTrait
{
    pub fn send(self,value:S::type_)->Result<Channel<Rem>,mpsc::SendError< Box<Any+Send> >>{
        self.sender.send(Box::new(value))?;
        Ok(self.change_list())
    }
}

impl<S,Rem> Channel<tlist![S,..Rem]>
where S:MustReceiveTrait
{
    pub fn recv(self)->Result<(Channel<Rem>,S::type_),mpsc::RecvError>{
        let value = self.receiver.recv()?.downcast().unwrap();
        Ok((self.change_list(), *value))
    }
}




```

This is the compiler error produced by that code:

```text

error[E0599]: no method named `recv` found for type `ChannelInner<type_level_values::ConstWrapper<type_level_values::new_types::TList<type_level_State::MustSend<std::string::String>, type_level_values::new_types::TList<type_level_State::MustReceive<usize>, type_level_values::new_types::TNil>>, type_level_values::const_wrapper::PhantomKind>>` in the current scope
  --> type_level_examples/src/bin/playground.rs:17:35
   |
17 |     let (channel0,value)=channel0.recv().unwrap();
   |                                   ^^^^
...
51 | pub struct ChannelInner<S>{
   | -------------------------- method `recv` not found for this
   |
   = note: the method `recv` exists but the following trait bounds were not satisfied:
           `type_level_State::MustSend<std::string::String> : type_level_State::MustReceiveTrait`

```

Note that the "main" error is not quite so readable now,
the note however tells us that the problem is 
that the ConstValue `MustSend<String>` does not implement `MustReceiveTrait`,
meaning that we made a mistake in following the sequence of operations.

The fix here is to call `send` with a String since that is the only thing that 
the `MustSend<String>` parameter allows us to do.

*/
