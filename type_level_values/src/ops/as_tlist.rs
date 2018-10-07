use crate_::discriminant::GetDiscriminant;
use new_types::type_list::{TList, TListType, TypeLevelListTrait};
use prelude::*;

/// Converts a type to a tlist.
pub trait AsTList_ {
    type Output: TypeLevelListTrait;
}

type_fn!{
    pub fn AsTListOp[This](This)where[ This:AsTList_ ]{ This::Output }
}

pub type AsTList<This> = <This as AsTList_>::Output;

/// Converts an enum variant to a tlist with the discriminant as the first element.
pub trait VariantAsTList_ {
    type Output: TypeLevelListTrait;
}

type_fn!{
    pub fn VariantAsTListOp[This](This)where[ This:VariantAsTList_ ]{ This::Output }
}

pub type VariantAsTList<This> = <This as VariantAsTList_>::Output;

impl<This, out> VariantAsTList_ for This
where
    This: GetDiscriminant + AsTList_,
    TList<<This as GetDiscriminant>::Discriminant, <This as AsTList_>::Output>:
        TypeIdentity<Type = out>,
    out: ConstTypeOf_<Type = TListType> + TypeLevelListTrait,
{
    type Output = out;
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_as_tlist(){
        let _:AssEqTy<AsTList<None_>, tlist![]>;
        
        let _:AssEqTy<AsTList<Some_<()>>, tlist![()]>;
        let _:AssEqTy<AsTList<Some_<U32>>, tlist![U32]>;
        
        let _:AssEqTy<AsTList<Ok_<()>>, tlist![()]>;
        let _:AssEqTy<AsTList<Ok_<U32>>, tlist![U32]>;
        
        let _:AssEqTy<AsTList<Err_<()>>, tlist![()]>;
        let _:AssEqTy<AsTList<Err_<U32>>, tlist![U32]>;
        
        let _:AssEqTy<AsTList<ConstRange<U10,U20>>, tlist![U10,U20]>;
        let _:AssEqTy<AsTList<ConstRangeInclusive<U10,U20>>, tlist![U10,U20]>;
        
        let _:AssEqTy<AsTList<()>, tlist![]>;
        let _:AssEqTy<AsTList<(U10,)>, tlist![U10]>;
        let _:AssEqTy<AsTList<(U10,U20)>, tlist![U10,U20]>;
        let _:AssEqTy<AsTList<(U10,U20,U30)>, tlist![U10,U20,U30]>;

        let _:AssEqTy<AsTList<tlist![]>, tlist![]>;
        let _:AssEqTy<AsTList<tlist![U10,]>, tlist![U10]>;
        let _:AssEqTy<AsTList<tlist![U10,U20]>, tlist![U10,U20]>;
        let _:AssEqTy<AsTList<tlist![U10,U20,U30]>, tlist![U10,U20,U30]>;
    }

    #[test]
    fn test_variant_as_tlist(){
        use std_types::option::{None__Discr,Some__Discr};
        use std_types::result::{Ok__Discr,Err__Discr};
        use std_types::range::Range_Discr;
        use std_types::range_inclusive::RangeInclusive_Discr;
        use std_types::tuples::Tuple_Discr;
        use new_types::type_list::{TList_Discr,TNil_Discr};

        type Test<L,R>=
            AssEqTy<VariantAsTList<L>,R>;

        let _:Test<None_, tlist![ None__Discr ]>;
        
        let _:Test<Some_<()>, tlist![ Some__Discr, ()]>;
        let _:Test<Some_<U32>, tlist![ Some__Discr, U32]>;
        
        let _:Test<Ok_<()>, tlist![ Ok__Discr, ()]>;
        let _:Test<Ok_<U32>, tlist![ Ok__Discr, U32]>;
        
        let _:Test<Err_<()>, tlist![ Err__Discr, ()]>;
        let _:Test<Err_<U32>, tlist![ Err__Discr, U32]>;
        
        let _:Test<ConstRange<U10,U20>, tlist![ Range_Discr, U10,U20]>;
        let _:Test<ConstRangeInclusive<U10,U20>, tlist![ RangeInclusive_Discr,U10,U20]>;

        let _:Test<(), tlist![Tuple_Discr]>;
        let _:Test<(U10,), tlist![Tuple_Discr,U10]>;
        let _:Test<(U10,U20), tlist![Tuple_Discr,U10,U20]>;
        let _:Test<(U10,U20,U30), tlist![Tuple_Discr,U10,U20,U30]>;

        let _:Test<tlist![], tlist![TNil_Discr]>;
        let _:Test<tlist![U10,], tlist![TList_Discr,U10]>;
        let _:Test<tlist![U10,U20], tlist![TList_Discr,U10,U20]>;
        let _:Test<tlist![U10,U20,U30], tlist![TList_Discr,U10,U20,U30]>;
    }
}