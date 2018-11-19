use std::any::Any;
use std::ops::Range;
use std::sync::mpsc::{self, Receiver as MPSCReceiver, RecvError, SendError, Sender as MPSCSender};

use type_level_values::new_types::{TList, TNil};
use type_level_values::collection_ops::{Len_, PushBack_};
use type_level_values::prelude::*;


#[allow(unused_imports)]
use type_level_values::core_extensions::TryInto;
use type_level_values::core_extensions::{Void,CallInto, TryFrom,TypePanic};

use super::generic_variant::{Impossible, MapVariants, VariantsTrait};

use super::ranged_usize::{RangedUsize, RangedTrait, IntOutsideRange};


pub type CEResult<T> = ::std::result::Result<T, ProtocolViolation>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, TypeLevel)]
pub enum LoopKind {
    Infinite,
    Finite { repetitions: u32 },
}

pub use self::type_level_LoopKind::variants::*;
pub use self::type_level_LoopKind::{Finite, Infinite};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, TypeLevel)]
#[typelevel(
    items(IntoConstType(NoImpls)),
    reexport(Variants, Discriminants, Traits),
)]
// #[typelevel(skip_derive)]
// #[typelevel(print_derive)]
pub enum Operation {
    #[typelevel(
        doc = "Trasfers a message of the type `type_` to the `to` ChannelEnd from the other end. "
    )]
    TransferTo { to: ChannelEnd, type_: () },

    #[typelevel(doc = "Runs `sequence` as many types as specified by `kind`. ")]
    Loop {
        kind: LoopKind,
        /// the full list of operations on this loop.
        sequence: Vec<Operation>,
    },

    #[typelevel(doc = "breaks out of Loop,unusable outside of a loop")]
    LoopBreak,

    #[typelevel(doc = "restarts the Loop,unusable outside of a loop")]
    LoopContinue,

    #[typelevel(
        doc = "Allows `who_choses` to decide which of the `branches` to run ,",
        doc = "forcing the other side to run the same branch.",
    )]
    Branch {
        who_choses: ChannelEnd,
        branches: Vec<Vec<Operation>>,
    },

    #[typelevel(doc = "An indidator that the channel is closed (not a command).")]
    Closed,
}

pub use self::type_level_Operation::fields as op_f;

////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////
//                  Runtime Component of the state machine
//

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RuntState {
    stack: Vec<StackFrame>,
}

