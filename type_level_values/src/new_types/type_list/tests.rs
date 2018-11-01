use super::*;

use crate_::std_ops::{SubOp,AddMt};

use crate_::field_traits::{GetField, SetField};
use crate_::ops::{
    AssertEq,AssertFnRet,ConstGEOp,
    Add1Op,Add1Op as AddOne,SafeDivOp,
    ConstInto,ConstIntoMt,
    ConstFrom,
    ConstEqMt,
};
use crate_::collection_ops::*;

#[test]
fn the_macro() {
    let _:AssertEq< TNil , tlist![] >;
    let _:AssertEq< TList<U0, TNil> , tlist![U0] >;
    let _:AssertEq< TList<U0, TList<U1, TNil>> , tlist![U0, U1] >;
    let _:AssertEq< TList<U0, TList<U1, TList<U2, TNil>>> , tlist![U0, U1, U2] >;
}
#[test]
fn insert() {
    type TestInsert<List,Pos,Val,Ret>=(
        AssertEq<Insert<List,Pos,Val>,Ret>,
        AssertFnRet<List,InsertMt<Pos,Val>,Ret>,
    );


    let _:TestInsert<tlist![], U0, U4,tlist![U4]>;
    let _:TestInsert<tlist![U0], U0, U4,tlist![U4, U0]>;
    let _:TestInsert<tlist![U0, U1], U0, U4,tlist![U4, U0, U1]>;
    let _:TestInsert<tlist![U0, U1, U2], U0, U4,tlist![U4, U0, U1, U2]>;

    let _:TestInsert<tlist![U0], U1, U4,tlist![U0, U4]>;
    let _:TestInsert<tlist![U0, U1], U1, U4,tlist![U0, U4, U1]>;
    let _:TestInsert<tlist![U0, U1, U2], U1, U4,tlist![U0, U4, U1, U2]>;

    let _:TestInsert<tlist![U0, U1], U2, U4,tlist![U0, U1, U4]>;
    let _:TestInsert<tlist![U0, U1, U2], U2, U4,tlist![U0, U1, U4, U2]>;

    let _:TestInsert<tlist![U0, U1, U2], U3, U4,tlist![U0, U1, U2, U4]>;

    let _:TestInsert<
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], 
        U0, 
        False,
        tlist![False, U0, U2, U4, U6, U8, U10, U12, U14, U16, U18]
    >;
    let _:TestInsert<
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], 
        U1, 
        False,
        tlist![U0, False, U2, U4, U6, U8, U10, U12, U14, U16, U18]
    >;
    let _:TestInsert<
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], 
        U2, 
        False,
        tlist![U0, U2, False, U4, U6, U8, U10, U12, U14, U16, U18]
    >;
    let _:TestInsert<
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], 
        U3, 
        False,
        tlist![U0, U2, U4, False, U6, U8, U10, U12, U14, U16, U18]
    >;
    let _:TestInsert<
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], 
        U4, 
        False,
        tlist![U0, U2, U4, U6, False, U8, U10, U12, U14, U16, U18]
    >;
    let _:TestInsert<
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], 
        U5, 
        False,
        tlist![U0, U2, U4, U6, U8, False, U10, U12, U14, U16, U18]
    >;
    let _:TestInsert<
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], 
        U6, 
        False,
        tlist![U0, U2, U4, U6, U8, U10, False, U12, U14, U16, U18]
    >;
    let _:TestInsert<
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], 
        U7, 
        False,
        tlist![U0, U2, U4, U6, U8, U10, U12, False, U14, U16, U18]
    >;
    let _:TestInsert<
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], 
        U8, 
        False,
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, False, U16, U18]
    >;
    let _:TestInsert<
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], 
        U9, 
        False,
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, False, U18]
    >;
    let _:TestInsert<
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], 
        U10, 
        False,
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18, False]
    >;
}

