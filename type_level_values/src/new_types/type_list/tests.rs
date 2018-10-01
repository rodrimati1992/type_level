use super::*;

use crate_::fn_types::SubOp;

use crate_::field_traits::{GetField, SetField};
use crate_::ops::{ConstInto, PopBack, PopFront, PushBack, PushFront};

#[test]
fn the_macro() {
    let _: TNil = <tlist![]>::MTVAL;
    let _: TList<U0, TNil> = <tlist![U0]>::MTVAL;
    let _: TList<U0, TList<U1, TNil>> = <tlist![U0, U1]>::MTVAL;
    let _: TList<U0, TList<U1, TList<U2, TNil>>> = <tlist![U0, U1, U2]>::MTVAL;
}
#[test]
fn insert() {
    let _: tlist![U4] = Insert::<tlist![], U0, U4>::MTVAL;
    let _: tlist![U4, U0] = Insert::<tlist![U0], U0, U4>::MTVAL;
    let _: tlist![U4, U0, U1] = Insert::<tlist![U0, U1], U0, U4>::MTVAL;
    let _: tlist![U4, U0, U1, U2] = Insert::<tlist![U0, U1, U2], U0, U4>::MTVAL;

    let _: tlist![U0, U4] = Insert::<tlist![U0], U1, U4>::MTVAL;
    let _: tlist![U0, U4, U1] = Insert::<tlist![U0, U1], U1, U4>::MTVAL;
    let _: tlist![U0, U4, U1, U2] = Insert::<tlist![U0, U1, U2], U1, U4>::MTVAL;

    let _: tlist![U0, U1, U4] = Insert::<tlist![U0, U1], U2, U4>::MTVAL;
    let _: tlist![U0, U1, U4, U2] = Insert::<tlist![U0, U1, U2], U2, U4>::MTVAL;

    let _: tlist![U0, U1, U2, U4] = Insert::<tlist![U0, U1, U2], U3, U4>::MTVAL;

    let _: tlist![False, U0, U2, U4, U6, U8, U10, U12, U14, U16, U18] =
        Insert::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U0, False>::MTVAL;

    let _: tlist![U0, False, U2, U4, U6, U8, U10, U12, U14, U16, U18] =
        Insert::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U1, False>::MTVAL;

    let _: tlist![U0, U2, False, U4, U6, U8, U10, U12, U14, U16, U18] =
        Insert::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U2, False>::MTVAL;

    let _: tlist![U0, U2, U4, False, U6, U8, U10, U12, U14, U16, U18] =
        Insert::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U3, False>::MTVAL;

    let _: tlist![U0, U2, U4, U6, False, U8, U10, U12, U14, U16, U18] =
        Insert::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U4, False>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, False, U10, U12, U14, U16, U18] =
        Insert::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U5, False>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, False, U12, U14, U16, U18] =
        Insert::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U6, False>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, False, U14, U16, U18] =
        Insert::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U7, False>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, U14, False, U16, U18] =
        Insert::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U8, False>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, False, U18] =
        Insert::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U9, False>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18, False] =
        Insert::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U10, False>::MTVAL;
}
#[test]
fn remove() {
    let _: tlist![] = Remove::<tlist![U0], U0>::MTVAL;

    let _: tlist![U1] = Remove::<tlist![U0, U1], U0>::MTVAL;
    let _: tlist![U0] = Remove::<tlist![U0, U1], U1>::MTVAL;

    let _: tlist![U1, U2] = Remove::<tlist![U0, U1, U2], U0>::MTVAL;
    let _: tlist![U0, U2] = Remove::<tlist![U0, U1, U2], U1>::MTVAL;
    let _: tlist![U0, U1] = Remove::<tlist![U0, U1, U2], U2>::MTVAL;

    let _: tlist![U1, U2, U3] = Remove::<tlist![U0, U1, U2, U3], U0>::MTVAL;
    let _: tlist![U0, U2, U3] = Remove::<tlist![U0, U1, U2, U3], U1>::MTVAL;
    let _: tlist![U0, U1, U3] = Remove::<tlist![U0, U1, U2, U3], U2>::MTVAL;
    let _: tlist![U0, U1, U2] = Remove::<tlist![U0, U1, U2, U3], U3>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18] =
        Remove::<tlist![False, U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U0>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18] =
        Remove::<tlist![U0, False, U2, U4, U6, U8, U10, U12, U14, U16, U18], U1>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18] =
        Remove::<tlist![U0, U2, False, U4, U6, U8, U10, U12, U14, U16, U18], U2>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18] =
        Remove::<tlist![U0, U2, U4, False, U6, U8, U10, U12, U14, U16, U18], U3>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18] =
        Remove::<tlist![U0, U2, U4, U6, False, U8, U10, U12, U14, U16, U18], U4>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18] =
        Remove::<tlist![U0, U2, U4, U6, U8, False, U10, U12, U14, U16, U18], U5>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18] =
        Remove::<tlist![U0, U2, U4, U6, U8, U10, False, U12, U14, U16, U18], U6>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18] =
        Remove::<tlist![U0, U2, U4, U6, U8, U10, U12, False, U14, U16, U18], U7>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18] =
        Remove::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, False, U16, U18], U8>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18] =
        Remove::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, False, U18], U9>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18] =
        Remove::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18, False], U10>::MTVAL;
}

