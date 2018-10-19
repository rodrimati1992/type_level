mod generated_impls;
#[cfg(all(test,feature="passed_tests"))]
mod tests;

use core_extensions::type_level_bool::{Boolean, False, True};

use crate_::extern_types::typenum::UnsignedInteger;
use crate_::field_traits::{GetField_, SetField_};
use crate_::ops::control_flow::{If, Lazy};
use crate_::ops::{
    AsTList_,
    ConstEq,ConstNE_, ConstOrd,
    ConstFrom_, 
    UnwrapOp,
};
use crate_::fn_adaptors::*;
use crate_::fn_types::{AddOp, ConstEqOp, ConstLtOp, ConstOrdOp, NotOp};
use crate_::collection_ops::*;
use crate_::std_types::cmp_ordering::{Equal_, Greater_, Less_, OrderingTrait};
use crate_::std_types::option::{None_, Some_};
use crate_::std_types::tuples::TupleType;
use prelude::*;

use std_::ops::{Add, BitAnd, BitOr, Index, Sub};

#[derive(TypeLevel)]
#[typelevel(
    reexport(Variants, Traits,Discriminants), 
    rename_consttype = "TListType",
    items(AsTList(NoImpls))
)]
pub enum TypeLevelList<Current, Remaining> {
    TNil,
    TList {
        current: VariantPhantom<Current>,
        remaining: VariantPhantom<Remaining>,
    },
}

////////////////////////////////////////////////////////////////////////////////

impl ConstEq_<TNil> for TNil {
    type Output = True;
}
impl<T1, Rem1> ConstEq_<TList<T1, Rem1>> for TNil {
    type Output = False;
}
impl<T0, Rem0> ConstEq_<TNil> for TList<T0, Rem0> {
    type Output = False;
}
impl<T0, T1, Rem0, Rem1, out> ConstEq_<TList<T1, Rem1>> for TList<T0, Rem0>
where
    T0: ConstEq_<T1>,
    If<Const<T0::Output>, Lazy<ConstEqOp, (Rem0, Rem1)>, Const<False>>: TypeFn_<(), Output = out>,
    out: Boolean,
{
    type Output = out;
}

////////////////////////////////////////////////////////////////////////////////

impl ConstOrd_<TNil> for TNil {
    type Output = Equal_;
}
impl<T1, Rem1> ConstOrd_<TList<T1, Rem1>> for TNil {
    type Output = Less_;
}
impl<T0, Rem0> ConstOrd_<TNil> for TList<T0, Rem0> {
    type Output = Greater_;
}
impl<T0, T1, Rem0, Rem1, ordering, out> ConstOrd_<TList<T1, Rem1>> for TList<T0, Rem0>
where
    T0: ConstOrd_<T1, Output = ordering>,
    ordering: OrderingTrait + ConstEq_<Equal_>,
    If<Const<ordering::Output>, Lazy<ConstOrdOp, (Rem0, Rem1)>, Const<ordering>>:
        TypeFn_<(), Output = out>,
    out: OrderingTrait,
{
    type Output = out;
}

//////////////////////////////////////////////////////////////////////////////////

impl<T0, Rem, index, is_lt8, Out> GetField_<index> for TList<T0, Rem>
where
    ConstLtOp: TypeFn_<(index, U8), Output = is_lt8>,
    GetFieldHelper: TypeFn_<(is_lt8, index, Self), Output = Out>,
{
    type Output = Out;
}