#[test]
fn remove() {
    type TestRemove<List,Pos,Ret>=(
        AssertEq<Remove<List,Pos>,Ret>,
        AssertFnRet<List,RemoveMt<Pos>,Ret>,
    );

    let _:TestRemove< tlist![U0], U0,tlist![]>;

    let _:TestRemove< tlist![U0, U1], U0,tlist![U1]>;
    let _:TestRemove< tlist![U0, U1], U1,tlist![U0]>;

    let _:TestRemove< tlist![U0, U1, U2], U0,tlist![U1, U2]>;
    let _:TestRemove< tlist![U0, U1, U2], U1,tlist![U0, U2]>;
    let _:TestRemove< tlist![U0, U1, U2], U2,tlist![U0, U1]>;

    let _:TestRemove< tlist![U0, U1, U2, U3], U0,tlist![U1, U2, U3]>;
    let _:TestRemove< tlist![U0, U1, U2, U3], U1,tlist![U0, U2, U3]>;
    let _:TestRemove< tlist![U0, U1, U2, U3], U2,tlist![U0, U1, U3]>;
    let _:TestRemove< tlist![U0, U1, U2, U3], U3,tlist![U0, U1, U2]>;

    let _:TestRemove<
        tlist![False, U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], 
        U0,
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18],
    >;
    let _:TestRemove<
        tlist![U0, False, U2, U4, U6, U8, U10, U12, U14, U16, U18], 
        U1,
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18],
    >;
    let _:TestRemove<
        tlist![U0, U2, False, U4, U6, U8, U10, U12, U14, U16, U18], 
        U2,
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18],
    >;
    let _:TestRemove<
        tlist![U0, U2, U4, False, U6, U8, U10, U12, U14, U16, U18], 
        U3,
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18],
    >;
    let _:TestRemove<
        tlist![U0, U2, U4, U6, False, U8, U10, U12, U14, U16, U18], 
        U4,
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18],
    >;
    let _:TestRemove<
        tlist![U0, U2, U4, U6, U8, False, U10, U12, U14, U16, U18], 
        U5,
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18],
    >;
    let _:TestRemove<
        tlist![U0, U2, U4, U6, U8, U10, False, U12, U14, U16, U18], 
        U6,
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18],
    >;
    let _:TestRemove<
        tlist![U0, U2, U4, U6, U8, U10, U12, False, U14, U16, U18], 
        U7,
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18],
    >;
    let _:TestRemove<
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, False, U16, U18], 
        U8,
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18],
    >;
    let _:TestRemove<
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, False, U18], 
        U9,
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18],
    >;
    let _:TestRemove<
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18, False], 
        U10,
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18],
    >;
}

macro_rules! test_push_pop_front {
    ($push_fn:ident => $push_ta:ident $pop_fn:ident => $pop_ta:ident) => {
        #[test]
        fn $push_fn() {
            let _:$push_ta<tlist![], U0,tlist![U0]>;
            let _:$push_ta<tlist![U0], U1,tlist![U1, U0]>;
            let _:$push_ta<tlist![U1, U0], U2,tlist![U2, U1, U0]>;
            let _:$push_ta<tlist![U2, U1, U0], U3,tlist![U3, U2, U1, U0]>;
        }
        #[test]
        fn $pop_fn() {
            let _:AssertEq< None_ , $pop_ta<tlist![]>>;
            let _:AssertEq< Some_<(U0, tlist![])> , $pop_ta<tlist![U0]>>;
            let _:AssertEq< Some_<(U0, tlist![U1])> , $pop_ta<tlist![U0, U1]>>;
            let _:AssertEq< Some_<(U0, tlist![U1, U2])> , $pop_ta<tlist![U0, U1, U2]>>;
            let _:AssertEq< Some_<(U0, tlist![U1, U2, U3])> , $pop_ta<tlist![U0, U1, U2, U3]>>;
        }
    };
}