impl RuntState {
    fn new() -> Self {
        Self {
            stack: vec![StackFrame::Normal],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum StackFrame {
    Normal,
    InLoop(LoopKind),
}

////////////////////////////////////////////////////////////////////////////
//      Control-flow

#[derive(Debug)]
pub enum LoopState<B, C = B> {
    Break(B),
    Continue(C),
}

////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, TypeLevel)]
pub enum TransferMethod {
    Send_(()),
    Receive(()),
}

pub use self::type_level_TransferMethod::{Receive, Send_};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, TypeLevel)]
pub enum ChannelEnd {
    Server,
    Client,
}

pub use self::type_level_ChannelEnd::{Client, Server};

type_fn!{
    /// Determines whether send or recv are callable
    /// based on the transfer direction and whether this is a server or a client
    ///
    /// fn GetTransferMethod( channel_end:ChannelEnd , transfer_to:TransferTo )->TransferMethod
    fn
        GetTransferMethod[T](Server,TransferTo<Server,T>){ Receive<T> }
        GetTransferMethod[T](Client,TransferTo<Server,T>){ Send_<T> }
        GetTransferMethod[T](Server,TransferTo<Client,T>){ Send_<T> }
        GetTransferMethod[T](Client,TransferTo<Client,T>){ Receive<T> }
}

//////////////////////////////////////////////////////////////////////////////////////////

/// Marker type declaring that the type parameter is erased.
pub struct Erased;

//////////////////////////////////////////////////////////////////////////////////////////

type_fn!{
    fn WrapPhantom[T](T){ ConstWrapper<T> }
}

type_fn!{
    /// A function that checks that the user provided list of operations
    /// does not contain a LoopContinue or a LoopBreak outside a Loop.
    pub fn
        CheckNoContinueBreak(TNil){ () }

        CheckNoContinueBreak[Current,Rem](TList<Current,Rem>)
        where[
            Current:GetDiscriminant,
            CheckNoContinueBreak:TypeFn_<(Current::Variant,Current)>,
        ]{ () }

        CheckNoContinueBreak[T](TransferTo_Variant,T){ () }

        CheckNoContinueBreak[T](Loop_Variant,T){ () }

        CheckNoContinueBreak[T](LoopBreak_Variant   ,T)
        where [InvalidOutsideLoop<T>:TypePanic]
        { () }

        CheckNoContinueBreak[T](LoopContinue_Variant,T)
        where [InvalidOutsideLoop<T>:TypePanic]
        { () }

        CheckNoContinueBreak[branch](Branch_Variant,branch)
        where[
            branch:BranchTrait,
            branch::branches:MapVariants< ConstWrapper<CheckNoContinueBreak> >,
        ]{ () }

        CheckNoContinueBreak[T](Closed_Variant,T){ () }

}

/// Used in error messages to indicate that an operation is invalid outside of loops.
pub struct InvalidOutsideLoop<T>(Void, T);

//////////////////////////////////////////////////////////////////////////////////////////

pub type Channel<CEnd, S> = Channel_Ty<ConstWrapper<CEnd>, ConstWrapper<S>>;

#[derive(MutConstValue)]
#[mcv(
    derive(Debug),
    Type(use_ = "Channel"), ConstValue = "S",
)]
pub struct __Channel<CEnd, S> {
    tx: MPSCSender<Message>,
    rx: MPSCReceiver<Message>,
    runt_state: RuntState,
    consts: ConstWrapper<(CEnd, S)>,
}
pub type Message = Box<Any + Send>;

//////////////////////////////////////////////////////////////////////////////////////////

mod channel {

    use super::MapVariants;
    use super::*;

    impl Channel<(), ()> {
        pub fn new<O, Instructions>(
            _ops: O,
        ) -> (Channel<Client, Instructions>, Channel<Server, Instructions>)
        where
            O: PushBack_<Closed, Output = Instructions>,
            CheckNoContinueBreak: TypeFn_<O>,
        {
            let (c_tx, s_rx) = mpsc::channel();
            let (s_tx, c_rx) = mpsc::channel();
            (
                Channel::new_inner(c_tx, c_rx),
                Channel::new_inner(s_tx, s_rx),
            )
        }

        fn new_inner<CEnd, Stack>(
            tx: MPSCSender<Message>,
            rx: MPSCReceiver<Message>,
        ) -> Channel<CEnd, Stack> {
            Channel {
                tx,
                rx,
                runt_state: RuntState::new(),
                consts: ConstWrapper::NEW,
            }
        }
    }

    impl<CEnd, Current, Rem> Channel<CEnd, TList<Current, Rem>> {
        fn remaining(self) -> Channel<CEnd, Rem> {
            Channel {
                tx: self.tx,
                rx: self.rx,
                runt_state: self.runt_state,
                consts: ConstWrapper::NEW,
            }
        }
    }

    impl<CEnd, List0> Channel<CEnd, List0> {
        fn change_list<List1>(self) -> Channel<CEnd, List1> {
            Channel {
                tx: self.tx,
                rx: self.rx,
                runt_state: self.runt_state,
                consts: ConstWrapper::NEW,
            }
        }
        fn erase_channel(self) -> Channel<Erased, Erased> {
            Channel {
                tx: self.tx,
                rx: self.rx,
                runt_state: self.runt_state,
                consts: ConstWrapper::NEW,
            }
        }
    }

    impl<CEnd, Current, Rem> Channel<CEnd, TList<Current, Rem>>
    where
        Current: TransferToTrait,
        Current::type_: Send + 'static,
    {
        pub fn send(self, value: Current::type_) -> CEResult<Channel<CEnd, Rem>>
        where
            GetTransferMethod: TypeFn_<(CEnd, Current), Output = Send_<Current::type_>>,
        {
            self.tx.send(Box::new(value))?;
            Ok(self.remaining())
        }

        pub fn recv(self) -> CEResult<(Channel<CEnd, Rem>, Box<Current::type_>)>
        where
            GetTransferMethod: TypeFn_<(CEnd, Current), Output = Receive<Current::type_>>,
        {
            let value = self
                .rx
                .recv()?
                .downcast()
                .map_err(ProtocolViolation::InvalidMessage)?;

            Ok((self.remaining(), value))
        }
    }

