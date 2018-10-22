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
use crate_::ops::{
    ConstFrom_,
    ConstInto_,ConstIntoOp,ConstIntoMt,
    IntoInnerOp,IntoInner_,
    If,
    AssertFnRet,
    ConstLtOp,
};

type_fn!{define_trait
    /// An iterator function that processes the collection incrementally from the start,
    /// starting with Defaultval and the first element.
    ///
    /// If the collection is empty it must return DefaultVal.
    trait=FoldL_ [DefaultVal,Func]
    type=FoldL
    fn_type=FoldLOp
    method_like=FoldLMt
}

type_fn!{define_trait
    /// An iterator function that processes the collection incrementally from the end,
    /// starting with Defaultval and the last element.
    ///
    /// If the collection is empty it must return DefaultVal.
    trait=FoldR_ [DefaultVal,Func]
    type=FoldR
    fn_type=FoldROp
    method_like=FoldRMt
}


type_fn!{define_trait
    /// An iterator function that processes the collection incrementally from the start,
    /// starting with the first element.
    trait=ReduceL_ [Func]
    type=ReduceL
    fn_type=ReduceLOp
    method_like=ReduceLMt
}

type_fn!{define_trait
    /// An iterator function that processes the collection incrementally from the end,
    /// starting with the last element.
    trait=ReduceR_ [Func]
    type=ReduceR
    fn_type=ReduceROp
    method_like=ReduceRMt
}

type_fn!{define_trait
    /// Transforms the elements of the collection with the `Func` function.
    trait=Map_ [Func]
    type=Map
    fn_type=MapOp
    method_like=MapMt
}

type_fn!{define_trait
    /// Returns the collection in which all the elements that 
    /// do not satisfy the `Predicate` are removed.
    ///
    /// Predicate is the equivalent to `Fn(&T)->bool`,where T is the element type.
    trait=Filter_ [Predicate]
    type=Filter
    fn_type=FilterOp
    method_like=FilterMt
}

type_fn!{define_trait
    /// Removes the element at the `Ìndex` position from the collection.
    trait=Remove_ [Index]
    type=Remove
    fn_type=RemoveOp
    method_like=RemoveMt
}

type_fn!{define_trait
    /// Inserts `Value` at the `Ìndex` position into the collection.
    trait=Insert_ [Index,Value]
    type=Insert
    fn_type=InsertOp
    method_like=InsertMt
}

