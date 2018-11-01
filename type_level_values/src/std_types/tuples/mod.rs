use core_extensions::SelfOps;

use crate_::new_types::TListType;
use crate_::fn_adaptors::ApplyRhs;
use crate_::ops::{ConstInto,ConstIntoMt, ConstInto_,AsTList_};
use crate_::collection_ops::{
    Filter_, 
    FoldL_, FoldR_, TryFoldL_, TryFoldR_, TryFoldLMt,
    Insert_, Len_, Map_, Remove_, Repeat_,
    ReverseOp,
};
use crate_::field_traits::{GetField_, SetField_};
use crate_::discriminant::{Discriminant,UIntFromDiscriminant};

use prelude::*;

#[cfg(test)]
// #[cfg(all(test,feature="passed_tests"))]
mod tests;
mod tuple_impls;

/// Marker type representing tuples up to 32 elements.
#[derive(Debug, Default, Copy, Clone)]
pub struct TupleType;

mod sealed {
    use super::*;
    pub trait Sealed {}
}
use self::sealed::Sealed;

pub trait TupleTrait: Sealed {}

impl ConstType for TupleType {}


pub type Tuple_Discr=Discriminant<TupleType, TupleType, U0>;

macro_rules! impl_tuple_trait {
    (with-idents;$( ($len:ty)=[ $($tparams:ident,)* => $($runtparams:ident,)* ])*) => {
        $(
            impl<$($tparams,)*> Sealed for ($($tparams,)*){  }
            impl<$($tparams,)*> TupleTrait for ($($tparams,)*){  }



            impl<$($tparams,)*> ConstTypeOf_ for ($($tparams,)*)
            where $($tparams:ConstTypeOf_,)*
            {
                type Type=TupleType;
            }

            impl<$($tparams),*>  GetDiscriminant for ($($tparams,)*){
                type Discriminant=Tuple_Discr;
                type UIntDiscr=TypeFn<UIntFromDiscriminant,Tuple_Discr>;
                type Variant=TupleType;
            }

            impl<$($tparams),*> AsTList_ for ($($tparams,)*) {
                type Output=tlist![$($tparams),*];
            }

            impl<$($runtparams,)*> IntoConstType_ for ($($runtparams,)*)
            where $($runtparams:IntoConstType_,)*
            {
                type ToConst=TupleType;
            }

            impl<$($tparams,)* $($runtparams,)*>
                IntoRuntime<($($runtparams,)*)>
            for ($($tparams,)*)
            where $($tparams:IntoRuntime<$runtparams>,)*
            {
                fn to_runtime()->($($runtparams,)*){
                    ($($tparams::to_runtime(),)*)
                }
            }

            #[cfg(rust_1_22)]
            impl<$($tparams,)* $($runtparams,)*>
                IntoConstant<($($runtparams,)*)>
            for ($($tparams,)*)
            where $($tparams:IntoConstant<$runtparams>,)*
            {
                const VALUE: ($($runtparams,)*) =($($tparams::VALUE,)*);
            }

            impl<$($tparams,)*> Len_ for ($($tparams,)*){
                type Output = $len;
            }

            impl<$($tparams,)* Predicate,var0> Filter_<Predicate> for ($($tparams,)*)
            where
                Self:ConstInto_<TListType>,
                ConstInto<Self,TListType>:Filter_<Predicate,Output=var0>,
                var0:ConstInto_<TupleType>,
            {
                type Output=var0::Output;
            }

            impl<$($tparams,)* list,Index> GetField_<Index> for ($($tparams,)*)
            where
                Self:ConstInto_<TListType,Output=list>,
                list:GetField_<Index>,
            {
                type Output=list::Output;
            }

            impl<$($tparams,)* list,Index,Value,new_list,Out>
                SetField_<Index,Value> for ($($tparams,)*)
            where
                Self:ConstInto_<TListType,Output=list>,
                list:SetField_<Index,Value,Output=new_list>,
                new_list:ConstInto_<TupleType,Output=Out>,
            {
                type Output=Out;
            }


            impl<$($tparams,)* Op,list,new_list,Out> 
                Map_<Op> for ($($tparams,)*)
            where
                Self:ConstInto_<TListType,Output=list>,
                list:Map_<Op,Output=new_list>,
                new_list:ConstInto_<TupleType,Output=Out>,
            {
                type Output = Out;
            }



            impl<$($tparams,)* Index,Value,list,Out>
                Insert_<Index, Value> for ($($tparams,)*)
            where
                Self:ConstInto_<TListType,Output=list>,
                list:Insert_<Index,Value>,
                list::Output:ConstInto_<TupleType,Output=Out>,
            {
                type Output = Out;
            }


            impl<$($tparams,)* Index,list,Out> Remove_<Index> for ($($tparams,)*)
            where
                Self:ConstInto_<TListType,Output=list>,
                list:Remove_<Index>,
                list::Output:ConstInto_<TupleType,Output=Out>,
            {
                type Output = Out;
            }


            impl<$($tparams,)* lista,listb,Other> ConstOrd_<Other> for ($($tparams,)*)
            where
                Self :ConstInto_<TListType,Output=lista>,
                Other:ConstInto_<TListType,Output=listb>,
                lista:ConstOrd_<listb>,
            {
                type Output=lista::Output;
            }

            impl<$($tparams,)* lista,listb,Other> ConstEq_<Other> for ($($tparams,)*)
            where
                Self :ConstInto_<TListType,Output=lista>,
                Other:ConstInto_<TListType,Output=listb>,
                lista:ConstEq_<listb>,
            {
                type Output=lista::Output;
            }


            impl<$($tparams,)* lista,Default,Op> FoldL_<Default,Op> for ($($tparams,)*)
            where
                Self :ConstInto_<TListType,Output=lista>,
                lista:FoldL_<Default,Op>,
            {
                type Output=lista::Output;
            }


            impl<$($tparams,)* lista,Default,Op> FoldR_<Default,Op> for ($($tparams,)*)
            where
                Self :ConstInto_<TListType,Output=lista>,
                lista:FoldR_<Default,Op>,
            {
                type Output=lista::Output;
            }


            impl<$($tparams,)* lista,Default,Op> TryFoldL_<Default,Op> for ($($tparams,)*)
            where
                Self :ConstInto_<TListType,Output=lista>,
                lista:TryFoldL_<Default,Op>,
            {
                type Output=lista::Output;
            }


            impl<$($tparams,)* Default,Op,Out> TryFoldR_<Default,Op> for ($($tparams,)*)
            where
                (
                    ReverseOp,
                    ConstIntoMt<TListType>,
                    TryFoldLMt<Default,Op>
                ):TypeFn_<Self,Output=Out>
            {
                type Output=Out;
            }


        )*
    };
    (repeated; $( ($len:ty)=[ $($tparams:ident),* ])* )=>{
        $(
            impl<V> Repeat_<V,$len> for TupleType{
                type Output=($($tparams,)*);
            }
        )*
    };
}

