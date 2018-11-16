use prelude::*;
use crate_::fn_adaptors::*;
use crate_::std_ops::{AddOp, SubOp,SubRevMt};
use crate_::ops::{
    AssertEq,
    ConstOrd_,
    ConstEq,ConstEqOp,
    ConstLtOp,ConstGEOp, ConstLEOp, ConstNEOp,
    Add1Op,SatSub1Op,SatSubOp,Sub1Op,IsZeroOp,SafeSubOp,
    ConstIntoMt,ConstInto_,ConstInto,
    If,

};
use crate_::collection_ops::{
    FoldL_   ,FoldL   , FoldR_   ,FoldR   ,
    TryFoldL_,TryFoldLMt,TryFoldL, TryFoldR_,TryFoldR,
    Map_,Map,MapMt,Len_,Len,
    PushOp,Repeat_,TFVal,TFBreak,
};

use crate_::std_types::cmp_ordering::{Equal_, Greater_, Less_, OrderingTrait};
use crate_::new_types::type_list::{TListType,TNil};

use core_extensions::type_level_bool::{Boolean, False, True};
use core_extensions::TypeIdentity;


use std_::ops::{
    Add, Range as StdRange, RangeFrom as StdRangeFrom, RangeFull as StdRangeFull,
    RangeTo as StdRangeTo, Sub,
};
#[cfg(rust_1_26)]
use std_::ops::{RangeInclusive as StdRangeInclusive, RangeToInclusive as StdRangeToInclusive};


/// Contains the type-level equivalent of std::ops::Range.
pub mod range {
    use super::*;
    #[derive(TypeLevel)]
    #[typelevel(
        // print_derive,
        // skip_derive,
        reexport = "pub",
        derive(ConstEq, ConstOrd),
        items(runtime_conv(Internal="StdRange")),
    )]
    #[allow(dead_code)]
    #[doc(hidden)]
    pub struct Range<T> {
        #[typelevel(doc="the start of the range")]
        pub start: T,
        pub end: T,
    }



}


//////////////////////////////////////////////////////////////////////////////////

/// Contains the type-level equivalent of std::ops::RangeFrom.
pub mod range_from {
    use super::*;

    #[derive(TypeLevel)]
    #[typelevel(
        // print_derive,
        // skip_derive,
        reexport = "pub",
        derive(ConstEq, ConstOrd),
        items(IntoRuntime(Internal (Type="StdRangeFrom",Manual))),
        items(IntoConstType(Internal = "StdRangeFrom")),
    )]
    #[allow(dead_code)]
    #[doc(hidden)]
    pub struct RangeFrom<Start> {
        pub start: Start,
    }

    impl<St, T> IntoRuntime<StdRangeFrom<T>> for ConstRangeFrom<St>
    where
        St: IntoRuntime<T>,
    {
        fn to_runtime() -> StdRangeFrom<T> {
            St::to_runtime()..
        }
    }

    #[cfg(rust_1_22)]
    impl<St, T> IntoConstant<StdRangeFrom<T>> for ConstRangeFrom<St>
    where
        St: IntoConstant<T>,
    {
        const VALUE: StdRangeFrom<T> = St::VALUE..;
    }

}

/////////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////////

/// Contains the type-level equivalent of std::ops::RangeFull.
pub mod range_full {
    use super::*;

    #[derive(TypeLevel)]
    #[typelevel(
        reexport = "pub",
        derive(ConstEq, ConstOrd),
        items(IntoRuntime(Internal(Type = "StdRangeFull", Manual))),
        items(IntoConstType(Internal = "StdRangeFull")),
    )]
    #[allow(dead_code)]
    #[doc(hidden)]
    pub struct RangeFull;

    impl IntoRuntime<StdRangeFull> for ConstRangeFull {
        fn to_runtime() -> StdRangeFull {
            ..
        }
    }

    #[cfg(rust_1_22)]
    impl IntoConstant<StdRangeFull> for ConstRangeFull {
        const VALUE: StdRangeFull = ..;
    }

}

