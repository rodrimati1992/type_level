use super::*;

use crate_::fn_adaptors::*;
use crate_::std_ops::*;

use crate_::field_traits::{GetField, SetField};
use crate_::ops::*;
use crate_::collection_ops::*;
use crate_::std_types::{Equal_, Greater_, Less_};

macro_rules! tuple_ {
    (
        $( $type_:ty ),* $(,)*
    ) => (
        ($( $type_ ,)*)
    )
}

#[test]
fn insert() {
    let _: tuple_![U4] = Insert::<tuple_![], U0, U4>::MTVAL;
    let _: tuple_![U4, U0] = Insert::<tuple_![U0], U0, U4>::MTVAL;
    let _: tuple_![U4, U0, U1] = Insert::<tuple_![U0, U1], U0, U4>::MTVAL;
    let _: tuple_![U4, U0, U1, U2] = Insert::<tuple_![U0, U1, U2], U0, U4>::MTVAL;

    let _: tuple_![U0, U4] = Insert::<tuple_![U0], U1, U4>::MTVAL;
    let _: tuple_![U0, U4, U1] = Insert::<tuple_![U0, U1], U1, U4>::MTVAL;
    let _: tuple_![U0, U4, U1, U2] = Insert::<tuple_![U0, U1, U2], U1, U4>::MTVAL;

    let _: tuple_![U0, U1, U4] = Insert::<tuple_![U0, U1], U2, U4>::MTVAL;
    let _: tuple_![U0, U1, U4, U2] = Insert::<tuple_![U0, U1, U2], U2, U4>::MTVAL;

    let _: tuple_![U0, U1, U2, U4] = Insert::<tuple_![U0, U1, U2], U3, U4>::MTVAL;
}
#[test]
fn remove() {
    let _: tuple_![] = Remove::<tuple_![U0], U0>::MTVAL;

    let _: tuple_![U1] = Remove::<tuple_![U0, U1], U0>::MTVAL;
    let _: tuple_![U0] = Remove::<tuple_![U0, U1], U1>::MTVAL;

    let _: tuple_![U1, U2] = Remove::<tuple_![U0, U1, U2], U0>::MTVAL;
    let _: tuple_![U0, U2] = Remove::<tuple_![U0, U1, U2], U1>::MTVAL;
    let _: tuple_![U0, U1] = Remove::<tuple_![U0, U1, U2], U2>::MTVAL;

    let _: tuple_![U1, U2, U3] = Remove::<tuple_![U0, U1, U2, U3], U0>::MTVAL;
    let _: tuple_![U0, U2, U3] = Remove::<tuple_![U0, U1, U2, U3], U1>::MTVAL;
    let _: tuple_![U0, U1, U3] = Remove::<tuple_![U0, U1, U2, U3], U2>::MTVAL;
    let _: tuple_![U0, U1, U2] = Remove::<tuple_![U0, U1, U2, U3], U3>::MTVAL;
}
#[test]
fn push() {
    let _: tuple_![U0] = Push::<tuple_![], U0>::MTVAL;
    let _: tuple_![U0, U1,] = Push::<tuple_![U0], U1>::MTVAL;
    let _: tuple_![U0, U1, U2,] = Push::<tuple_![U0, U1], U2>::MTVAL;
    let _: tuple_![U0, U1, U2, U3] = Push::<tuple_![U0, U1, U2], U3>::MTVAL;
}
#[test]
fn pop() {
    let _: None_ = Pop::<tuple_![]>::MTVAL;
    let _: Some_<(U0, tuple_![])> = Pop::<tuple_![U0]>::MTVAL;
    let _: Some_<(U1, tuple_![U0])> = Pop::<tuple_![U0, U1]>::MTVAL;
    let _: Some_<(U2, tuple_![U0, U1])> = Pop::<tuple_![U0, U1, U2]>::MTVAL;
    let _: Some_<(U3, tuple_![U0, U1, U2])> = Pop::<tuple_![U0, U1, U2, U3]>::MTVAL;
}
#[test]
fn len() {
    let _: U0 = Len::<tuple_![]>::MTVAL;
    let _: U1 = Len::<tuple_![U0]>::MTVAL;
    let _: U2 = Len::<tuple_![U0, U1]>::MTVAL;
    let _: U3 = Len::<tuple_![U0, U1, U2]>::MTVAL;
    let _: U4 = Len::<tuple_![U0, U1, U2, U3]>::MTVAL;
}

