/*!
types and trait related to the variant/disciminant of ConstValues.


*/

use prelude::*;

use initialization::InitializationValues;

use extern_types::typenum::UnsignedInteger;

////////////////////////////////////////////////////////////////////////////////

/// The Const-type of the discriminant of an enum.
#[derive(Debug, Copy, Clone, Default)]
pub struct DiscriminantFor<T>(T);

impl<T> ConstType for DiscriminantFor<T> where T: ConstType {}

/// The type of all the discriminants  derived in the `TypeLevel` derive macro.
///
/// The type parameters are :
///
/// - N: A marker type used as the name for the disciminant in error messages.
///
/// - T: The ConstType the discriminant belongs to.
///
/// - I: The unsigned integer index for this discriminant,in source order,starting from U0.
///
pub struct Discriminant<N, T, I>(VariantPhantom<(N, T, I)>);

impl<N, T, I> Discriminant<N, T, I> {
    pub fn new() -> Self {
        Discriminant(PhantomData)
    }
}

impl<N, T, I> Copy for Discriminant<N, T, I> {}
impl<N, T, I> Clone for Discriminant<N, T, I> {
    #[inline(always)]
    fn clone(&self) -> Self {
        MarkerType::MTVAL
    }
}
impl<N, T, I> Default for Discriminant<N, T, I> {
    #[inline(always)]
    fn default() -> Self {
        Discriminant(PhantomData)
    }
}

impl<N, T, I> ConstTypeOf_ for Discriminant<N, T, I>
where
    T: ConstType,
{
    type Type = DiscriminantFor<T>;
}

unsafe impl<N, T, I> MarkerType for Discriminant<N, T, I> {}

impl<N, T, I> InitializationValues for Discriminant<N, T, I>
where
    N: InitializationValues,
{
    type Uninitialized = N::Uninitialized;
    type Initialized = N::Initialized;
}

impl<T, I0, I1, N0, N1> ConstEq_<Discriminant<N1, T, I1>> for Discriminant<N0, T, I0>
where
    I0: ConstEq_<I1>,
{
    type Output = I0::Output;
}

impl<T, I0, I1, N0, N1> ConstOrd_<Discriminant<N1, T, I1>> for Discriminant<N0, T, I0>
where
    I0: ConstOrd_<I1>,
{
    type Output = I0::Output;
}

////////////////////////////////////////////////////////////////////////////////

/// Returns the discriminant of an enum variant.
pub trait GetDiscriminant {
    /// The discriminant for this ConstValue.
    ///     
    type Discriminant;

    /// The marker type used to have the name of the variant of this ConstValue in error messages.
    ///
    /// Used in the `construct` macro since it implements InitializationValues.
    type Variant;
}


/// Gets the discriminant for this ConstValue.
pub type GetDiscrOf<This>=
    <This as GetDiscriminant>::Discriminant;


/// Gets the marker type used to have the name of the variant of this ConstValue in error messages.
pub type GetVariantOf<This>=
    <This as GetDiscriminant>::Variant;


////////////////////////////////////////////////////////////////////////////////


#[cfg(all(test,feature="passed_tests"))]
mod tests{
    use super::*;
    use crate_::ops::*;

    #[test]
    fn test_type_aliases(){
        use std_types::option::type_level_Option::variants::{
            Some__Discr,Some__Variant,
            None__Discr,None__Variant,
        };
        use std_types::result::type_level_Result::variants::{
            Ok__Discr,Ok__Variant,
            Err__Discr,Err__Variant,
        };
        use std_types::range   ::type_level_Range  ::variants::{Range_Discr  ,Range_Variant};
        use std_types::range_to::type_level_RangeTo::variants::{RangeTo_Discr,RangeTo_Variant};

        let _:AssertEq< GetDiscrOf<Some_<()>> , Some__Discr >;
        let _:AssertEq< GetVariantOf<Some_<()>> , Some__Variant >;

        let _:AssertEq< GetDiscrOf<Ok_<()>> , Ok__Discr >;
        let _:AssertEq< GetVariantOf<Ok_<()>> , Ok__Variant >;

        let _:AssertEq< GetDiscrOf<Err_<()>> , Err__Discr >;
        let _:AssertEq< GetVariantOf<Err_<()>> , Err__Variant >;

        let _:AssertEq< GetDiscrOf<None_> , None__Discr >;
        let _:AssertEq< GetVariantOf<None_> , None__Variant >;

        let _:AssertEq< GetDiscrOf<ConstRange<U0,U0>> , Range_Discr >;
        let _:AssertEq< GetVariantOf<ConstRange<U0,U0>> , Range_Variant >;

        let _:AssertEq< GetDiscrOf<ConstRangeTo<U0>> , RangeTo_Discr >;
        let _:AssertEq< GetVariantOf<ConstRangeTo<U0>> , RangeTo_Variant >;


    }
}