type_fn!{
    fn
        GetFieldHelper[Rem,T0]
        (True,U0,tlist![T0,..Rem])
        {T0}

        GetFieldHelper[Rem,T0,T1]
        (True,U1,tlist![T0,T1,..Rem])
        {T1}

        GetFieldHelper[Rem,T0,T1,T2]
        (True,U2,tlist![T0,T1,T2,..Rem])
        {T2}

        GetFieldHelper[Rem,T0,T1,T2,T3]
        (True,U3,tlist![T0,T1,T2,T3,..Rem])
        {T3}

        GetFieldHelper[Rem,T0,T1,T2,T3,T4]
        (True,U4,tlist![T0,T1,T2,T3,T4,..Rem])
        {T4}

        GetFieldHelper[Rem,T0,T1,T2,T3,T4,T5]
        (True,U5,tlist![T0,T1,T2,T3,T4,T5,..Rem])
        {T5}

        GetFieldHelper[Rem,T0,T1,T2,T3,T4,T5,T6]
        (True,U6,tlist![T0,T1,T2,T3,T4,T5,T6,..Rem])
        {T6}

        GetFieldHelper[Rem,T0,T1,T2,T3,T4,T5,T6,T7]
        (True,U7,tlist![T0,T1,T2,T3,T4,T5,T6,T7,..Rem])
        {T7}

        GetFieldHelper[Rem,index,T0,T1,T2,T3,T4,T5,T6,T7]
        (False,index,tlist![T0,T1,T2,T3,T4,T5,T6,T7,..Rem])
        where[
            index:Sub<U8,Output=sub8>,
            ConstLtOp:TypeFn_<(sub8,U8),Output=is_lt8>,
            Self:TypeFn_<(is_lt8,sub8,Rem),Output=Out>,
        ]{
            let sub8;let is_lt8;let Out;
            Out
        }
}

//////////////////////////////////////////////////////////////////////////////////

impl<T0, Rem, index, value, is_lt8, Out> SetField_<index, value> for TList<T0, Rem>
where
    ConstLtOp: TypeFn_<(index, U8), Output = is_lt8>,
    SetFieldHelper: TypeFn_<(is_lt8, index, value, Self), Output = Out>,
{
    type Output = Out;
}

type_fn!{
    fn
        SetFieldHelper[Rem,val,T0]
        (True,U0,val,tlist![T0,..Rem])
        {tlist![val,..Rem]}

        SetFieldHelper[Rem,val,T0,T1]
        (True,U1,val,tlist![T0,T1,..Rem])
        {tlist![T0,val,..Rem]}

        SetFieldHelper[Rem,val,T0,T1,T2]
        (True,U2,val,tlist![T0,T1,T2,..Rem])
        {tlist![T0,T1,val,..Rem]}

        SetFieldHelper[Rem,val,T0,T1,T2,T3]
        (True,U3,val,tlist![T0,T1,T2,T3,..Rem])
        {tlist![T0,T1,T2,val,..Rem]}

        SetFieldHelper[Rem,val,T0,T1,T2,T3,T4]
        (True,U4,val,tlist![T0,T1,T2,T3,T4,..Rem])
        {tlist![T0,T1,T2,T3,val,..Rem]}

        SetFieldHelper[Rem,val,T0,T1,T2,T3,T4,T5]
        (True,U5,val,tlist![T0,T1,T2,T3,T4,T5,..Rem])
        {tlist![T0,T1,T2,T3,T4,val,..Rem]}

        SetFieldHelper[Rem,val,T0,T1,T2,T3,T4,T5,T6]
        (True,U6,val,tlist![T0,T1,T2,T3,T4,T5,T6,..Rem])
        {tlist![T0,T1,T2,T3,T4,T5,val,..Rem]}

        SetFieldHelper[Rem,val,T0,T1,T2,T3,T4,T5,T6,T7]
        (True,U7,val,tlist![T0,T1,T2,T3,T4,T5,T6,T7,..Rem])
        {tlist![T0,T1,T2,T3,T4,T5,T6,val,..Rem]}

        SetFieldHelper[Rem,val,index,T0,T1,T2,T3,T4,T5,T6,T7]
        (False,index,val,tlist![T0,T1,T2,T3,T4,T5,T6,T7,..Rem])
        where[
            index:Sub<U8,Output=sub8>,
            ConstLtOp:TypeFn_<(sub8,U8),Output=is_lt8>,
            Self:TypeFn_<(is_lt8,sub8,val,Rem),Output=Out>,
        ]{
            let sub8;let is_lt8;let Out;
            tlist![T0,T1,T2,T3,T4,T5,T6,T7,..Out]
        }
}

//////////////////////////////////////////////////////////////////////////////////

impl<Value> Insert_<U0, Value> for TNil {
    type Output = tlist![Value];
}