macro_rules! test_push_pop_front {
    ($push_fn:ident => $push_ta:ident $pop_fn:ident => $pop_ta:ident) => {
        #[test]
        fn $push_fn() {
            let _: tlist![U0] = $push_ta::<tlist![], U0>::MTVAL;
            let _: tlist![U1, U0] = $push_ta::<tlist![U0], U1>::MTVAL;
            let _: tlist![U2, U1, U0] = $push_ta::<tlist![U1, U0], U2>::MTVAL;
            let _: tlist![U3, U2, U1, U0] = $push_ta::<tlist![U2, U1, U0], U3>::MTVAL;
        }
        #[test]
        fn $pop_fn() {
            let _: None_ = $pop_ta::<tlist![]>::MTVAL;
            let _: Some_<(U0, tlist![])> = $pop_ta::<tlist![U0]>::MTVAL;
            let _: Some_<(U0, tlist![U1])> = $pop_ta::<tlist![U0, U1]>::MTVAL;
            let _: Some_<(U0, tlist![U1, U2])> = $pop_ta::<tlist![U0, U1, U2]>::MTVAL;
            let _: Some_<(U0, tlist![U1, U2, U3])> = $pop_ta::<tlist![U0, U1, U2, U3]>::MTVAL;
        }
    };
}

test_push_pop_front!{
    push=>Push
    pop=>Pop
}

test_push_pop_front!{
    push_front=>PushFront
    pop_front=>PopFront
}

#[test]
fn len() {
    let _: U0 = Len::<tlist![]>::MTVAL;
    let _: U1 = Len::<tlist![U0]>::MTVAL;
    let _: U2 = Len::<tlist![U0, U1]>::MTVAL;
    let _: U3 = Len::<tlist![U0, U1, U2]>::MTVAL;
    let _: U4 = Len::<tlist![U0, U1, U2, U3]>::MTVAL;
}