/// Contains the type-level equivalent of std::ops::RangeTo.
pub mod range_to {
    use super::*;

    #[derive(TypeLevel)]
    #[typelevel(
        reexport = "pub",
        derive(ConstEq, ConstOrd),
        items(IntoRuntime(Internal(Type = "StdRangeTo", Manual))),
        items(IntoConstType(Internal = "StdRangeTo")),
    )]
    #[allow(dead_code)]
    #[doc(hidden)]
    pub struct RangeTo<End> {
        pub end: End,
    }

    impl<End, T> IntoRuntime<StdRangeTo<T>> for ConstRangeTo<End>
    where
        End: IntoRuntime<T>,
    {
        fn to_runtime() -> StdRangeTo<T> {
            ..End::to_runtime()
        }
    }

    #[cfg(rust_1_22)]
    impl<End, T> IntoConstant<StdRangeTo<T>> for ConstRangeTo<End>
    where
        End: IntoConstant<T>,
    {
        const VALUE: StdRangeTo<T> = ..End::VALUE;
    }

}

/// Contains the type-level equivalent of std::ops::RangeInclusive.
#[cfg(rust_1_26)]
pub mod range_inclusive {
    use super::*;

    #[derive(TypeLevel)]
    #[typelevel(
        reexport = "pub",
        derive(ConstEq, ConstOrd),
        items(IntoRuntime(Internal(Type = "StdRangeInclusive", Manual))),
        items(IntoConstType(Internal = "StdRangeInclusive")),
    )]
    #[allow(dead_code)]
    #[doc(hidden)]
    pub struct RangeInclusive<T> {
        pub start: T,
        pub end: T,
    }

    #[cfg(rust_1_27)]
    impl<St, End, T> IntoRuntime<StdRangeInclusive<T>> for ConstRangeInclusive<St, End>
    where
        St: IntoRuntime<T>,
        End: IntoRuntime<T>,
    {
        fn to_runtime() -> StdRangeInclusive<T> {
            StdRangeInclusive::new(St::to_runtime(), End::to_runtime())
        }
    }

    #[cfg(rust_1_27)]
    impl<St, End, T> IntoConstant<StdRangeInclusive<T>> for ConstRangeInclusive<St, End>
    where
        St: IntoConstant<T>,
        End: IntoConstant<T>,
    {
        const VALUE: StdRangeInclusive<T> = StdRangeInclusive::new(St::VALUE, End::VALUE);
    }

}


/// Contains the type-level equivalent of std::ops::RangeToInclusive.
#[cfg(rust_1_26)]
pub mod range_to_inclusive {
    use super::*;

    #[derive(TypeLevel)]
    #[typelevel(
        reexport = "pub",
        derive(ConstEq, ConstOrd),
        items(IntoRuntime(Internal(Type = "StdRangeToInclusive", Manual))),
        items(IntoConstType(Internal = "StdRangeToInclusive")),
    )]
    #[allow(dead_code)]
    #[doc(hidden)]
    pub struct RangeToInclusive<End> {
        pub end: End,
    }

    #[cfg(rust_1_27)]
    impl<End, T> IntoRuntime<StdRangeToInclusive<T>> for ConstRangeToInclusive<End>
    where
        End: IntoRuntime<T>,
    {
        fn to_runtime() -> StdRangeToInclusive<T> {
            StdRangeToInclusive {
                end: End::to_runtime(),
            }
        }
    }

    #[cfg(rust_1_27)]
    impl<End, T> IntoConstant<StdRangeToInclusive<T>> for ConstRangeToInclusive<End>
    where
        End: IntoConstant<T>,
    {
        const VALUE: StdRangeToInclusive<T> = StdRangeToInclusive { end: End::VALUE };
    }

}

//////////////////////////////////////////////////////////////////////////////////
////////////            Collection Trait impls
//////////////////////////////////////////////////////////////////////////////////


type_fn!{
    captures(GetNextPos,Func)
    fn FoldNext[Pos,State]((Pos,State),())
    where[
        GetNextPos:TypeFn_<Pos,Output=NPos>,
        Func:TypeFn_<(State,Pos),Output=NState>
    ]{
        let NPos;let NState;
        (NPos,NState)
    }
}

