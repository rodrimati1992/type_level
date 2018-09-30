use std_::ops::Sub;

use prelude::*;


use crate_::field_traits::{
    GetField, GetFieldFn, MapField, MapFieldOp, MapIntoField, MapIntoFieldOp, SetField, SetFieldFn,
};
use crate_::ops::{
    fn_adaptors as op_a, fn_types as op_t,TypeFn, TypeFn_,
};

use core_extensions::type_level_bool::{BooleanType, False, True};

use self::op_a::*;
use self::op_t::*;

type_fn!{define-trait
    fn_type=FoldLOp
    /// An iterator function that processes the collection incrementally from the start,
    /// starting with Defaultval and the first element.
    ///
    /// If the collection is empty it must return DefaultVal.
    trait=FoldL_ [DefaultVal,Func]
    type=FoldL
}

type_fn!{define-trait
    fn_type=FoldROp
    /// An iterator function that processes the collection incrementally from the end,
    /// starting with Defaultval and the last element.
    ///
    /// If the collection is empty it must return DefaultVal.
    trait=FoldR_ [DefaultVal,Func]
    type=FoldR
}

type_fn!{define-trait
    fn_type=ReduceLOp
    trait=ReduceL_ [Func]
    type=ReduceL
}

type_fn!{define-trait
    fn_type=ReduceROp
    trait=ReduceR_ [Func]
    type=ReduceR
}

type_fn!{define-trait
    fn_type=MapOp
    trait=Map_ [Func]
    type=Map
}

type_fn!{define-trait
    fn_type=FilterOp
    trait=Filter_ [Predicate]
    type=Filter
}

type_fn!{define-trait
    fn_type=RemoveOp
    trait=Remove_ [Index]
    type=Remove
}

type_fn!{define-trait
    fn_type=InsertOp
    trait=Insert_ [Index,Value]
    type=Insert
}

type_fn!{define-trait
    fn_type=PushOp
    /// Returns the collection with the value added at one end.
    ///
    /// Push followed by Pop must return the pushed value and
    /// the collection as it was before pushing.
    trait=Push_ [Value]
    type=Push
}

type_fn!{define-trait
    fn_type=PopOp
    /// Returns the collection with the last/first element removed alongside that element.
    ///
    /// Returns Some_<(Element,CollectionWithoutValue)> if the collection is not empty,
    /// otherwise returns None_.
    trait=Pop_ []
    type=Pop
}

type_fn!{define-trait
    fn_type=PushBackOp
    /// Returns the collection with the value added after the last element.
    ///
    /// PushBack followed by PopBack must return the pushed value and
    /// the collection as it was before pushing.
    trait=PushBack_ [Value]
    type=PushBack
}

type_fn!{define-trait
    fn_type=PopBackOp
    /// Returns the collection with the last element removed,alongside the last element.
    ///
    /// Returns Some_<(Element,CollectionWithoutValue)> if the collection is not empty,
    /// otherwise returns None_.
    trait=PopBack_ []
    type=PopBack
}

type_fn!{define-trait
    fn_type=PushFrontOp
    /// Returns the collection with the value added before the first element.
    ///
    /// PushFront followed by PopFront must return the pushed value and
    /// the collection as it was before pushing.
    trait=PushFront_ [Value]
    type=PushFront
}

type_fn!{define-trait
    fn_type=PopFrontOp
    /// Returns the collection with the first element removed,alongside the first element.
    ///
    /// Returns None if the collection is empty ,
    /// otherwise retuns the first value and remaining collection in
    /// Some_<(Value,CollectionWithoutValue)>.
    trait=PopFront_ []
    type=PopFront
}

type_fn!{define-trait
    fn_type=LenOp
    /// The ammount of elements in the collection that can be iterated over in FoldL_ .
    ///
    trait=Len_ []
    type=Len
}

type_fn!{define-trait
    fn_type=RepeatOp
    /// Creates a value of by repeating  `Value` `Repeated` times
    ///
    trait=Repeat_ [ Value,Repeated ]
    type=Repeat
}

type_fn!{define-trait
    fn_type=ReverseOp
    /// Reverses this data structure
    ///
    trait=Reverse_ []
    type=Reverse
}


/// Checks whether the Collection contains Element.
pub type Contains<Collection,Element>=
    TypeFn<ContainsOp,(Collection,Element)>;

type_fn!{
    /// Checks whether the Collection contains Element.
    pub fn ContainsOp[Collection,Element](Collection,Element)
    where[ AnyOp:TypeFn_<(Collection,EqualsElement),Output=Out> ]
    {
        let EqualsElement=ApplyRhs<ConstEqOp,Element>;
        let Out;Out
    }
}


/// Checks whether all elements of the Collection satisfy the Predicate
pub type All<Collection,Predicate>=
    TypeFn<AllOp,(Collection,Predicate)>;

type_fn!{
    /// Checks whether all elements of the Collection satisfy the Predicate
    pub fn AllOp[Collection,Predicate](Collection,Predicate)
    where[ Collection:FoldL_<True,MappedUnary>,]
    {
        let MappedUnary=MapRhs<BitAndOp,Predicate>;
        Collection::Output
    }
}


/// Checks whether any element of the Collection satisfy the Predicate
pub type Any<Collection,Predicate>=
    TypeFn<AnyOp,(Collection,Predicate)>;

type_fn!{
    /// Checks whether any element of the Collection satisfy the Predicate
    pub fn AnyOp[Collection,Predicate](Collection,Predicate)
    where[ Collection:FoldL_<False,MappedUnary> ]
    {
        let MappedUnary=MapRhs<BitOrOp,Predicate>;
        Collection::Output
    }
}




type_fn!{
    /// Returns the last value of Collection,or returns Defaultval if it's empty.
    pub fn LastOrDefaultOp[Collection,Defaultval](Collection,Defaultval)
    where[ Collection:FoldL_<Defaultval,GetRhs,Output=Out> ]
    {
        let GetRhs=ApplyRhs<GetFieldFn, U1>;
        let Out; 
        Out
    }
}

/// Returns the last value of Collection,or returns Defaultval if it's empty.
pub type LastOrDefault<Collection,DefaultVal> = 
    TypeFn<LastOrDefaultOp, (Collection,DefaultVal)>;



impl<This,Op,Val,Rem,Out> ReduceL_<Op> for This
where
    This:PopFront_<Output=Some_<(Val,Rem)>>,
    Rem:FoldL_<Val,Op,Output=Out>,
{
    type Output=Out;
}


impl<This,Op,Val,Rem,Out> ReduceR_<Op> for This
where
    This:PopBack_<Output=Some_<(Val,Rem)>>,
    Rem:FoldR_<Val,Op,Output=Out>,
{
    type Output=Out;
}