#[test]
fn const_ord() {
    let _: Equal_ = ConstOrd::<tlist![], tlist![]>::MTVAL;
    let _: Less_ = ConstOrd::<tlist![], tlist![U0]>::MTVAL;
    let _: Greater_ = ConstOrd::<tlist![U0], tlist![]>::MTVAL;

    let _: Less_ = ConstOrd::<tlist![U0], tlist![U1]>::MTVAL;
    let _: Equal_ = ConstOrd::<tlist![U1], tlist![U1]>::MTVAL;
    let _: Greater_ = ConstOrd::<tlist![U2], tlist![U1]>::MTVAL;

    let _: Equal_ = ConstOrd::<tlist![U1, U2], tlist![U1, U2]>::MTVAL;
    let _: Greater_ = ConstOrd::<tlist![U2], tlist![U0]>::MTVAL;
    let _: Greater_ = ConstOrd::<tlist![U2], tlist![U1, U3]>::MTVAL;
    let _: Greater_ = ConstOrd::<tlist![U2], tlist![U1, U3, U4, U5]>::MTVAL;

    let _: Less_ = ConstOrd::<tlist![U0], tlist![U1, U3]>::MTVAL;
    let _: Less_ = ConstOrd::<tlist![U0], tlist![U1, U3, U4, U5]>::MTVAL;
}

#[test]
fn const_eq() {
    let _: True = ConstEq::<tlist![], tlist![]>::MTVAL;
    let _: False = ConstEq::<tlist![], tlist![U0]>::MTVAL;
    let _: False = ConstEq::<tlist![U0], tlist![]>::MTVAL;

    let _: False = ConstEq::<tlist![U0], tlist![U1]>::MTVAL;
    let _: True = ConstEq::<tlist![U1], tlist![U1]>::MTVAL;
    let _: False = ConstEq::<tlist![U2], tlist![U1]>::MTVAL;

    let _: True = ConstEq::<tlist![U1, U2], tlist![U1, U2]>::MTVAL;
    let _: False = ConstEq::<tlist![U2], tlist![U0]>::MTVAL;
    let _: False = ConstEq::<tlist![U2], tlist![U1, U3]>::MTVAL;
    let _: False = ConstEq::<tlist![U2], tlist![U1, U3, U4, U5]>::MTVAL;

    let _: False = ConstEq::<tlist![U0], tlist![U1, U3]>::MTVAL;
    let _: False = ConstEq::<tlist![U0], tlist![U1, U3, U4, U5]>::MTVAL;
}

#[test]
fn fold_l() {
    let _: () = FoldL::<tlist![], (), PushOp>::MTVAL;
    let _: (U0,) = FoldL::<tlist![U0], (), PushOp>::MTVAL;
    let _: (U0, U1) = FoldL::<tlist![U0, U1], (), PushOp>::MTVAL;
    let _: (U0, U1, U2) = FoldL::<tlist![U0, U1, U2], (), PushOp>::MTVAL;
    let _: (U0, U1, U2, U3) = FoldL::<tlist![U0, U1, U2, U3], (), PushOp>::MTVAL;
    let _: U32 = FoldL::<Repeat<TListType, U1, U64>, U96, SubOp>::MTVAL;
    let _: U1 = FoldL::<Repeat<TListType, U1, U64>, U65, SubOp>::MTVAL;
}

#[test]
fn fold_r() {
    let _: () = FoldR::<tlist![], (), PushOp>::MTVAL;
    let _: (U0,) = FoldR::<tlist![U0], (), PushOp>::MTVAL;
    let _: (U1, U0) = FoldR::<tlist![U0, U1], (), PushOp>::MTVAL;
    let _: (U2, U1, U0) = FoldR::<tlist![U0, U1, U2], (), PushOp>::MTVAL;
    let _: (U3, U2, U1, U0) = FoldR::<tlist![U0, U1, U2, U3], (), PushOp>::MTVAL;

    let _: U32 = FoldR::<Repeat<TListType, U1, U64>, U96, SubOp>::MTVAL;
    let _: U1 = FoldR::<Repeat<TListType, U1, U64>, U65, SubOp>::MTVAL;
}

#[test]
fn map() {
    type AddOne = ApplyRhs<AddOp, U1>;
    let _: tlist![] = Map::<tlist![], AddOne>::MTVAL;
    let _: tlist![U1,] = Map::<tlist![U0], AddOne>::MTVAL;
    let _: tlist![U1, U2] = Map::<tlist![U0, U1], AddOne>::MTVAL;
    let _: tlist![U1, U2, U3] = Map::<tlist![U0, U1, U2], AddOne>::MTVAL;
    let _: tlist![U1, U2, U3, U4] = Map::<tlist![U0, U1, U2, U3], AddOne>::MTVAL;
    let _: Repeat<TListType, False, U64> = Map::<Repeat<TListType, True, U64>, Const<False>>::MTVAL;
}