type_fn!{
    captures(GetNextPos,Func)
    fn TryFoldNext[Pos,State]((Pos,State),())
    where[
        GetNextPos:TypeFn_<Pos,Output=NPos>,
        (Func,MapMt<TryFoldNextTuple<NPos>>):TypeFn_<(State,Pos),Output=Out>,
    ]{
        let NPos;let Out;
        Out
    }
}
type_fn!{
    captures(position)
    pub fn TryFoldNextTuple[state](state){
        (position,state)
    }
}
type_fn!{
    pub fn
        ExtractState[Pos,State](TFVal<(Pos,State)>){ TFVal<State> }
        ExtractState[E](TFBreak<E>){ TFBreak<E> }

}

impl<DefVal,Func,Leng,list,S,E,_0,Out> FoldL_<DefVal,Func> for ConstRange<S,E>
where 
    Self:Len_<Output=Leng>,
    TListType:Repeat_<(),Leng,Output=list>,
    list:FoldL_<(S,DefVal) , FoldNext<Add1Op,Func>,Output=(_0,Out)>
{
    type Output=Out;
}

impl<DefVal,Func,Leng,list,S,E,_0,EndSub1,Out> FoldR_<DefVal,Func> for ConstRange<S,E>
where 
    SatSub1Op:TypeFn_<E,Output=EndSub1>,
    Self:Len_<Output=Leng>,
    TListType:Repeat_<(),Leng,Output=list>,
    list:FoldL_<(EndSub1,DefVal) , FoldNext<SatSub1Op,Func>,Output=(_0,Out)>
{
    type Output=Out;
}

/////////////////////////////////////


#[cfg(rust_1_26)]
impl<DefVal,Func,Leng,list,S,E,_0,Out> FoldL_<DefVal,Func> for ConstRangeInclusive<S,E>
where 
    Self:Len_<Output=Leng>,
    TListType:Repeat_<(),Leng,Output=list>,
    list:FoldL_<(S,DefVal) , FoldNext<Add1Op,Func>,Output=(_0,Out)>
{
    type Output=Out;
}

#[cfg(rust_1_26)]
impl<DefVal,Func,Leng,list,S,E,_0,Out> FoldR_<DefVal,Func> for ConstRangeInclusive<S,E>
where 
    Self:Len_<Output=Leng>,
    TListType:Repeat_<(),Leng,Output=list>,
    list:FoldL_<(E,DefVal) , FoldNext<SatSub1Op,Func>,Output=(_0,Out)>
{
    type Output=Out;
}

/////////////////////////////////////


impl<DefVal,Func,Leng,list,S,E,Out> TryFoldL_<DefVal,Func> for ConstRange<S,E>
where 
    Self:Len_<Output=Leng>,
    TListType:Repeat_<(),Leng,Output=list>,
    (   
        TryFoldLMt<(S,DefVal),TryFoldNext<Add1Op,Func>>,
        ExtractState
    ):TypeFn_<list,Output=Out>
{
    type Output=Out;
}

impl<DefVal,Func,Leng,list,S,E,EndSub1,Out> TryFoldR_<DefVal,Func> for ConstRange<S,E>
where 
    SatSub1Op:TypeFn_<E,Output=EndSub1>,
    Self:Len_<Output=Leng>,
    TListType:Repeat_<(),Leng,Output=list>,
    (
        TryFoldLMt<(EndSub1,DefVal),TryFoldNext<SatSub1Op,Func>>,
        ExtractState
    ):TypeFn_<list,Output=Out>
{
    type Output=Out;
}

/////////////////////////////////////


#[cfg(rust_1_26)]
impl<DefVal,Func,Leng,list,S,E,Out> TryFoldL_<DefVal,Func> for ConstRangeInclusive<S,E>
where 
    Self:Len_<Output=Leng>,
    TListType:Repeat_<(),Leng,Output=list>,
    (
        TryFoldLMt<(S,DefVal),TryFoldNext<Add1Op,Func>>,
        ExtractState
    ):TypeFn_<list,Output=Out>
{
    type Output=Out;
}