type TestPushFront<List,Val,Expected>=(
    AssertEq<PushFront<List,Val>,Expected>,
    AssertFnRet<List,PushFrontMt<Val>,Expected>
);

type TestPush<List,Val,Expected>=(
    AssertEq<Push<List,Val>,Expected>,
    AssertFnRet<List,PushMt<Val>,Expected>
);

test_push_pop_front!{
    push=>TestPush
    pop=>Pop
}

test_push_pop_front!{
    push_front=>TestPushFront
    pop_front=>PopFront
}

#[test]
fn len() {
    let _:AssertEq< U0 , Len<tlist![]>>;
    let _:AssertEq< U1 , Len<tlist![U0]>>;
    let _:AssertEq< U2 , Len<tlist![U0, U1]>>;
    let _:AssertEq< U3 , Len<tlist![U0, U1, U2]>>;
    let _:AssertEq< U4 , Len<tlist![U0, U1, U2, U3]>>;
}

#[test]
fn const_ord() {
    let _:AssertEq< Equal_ , ConstOrd<tlist![], tlist![]>>;
    let _:AssertEq< Less_ , ConstOrd<tlist![], tlist![U0]>>;
    let _:AssertEq< Greater_ , ConstOrd<tlist![U0], tlist![]>>;

    let _:AssertEq< Less_ , ConstOrd<tlist![U0], tlist![U1]>>;
    let _:AssertEq< Equal_ , ConstOrd<tlist![U1], tlist![U1]>>;
    let _:AssertEq< Greater_ , ConstOrd<tlist![U2], tlist![U1]>>;

    let _:AssertEq< Equal_ , ConstOrd<tlist![U1, U2], tlist![U1, U2]>>;
    let _:AssertEq< Greater_ , ConstOrd<tlist![U2], tlist![U0]>>;
    let _:AssertEq< Greater_ , ConstOrd<tlist![U2], tlist![U1, U3]>>;
    let _:AssertEq< Greater_ , ConstOrd<tlist![U2], tlist![U1, U3, U4, U5]>>;

    let _:AssertEq< Less_ , ConstOrd<tlist![U0], tlist![U1, U3]>>;
    let _:AssertEq< Less_ , ConstOrd<tlist![U0], tlist![U1, U3, U4, U5]>>;
}

#[test]
fn const_eq() {
    let _:AssertEq< True , ConstEq<tlist![], tlist![]>>;
    let _:AssertEq< False , ConstEq<tlist![], tlist![U0]>>;
    let _:AssertEq< False , ConstEq<tlist![U0], tlist![]>>;

    let _:AssertEq< False , ConstEq<tlist![U0], tlist![U1]>>;
    let _:AssertEq< True , ConstEq<tlist![U1], tlist![U1]>>;
    let _:AssertEq< False , ConstEq<tlist![U2], tlist![U1]>>;

    let _:AssertEq< True , ConstEq<tlist![U1, U2], tlist![U1, U2]>>;
    let _:AssertEq< False , ConstEq<tlist![U2], tlist![U0]>>;
    let _:AssertEq< False , ConstEq<tlist![U2], tlist![U1, U3]>>;
    let _:AssertEq< False , ConstEq<tlist![U2], tlist![U1, U3, U4, U5]>>;

    let _:AssertEq< False , ConstEq<tlist![U0], tlist![U1, U3]>>;
    let _:AssertEq< False , ConstEq<tlist![U0], tlist![U1, U3, U4, U5]>>;
}

#[test]
fn fold_l() {
    type TestFoldL<List,DefVal,Func,Expected>=(
        AssertEq<FoldL<List,DefVal,Func>,Expected>,
        AssertFnRet<List,FoldLMt<DefVal,Func>,Expected>
    );


    let _:TestFoldL< tlist![], (), PushOp ,()>;
    let _:TestFoldL< tlist![U0], (), PushOp ,(U0,)>;
    let _:TestFoldL< tlist![U0, U1], (), PushOp ,(U0, U1)>;
    let _:TestFoldL< tlist![U0, U1, U2], (), PushOp ,(U0, U1, U2)>;
    let _:TestFoldL< tlist![U0, U1, U2, U3], (), PushOp ,(U0, U1, U2, U3)>;
    let _:TestFoldL< Repeat<TListType, U1, U64>, U96, SubOp ,U32>;
    let _:TestFoldL< Repeat<TListType, U1, U64>, U65, SubOp ,U1>;
}