impl<T0, Rem, index, value, is_lt8, Out> Insert_<index, value> for TList<T0, Rem>
where
    ConstLtOp: TypeFn_<(index, U8), Output = is_lt8>,
    InsertHelper: TypeFn_<(is_lt8, index, value, Self), Output = Out>,
{
    type Output = Out;
}

type_fn!{
    fn
        InsertHelper[Rem,val]
        (True,U0,val,Rem)
        {tlist![val,..Rem]}

        InsertHelper[Rem,val,T0]
        (True,U1,val,tlist![T0,..Rem])
        {tlist![T0,val,..Rem]}

        InsertHelper[Rem,val,T0,T1]
        (True,U2,val,tlist![T0,T1,..Rem])
        {tlist![T0,T1,val,..Rem]}

        InsertHelper[Rem,val,T0,T1,T2]
        (True,U3,val,tlist![T0,T1,T2,..Rem])
        {tlist![T0,T1,T2,val,..Rem]}

        InsertHelper[Rem,val,T0,T1,T2,T3]
        (True,U4,val,tlist![T0,T1,T2,T3,..Rem])
        {tlist![T0,T1,T2,T3,val,..Rem]}

        InsertHelper[Rem,val,T0,T1,T2,T3,T4]
        (True,U5,val,tlist![T0,T1,T2,T3,T4,..Rem])
        {tlist![T0,T1,T2,T3,T4,val,..Rem]}

        InsertHelper[Rem,val,T0,T1,T2,T3,T4,T5]
        (True,U6,val,tlist![T0,T1,T2,T3,T4,T5,..Rem])
        {tlist![T0,T1,T2,T3,T4,T5,val,..Rem]}

        InsertHelper[Rem,val,T0,T1,T2,T3,T4,T5,T6]
        (True,U7,val,tlist![T0,T1,T2,T3,T4,T5,T6,..Rem])
        {tlist![T0,T1,T2,T3,T4,T5,T6,val,..Rem]}

        InsertHelper[Rem,val,index,T0,T1,T2,T3,T4,T5,T6,T7]
        (False,index,val,tlist![T0,T1,T2,T3,T4,T5,T6,T7,..Rem])
        where[
            index:Sub<U8,Output=sub8>,
            ConstLtOp:TypeFn_<(sub8,U8),Output=is_lt8>,
            Self:TypeFn_<(is_lt8,sub8,val,Rem),Output=Out>,
        ]{
            let sub8;let is_lt8;let Out;
            tlist![T0,T1,T2,T3,T4,T5,T6,T7,..Out]
        }
}

//////////////////////////////////////////////////////////////////////////////////

impl<T0, Rem, index, is_lt8, Out> Remove_<index> for TList<T0, Rem>
where
    ConstLtOp: TypeFn_<(index, U8), Output = is_lt8>,
    RemoveHelper: TypeFn_<(is_lt8, index, Self), Output = Out>,
{
    type Output = Out;
}

type_fn!{
    fn
        RemoveHelper[Rem,T0]
        (True,U0,tlist![T0,..Rem])
        {Rem}

        RemoveHelper[Rem,T0,T1]
        (True,U1,tlist![T0,T1,..Rem])
        {tlist![T0,..Rem]}

        RemoveHelper[Rem,T0,T1,T2]
        (True,U2,tlist![T0,T1,T2,..Rem])
        {tlist![T0,T1,..Rem]}

        RemoveHelper[Rem,T0,T1,T2,T3]
        (True,U3,tlist![T0,T1,T2,T3,..Rem])
        {tlist![T0,T1,T2,..Rem]}

        RemoveHelper[Rem,T0,T1,T2,T3,T4]
        (True,U4,tlist![T0,T1,T2,T3,T4,..Rem])
        {tlist![T0,T1,T2,T3,..Rem]}

        RemoveHelper[Rem,T0,T1,T2,T3,T4,T5]
        (True,U5,tlist![T0,T1,T2,T3,T4,T5,..Rem])
        {tlist![T0,T1,T2,T3,T4,..Rem]}

        RemoveHelper[Rem,T0,T1,T2,T3,T4,T5,T6]
        (True,U6,tlist![T0,T1,T2,T3,T4,T5,T6,..Rem])
        {tlist![T0,T1,T2,T3,T4,T5,..Rem]}

        RemoveHelper[Rem,T0,T1,T2,T3,T4,T5,T6,T7]
        (True,U7,tlist![T0,T1,T2,T3,T4,T5,T6,T7,..Rem])
        {tlist![T0,T1,T2,T3,T4,T5,T6,..Rem]}

        RemoveHelper[Rem,index,T0,T1,T2,T3,T4,T5,T6,T7]
        (False,index,tlist![T0,T1,T2,T3,T4,T5,T6,T7,..Rem])
        where[
            index:Sub<U8,Output=sub8>,
            ConstLtOp:TypeFn_<(sub8,U8),Output=is_lt8>,
            Self:TypeFn_<(is_lt8,sub8,Rem),Output=Out>,
        ]{
            let sub8;let is_lt8;let Out;
            tlist![T0,T1,T2,T3,T4,T5,T6,T7,..Out]
        }
}

