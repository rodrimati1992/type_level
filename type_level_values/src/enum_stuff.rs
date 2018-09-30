use prelude::*;

use field_traits::InitializationValues;

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
    /// enum_stuff::Discriminant.Implements many traits,including ConstOrd and ConstEq.
    ///     
    type Discriminant;

    /// The marker type used to have the name of the variant in error messages.
    ///
    /// Used in the `construct` macro since it implements InitializationValues.
    type Variant;
}

////////////////////////////////////////////////////////////////////////////////