    type_fn!{
        fn
            BranchIfNotChooser( Server,Client ){ () }
            BranchIfNotChooser( Client,Server ){ () }
    }

    type_fn!{
        fn
            BranchIfChooser( Client,Client ){ () }
            BranchIfChooser( Server,Server ){ () }
    }

    impl<CEnd, Current, Rem, Len: 'static, PhantomVariants, MappedVariants>
        Channel<CEnd, TList<Current, Rem>>
    where
        Current: BranchTrait,
        Current::branches: MapVariants<ConstWrapper<WrapPhantom>, Output = PhantomVariants>,
        Current::branches: Len_<Output = Len>,
        PhantomVariants: From<RangedUsize<U0, Len>>,
        PhantomVariants: MapVariants<ChannelInto<Self>, Output = MappedVariants>,
        MappedVariants: VariantsTrait,
    {
        pub fn branch<F>(self, f: F) -> CEResult<Channel<CEnd, Rem>>
        where
            F: FnOnce(MappedVariants) -> CEResult<Channel<CEnd, TNil>>,
            BranchIfNotChooser: TypeFn_<(Current::who_choses, CEnd)>,
        {
            let index: RangedUsize<U0, Len> = *self
                .rx
                .recv()?
                .downcast()
                .map_err(ProtocolViolation::InvalidMessage)?;
            f(index
                .piped(PhantomVariants::from)
                .map_variants(ChannelInto(self)))
                .map(|s| s.change_list())
        }

        /// Validates the index used to choose among the different branches
        /// to take in the choose mwthos.
        /// Helper method used to aid type inference.
        pub fn validate_choice(
            &self,
            index: usize,
        ) -> Result<RangedUsize<U0, Len>, IntOutsideRange<usize>>
        where
            RangedUsize<U0, Len>: TryFrom<usize, Error = IntOutsideRange<usize>>,
        {
            index.try_into()
        }

        pub fn range_ty(&self, v: RangedUsize<U0, Len>) -> RangedUsize<U0, Len> {
            v
        }

        /// Returns a VariantPhantom wrapping the type used to choose which branch to take.
        pub fn choice_range(&self) -> Range<usize>
        where
            RangedUsize<U0, Len>: RangedTrait<Integer=usize>,
        {
            RangedUsize::<U0, Len>::start()..
            RangedUsize::<U0, Len>::end().unwrap_or(!0usize)
        }

        pub fn choose<F>(self, index: RangedUsize<U0, Len>, f: F) -> CEResult<Channel<CEnd, Rem>>
        where
            F: FnOnce(MappedVariants) -> CEResult<Channel<CEnd, TNil>>,
            BranchIfChooser: TypeFn_<(Current::who_choses, CEnd)>,
        {
            self.tx.send(Box::new(index))?;
            f(index
                .piped(PhantomVariants::from)
                .map_variants(ChannelInto(self)))
                .map(|s| s.change_list())
        }
    }