////////////////////////////////////////////////////////////////////////////////

impl<DefaultVal, Func> FoldL_<DefaultVal, Func> for tlist![] {
    type Output = DefaultVal;
}

////////////////////////////////////////////////////////////////////////////////

impl<DefaultVal, Func> FoldR_<DefaultVal, Func> for tlist![] {
    type Output = DefaultVal;
}

////////////////////////////////////////////////////////////////////////////////

impl<DefaultVal, Func> TryFoldL_<DefaultVal, Func> for tlist![] {
    type Output = TFVal<DefaultVal>;
}

impl<Curr,Rem, DefVal,Func,Out> TryFoldL_<DefVal, Func> for tlist![Curr,..Rem] 
where TryFoldLHelper<Func>:TypeFn_<(TFVal<DefVal>,Self),Output=Out>
{
    type Output=Out;
}


type_fn!{
    captures(F)
    fn 
        TryFoldLHelper[Accum,Curr,Rem](TFBreak<Accum>,tlist![Curr,..Rem]){ 
            TFBreak<Accum> 
        }
        
        TryFoldLHelper[Accum,Curr,Rem](TFVal<Accum>,tlist![Curr,..Rem])
        where[
            tlist![F,IntoTryFold]:TypeFn_<(Accum,Curr),Output=NewAccum>,
            Self:TypeFn_<(NewAccum,Rem),Output=Out>
        ]{
            let NewAccum;let Out;
            Out
        }
        
        TryFoldLHelper[Accum](Accum,TNil){ Accum }
}


////////////////////////////////////////////////////////////////////////////////

impl<DefaultVal, Func> TryFoldR_<DefaultVal, Func> for tlist![] {
    type Output = TFVal<DefaultVal>;
}

impl<Curr,Rem, DefaultVal,Reversed,Func> 
    TryFoldR_<DefaultVal, Func> 
for tlist![Curr,..Rem] 
where 
    Self:Reverse_<Output=Reversed>,
    Reversed:TryFoldL_<DefaultVal,Func>,
{
    type Output=Reversed::Output;
}

////////////////////////////////////////////////////////////////////////////////

impl Len_ for TNil {
    type Output = U0;
}

impl<T, Rem, out> Len_ for TList<T, Rem>
where
    Rem: Len_,
    Rem::Output: Add<U1, Output = out>,
{
    type Output = out;
}

////////////////////////////////////////////////////////////////////////////////

impl<Mapper> Map_<Mapper> for TNil {
    type Output = TNil;
}

////////////////////////////////////////////////////////////////////////////////

type_fn!{
    captures(predicate)
    fn PredicateRhs[_0,T](_0,T)where[predicate:TypeFn_<T>]{ predicate::Output }
}

impl<T, Rem, Predicate, out> Filter_<Predicate> for TList<T, Rem>
where
    Self: FoldR_<TNil, If<PredicateRhs<Predicate>, PushOp>, Output = out>,
{
    type Output = out;
}

impl<Predicate> Filter_<Predicate> for TNil {
    type Output = TNil;
}

////////////////////////////////////////////////////////////////////////////////

impl<Value> Push_<Value> for TNil {
    type Output = TList<Value, Self>;
}
impl<T, Rem, Value> Push_<Value> for TList<T, Rem> {
    type Output = TList<Value, Self>;
}