#[test]
fn into_() {
    let _: tlist![] = ConstInto::<(), TListType>::MTVAL;
    let _: tlist![U1,] = ConstInto::<(U1,), TListType>::MTVAL;
    let _: tlist![U1, U2] = ConstInto::<(U1, U2), TListType>::MTVAL;
    let _: tlist![U1, U2, U3] = ConstInto::<(U1, U2, U3), TListType>::MTVAL;
    let _: tlist![U1, U2, U3, U4] = ConstInto::<(U1, U2, U3, U4), TListType>::MTVAL;

    let _: () = ConstInto::<tlist![], TupleType>::MTVAL;
    let _: (U1,) = ConstInto::<tlist![U1], TupleType>::MTVAL;
    let _: (U1, U2) = ConstInto::<tlist![U1, U2], TupleType>::MTVAL;
    let _: (U1, U2, U3) = ConstInto::<tlist![U1, U2, U3], TupleType>::MTVAL;
    let _: (U1, U2, U3, U4) = ConstInto::<tlist![U1, U2, U3, U4], TupleType>::MTVAL;
}

#[test]
fn repeat() {
    let _: Repeat<TListType, U0, U0> = <tlist![]>::MTVAL;
    let _: Repeat<TListType, U0, U1> = <tlist![U0]>::MTVAL;
    let _: Repeat<TListType, U0, U2> = <tlist![U0, U0]>::MTVAL;
    let _: Repeat<TListType, U0, U3> = <tlist![U0, U0, U0]>::MTVAL;
    let _: Repeat<TListType, U0, U4> = <tlist![U0, U0, U0, U0]>::MTVAL;
}

#[test]
fn push_back() {
    let _: tlist![U0] = PushBack::<tlist![], U0>::MTVAL;
    let _: tlist![U0, U1] = PushBack::<tlist![U0], U1>::MTVAL;
    let _: tlist![U0, U1, U2] = PushBack::<tlist![U0, U1], U2>::MTVAL;
    let _: tlist![U0, U1, U2, U3] = PushBack::<tlist![U0, U1, U2], U3>::MTVAL;
}

#[test]
fn pop_back() {
    let _: Some_<(U0, tlist![])> = PopBack::<tlist![U0]>::MTVAL;
    let _: Some_<(U1, tlist![U0])> = PopBack::<tlist![U0, U1]>::MTVAL;
    let _: Some_<(U2, tlist![U0, U1])> = PopBack::<tlist![U0, U1, U2]>::MTVAL;
    let _: Some_<(U3, tlist![U0, U1, U2])> = PopBack::<tlist![U0, U1, U2, U3]>::MTVAL;
}

#[test]
fn repeat_expr() {
    let _: tlist![U0;U0] = <tlist![]>::MTVAL;
    let _: tlist![U0;U1] = <tlist![U0]>::MTVAL;
    let _: tlist![U0;U2] = <tlist![U0, U0]>::MTVAL;
    let _: tlist![U0;U3] = <tlist![U0, U0, U0]>::MTVAL;
    let _: tlist![U0;U4] = <tlist![U0, U0, U0, U0]>::MTVAL;
}

