/*!
Operations for collection types,including TypeList,tuples,Option,Result.
*/

use std_::ops::Sub;

use prelude::*;

use crate_::field_traits::{
    GetField, GetFieldOp, MapField, MapFieldOp, MapIntoField, MapIntoFieldOp, SetField,
};
use crate_::fn_adaptors::*;
use crate_::fn_types::*;
use crate_::ops::{ConstFrom_,ConstInto_,ConstIntoOp};

type_fn!{define_trait
    /// An iterator function that processes the collection incrementally from the start,
    /// starting with Defaultval and the first element.
    ///
    /// If the collection is empty it must return DefaultVal.
    trait=FoldL_ [DefaultVal,Func]
    type=FoldL
    fn_type=FoldLOp
}

type_fn!{define_trait
    /// An iterator function that processes the collection incrementally from the end,
    /// starting with Defaultval and the last element.
    ///
    /// If the collection is empty it must return DefaultVal.
    trait=FoldR_ [DefaultVal,Func]
    type=FoldR
    fn_type=FoldROp
}


type_fn!{define_trait
    /// An iterator function that processes the collection incrementally from the start,
    /// starting with the first element.
    trait=ReduceL_ [Func]
    type=ReduceL
    fn_type=ReduceLOp
}

type_fn!{define_trait
    /// An iterator function that processes the collection incrementally from the end,
    /// starting with the last element.
    trait=ReduceR_ [Func]
    type=ReduceR
    fn_type=ReduceROp
}

type_fn!{define_trait
    /// Transforms the elements of the collection with the `Func` function.
    trait=Map_ [Func]
    type=Map
    fn_type=MapOp
}

type_fn!{define_trait
    /// Returns the collection in which all the elements that 
    /// do not satisfy the `Predicate` are removed.
    ///
    /// Predicate is the equivalent to `Fn(&T)->bool`,where T is the element type.
    trait=Filter_ [Predicate]
    type=Filter
    fn_type=FilterOp
}

type_fn!{define_trait
    /// Removes the element at the `Ìndex` position from the collection.
    trait=Remove_ [Index]
    type=Remove
    fn_type=RemoveOp
}

type_fn!{define_trait
    /// Inserts `Value` at the `Ìndex` position into the collection.
    trait=Insert_ [Index,Value]
    type=Insert
    fn_type=InsertOp
}

type_fn!{define_trait
    /// Returns the collection with the value added at one end.
    ///
    /// Push followed by Pop must return the pushed value and
    /// the collection as it was before pushing.
    trait=Push_ [Value]
    type=Push
    fn_type=PushOp
}

type_fn!{define_trait
    /// Returns the collection with the last/first element removed alongside that element.
    ///
    /// Returns Some_<(Element,CollectionWithoutValue)> if the collection is not empty,
    /// otherwise returns None_.
    trait=Pop_ []
    type=Pop
    fn_type=PopOp
}

type_fn!{define_trait
    /// Returns the collection with the value added after the last element.
    ///
    /// PushBack followed by PopBack must return the pushed value and
    /// the collection as it was before pushing.
    trait=PushBack_ [Value]
    type=PushBack
    fn_type=PushBackOp
}

type_fn!{define_trait
    /// Returns the collection with the last element removed,alongside the last element.
    ///
    /// Returns Some_<(Element,CollectionWithoutValue)> if the collection is not empty,
    /// otherwise returns None_.
    trait=PopBack_ []
    type=PopBack
    fn_type=PopBackOp
}

type_fn!{define_trait
    /// Returns the collection with the value added before the first element.
    ///
    /// PushFront followed by PopFront must return the pushed value and
    /// the collection as it was before pushing.
    trait=PushFront_ [Value]
    type=PushFront
    fn_type=PushFrontOp
}

type_fn!{define_trait
    /// Returns the collection with the first element removed,alongside the first element.
    ///
    /// Returns None if the collection is empty ,
    /// otherwise retuns the first value and remaining collection in
    /// Some_<(Value,CollectionWithoutValue)>.
    trait=PopFront_ []
    type=PopFront
    fn_type=PopFrontOp
}

type_fn!{define_trait
    /// The ammount of elements in the collection that can be iterated over in FoldL_ .
    ///
    trait=Len_ []
    type=Len
    fn_type=LenOp
}

type_fn!{define_trait
    /// Creates a value of by repeating  `Value` `Repeated` times
    ///
    trait=Repeat_ [ Value,Repeated ]
    type=Repeat
    fn_type=RepeatOp
}

type_fn!{define_trait
    /// Reverses this data structure
    ///
    trait=Reverse_ []
    type=Reverse
    fn_type=ReverseOp
}

impl<This, Op, Val, Rem, Out> ReduceL_<Op> for This
where
    This: PopFront_<Output = Some_<(Val, Rem)>>,
    Rem: FoldL_<Val, Op, Output = Out>,
{
    type Output = Out;
}

impl<This, Op, Val, Rem, Out> ReduceR_<Op> for This
where
    This: PopBack_<Output = Some_<(Val, Rem)>>,
    Rem: FoldR_<Val, Op, Output = Out>,
{
    type Output = Out;
}



/////////////////////////////////////////////////////////////////////////////////////////


type_fn!{define_trait
    /** 
    An iterator function that processes the collection incrementally from the start,
    starting with Defaultval and the first element,
    returning early when Func returns a value that converts to TFBreak like Err_<_>/None_,
    
    If the collection is empty it must return TFVal<DefaultVal>.
    */
    trait=TryFoldL_ [DefaultVal,Func]
    type=TryFoldL
    fn_type=TryFoldLOp
}

type_fn!{define_trait
    /** 
    An iterator function that processes the collection incrementally from the end,
    starting with Defaultval and the last element,
    returning early when Func returns a value that converts to TFBreak like Err_<_>/None_,
    
    If the collection is empty it must return TFVal<DefaultVal>.
    */
    trait=TryFoldR_ [DefaultVal,Func]
    type=TryFoldR
    fn_type=TryFoldROp
}


#[derive(TypeLevel)]
#[typelevel(reexport(Variants))]
pub enum TryFold<T,B>{
    TFVal(T),
    TFBreak(B),
}


/** 
Alias for converting a value to a TryFoldType.

Type-level equivalent to:
```ignore
fn into_try_fold<S,T,B>(from:S)->TryFold<T,B>
where S:Into<TryFold<T,B>>
{ from.into() }`
```

*/
pub type IntoTryFold=ApplyRhs<ConstIntoOp,TryFoldType>;


macro_rules! define_conversions {
    ( generics[$($generic:tt)*] $from:ty : $from_consttype:ty => $try_flow:ty ) => (
        impl<$($generic)*> ConstFrom_<$from> for TryFoldType{
            type Output=$try_flow;
        }
        impl<$($generic)*> ConstInto_<$from_consttype> for $try_flow{
            type Output=$from;
        }
    )
}

define_conversions!{ generics[T] Ok_<T>:ResultType => TFVal<T> }
define_conversions!{ generics[T] Err_<T>:ResultType => TFBreak<Err_<T>> }

define_conversions!{ generics[T] Some_<T>:OptionType => TFVal<T> }
define_conversions!{ generics[]  None_   :OptionType => TFBreak<None_> }



/////////////////////////////////////////////////////////////////////////////////////////

