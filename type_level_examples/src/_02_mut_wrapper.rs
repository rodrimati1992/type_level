//! This example demonstrates a wrapper type with a mutability parameter.
//!
//! MutabilityWrapper<T,Mutability> is a wrapper type which allows accessing T mutably
//! if Mutability==Mutable,otherwise T is only accessible immutably.
//!


use type_level_values::prelude::*;

use std::mem::transmute;
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

/// A Wrapper type whose mutability is a const-parameter.
/// Many impls are also implemented on [MutabilityWrapperInternal].
pub type MutabilityWrapper<T, M> = MutabilityWrapperInternal<T, PhantomWrapper<M>>;

/// A Wrapper type whose mutability is a const-parameter.
/// Many impls are implemented on [MutabilityWrapper].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct MutabilityWrapperInternal<T, M> {
    value: T,
    mutability: M,
}

impl<T, M> MutabilityWrapper<T, M>
where
    M: MutabilityTrait,
{
    pub fn new(value: T, _mutability: M) -> Self {
        Self {
            value,
            mutability: PhantomWrapper::NEW,
        }
    }

    pub fn to_immutable(&self) -> MutabilityWrapper<T, Immutable>
    where
        T: Clone,
    {
        MutabilityWrapper::new(self.value.clone(), Immutable {})
    }
    pub fn into_immutable(self) -> MutabilityWrapper<T, Immutable> {
        MutabilityWrapper::new(self.value, Immutable {})
    }
    pub fn as_immutable(&self) -> &MutabilityWrapper<T, Immutable> {
        unsafe { transmute(self) }
    }
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

pub fn main_ () {
    let mut wrapper: MutabilityWrapper<_, Mutable> = MutabilityWrapper::new(100, Mutable {});
    assert_eq!(*wrapper, 100);
    *wrapper = 200;
    assert_eq!(*wrapper, 200);
    *wrapper = 200;

    #[allow(unused_mut)]
    let mut frozen: MutabilityWrapper<_, Immutable> = wrapper.to_immutable();
    assert_eq!(*frozen, 200);

    // The line bellow won't compile because the contents of the wrapper are immutable
    // *frozen=300;
}
