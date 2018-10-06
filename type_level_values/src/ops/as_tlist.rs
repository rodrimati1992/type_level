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