////////////////////////////////////////////////////////////////////////////////

impl<T, Rem> Pop_ for TList<T, Rem> {
    type Output = Some_<(T, Rem)>;
}
impl Pop_ for TNil {
    type Output = None_;
}

////////////////////////////////////////////////////////////////////////////////

impl<Value> PushFront_<Value> for TNil {
    type Output = TList<Value, TNil>;
}
impl<T, Rem, Value> PushFront_<Value> for TList<T, Rem> {
    type Output = TList<Value, Self>;
}

////////////////////////////////////////////////////////////////////////////////

impl<T, Rem> PopFront_ for TList<T, Rem> {
    type Output = Some_<(T, Rem)>;
}
impl PopFront_ for TNil {
    type Output = None_;
}

/////////////////////////////////////////////////////////////////////////////////

impl<Current, Rem, Elem, Out> PushBack_<Elem> for TList<Current, Rem>
where
    Rem: PushBack_<Elem, Output = Out>,
{
    type Output = TList<Current, Out>;
}

impl<Elem> PushBack_<Elem> for TNil {
    type Output = TList<Elem, TNil>;
}

/////////////////////////////////////////////////////////////////////////////////

type_fn!{
    fn
        PopBackHelper[T0,T1,Rem](tlist![T0,T1,..Rem])
        where [
            PopBackHelper:TypeFn_<tlist![T1,..Rem],Output=(last,RemOut)>
        ]{
            let last;let RemOut;
            (last,tlist![T0,..RemOut])
        }
        PopBackHelper[T](tlist![T]){
            (T,TNil)
        }
}

impl<T0, T1, Rem, last, remaining> PopBack_ for TList<T0, TList<T1, Rem>>
where
    PopBackHelper: TypeFn_<Self, Output = (last, remaining)>,
{
    type Output = Some_<(last, remaining)>;
}

impl<T> PopBack_ for TList<T, TNil> {
    type Output = Some_<(T, TNil)>;
}
impl PopBack_ for TNil {
    type Output = None_;
}

////////////////////////////////////////////////////////////////////////////////

impl<Current, Rem> AsTList_ for TList<Current, Rem>{
    type Output = Self;
}

impl AsTList_ for TNil {
    type Output = TNil;
}



////////////////////////////////////////////////////////////////////////////////

impl Reverse_ for TNil {
    type Output = TNil;
}

impl<T, Rem, out> Reverse_ for TList<T, Rem>
where
    ReverseHelper: TypeFn_<(TNil, Self), Output = out>,
{
    type Output = out;
}

type_fn!{
    fn
    ReverseHelper[Suffix,T,Rem](Suffix,TList<T,Rem>)
    where [ ReverseHelper:TypeFn_< (TList<T,Suffix>,Rem),Output=out > ]
    { let out;out }

    ReverseHelper[Suffix](Suffix,TNil){Suffix}
}

////////////////////////////////////////////////////////////////////////////////

impl<V, L, Out> Repeat_<V, L> for TListType
where
    L: ConstEq_<U0>,
    RepeatHelper<V>: TypeFn_<(L::Output, L), Output = Out>,
{
    type Output = Out;
}

type_fn!{
    #[doc(hidden)]

    captures(Value)
    pub fn RepeatHelper(True,U0){TNil}

    RepeatHelper[Repeating](False,Repeating)
    where [
        Repeating:Sub<U1,Output=Subbed>,
        Subbed:ConstEq_<U0,Output=is0>,
        is0:Boolean,
        Self:TypeFn_<(is0,Subbed),Output=OutRec>
    ]{
        let OutRec;
        let is0;
        let Subbed;
        TList<Value,OutRec>
    }
}


////////////////////////////////////////////////////////////////////////////////

