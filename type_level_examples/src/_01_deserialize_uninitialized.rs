//! This example demonstrates a type that is partially deserialized.
//!
//! It is deserialized into a SortedList<T,Uninitialized>,which requires calling a method
//! taking some configuration/state,which fully initializes the type.
//!


use serde::{Deserialize, Deserializer, Serialize, Serializer};

use type_level_values::prelude::*;

use std::ops::Deref;

/////////////////////////////////////////////////////////////////////

/// The state of initialization of the type.
#[derive(Clone, Copy, Debug, TypeLevel)]
#[typelevel(derive(ConstEq, ConstOrd))]
pub enum State {
    Initialized,
    Uninitialized,
}

use self::type_level_State::{Initialized, Uninitialized};

/////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Reversed(pub bool);

/////////////////////////////////////////////////////////////////////

/// The way to deserialize a SortedList.After deserializing it,call SortedList::initialize.
pub type DeserializeSortedList<T> = SortedList<T, Uninitialized>;

/// A List that is sorted (its type being SortedList<T>) .
pub type SortedList<T, I = Initialized> = SortedListInner<T, ConstWrapper<I>>;

/// A List that is sorted (its type being SortedList<T>) .
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, ConstConstructor)]
#[cconstructor(Type(use_ = "SortedList"), ConstParam = "I")]
pub struct SortedListInner<T, I = ConstWrapper<Initialized>> {
    list: Vec<T>,

    const_value: I,
}

impl<T> Serialize for SortedList<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.list.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for SortedList<T, Uninitialized>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<T>::deserialize(deserializer).map(|list| SortedList {
            list,
            const_value: ConstWrapper::NEW,
        })
    }
}

impl<T> SortedList<T>
where
    T: Ord,
{
    pub fn new(mut list: Vec<T>, reversed: Reversed) -> Self {
        list.sort();
        if reversed.0 {
            list.reverse();
        }
        SortedList {
            list: list,
            const_value: ConstWrapper::NEW,
        }
    }
}

impl<T> SortedList<T, Uninitialized>
where
    T: Ord,
{
    pub fn initialize(self, reversed: Reversed) -> SortedList<T> {
        SortedList::new(self.list, reversed)
    }
}

impl<T> Deref for SortedList<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.list
    }
}

/////////////////////////////////////////////////////////////////////

pub fn main_ () {
    let reversed = Reversed(true);
    let list = ::serde_json::from_str::<DeserializeSortedList<u64>>("[0,10,5,1,4]")
        .unwrap()
        .initialize(reversed);

    assert_eq!(list, SortedList::new(vec![10, 5, 4, 1, 0], reversed));
}
