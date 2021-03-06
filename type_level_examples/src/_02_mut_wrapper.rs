//! This example demonstrates a wrapper type with a mutability parameter.
//!
//! MutabilityWrapper<T,Mutability> is a wrapper type which allows accessing T mutably
//! if Mutability==Mutable,otherwise T is only accessible immutably.
//!

use type_level_values::prelude::*;

use std::ops::{Deref, DerefMut};

/////////////////////////////////////////////////////////////////////

/// The state of initialization of the type.
#[derive(Clone, Copy, Debug, TypeLevel)]
#[typelevel(derive(ConstEq, ConstOrd))]
pub enum Mutability {
    Mutable,
    Immutable,
}

use self::type_level_Mutability::{Immutable, MutabilityTrait, Mutable};

/////////////////////////////////////////////////////////////////////

#[derive(MutConstValue)]
#[mcv(
    doc = "
        A Wrapper type whose mutability is a ConstValue-parameter.
        Many impls are implemented on [MutabilityWrapper].
    ",
    derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd),
    Type = "MutabilityWrapper",
    ConstValue = "M",
)]
pub struct __MutabilityWrapper<T, M> {
    value: T,
    mutability: ConstWrapper<M>,
}

impl<T, M> MutabilityWrapper<T, M>
where
    M: MutabilityTrait,
{
    pub fn new(value: T, _mutability: M) -> Self {
        Self {
            value,
            mutability: ConstWrapper::NEW,
        }
    }

    pub fn into_immutable(self) -> MutabilityWrapper<T, Immutable> {
        self.mutparam(ChangeMutability::NEW, Immutable::T)
    }
    pub fn as_immutable(&self) -> &MutabilityWrapper<T, Immutable> {
        self.mutparam_ref(ChangeMutability::NEW, Immutable::T)
    }
}

mutator_fn!{
    type This[T, M]=(MutabilityWrapper<T, M>)
    type AllowedSelf=(allowed_self_constructors::All)

    fn ChangeMutability[I,I2](I,I2) { I2 }
}

impl<T, M> Deref for MutabilityWrapper<T, M> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> DerefMut for MutabilityWrapper<T, Mutable> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

/////////////////////////////////////////////////////////////////////

#[allow(unused_mut)]
pub fn main_() {
    let mut wrapper: MutabilityWrapper<_, Mutable> = MutabilityWrapper::new(100, Mutable {});
    assert_eq!(*wrapper, 100);
    *wrapper = 200;
    assert_eq!(*wrapper, 200);
    *wrapper = 200;

    let mut frozen: MutabilityWrapper<_, Immutable> = wrapper.into_immutable();
    assert_eq!(*frozen, 200);

    // The line bellow won't compile because the contents of the wrapper are immutable
    // *frozen=300;
}