#[cfg(rust_1_26)]
impl<DefVal,Func,Leng,list,S,E,Out> TryFoldR_<DefVal,Func> for ConstRangeInclusive<S,E>
where 
    Self:Len_<Output=Leng>,
    TListType:Repeat_<(),Leng,Output=list>,
    (
        TryFoldLMt<(E,DefVal),TryFoldNext<SatSub1Op,Func>>,
        ExtractState
    ):TypeFn_<list,Output=Out>
{
    type Output=Out;
}

/////////////////////////////////////


impl<S,E,list,Mapper,Out> Map_<Mapper> for ConstRange<S,E>
where 
    Self:ConstInto_<TListType,Output=list>,
    list:Map_<Mapper,Output=Out>,
{
    type Output=Out;
}

#[cfg(rust_1_26)]
impl<S,E,list,Mapper,Out> Map_<Mapper> for ConstRangeInclusive<S,E>
where 
    Self:ConstInto_<TListType,Output=list>,
    list:Map_<Mapper,Output=Out>,
{
    type Output=Out;
}

/////////////////////////////////////

impl<S,E,Out> Len_ for ConstRange<S,E>
where 
    SatSubOp:TypeFn_<(E,S),Output=Out>,
{
    type Output=Out;
}

#[cfg(rust_1_26)]
impl<S,E,Out> Len_ for ConstRangeInclusive<S,E>
where 
    If<ConstLtOp,Const<U0>,(SubOp,Add1Op)>:TypeFn_<(E,S),Output=Out>,
{
    type Output=Out;
}

/////////////////////////////////////

impl<S,E,Out> ConstInto_<TListType> for ConstRange<S,E>
where 
    Self:FoldR_<TNil,PushOp,Output=Out>
{
    type Output=Out;
}

#[cfg(rust_1_26)]
impl<S,E,Out> ConstInto_<TListType> for ConstRangeInclusive<S,E>
where 
    Self:FoldR_<TNil,PushOp,Output=Out>
{
    type Output=Out;
}


//////////////////////////////////////////////////////////////////////////////////
////////////                Constructors
//////////////////////////////////////////////////////////////////////////////////


#[allow(unused_macros)]
macro_rules! rangei_ {
    (value,$($rest:tt)*)=>{ <rangei_!($($rest)*) as $crate::RuntimeValue>::MTVAL };
    ($start:ty => $end:ty) =>($crate::std_types::range_inclusive::ConstRangeInclusive<$start,$end>);
    (          => $end:ty) =>($crate::std_types::range_to_inclusive::ConstRangeToInclusive<$end> );
    ($start:ty =>        ) =>($crate::std_types::range_from::ConstRangeFrom<$start> );
    (                    ) =>($crate::std_types::range_full::ConstRangeFull );
}

#[allow(unused_macros)]
macro_rules! range_ {
    (value,$($rest:tt)*)=>{ <range_!($($rest)*) as $crate::RuntimeValue>::MTVAL };
    ($start:ty => $end:ty) => ( $crate::std_types::range::ConstRange<$start,$end> );
    (          => $end:ty) => ( $crate::std_types::range_to::ConstRangeTo<$end> );
    ($start:ty =>        ) => ( $crate::std_types::range_from::ConstRangeFrom<$start> );
    (                    ) => ( $crate::std_types::range_full::ConstRangeFull );
}

// #[cfg(test)]
#[cfg(all(test,feature="passed_tests"))]
mod test_eq {
    use super::*;

