//! This example demonstrates a channel that can only send a limited ammount of values,
//! checked at compile-time.
//!
//!



// #[allow(unused_imports)]
// use std::ops::*;
use std::sync::mpsc::{self, Receiver as MPSCReceiver, RecvError, SendError, Sender as MPSCSender};

// use core_extensions::ResultLike;

use type_level_values::field_traits::*;
use type_level_values::fn_adaptors::*;
use type_level_values::std_ops::*;
#[allow(unused_imports)]
use type_level_values::ops::*;
use type_level_values::prelude::*;


use type_level_values::new_types::TList;
// use type_level_values::new_types::TListType;
// use type_level_values::std_types::cmp_ordering::{Equal_, Greater_, Less_};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, TypeLevel)]
// #[typelevel(print_derive)]
pub enum State {
    Open { remaining: u64 },
    Closed,
}

use self::type_level_State::{Closed, Open, OpenTrait};

/////////////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////////////

#[derive(MutConstValue)]
#[mcv(
    Type = "ChannelEnd", ConstValue = "S"
)]
pub struct ChannelEndInner<Chan, S: WrapperTrait> {
    channel: Chan,
    #[allow(dead_code)]
    state: ConstWrapperFromTrait<S>,
}

pub type Sender<T, S> = ChannelEnd<MPSCSender<T>, S>;
pub type Receiver<T, S> = ChannelEnd<MPSCReceiver<T>, S>;

pub fn channel<T, L>() -> (Sender<T, Open<L>>, Receiver<T, Open<L>>)
where
    T: Send,
{
    let (tx, rx) = mpsc::channel();
    (
        Sender {
            channel: tx,
            state: ConstWrapper::NEW,
        },
        Receiver {
            channel: rx,
            state: ConstWrapper::NEW,
        },
    )
}

impl<T, L> Sender<T, L>
where
    T: Send,
{
    #[inline(always)]
    pub fn send<__NextSelf>(self, value: T) -> Result<__NextSelf, SendError<T>>
    where
        Self: MCPBounds<TransferValue, (), NextSelf = __NextSelf>,
    {
        self.channel.send(value)?;
        Ok(self.mutparam(TransferValue::NEW, Default::default()))
    }
}

impl<T, L> Receiver<T, L>
where
    T: Send,
{
    #[inline(always)]
    pub fn recv<__NextSelf>(self) -> Result<(__NextSelf, T), RecvError>
    where
        Self: MCPBounds<TransferValue, (), NextSelf = __NextSelf>,
    {
        let ret = self.channel.recv()?;
        Ok((self.mutparam(TransferValue::NEW, Default::default()), ret))
    }
}

type_fn!{
    pub fn NewOpen[v](v){ Open<v> }
}
pub type NewClosed=Const<Closed>;

mutator_fn!{
    type This[Chan,I]=(ChannelEnd<Chan,I>)
    type AllowedSelf=(allowed_self_constructors::ByVal)

    fn TransferValue[I](I,())
    where [
        I:OpenTrait,
        I::remaining : Piped_<
            If<ConstLEMt<U1>,
                NewClosed,
                (Sub1Op,NewOpen)
            >,Output=Out
        >,
    ]{
        let Out;
        Out
    }
}


pub fn main_ () {
    let (tx, rx) = channel::<&'static str, U4>();

    #[allow(unused_variables)]
    let tx: Sender<_, Closed> = tx
        .send("hello")
        .unwrap()
        .send("hello")
        .unwrap()
        .send("hello")
        .unwrap()
        .send("hello")
        .unwrap();

    // the channel is already closed
    // let tx=tx.send("hello",()).unwrap();

    macro_rules! receive {
        ($rx:ident) => {
            #[allow(unused_variables)]
            let $rx = {
                let (rx, val) = $rx.recv().unwrap();
                println!("{}", val);
                rx
            };
        };
    }

    receive!(rx);
    receive!(rx);
    receive!(rx);
    receive!(rx);

    // the channel is already closed
    // receive!(rx);
}