impl_tuple_trait!{with-idents;
    (U0)=[=>
        ]
    (U1)=[C0,=>
        R0,]
    (U2)=[C0,C1,=>
        R0,R1,]
    (U3)=[C0,C1,C2,=>
        R0,R1,R2,]
    (U4)=[C0,C1,C2,C3,=>
        R0,R1,R2,R3,]
    (U5)=[C0,C1,C2,C3,C4,=>
        R0,R1,R2,R3,R4,]
    (U6)=[C0,C1,C2,C3,C4,C5,=>
        R0,R1,R2,R3,R4,R5,]
    (U7)=[C0,C1,C2,C3,C4,C5,C6,=>
        R0,R1,R2,R3,R4,R5,R6,]
    (U8)=[C0,C1,C2,C3,C4,C5,C6,C7,=>
       R0,R1,R2,R3,R4,R5,R6,R7,]
    (U9)=[C0,C1,C2,C3,C4,C5,C6,C7,C8,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,]
    (U10)=[C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,]
    (U11)=[C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,]
    (U12)=[C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,R11,]
    (U13)=[C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,R11,R12,]
    (U14)=[C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,C13,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,R11,R12,R13,]
    (U15)=[C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,C13,C14,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,R11,R12,R13,R14,]
    (U16)=[C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,C13,C14,C15,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,R11,R12,R13,R14,R15,]
}

impl_tuple_trait!{repeated;
    (U0)=[]
    (U1)=[V]
    (U2)=[V,V]
    (U3)=[V,V,V]
    (U4)=[V,V,V,V]
    (U5)=[V,V,V,V,V]
    (U6)=[V,V,V,V,V,V]
    (U7)=[V,V,V,V,V,V,V]
    (U8)=[V,V,V,V,V,V,V,V]
    (U9)=[V,V,V,V,V,V,V,V,V]
    (U10)=[V,V,V,V,V,V,V,V,V,V]
    (U11)=[V,V,V,V,V,V,V,V,V,V,V]
    (U12)=[V,V,V,V,V,V,V,V,V,V,V,V]
    (U13)=[V,V,V,V,V,V,V,V,V,V,V,V,V]
    (U14)=[V,V,V,V,V,V,V,V,V,V,V,V,V,V]
    (U15)=[V,V,V,V,V,V,V,V,V,V,V,V,V,V,V]
    (U16)=[V,V,V,V,V,V,V,V,V,V,V,V,V,V,V,V]
}