    #[test]
    #[cfg(rust_1_26)]
    fn test_eq_range() {
        let _:AssertEq< ConstEq< range_!(U0=>U0), range_!(U0=>U0)> , True >;
        let _:AssertEq< ConstEq< rangei_!(U0=>U0), rangei_!(U0=>U0)> , True >;
        let _:AssertEq< ConstEq< range_!(U0=>U1), range_!(U0=>U0)> , False >;
        let _:AssertEq< ConstEq< rangei_!(U0=>U1), rangei_!(U0=>U0)> , False >;

        let _:AssertEq< ConstEq< range_!(=>U0), range_!(=>U0)> , True >;
        let _:AssertEq< ConstEq< rangei_!(=>U0), rangei_!(=>U0)> , True >;
        let _:AssertEq< ConstEq< range_!(=>U1), range_!(=>U0)> , False >;
        let _:AssertEq< ConstEq< rangei_!(=>U1), rangei_!(=>U0)> , False >;

        let _:AssertEq< ConstEq< range_!(U0=>), range_!(U0=>)> , True >;
        let _:AssertEq< ConstEq< rangei_!(U0=>), rangei_!(U0=>)> , True >;
        let _:AssertEq< ConstEq< range_!(U1=>), range_!(U0=>)> , False >;
        let _:AssertEq< ConstEq< rangei_!(U1=>), rangei_!(U0=>)> , False >;

        let _:AssertEq< ConstEq< range_!(), range_!()> , True >;
        let _:AssertEq< ConstEq< rangei_!(), rangei_!()> , True >;
    }

    #[test]
    fn len_(){
        type Test<Rang,Leng>=
            AssertEq< Len<Rang> , Leng>;

        let _:Test<ConstRange<U2,U0>,U0>;
        let _:Test<ConstRange<U1,U0>,U0>;
        let _:Test<ConstRange<U0,U0>,U0>;
        let _:Test<ConstRange<U0,U1>,U1>;
        let _:Test<ConstRange<U0,U2>,U2>;
        let _:Test<ConstRange<U0,U3>,U3>;


        #[cfg(rust_1_26)]
        {
            let _:Test<ConstRangeInclusive<U2,U0>,U0>;
            let _:Test<ConstRangeInclusive<U1,U0>,U0>;
            let _:Test<ConstRangeInclusive<U0,U0>,U1>;
            let _:Test<ConstRangeInclusive<U0,U1>,U2>;
            let _:Test<ConstRangeInclusive<U0,U2>,U3>;
            let _:Test<ConstRangeInclusive<U0,U3>,U4>;
        }

    }
    #[test]
    fn into_list(){
        type Test<Rang,List>=
            AssertEq<ConstInto<Rang,TListType>,List>;

        let _:Test<ConstRange<U2,U0>,TNil>;
        let _:Test<ConstRange<U1,U0>,TNil>;
        let _:Test<ConstRange<U0,U0>,TNil>;
        let _:Test<ConstRange<U0,U1>,tlist![U0]>;
        let _:Test<ConstRange<U0,U2>,tlist![U0,U1]>;
        let _:Test<ConstRange<U0,U3>,tlist![U0,U1,U2]>;

        #[cfg(rust_1_26)]
        {
            let _:Test<ConstRangeInclusive<U2,U0>,TNil>;
            let _:Test<ConstRangeInclusive<U1,U0>,TNil>;
            let _:Test<ConstRangeInclusive<U0,U0>,tlist![U0]>;
            let _:Test<ConstRangeInclusive<U0,U1>,tlist![U0,U1]>;
            let _:Test<ConstRangeInclusive<U0,U2>,tlist![U0,U1,U2]>;
            let _:Test<ConstRangeInclusive<U0,U3>,tlist![U0,U1,U2,U3]>;
        }
    }

