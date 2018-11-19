/*!
Items for  std::marker::PhantomData.
*/

use crate_::field_traits::{GetField_, SetField_};
use crate_::user_traits::const_traits;
use prelude::*;

use core_extensions::type_level_bool::{False, True};
use crate_::std_types::cmp_ordering::Equal_;
use crate_::std_types::option::fields as option_f;
use crate_::std_types::Some_;

/// The ConstType of a PhantomData
#[derive(Debug, Default, Copy, Clone)]
pub struct PhantomDataType;

impl ConstType for PhantomDataType {}

impl<T: ?Sized> ConstTypeOf_ for PhantomData<T> {
    type Type = PhantomDataType;
}

/// To access a PhantomData in generic contexts.
pub trait PhantomDataTrait {
    type field_0: ?Sized;
}

impl<T> PhantomDataTrait for PhantomData<T> {
    type field_0 = T;
}

impl<T: ?Sized> IntoRuntime<PhantomData<T>> for PhantomData<T> {
    fn to_runtime() -> Self {
        PhantomData
    }
}

#[cfg(rust_1_22)]
impl<T: ?Sized> IntoConstant<PhantomData<T>> for PhantomData<T> {
    const VALUE: Self = PhantomData;
}

impl<T: ?Sized> IntoConstType_ for PhantomData<T> {
    type ToConst = PhantomDataType;
}

impl<T: ?Sized> ConstOrd_<PhantomData<T>> for PhantomData<T> {
    type Output = Equal_;
}

impl<T: ?Sized> ConstEq_<PhantomData<T>> for PhantomData<T> {
    type Output = True;
}

impl<T: ?Sized, Field> GetField_<Field> for PhantomData<T>
where
    T: GetField_<Field>,
{
    type Output = T::Output;
}

impl<T, Field, Value: ?Sized> SetField_<Field, Value> for PhantomData<T>
where
    T: SetField_<Field, Value>,
{
    type Output = PhantomData<T::Output>;
}

impl<T> const_traits::GetConstParam_ for PhantomData<T> {
    type Const = T;
}
impl<T> const_traits::GetConstConstructor_ for PhantomData<T> {
    type Constructor = PhantomDataType;
}
impl const_traits::ConstConstructor for PhantomDataType {}

impl<T> const_traits::ApplyConstParam_<T> for PhantomDataType {
    type Applied = PhantomData<T>;
}



//////////////////////////////////////////////////////////////////////////////////


type_fn!{
    /// Constructs a PhantomData<v>
    pub fn NewPhantomData[v](v)
    where[ v:?Sized ]
    { PhantomData<v> }
}

type_fn!{
    /// Constructs a VariantPhantom<v>
    pub fn NewVariantPhantom[v](v)
    where[ v:?Sized ]
    { VariantPhantom<v> }
}