#[test]
fn get_field() {
    let _: U0 = GetField::<tlist![U0], U0>::MTVAL;
    let _: U0 = GetField::<tlist![U0, U2], U0>::MTVAL;
    let _: U2 = GetField::<tlist![U0, U2], U1>::MTVAL;
    let _: U0 = GetField::<tlist![U0, U2, U5], U0>::MTVAL;
    let _: U2 = GetField::<tlist![U0, U2, U5], U1>::MTVAL;
    let _: U5 = GetField::<tlist![U0, U2, U5], U2>::MTVAL;
    let _: U0 = GetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U0>::MTVAL;
    let _: U2 = GetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U1>::MTVAL;
    let _: U4 = GetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U2>::MTVAL;
    let _: U6 = GetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U3>::MTVAL;
    let _: U8 = GetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U4>::MTVAL;
    let _: U10 = GetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U5>::MTVAL;
    let _: U12 = GetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U6>::MTVAL;
    let _: U14 = GetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U7>::MTVAL;
    let _: U16 = GetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U8>::MTVAL;
    let _: U18 = GetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U9>::MTVAL;
}

#[test]
fn set_field() {
    let _: tlist![False] = SetField::<tlist![U0], U0, False>::MTVAL;
    let _: tlist![False, U2] = SetField::<tlist![U0, U2], U0, False>::MTVAL;
    let _: tlist![U0, False] = SetField::<tlist![U0, U2], U1, False>::MTVAL;
    let _: tlist![False, U2, U5] = SetField::<tlist![U0, U2, U5], U0, False>::MTVAL;
    let _: tlist![U0, False, U5] = SetField::<tlist![U0, U2, U5], U1, False>::MTVAL;
    let _: tlist![U0, U2, False] = SetField::<tlist![U0, U2, U5], U2, False>::MTVAL;
    let _: tlist![False, U2, U5, U10] = SetField::<tlist![U0, U2, U5, U10], U0, False>::MTVAL;
    let _: tlist![U0, False, U5, U10] = SetField::<tlist![U0, U2, U5, U10], U1, False>::MTVAL;
    let _: tlist![U0, U2, False, U10] = SetField::<tlist![U0, U2, U5, U10], U2, False>::MTVAL;
    let _: tlist![U0, U2, U5, False] = SetField::<tlist![U0, U2, U5, U10], U3, False>::MTVAL;

    let _: tlist![False, U2, U4, U6, U8, U10, U12, U14, U16, U18] =
        SetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U0, False>::MTVAL;

    let _: tlist![U0, False, U4, U6, U8, U10, U12, U14, U16, U18] =
        SetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U1, False>::MTVAL;

    let _: tlist![U0, U2, False, U6, U8, U10, U12, U14, U16, U18] =
        SetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U2, False>::MTVAL;

    let _: tlist![U0, U2, U4, False, U8, U10, U12, U14, U16, U18] =
        SetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U3, False>::MTVAL;

    let _: tlist![U0, U2, U4, U6, False, U10, U12, U14, U16, U18] =
        SetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U4, False>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, False, U12, U14, U16, U18] =
        SetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U5, False>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, False, U14, U16, U18] =
        SetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U6, False>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, False, U16, U18] =
        SetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U7, False>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, U14, False, U18] =
        SetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U8, False>::MTVAL;

    let _: tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, False] =
        SetField::<tlist![U0, U2, U4, U6, U8, U10, U12, U14, U16, U18], U9, False>::MTVAL;
}

#[test]
fn type_fn_() {
    let _: U0 = TypeFn::<tlist![], U0>::MTVAL;
    let _: U1 = TypeFn::<tlist![ApplyRhs<AddOp,U1>], U0>::MTVAL;
    let _: U2 = TypeFn::<tlist![ApplyRhs<AddOp,U1>,ApplyRhs<AddOp,U1 >], U0>::MTVAL;
    let _: U21 = TypeFn::<tlist![ApplyRhs<AddOp,U1>,ApplyRhs<AddOp,U10>], U10>::MTVAL;
    let _: U41 = TypeFn::<
        tlist![
            ApplyRhs<AddOp,U1>,
            ApplyRhs<AddOp,U10>,
            ApplyRhs<AddOp,U20>
        ],
        U10,
    >::MTVAL;
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
    let _: List = Map::<List, IdentityFn>::MTVAL;
}