    #[test]
    fn fold(){
        type TestL<Rang,Val>=
            AssertEq< FoldL<Rang,TNil,PushOp>,Val>;
        type TestR<Rang,Val>=
            AssertEq< FoldR<Rang,TNil,PushOp>,Val>;
            

        let _:TestL<ConstRange<U2,U0>,tlist![] >;
        let _:TestL<ConstRange<U1,U0>,tlist![] >;
        let _:TestL<ConstRange<U0,U0>,tlist![] >;
        let _:TestL<ConstRange<U0,U1>,tlist![U0] >;
        let _:TestL<ConstRange<U0,U2>,tlist![U1,U0] >;
        let _:TestL<ConstRange<U0,U3>,tlist![U2,U1,U0] >;

        let _:TestR<ConstRange<U2,U0>,tlist![] >;
        let _:TestR<ConstRange<U1,U0>,tlist![] >;
        let _:TestR<ConstRange<U0,U0>,tlist![] >;
        let _:TestR<ConstRange<U0,U1>,tlist![U0] >;
        let _:TestR<ConstRange<U0,U2>,tlist![U0,U1] >;
        let _:TestR<ConstRange<U0,U3>,tlist![U0,U1,U2] >;

        #[cfg(rust_1_26)]
        {
            let _:TestL<ConstRangeInclusive<U2,U0>,tlist![] >;
            let _:TestL<ConstRangeInclusive<U1,U0>,tlist![] >;
            let _:TestL<ConstRangeInclusive<U0,U0>,tlist![U0] >;
            let _:TestL<ConstRangeInclusive<U0,U1>,tlist![U1,U0] >;
            let _:TestL<ConstRangeInclusive<U0,U2>,tlist![U2,U1,U0] >;
            let _:TestL<ConstRangeInclusive<U0,U3>,tlist![U3,U2,U1,U0] >;

            let _:TestR<ConstRangeInclusive<U2,U0>,tlist![] >;
            let _:TestR<ConstRangeInclusive<U1,U0>,tlist![] >;
            let _:TestR<ConstRangeInclusive<U0,U0>,tlist![U0] >;
            let _:TestR<ConstRangeInclusive<U0,U1>,tlist![U0,U1] >;
            let _:TestR<ConstRangeInclusive<U0,U2>,tlist![U0,U1,U2] >;
            let _:TestR<ConstRangeInclusive<U0,U3>,tlist![U0,U1,U2,U3] >;
        }
    }