#[test]
fn fold_r() {
    type TestFoldR<List,DefVal,Func,Expected>=(
        AssertEq<FoldR<List,DefVal,Func>,Expected>,
        AssertFnRet<List,FoldRMt<DefVal,Func>,Expected>
    );

    let _:TestFoldR<tlist![], (), PushOp ,()>;
    let _:TestFoldR<tlist![U0], (), PushOp ,(U0,)>;
    let _:TestFoldR<tlist![U0, U1], (), PushOp ,(U1, U0)>;
    let _:TestFoldR<tlist![U0, U1, U2], (), PushOp ,(U2, U1, U0)>;
    let _:TestFoldR<tlist![U0, U1, U2, U3], (), PushOp ,(U3, U2, U1, U0)>;

    let _:TestFoldR<Repeat<TListType, U1, U64>, U96, SubOp,U32>;
    let _:TestFoldR<Repeat<TListType, U1, U64>, U65, SubOp,U1>;
}



pub struct CannotSubstract<Lhs,Rhs>{
    lhs:Lhs,
    rhs:Rhs,
}

type_fn!{
    fn safe_sub[Lhs,Rhs](Lhs,Rhs)
    where [
        If<ConstGEOp,
            (SubOp,NewOk),
            Const<Err_<CannotSubstract<Lhs,Rhs>>>
        >:TypeFn_<(Lhs,Rhs),Output=Out>,
    ]{
        let Out;Out
    }
}


#[test]
fn try_fold_l() {
    type TestTryFoldL<List,DefVal,Func,Expected>=(
        AssertEq<TryFoldL<List,DefVal,Func>,Expected>,
        AssertFnRet<List,TryFoldLMt<DefVal,Func>,Expected>
    );


    let _: TestTryFoldL<tlist![]            , U10 , SafeDivOp , TFVal<U10>>;
    let _: TestTryFoldL<tlist![U2]          , U10 , SafeDivOp , TFVal<U5>>;
    let _: TestTryFoldL<tlist![U2,U2]       , U10 , SafeDivOp , TFVal<U2>>;
    let _: TestTryFoldL<tlist![U2,U2,U2]    , U10 , SafeDivOp , TFVal<U1>>;
    let _: TestTryFoldL<tlist![U2,U2,U2,U2] , U10 , SafeDivOp , TFVal<U0>>;
    let _: TestTryFoldL<tlist![U0,()]      , U10 , SafeDivOp , TFBreak<None_>>;

    let _: TestTryFoldL<tlist![]          , U4 , safe_sub , TFVal<U4>>;
    let _: TestTryFoldL<tlist![U2]       , U4 , safe_sub , TFVal<U2>>;
    let _: TestTryFoldL<tlist![U2,U2]    , U4 , safe_sub , TFVal<U0>>;
    let _: TestTryFoldL< 
        tlist![U5,()], 
        U4 , 
        safe_sub, 
        TFBreak<Err_<CannotSubstract<U4,U5>>>, 
    >;
}

