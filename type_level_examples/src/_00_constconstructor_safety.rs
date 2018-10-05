//!
//!
//!
//!
//!
//!
//!
//!
//!

use type_level_values::prelude::*;


pub struct Marker;

pub trait GetCubed<T> {
    type GetCubed;
}

impl GetCubed<True> for Marker {
    type GetCubed = ConstWrapper<U1>;
}

impl GetCubed<False> for Marker {
    type GetCubed = VariantPhantom<()>;
}

impl GetCubed<U0> for Marker {
    type GetCubed = VariantPhantom<U0>;
}

impl GetCubed<U1> for Marker {
    type GetCubed = VariantPhantom<U1>;
}

impl GetCubed<U2> for Marker {
    type GetCubed = VariantPhantom<U2>;
}

impl GetCubed<U3> for Marker {
    type GetCubed = usize;
}

impl GetCubed<U4> for Marker {
    type GetCubed = u16;
}

#[derive(ConstConstructor)]
#[cconstructor(Type = "Value", ConstParam = "R")]
pub struct ValueInner<R>
where
    R: WrapperTrait,
    Marker: GetCubed<R::ConstValue>,
{
    #[allow(dead_code)]
    type_: <Marker as GetCubed<R::ConstValue>>::GetCubed,
}

///////////////////////////////////////////////////////////////////////////

impl<R> Value<R>
where
    Marker: GetCubed<R>,
    <Marker as GetCubed<R>>::GetCubed: Default,
{
    fn new(_range: R) -> Self {
        Self {
            type_: Default::default(),
        }
    }
}

const_method!{
    type ConstConstructor[]=( ValueCC )
    type AllowedConversions=( allowed_conversions::All )

    pub fn ChangeConst[C,C2](C,C2){ C2 }
}

pub fn main_ () {
    {
        Value::new(True)
            .mutparam(ChangeConst, True::T)
            .mutparam(ChangeConst, False::T);
    }

    {
        Value::new(U0::MTVAL)
            .mutparam(ChangeConst, U1::T)
            .mutparam(ChangeConst, U2::T)
            .mutparam(ChangeConst, True::T)
            
            // this line doesn't compile since it changes the memory layout of the type.
            // .mutparam(ChangeConst, U3::T)
            
        ;
    }
}