#[test]
fn const_ord() {
    let _: Equal_ = ConstOrd::<tuple_![], tuple_![]>::MTVAL;
    let _: Less_ = ConstOrd::<tuple_![], tuple_![U0]>::MTVAL;
    let _: Greater_ = ConstOrd::<tuple_![U0], tuple_![]>::MTVAL;

    let _: Less_ = ConstOrd::<tuple_![U0], tuple_![U1]>::MTVAL;
    let _: Equal_ = ConstOrd::<tuple_![U1], tuple_![U1]>::MTVAL;
    let _: Greater_ = ConstOrd::<tuple_![U2], tuple_![U1]>::MTVAL;

    let _: Equal_ = ConstOrd::<tuple_![U1, U2], tuple_![U1, U2]>::MTVAL;
    let _: Greater_ = ConstOrd::<tuple_![U2], tuple_![U0]>::MTVAL;
    let _: Greater_ = ConstOrd::<tuple_![U2], tuple_![U1, U3]>::MTVAL;
    let _: Greater_ = ConstOrd::<tuple_![U2], tuple_![U1, U3, U4, U5]>::MTVAL;

    let _: Less_ = ConstOrd::<tuple_![U0], tuple_![U1, U3]>::MTVAL;
    let _: Less_ = ConstOrd::<tuple_![U0], tuple_![U1, U3, U4, U5]>::MTVAL;
}

#[test]
fn const_eq() {
    let _: True = ConstEq::<tuple_![], tuple_![]>::MTVAL;
    let _: False = ConstEq::<tuple_![], tuple_![U0]>::MTVAL;
    let _: False = ConstEq::<tuple_![U0], tuple_![]>::MTVAL;

    let _: False = ConstEq::<tuple_![U0], tuple_![U1]>::MTVAL;
    let _: True = ConstEq::<tuple_![U1], tuple_![U1]>::MTVAL;
    let _: False = ConstEq::<tuple_![U2], tuple_![U1]>::MTVAL;

    let _: True = ConstEq::<tuple_![U1, U2], tuple_![U1, U2]>::MTVAL;
    let _: False = ConstEq::<tuple_![U2], tuple_![U0]>::MTVAL;
    let _: False = ConstEq::<tuple_![U2], tuple_![U1, U3]>::MTVAL;
    let _: False = ConstEq::<tuple_![U2], tuple_![U1, U3, U4, U5]>::MTVAL;

    let _: False = ConstEq::<tuple_![U0], tuple_![U1, U3]>::MTVAL;
    let _: False = ConstEq::<tuple_![U0], tuple_![U1, U3, U4, U5]>::MTVAL;
}

#[test]
fn fold_l() {
    let _: () = FoldL::<tuple_![], (), PushOp>::MTVAL;
    let _: (U0,) = FoldL::<tuple_![U0], (), PushOp>::MTVAL;
    let _: (U0, U1) = FoldL::<tuple_![U0, U1], (), PushOp>::MTVAL;
    let _: (U0, U1, U2) = FoldL::<tuple_![U0, U1, U2], (), PushOp>::MTVAL;
    let _: (U0, U1, U2, U3) = FoldL::<tuple_![U0, U1, U2, U3], (), PushOp>::MTVAL;
    let _: U80 = FoldL::<Repeat<TupleType, U1, U16>, U96, SubOp>::MTVAL;
    let _: U50 = FoldL::<Repeat<TupleType, U1, U16>, U66, SubOp>::MTVAL;
}