#[test]
fn try_fold_r() {
    type TestTryFoldR<List,DefVal,Func,Expected>=(
        AssertEq<TryFoldR<List,DefVal,Func>,Expected>,
        AssertFnRet<List,TryFoldRMt<DefVal,Func>,Expected>
    );

    let _: TestTryFoldR<tlist![]            , U10 , SafeDivOp , TFVal<U10> >;
    let _: TestTryFoldR<tlist![U2]          , U10 , SafeDivOp , TFVal<U5> >;
    let _: TestTryFoldR<tlist![U2,U2]       , U10 , SafeDivOp , TFVal<U2> >;
    let _: TestTryFoldR<tlist![U2,U2,U2]    , U10 , SafeDivOp , TFVal<U1> >;
    let _: TestTryFoldR<tlist![U2,U2,U2,U2] , U10 , SafeDivOp , TFVal<U0> >;
    let _: TestTryFoldR<tlist![(),U0]       , U10 , SafeDivOp , TFBreak<None_> >;

    let _: TestTryFoldR<tlist![]          , U4 , safe_sub , TFVal<U4> >;
    let _: TestTryFoldR<tlist![U2]       , U4 , safe_sub , TFVal<U2> >;
    let _: TestTryFoldR<tlist![U2,U2]    , U4 , safe_sub , TFVal<U0> >;
    let _: TestTryFoldR< 
        tlist![(),U5], 
        U4 , 
        safe_sub,
        TFBreak<Err_<CannotSubstract<U4,U5>>>, 
    >;

}

#[test]
fn map() {
    type TestMap<List,Func,Expected>=(
        AssertEq<Map<List,Func>,Expected>,
        AssertFnRet<List,MapMt<Func>,Expected>
    );

    let _:TestMap<tlist![], AddOne,tlist![]>;
    let _:TestMap<tlist![U0], AddOne,tlist![U1,]>;
    let _:TestMap<tlist![U0, U1], AddOne,tlist![U1, U2]>;
    let _:TestMap<tlist![U0, U1, U2], AddOne,tlist![U1, U2, U3]>;
    let _:TestMap<tlist![U0, U1, U2, U3], AddOne,tlist![U1, U2, U3, U4]>;
    let _:TestMap<
        Repeat<TListType, True, U64>, 
        Const<False>,
        Repeat<TListType, False, U64> , 
    >;
}

#[test]
fn filter() {
    type TestFilter<List,Pred,Expected>=(
        AssertEq<Filter<List,Pred>,Expected>,
        AssertFnRet<List,FilterMt<Pred>,Expected>
    );

    type Val0=tlist![U10,U11,U12,U13,U14];
    type IsOdd =(BitAndMt<U1>,ConstEqMt<U1>);
    type IsEven=(BitAndMt<U1>,ConstEqMt<U0>);
    
    let _:TestFilter<Val0,IsEven,tlist![U10,U12,U14]>;
    let _:TestFilter<Val0,IsOdd ,tlist![U11,U13]>;
    let _:TestFilter<Val0,Const<True>,Val0>;
    let _:TestFilter<Val0,Const<False>,tlist![]>;
    
}

#[test]
fn into_() {
    type TestInto<From_,Type,Expected>=(
        AssertEq<ConstInto<From_,Type>,Expected>,
        AssertFnRet<From_,ConstIntoMt<Type>,Expected>,
        AssertEq<ConstFrom<Type,From_>,Expected>,
    );

    let _:TestInto<(), TListType,tlist![]>;
    let _:TestInto<(U1,), TListType,tlist![U1,]>;
    let _:TestInto<(U1, U2), TListType,tlist![U1, U2]>;
    let _:TestInto<(U1, U2, U3), TListType,tlist![U1, U2, U3]>;
    let _:TestInto<(U1, U2, U3, U4), TListType,tlist![U1, U2, U3, U4]>;

    let _:TestInto<tlist![], TupleType,()>;
    let _:TestInto<tlist![U1], TupleType,(U1,)>;
    let _:TestInto<tlist![U1, U2], TupleType,(U1, U2)>;
    let _:TestInto<tlist![U1, U2, U3], TupleType,(U1, U2, U3)>;
    let _:TestInto<tlist![U1, U2, U3, U4], TupleType,(U1, U2, U3, U4)>;
}