type_fn!{define_trait
    /// Returns the collection with the value added at one end.
    ///
    /// Push followed by Pop must return the pushed value and
    /// the collection as it was before pushing.
    trait=Push_ [Value]
    type=Push
    fn_type=PushOp
    method_like=PushMt
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
    method_like=PushBackMt
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
    method_like=PushFrontMt
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


type_fn!{
    fn FindOp[This,Pred](This,Pred)
    where[
        (
            TryFoldLMt< None_, (GetRhs,If<Pred,(NewSome,NewTFBreak),(NewNone,NewTFVal)>) >,
            IntoInnerOp
        ):TypeFn_< This, Output=Out >
    ]{ let Out; Out }
}

type_fn!{
    captures(Func)
    fn FindMt[This](This)
    where[ FindOp:TypeFn_<(This,Func),Output=Out> ]
    { let Out;Out }
}



type_fn!{
    fn AllOp[This,Pred](This,Pred)
    where[
        (
            TryFoldLMt<True,(GetRhs,Pred,If<IdentityFn,NewTFVal,NewTFBreak>)>,
            IntoInnerOp
        ):TypeFn_< This, Output=Out >
    ]{ let Out; Out }
}

type_fn!{
    captures(Func)
    fn AllMt[This](This)
    where[ AllOp:TypeFn_<(This,Func),Output=Out> ]
    { let Out;Out }
}



type_fn!{
    fn AnyOp[This,Pred](This,Pred)
    where[
        (
            TryFoldLMt<False,(GetRhs,Pred,If<IdentityFn,NewTFBreak,NewTFVal>)>,
            IntoInnerOp
        ):TypeFn_< This, Output=Out >
    ]{ let Out; Out }
}

type_fn!{
    captures(Func)
    fn AnyMt[This](This)
    where[ AnyOp:TypeFn_<(This,Func),Output=Out> ]
    { let Out;Out }
}

type_fn!{
    fn ContainsOp[This,Elem](This,Elem)
    where[
        AnyOp:TypeFn_< (This,ConstEqMt<Elem>), Output=Out >
    ]{
        let Out; Out 
    }
}

type_fn!{
    captures(Elem)
    fn ContainsMt[This](This)
    where[ ContainsOp:TypeFn_<(This,Elem),Output=Out> ]
    { let Out;Out }
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
    method_like=TryFoldLMt
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
    method_like=TryFoldRMt
}


#[derive(TypeLevel)]
#[typelevel(reexport(Variants))]
pub enum TryFold<T,B>{
    TFVal(T),
    TFBreak(B),
}

type_fn!{
    pub fn NewTFVal[v](v){ TFVal<v> }
}
type_fn!{
    pub fn NewTFBreak[v](v){ TFBreak<v> }
}

impl<T> IntoInner_ for TFVal<T> {
    type Output=T;
}
impl<T> IntoInner_ for TFBreak<T> {
    type Output=T;
}

/** 
Alias for converting a value to a TryFoldType.
*/
pub type IntoTryFold=ConstIntoMt<TryFoldType>;


macro_rules! define_tryfold_conv {
    ( generics[$($generic:tt)*] $from:ty : $from_consttype:ty => $try_flow:ty ) => (
        impl<$($generic)*> ConstFrom_<$from> for TryFoldType{
            type Output=$try_flow;
        }
        impl<$($generic)*> ConstInto_<$from_consttype> for $try_flow{
            type Output=$from;
        }
    )
}

define_tryfold_conv!{ generics[T] Ok_<T>:ResultType => TFVal<T> }
define_tryfold_conv!{ generics[T] Err_<T>:ResultType => TFBreak<Err_<T>> }

define_tryfold_conv!{ generics[T] Some_<T>:OptionType => TFVal<T> }
define_tryfold_conv!{ generics[]  None_   :OptionType => TFBreak<None_> }


/////////////////////////////////////////////////////////////////////////////////////////



#[cfg(test)]
mod test{
    use super::*;
    type Val0=tlist![U10,U11,U12,U13,U14];
    type ValEven=tlist![U10,U12,U14];
    type ValOdd =tlist![U11,U13,U15];

    type IsOdd =(BitAndMt<U1>,ConstEqMt<U1>);
    type IsEven=(BitAndMt<U1>,ConstEqMt<U0>);
    type IsEq<Val>=ConstEqMt<Val>;
    type IsLt<Val>=ConstLtMt<Val>;
    
    #[test]
    fn find_contains(){
        type TestFind<Val,Func,Equal>=(
            AssertFnRet<(Val,Func),FindOp, Equal >,
            AssertFnRet<Val,FindMt<Func>, Equal >,
        );
        let _:TestFind<Val0,IsOdd , Some_<U11> >;
        let _:TestFind<Val0,IsEven, Some_<U10> >;
        let _:TestFind<ValEven,IsOdd , None_ >;
        let _:TestFind<ValOdd ,IsEven, None_ >;

        let _:TestFind<Val0,IsEq<U10>, Some_<U10> >;
        let _:TestFind<Val0,IsEq<U11>, Some_<U11> >;
        let _:TestFind<Val0,IsEq<U12>, Some_<U12> >;
        let _:TestFind<Val0,IsEq<U13>, Some_<U13> >;
        let _:TestFind<Val0,IsEq<U14>, Some_<U14> >;



        type TestContains<Val,Elem,Equal>=(
            AssertFnRet<(Val,Elem),ContainsOp,Equal >,
            AssertFnRet<Val,ContainsMt<Elem>,Equal >,
        );

        let _:TestContains<ValEven,U10,True>;
        let _:TestContains<ValEven,U12,True>;
        let _:TestContains<ValEven,U14,True>;
        let _:TestContains<ValOdd ,U11,True>;
        let _:TestContains<ValOdd ,U13,True>;
        let _:TestContains<ValOdd ,U15,True>;
        
        let _:TestContains<ValOdd ,U10,False>;
        let _:TestContains<ValOdd ,U12,False>;
        let _:TestContains<ValOdd ,U14,False>;
        let _:TestContains<ValEven,U11,False>;
        let _:TestContains<ValEven,U13,False>;
        let _:TestContains<ValEven,U15,False>;
    }

    #[test]
    fn all_any(){
        type TestAll<Val,Func,Equal>=(
            AssertFnRet<(Val,Func),AllOp,Equal >,
            AssertFnRet<Val,AllMt<Func>, Equal >,
        );

        let _:TestAll<Val0,IsLt<U12>,False>;
        let _:TestAll<Val0,IsLt<U13>,False>;
        let _:TestAll<Val0,IsLt<U14>,False>;
        let _:TestAll<Val0,IsLt<U15>,True>;
        let _:TestAll<Val0,IsLt<U16>,True>;
        let _:TestAll<Val0,IsLt<U17>,True>;



        type TestAny<Val,Func,Equal>=(
            AssertFnRet<(Val,Func),AnyOp, Equal >,
            AssertFnRet<Val,AnyMt<Func>, Equal >,
        );

        let _:TestAny<Val0,IsLt<U8 >,False>;
        let _:TestAny<Val0,IsLt<U9 >,False>;
        let _:TestAny<Val0,IsLt<U10>,False>;
        let _:TestAny<Val0,IsLt<U11>,True>;
        let _:TestAny<Val0,IsLt<U12>,True>;
        let _:TestAny<Val0,IsLt<U13>,True>;
        let _:TestAny<Val0,IsLt<U14>,True>;
        let _:TestAny<Val0,IsLt<U15>,True>;
        let _:TestAny<Val0,IsLt<U16>,True>;
    }
}