#[test]
fn fold_r() {
    let _: () = FoldR::<tuple_![], (), PushOp>::MTVAL;
    let _: (U0,) = FoldR::<tuple_![U0], (), PushOp>::MTVAL;
    let _: (U1, U0) = FoldR::<tuple_![U0, U1], (), PushOp>::MTVAL;
    let _: (U2, U1, U0) = FoldR::<tuple_![U0, U1, U2], (), PushOp>::MTVAL;
    let _: (U3, U2, U1, U0) = FoldR::<tuple_![U0, U1, U2, U3], (), PushOp>::MTVAL;

    let _: U80 = FoldR::<Repeat<TupleType, U1, U16>, U96, SubOp>::MTVAL;
    let _: U50 = FoldR::<Repeat<TupleType, U1, U16>, U66, SubOp>::MTVAL;
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
        AssertPipedRet<List,TryFoldLMt<DefVal,Func>,Expected>
    );


    let _: TestTryFoldL<tuple_![]            , U10 , SafeDivOp , TFVal<U10>>;
    let _: TestTryFoldL<tuple_![U2]          , U10 , SafeDivOp , TFVal<U5>>;
    let _: TestTryFoldL<tuple_![U2,U2]       , U10 , SafeDivOp , TFVal<U2>>;
    let _: TestTryFoldL<tuple_![U2,U2,U2]    , U10 , SafeDivOp , TFVal<U1>>;
    let _: TestTryFoldL<tuple_![U2,U2,U2,U2] , U10 , SafeDivOp , TFVal<U0>>;
    let _: TestTryFoldL<tuple_![U0,()]      , U10 , SafeDivOp , TFBreak<None_>>;

    let _: TestTryFoldL<tuple_![]          , U4 , safe_sub , TFVal<U4>>;
    let _: TestTryFoldL<tuple_![U2]       , U4 , safe_sub , TFVal<U2>>;
    let _: TestTryFoldL<tuple_![U2,U2]    , U4 , safe_sub , TFVal<U0>>;
    let _: TestTryFoldL< 
        tuple_![U5,()], 
        U4 , 
        safe_sub, 
        TFBreak<Err_<CannotSubstract<U4,U5>>>, 
    >;
}

#[test]
fn try_fold_r() {
    type TestTryFoldR<List,DefVal,Func,Expected>=(
        AssertEq<TryFoldR<List,DefVal,Func>,Expected>,
        AssertPipedRet<List,TryFoldRMt<DefVal,Func>,Expected>
    );

    let _: TestTryFoldR<tuple_![]            , U10 , SafeDivOp , TFVal<U10> >;
    let _: TestTryFoldR<tuple_![U2]          , U10 , SafeDivOp , TFVal<U5> >;
    let _: TestTryFoldR<tuple_![U2,U2]       , U10 , SafeDivOp , TFVal<U2> >;
    let _: TestTryFoldR<tuple_![U2,U2,U2]    , U10 , SafeDivOp , TFVal<U1> >;
    let _: TestTryFoldR<tuple_![U2,U2,U2,U2] , U10 , SafeDivOp , TFVal<U0> >;
    let _: TestTryFoldR<tuple_![(),U0]       , U10 , SafeDivOp , TFBreak<None_> >;

    let _: TestTryFoldR<tuple_![]          , U4 , safe_sub , TFVal<U4> >;
    let _: TestTryFoldR<tuple_![U2]       , U4 , safe_sub , TFVal<U2> >;
    let _: TestTryFoldR<tuple_![U2,U2]    , U4 , safe_sub , TFVal<U0> >;
    let _: TestTryFoldR< 
        tuple_![(),U5], 
        U4 , 
        safe_sub,
        TFBreak<Err_<CannotSubstract<U4,U5>>>, 
    >;

}




#[test]
fn map() {
    type AddOne = AddMt< U1>;
    let _: tuple_![] = Map::<tuple_![], AddOne>::MTVAL;
    let _: tuple_![U1,] = Map::<tuple_![U0], AddOne>::MTVAL;
    let _: tuple_![U1, U2] = Map::<tuple_![U0, U1], AddOne>::MTVAL;
    let _: tuple_![U1, U2, U3] = Map::<tuple_![U0, U1, U2], AddOne>::MTVAL;
    let _: tuple_![U1, U2, U3, U4] = Map::<tuple_![U0, U1, U2, U3], AddOne>::MTVAL;
    let _: Repeat<TupleType, False, U16> = Map::<Repeat<TupleType, True, U16>, Const<False>>::MTVAL;
}