macro_rules! fixed_size_impls {
    (with-idents;
        $( ($len:ty,$len_expr:expr)=[ $($tparams:ident,)* => $($runtparams:ident,)* ])*
    ) => {
        $(
            impl<$($tparams),*> ConstFrom_<tlist![$($tparams),*]> for TupleType{
                type Output=($($tparams,)*);
            }

            impl<$($tparams),*> ConstFrom_<($($tparams,)*)> for TListType{
                type Output=tlist![$($tparams),*];
            }

            impl<$($tparams,)* $($runtparams,)*>
                IntoRuntime<($($runtparams,)*)>
            for tlist![$($tparams),*]
            where
                $( $tparams:IntoRuntime<$runtparams> ,)*
            {
                fn to_runtime()->($($runtparams,)*){
                    ( $( $tparams::to_runtime(), )* )
                }
            }

            #[cfg(rust_1_22)]
            impl<$($tparams,)* $($runtparams,)*>
                IntoConstant<($($runtparams,)*)>
            for tlist![$($tparams),*]
            where
                $( $tparams:IntoConstant<$runtparams> ,)*
            {
                const VALUE: ($($runtparams,)*) =
                    ( $( <$tparams as IntoConstant<$runtparams>>::VALUE, )* );
            }

            impl<$($tparams,)* T> IntoRuntime<[T;$len_expr]> for tlist![$($tparams),*]
            where
                $( $tparams:IntoRuntime<T> ,)*
            {
                fn to_runtime()-> [T;$len_expr] {
                    [ $( $tparams::to_runtime(), )* ]
                }
            }


            #[cfg(rust_1_22)]
            impl<$($tparams,)* T> IntoConstant<[T;$len_expr]> for tlist![$($tparams),*]
            where
                $( $tparams:IntoConstant<T> ,)*
            {
                const VALUE:[T;$len_expr]=
                    [ $( $tparams::VALUE, )* ];
            }


            impl<$($tparams,)*> tlist![$($tparams),*] {
                #[inline(always)]
                pub fn to_array<T>()-> [T;$len_expr]
                where
                    Self:IntoRuntime< [T;$len_expr] >
                {
                    Self::to_runtime()
                }

                #[inline(always)]
                pub fn to_array_ty<T>(_ty:VariantPhantom<T>)-> [T;$len_expr]
                where
                    Self:IntoRuntime< [T;$len_expr] >
                {
                    Self::to_runtime()
                }

                #[inline(always)]
                pub fn into_array<T>(self)-> [T;$len_expr]
                where
                    Self:IntoRuntime< [T;$len_expr] >
                {
                    Self::to_runtime()
                }

                #[inline(always)]
                pub fn into_array_ty<T>(self,_ty:VariantPhantom<T>)-> [T;$len_expr]
                where
                    Self:IntoRuntime< [T;$len_expr] >
                {
                    Self::to_runtime()
                }
            }


        )*
    };
}

fixed_size_impls!{with-idents;
    (U0,0)=[
        =>

    ]
    (U1,1)=[
        C0,=>
        R0,
    ]
    (U2,2)=[
        C0,C1,=>
        R0,R1,
    ]
    (U3,3)=[
        C0,C1,C2,=>
        R0,R1,R2,
    ]
    (U4,4)=[
        C0,C1,C2,C3,=>
        R0,R1,R2,R3,
    ]
    (U5,5)=[
        C0,C1,C2,C3,C4,=>
        R0,R1,R2,R3,R4,
    ]
    (U6,6)=[
        C0,C1,C2,C3,C4,C5,=>
        R0,R1,R2,R3,R4,R5,
    ]
    (U7,7)=[
        C0,C1,C2,C3,C4,C5,C6,=>
        R0,R1,R2,R3,R4,R5,R6,
    ]
    (U8,8)=[
        C0,C1,C2,C3,C4,C5,C6,C7,=>
       R0,R1,R2,R3,R4,R5,R6,R7,
    ]
    (U9,9)=[
        C0,C1,C2,C3,C4,C5,C6,C7,C8,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,
    ]
    (U10,10)=[
        C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,
    ]
    (U11,11)=[
        C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,
    ]
    (U12,12)=[
        C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,R11,
    ]
    (U13,13)=[
        C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,R11,R12,
    ]
    (U14,14)=[
        C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,C13,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,R11,R12,R13,
    ]
    (U15,15)=[
        C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,C13,C14,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,R11,R12,R13,R14,
    ]
    (U16,16)=[
        C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,C13,C14,C15,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,R11,R12,R13,R14,R15,
    ]
}

#[cfg(feature = "large_tlist")]
mod large_impls;