    impl<CEnd, Current, Rem, _Rem> Channel<CEnd, TList<Current, Rem>>
    where
        Current: LoopTrait,
        Current::kind: IntoRuntime<LoopKind>,
        Current::sequence: PushBack_<Current, Output = _Rem>,
    {
        pub fn loop_<F>(mut self, mut f: F) -> CEResult<Channel<CEnd, Rem>>
        where
            F: FnMut(Channel<CEnd, _Rem>) -> CEResult<Channel<CEnd, TList<Current, TNil>>>,
        {
            self.runt_state
                .stack
                .push(StackFrame::InLoop(Current::kind::to_runtime()));
            (|| loop {
                match self.start_loop() {
                    LoopState::Break(c) => break Ok(c),
                    LoopState::Continue(client) => {
                        use self::ProtocolViolation as PV;
                        match f(client) {
                            Ok(channel) => self = channel.change_list(),
                            Err(PV::ControlFlow(ControlFlowPrivate {
                                priv_: LoopState::Continue(channel),
                            })) => self = channel.unerase_channel(),
                            Err(PV::ControlFlow(ControlFlowPrivate {
                                priv_: LoopState::Break(channel),
                            })) => break Ok(channel.unerase_channel()),
                            Err(e) => return Err(e),
                        }
                    }
                }
            })().map(|mut client| {
                client.runt_state.stack.pop();
                client
            })
        }
        fn start_loop(mut self) -> LoopState<Channel<CEnd, Rem>, Channel<CEnd, _Rem>> {
            let returned = {
                let last = self.runt_state.stack.last_mut().unwrap();
                match last {
                    &mut StackFrame::Normal => unreachable!(),
                    &mut StackFrame::InLoop(LoopKind::Infinite) => LoopState::Continue(()),
                    &mut StackFrame::InLoop(LoopKind::Finite { repetitions })
                        if repetitions <= 1 =>
                    {
                        LoopState::Break(())
                    }
                    &mut StackFrame::InLoop(LoopKind::Finite {
                        ref mut repetitions,
                    }) => {
                        *repetitions -= 1;
                        LoopState::Continue(())
                    }
                }
            };
            match returned {
                LoopState::Break(()) => LoopState::Break(self.change_list()),
                LoopState::Continue(()) => LoopState::Continue(self.change_list()),
            }
        }
    }

    impl<CEnd, Current, Rem> Channel<CEnd, TList<Current, Rem>>
    where
        Current: LoopBreakTrait,
    {
        pub fn break_(self) -> Result<Channel<CEnd, Rem>, ProtocolViolation> {
            self.erase_channel()
                .piped(LoopState::Break)
                .piped(ControlFlowPrivate::new)
                .piped(ProtocolViolation::ControlFlow)
                .piped(Err)
        }
    }

    impl<CEnd, Current, Rem> Channel<CEnd, TList<Current, Rem>>
    where
        Current: LoopContinueTrait,
    {
        pub fn continue_(self) -> Result<Channel<CEnd, Rem>, ProtocolViolation> {
            self.erase_channel()
                .piped(LoopState::Continue)
                .piped(ControlFlowPrivate::new)
                .piped(ProtocolViolation::ControlFlow)
                .piped(Err)
        }
    }

    pub struct ChannelInto<Ch>(pub Ch);

    impl<Params, End, List> CallInto<ConstWrapper<Params>> for ChannelInto<Channel<End, List>> {
        type Returns = Channel<End, Params>;
        /// calls this function
        fn call_into(self, _params: ConstWrapper<Params>) -> Self::Returns {
            self.0.change_list()
        }
    }

    impl<Ch> CallInto<Impossible> for ChannelInto<Ch> {
        type Returns = Impossible;
        /// calls this function
        fn call_into(self, params: Impossible) -> Self::Returns {
            params
        }
    }

    impl Channel<Erased, Erased> {
        fn unerase_channel<CEnd, List>(self) -> Channel<CEnd, List> {
            Channel {
                tx: self.tx,
                rx: self.rx,
                runt_state: self.runt_state,
                consts: ConstWrapper::NEW,
            }
        }
    }

}

//////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum ProtocolViolation {
    InvalidMessage(Message),
    SendError(SendError<Message>),
    RecvError(RecvError),

    /// This is a hack to get around the lack of a stable way to override the `?` operator.
    ControlFlow(ControlFlowPrivate<LoopState<Channel<Erased, Erased>>>),
}

impl From<SendError<Message>> for ProtocolViolation {
    fn from(v: SendError<Message>) -> Self {
        ProtocolViolation::SendError(v)
    }
}

impl From<RecvError> for ProtocolViolation {
    fn from(v: RecvError) -> Self {
        ProtocolViolation::RecvError(v)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct ControlFlowPrivate<T> {
    priv_: T,
}

impl<T> ControlFlowPrivate<T> {
    fn new(priv_: T) -> Self {
        Self { priv_ }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