    #[test]
    fn try_fold(){
        type_fn!{
            fn TryIteration[Val,List,Elem]((Val,List),Elem)
            where [
                (SafeSubOp,MapMt<push_keep<List>>):TypeFn_<(Val,Elem),Output=Out>,
            ]{
                let Out;
                Out
            }
        }
        type_fn!{
            captures(list)
            fn push_keep[Val](Val)
            where [ PushOp:TypeFn_<(list,Val),Output=NewList> ]
            {
                let NewList;
                (Val,NewList)
            }
        }

        type TestL<Rang,Initial,Val>=
            AssertEq< TryFoldL<Rang,Initial,TryIteration>,Val>;
        type TestR<Rang,Initial,Val>=
            AssertEq< TryFoldR<Rang,Initial,TryIteration>,Val>;

        let _:TestL<ConstRange<U2,U0>,(U10,TNil),TFVal<(U10,tlist![])> >;
        let _:TestL<ConstRange<U1,U0>,(U10,TNil),TFVal<(U10,tlist![])> >;
        let _:TestL<ConstRange<U0,U0>,(U10,TNil),TFVal<(U10,tlist![])> >;
        let _:TestL<ConstRange<U0,U1>,(U10,TNil),TFVal<(U10,tlist![U10])> >;
        let _:TestL<ConstRange<U0,U2>,(U10,TNil),TFVal<(U9,tlist![U9,U10])> >;
        let _:TestL<ConstRange<U0,U3>,(U10,TNil),TFVal<(U7,tlist![U7,U9,U10])> >;
        let _:TestL<ConstRange<U0,U4>,(U10,TNil),TFVal<(U4,tlist![U4,U7,U9,U10])> >;
        let _:TestL<ConstRange<U0,U5>,(U10,TNil),TFVal<(U0,tlist![U0,U4,U7,U9,U10])> >;
        let _:TestL<ConstRange<U0,U6>,(U10,TNil),TFBreak<None_>>;

        let _:TestR<ConstRange<U2,U0>,(U10,TNil),TFVal<(U10,tlist![])> >;
        let _:TestR<ConstRange<U1,U0>,(U10,TNil),TFVal<(U10,tlist![])> >;
        let _:TestR<ConstRange<U0,U0>,(U10,TNil),TFVal<(U10,tlist![])> >;
        let _:TestR<ConstRange<U0,U1>,(U10,TNil),TFVal<(U10,tlist![U10])> >;
        let _:TestR<ConstRange<U0,U2>,(U10,TNil),TFVal<(U9,tlist![U9,U9])> >;
        let _:TestR<ConstRange<U0,U3>,(U10,TNil),TFVal<(U7,tlist![U7,U7,U8])> >;
        let _:TestR<ConstRange<U0,U4>,(U10,TNil),TFVal<(U4,tlist![U4,U4,U5,U7])> >;
        let _:TestR<ConstRange<U0,U5>,(U10,TNil),TFVal<(U0,tlist![U0,U0,U1,U3,U6])> >;
        let _:TestR<ConstRange<U0,U6>,(U10,TNil),TFBreak<None_>>;

        #[cfg(rust_1_26)]
        {
            use self::ConstRangeInclusive as CRI;
            let _:TestL<CRI<U2,U0>,(U10,TNil),TFVal<(U10,tlist![])> >;
            let _:TestL<CRI<U1,U0>,(U10,TNil),TFVal<(U10,tlist![])> >;
            let _:TestL<CRI<U0,U0>,(U10,TNil),TFVal<(U10,tlist![U10])> >;
            let _:TestL<CRI<U0,U1>,(U10,TNil),TFVal<(U9,tlist![U9,U10])> >;
            let _:TestL<CRI<U0,U2>,(U10,TNil),TFVal<(U7,tlist![U7,U9,U10])> >;
            let _:TestL<CRI<U0,U3>,(U10,TNil),TFVal<(U4,tlist![U4,U7,U9,U10])> >;
            let _:TestL<CRI<U0,U4>,(U10,TNil),TFVal<(U0,tlist![U0,U4,U7,U9,U10])> >;
            let _:TestL<CRI<U0,U5>,(U10,TNil),TFBreak<None_>>;

            let _:TestR<CRI<U2,U0>,(U10,TNil),TFVal<(U10,tlist![])> >;
            let _:TestR<CRI<U1,U0>,(U10,TNil),TFVal<(U10,tlist![])> >;
            let _:TestR<CRI<U0,U0>,(U10,TNil),TFVal<(U10,tlist![U10])> >;
            let _:TestR<CRI<U0,U1>,(U10,TNil),TFVal<(U9,tlist![U9,U9])> >;
            let _:TestR<CRI<U0,U2>,(U10,TNil),TFVal<(U7,tlist![U7,U7,U8])> >;
            let _:TestR<CRI<U0,U3>,(U10,TNil),TFVal<(U4,tlist![U4,U4,U5,U7])> >;
            let _:TestR<CRI<U0,U4>,(U10,TNil),TFVal<(U0,tlist![U0,U0,U1,U3,U6])> >;
            let _:TestR<CRI<U0,U5>,(U10,TNil),TFBreak<None_>>;            
        }
    }


    #[test]
    fn map(){
        type TestA<Rang,Val>=
            AssertEq<Map<Rang,SubRevMt<U10>>,Val>;

        let _:TestA<ConstRange<U2,U0>,tlist![]>;
        let _:TestA<ConstRange<U2,U1>,tlist![]>;
        let _:TestA<ConstRange<U2,U2>,tlist![]>;
        let _:TestA<ConstRange<U2,U3>,tlist![U8]>;
        let _:TestA<ConstRange<U2,U4>,tlist![U8,U7]>;


        #[cfg(rust_1_26)]
        {
            let _:TestA<ConstRangeInclusive<U2,U0>,tlist![]>;
            let _:TestA<ConstRangeInclusive<U2,U1>,tlist![]>;
            let _:TestA<ConstRangeInclusive<U2,U2>,tlist![U8]>;
            let _:TestA<ConstRangeInclusive<U2,U3>,tlist![U8,U7]>;
            let _:TestA<ConstRangeInclusive<U2,U4>,tlist![U8,U7,U6]>;
        }
    }
}