#[test]
fn repeat() {
    let _: Repeat<TListType, U0, U0> = <tlist![]>::MTVAL;
    let _: Repeat<TListType, U0, U1> = <tlist![U0]>::MTVAL;
    let _: Repeat<TListType, U0, U2> = <tlist![U0, U0]>::MTVAL;
    let _: Repeat<TListType, U0, U3> = <tlist![U0, U0, U0]>::MTVAL;
    let _: Repeat<TListType, U0, U4> = <tlist![U0, U0, U0, U0]>::MTVAL;

    let _: AssertEq< 
        Map<Repeat<TListType, U0, U100>,Const<()>>,
        Repeat<TListType,(),U100 > 
    >;
}

#[test]
fn push_back() {
    type TestPushBack<List,Val,Expected>=(
        AssertEq<PushBack<List,Val>,Expected>,
        AssertFnRet<List,PushBackMt<Val>,Expected>
    );


    let _:TestPushBack<tlist![], U0,tlist![U0]>;
    let _:TestPushBack<tlist![U0], U1,tlist![U0, U1]>;
    let _:TestPushBack<tlist![U0, U1], U2,tlist![U0, U1, U2]>;
    let _:TestPushBack<tlist![U0, U1, U2], U3,tlist![U0, U1, U2, U3]>;
}

#[test]
fn pop_back() {
    let _:AssertEq< Some_<(U0, tlist![])> , PopBack<tlist![U0]>>;
    let _:AssertEq< Some_<(U1, tlist![U0])> , PopBack<tlist![U0, U1]>>;
    let _:AssertEq< Some_<(U2, tlist![U0, U1])> , PopBack<tlist![U0, U1, U2]>>;
    let _:AssertEq< Some_<(U3, tlist![U0, U1, U2])> , PopBack<tlist![U0, U1, U2, U3]>>;
}

#[test]
fn repeat_expr() {
    let _: AssertEq< tlist![U0;U0] , tlist![]>;
    let _: AssertEq< tlist![U0;U1] , tlist![U0]>;
    let _: AssertEq< tlist![U0;U2] , tlist![U0, U0]>;
    let _: AssertEq< tlist![U0;U3] , tlist![U0, U0, U0]>;
    let _: AssertEq< tlist![U0;U4] , tlist![U0, U0, U0, U0]>;

    let _: AssertEq< FoldL<tlist![U1;U511] , U0 , AddOp > ,U511 >;
}

#[test]
fn get_field() {
    let _:AssertEq< U0 , GetField<tlist![U0], U0>>;
    let _:AssertEq< U0 , GetField<tlist![U0, U2], U0>>;
    let _:AssertEq< U2 , GetField<tlist![U0, U2], U1>>;
    let _:AssertEq< U0 , GetField<tlist![U0, U2, U5], U0>>;
    let _:AssertEq< U2 , GetField<tlist![U0, U2, U5], U1>>;
    let _:AssertEq< U5 , GetField<tlist![U0, U2, U5], U2>>;
    let _:AssertEq< U0 , GetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U0>>;
    let _:AssertEq< U2 , GetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U1>>;
    let _:AssertEq< U4 , GetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U2>>;
    let _:AssertEq< U6 , GetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U3>>;
    let _:AssertEq< U8 , GetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U4>>;
    let _:AssertEq< U10 , GetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U5>>;
    let _:AssertEq< U12 , GetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U6>>;
    let _:AssertEq< U14 , GetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U7>>;
    let _:AssertEq< U16 , GetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U8>>;
    let _:AssertEq< U18 , GetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U9>>;
}