#[test]
fn repeat() {
    let _: Repeat<TupleType, U0, U0> = <tuple_![]>::MTVAL;
    let _: Repeat<TupleType, U0, U1> = <tuple_![U0]>::MTVAL;
    let _: Repeat<TupleType, U0, U2> = <tuple_![U0, U0]>::MTVAL;
    let _: Repeat<TupleType, U0, U3> = <tuple_![U0, U0, U0]>::MTVAL;
    let _: Repeat<TupleType, U0, U4> = <tuple_![U0, U0, U0, U0]>::MTVAL;
}

#[test]
fn push_back() {
    let _: tuple_![U0] = PushBack::<tuple_![], U0>::MTVAL;
    let _: tuple_![U0, U1] = PushBack::<tuple_![U0], U1>::MTVAL;
    let _: tuple_![U0, U1, U2] = PushBack::<tuple_![U0, U1], U2>::MTVAL;
    let _: tuple_![U0, U1, U2, U3] = PushBack::<tuple_![U0, U1, U2], U3>::MTVAL;
}

#[test]
fn pop_back() {
    let _: Some_<(U0, tuple_![])> = PopBack::<tuple_![U0]>::MTVAL;
    let _: Some_<(U1, tuple_![U0])> = PopBack::<tuple_![U0, U1]>::MTVAL;
    let _: Some_<(U2, tuple_![U0, U1])> = PopBack::<tuple_![U0, U1, U2]>::MTVAL;
    let _: Some_<(U3, tuple_![U0, U1, U2])> = PopBack::<tuple_![U0, U1, U2, U3]>::MTVAL;
}

#[test]
fn get_field() {
    let _: U0 = GetField::<tuple_![U0], U0>::MTVAL;
    let _: U0 = GetField::<tuple_![U0, U2], U0>::MTVAL;
    let _: U2 = GetField::<tuple_![U0, U2], U1>::MTVAL;
    let _: U0 = GetField::<tuple_![U0, U2, U5], U0>::MTVAL;
    let _: U2 = GetField::<tuple_![U0, U2, U5], U1>::MTVAL;
    let _: U5 = GetField::<tuple_![U0, U2, U5], U2>::MTVAL;
    let _: U0 = GetField::<tuple_![U0, U2, U5, U10], U0>::MTVAL;
    let _: U2 = GetField::<tuple_![U0, U2, U5, U10], U1>::MTVAL;
    let _: U5 = GetField::<tuple_![U0, U2, U5, U10], U2>::MTVAL;
    let _: U10 = GetField::<tuple_![U0, U2, U5, U10], U3>::MTVAL;
}

#[test]
fn set_field() {
    let _: tuple_![False] = SetField::<tuple_![U0], U0, False>::MTVAL;
    let _: tuple_![False, U2] = SetField::<tuple_![U0, U2], U0, False>::MTVAL;
    let _: tuple_![U0, False] = SetField::<tuple_![U0, U2], U1, False>::MTVAL;
    let _: tuple_![False, U2, U5] = SetField::<tuple_![U0, U2, U5], U0, False>::MTVAL;
    let _: tuple_![U0, False, U5] = SetField::<tuple_![U0, U2, U5], U1, False>::MTVAL;
    let _: tuple_![U0, U2, False] = SetField::<tuple_![U0, U2, U5], U2, False>::MTVAL;
    let _: tuple_![False, U2, U5, U10] = SetField::<tuple_![U0, U2, U5, U10], U0, False>::MTVAL;
    let _: tuple_![U0, False, U5, U10] = SetField::<tuple_![U0, U2, U5, U10], U1, False>::MTVAL;
    let _: tuple_![U0, U2, False, U10] = SetField::<tuple_![U0, U2, U5, U10], U2, False>::MTVAL;
    let _: tuple_![U0, U2, U5, False] = SetField::<tuple_![U0, U2, U5, U10], U3, False>::MTVAL;
}

#[test]
fn type_fn_() {
    let _: AssertPipedRet<U0,tuple_![],U0>;
    let _: AssertPipedRet<U0,tuple_![AddMt<U1>],U1>;
    let _: AssertPipedRet<U0,tuple_![AddMt<U1>,AddMt<U1 >],U2>;
    let _: AssertPipedRet<U10,tuple_![AddMt<U1>,AddMt<U10>],U21>;
    let _: AssertPipedRet<
        U10,
        tuple_![
            AddMt<U1>,
            AddMt<U10>,
            AddMt<U20>
        ],
        U41
    >;
}
