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

    /// This is the unsigned integer of the discriminant of this ConstValue.
    type UIntDiscr;

    /// The marker type used to have the name of the variant of this ConstValue in error messages.
    ///
    /// Used in the `construct` macro since it implements InitializationValues.
    type Variant;
}


/// Gets the discriminant for this ConstValue.
pub type GetDiscrOf<This>=
    <This as GetDiscriminant>::Discriminant;

/// Gets the the unsigned integer of the discriminant of this ConstValue..
pub type GetUIntDiscrOf<This>=
    <This as GetDiscriminant>::UIntDiscr;


/// Gets the marker type used to have the name of the variant of this ConstValue in error messages.
pub type GetVariantOf<This>=
    <This as GetDiscriminant>::Variant;


type_fn!{
    /// Extracts the integer value of a Discriminant<..>.
    pub fn UIntFromDiscriminant[N, T, I](Discriminant<N, T, I>){ I }
}

type_fn!{
    /// Gets the Discriminant of This.
    pub fn GetDiscrOp[This](This)
    where[ This:GetDiscriminant ]
    { This::Discriminant }
}

type_fn!{
    /// Gets the integer value of the discriminant of This.
    pub fn GetUIntDiscrOp[This](This)
    where[ This:GetDiscriminant ]
    { This::UIntDiscr }
}

type_fn!{
    /// Gets the unit struct representing the variant of This.
    pub fn GetVariantOp[This](This)
    where[ This:GetDiscriminant ]
    { This::Variant }
}



////////////////////////////////////////////////////////////////////////////////


// #[cfg(test)]
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
        use std_types::range   ::type_level_Range  ::RangeType;
        use std_types::range   ::type_level_Range  ::variants::{Range_Discr  ,Range_Variant};
        use std_types::range_to::type_level_RangeTo::RangeToType;
        use std_types::range_to::type_level_RangeTo::variants::{RangeTo_Discr,RangeTo_Variant};

        type TestDiscr<This,Expected>=(
            AssertEq<GetDiscrOf<This>,Expected>,
            AssertFnRet<This,GetDiscrOp,Expected>,
        );
        type TestVariant<This,Expected>=(
            AssertEq<GetVariantOf<This>,Expected>,
            AssertFnRet<This,GetVariantOp,Expected>,
        );
        type TestUIntDiscr<This,Expected>=(
            AssertEq<GetUIntDiscrOf<This>,Expected>,
            AssertFnRet<This,GetUIntDiscrOp,Expected>,
        );

        let _:TestDiscr<Some_<False>, Some__Discr>;
        let _:TestDiscr<Some_<False>, Discriminant<Some__Variant,OptionType,U0>>;
        let _:TestVariant<Some_<False> , Some__Variant >;
        let _:TestUIntDiscr<Some_<False> , U0 >;

        let _:TestDiscr<None_, Discriminant<None__Variant,OptionType,U1> >;
        let _:TestDiscr<None_, None__Discr >;
        let _:TestVariant<None_ , None__Variant >;
        let _:TestUIntDiscr<None_ , U1 >;


        let _:TestDiscr<Ok_<False>, Discriminant<Ok__Variant,ResultType,U0> >;
        let _:TestDiscr<Ok_<False>, Ok__Discr >;
        let _:TestVariant<Ok_<False> , Ok__Variant >;
        let _:TestUIntDiscr<Ok_<False> , U0 >;

        let _:TestDiscr<Err_<False>, Discriminant<Err__Variant,ResultType,U1> >;
        let _:TestDiscr<Err_<False>, Err__Discr >;
        let _:TestVariant<Err_<False> , Err__Variant >;
        let _:TestUIntDiscr<Err_<False> , U1 >;


        let _:TestDiscr<ConstRange<U0,U0> , Discriminant<Range_Variant,RangeType,U0> >;
        let _:TestDiscr<ConstRange<U0,U0> , Range_Discr >;
        let _:TestVariant<ConstRange<U0,U0> , Range_Variant >;
        let _:TestUIntDiscr<ConstRange<U0,U0> , U0 >;

        let _:TestDiscr<ConstRangeTo<U0> , Discriminant<RangeTo_Variant,RangeToType,U0> >;
        let _:TestDiscr<ConstRangeTo<U0> , RangeTo_Discr >;
        let _:TestVariant<ConstRangeTo<U0> , RangeTo_Variant >;
        let _:TestUIntDiscr<ConstRangeTo<U0> , U0 >;


    }
}