#[test]
fn set_field() {
    let _:AssertEq< tlist![False] , SetField<tlist![U0], U0, False>>;
    let _:AssertEq< tlist![False, U2] , SetField<tlist![U0, U2], U0, False>>;
    let _:AssertEq< tlist![U0, False] , SetField<tlist![U0, U2], U1, False>>;
    let _:AssertEq< tlist![False, U2, U5] , SetField<tlist![U0, U2, U5], U0, False>>;
    let _:AssertEq< tlist![U0, False, U5] , SetField<tlist![U0, U2, U5], U1, False>>;
    let _:AssertEq< tlist![U0, U2, False] , SetField<tlist![U0, U2, U5], U2, False>>;
    let _:AssertEq< tlist![False, U2, U5, U10] , SetField<tlist![U0, U2, U5, U10], U0, False>>;
    let _:AssertEq< tlist![U0, False, U5, U10] , SetField<tlist![U0, U2, U5, U10], U1, False>>;
    let _:AssertEq< tlist![U0, U2, False, U10] , SetField<tlist![U0, U2, U5, U10], U2, False>>;
    let _:AssertEq< tlist![U0, U2, U5, False] , SetField<tlist![U0, U2, U5, U10], U3, False>>;

    let _: AssertEq<
        tlist![False, U2, U4, U6, U8, U10, U12, U14, U16, U18],
        SetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U0, False>
    >;

    let _: AssertEq<
        tlist![U0, False, U4, U6, U8, U10, U12, U14, U16, U18],
        SetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U1, False>
    >;

    let _: AssertEq<
        tlist![U0, U2, False, U6, U8, U10, U12, U14, U16, U18],
        SetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U2, False>
    >;

    let _: AssertEq<
        tlist![U0, U2, U4, False, U8, U10, U12, U14, U16, U18],
        SetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U3, False>
    >;

    let _: AssertEq<
        tlist![U0, U2, U4, U6, False, U10, U12, U14, U16, U18],
        SetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U4, False>
    >;

    let _: AssertEq<
        tlist![U0, U2, U4, U6, U8, False, U12, U14, U16, U18],
        SetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U5, False>
    >;

    let _: AssertEq<
        tlist![U0, U2, U4, U6, U8, U10, False, U14, U16, U18],
        SetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U6, False>
    >;

    let _: AssertEq<
        tlist![U0, U2, U4, U6, U8, U10, U12, False, U16, U18],
        SetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U7, False>
    >;

    let _: AssertEq<
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, False, U18],
        SetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U8, False>
    >;

    let _: AssertEq<
        tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, False],
        SetField<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U9, False>
    >;
}

#[test]
fn type_fn_() {
    let _:AssertEq< U0 , TypeFn<tlist![], U0>>;
    let _:AssertEq< U1 , TypeFn<tlist![AddMt<U1>], U0>>;
    let _:AssertEq< U2 , TypeFn<tlist![AddMt<U1>,AddMt<U1 >], U0>>;
    let _:AssertEq< U21 , TypeFn<tlist![AddMt<U1>,AddMt<U10>], U10>>;

    type AddA=tlist![ AddMt<U1>, AddMt<U10>, AddMt<U20> ];
    let _: AssertEq<U41,TypeFn<AddA,U10>>;
}

#[test]
fn no_trait_eval_overflow() {
    type List = tlist![
        U0, U0, U0, U1, U0, U0, U0, U1, U0, U0, U0, U1, U0, U0, U0, U1, U0, U0, U0, U1, U0, U0, U0,
        U1, U0, U0, U0, U1, U0, U0, U0, U1, U0, U0, U0, U1, U0, U0, U0, U1, U0, U0, U0, U1, U0, U0,
        U0, U1, U0, U0, U0, U1, U0, U0, U0, U1, U0, U0, U0, U1, U0, U0, U0, U1, U0, U0, U0, U1, U0,
        U0, U0, U1, U0, U0, U0, U1, U0, U0, U0, U1, U0, U0, U0, U1, U0, U0, U0, U1, U0, U0, U0, U1,
        U0, U0, U0, U1, U0, U0, U0, U1, U0, U0, U0, U1, U0, U0, U0, U1, U0, U0, U0, U1,
    ];
    let _:AssertEq< List , Map<List, IdentityFn>>;
}
