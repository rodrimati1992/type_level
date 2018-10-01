use super::*;

use user_traits::const_traits::{
    AllowOp, ConstLayoutIndependent, GetConstConstructor_, SetConstParam, SetConstParam_,
};

use user_traits::const_methods::{ComputeConstParam_, OpAttrs};

use user_traits::allowed_conversions_type::type_level_AllowedConversions::fields as fields_ac;
use user_traits::allowed_conversions_type::AllowedConversionsTrait;

use std_::mem::{size_of, size_of_val};
#[cfg(feature = "std")]
use std_::rc::Rc;
#[cfg(feature = "std")]
use std_::sync::Arc;

/// Extension trait to
/// mutate the Const-parameter of Self (these methods are side-effect free).
///
/// The parameters of the methods in this trait follow this pattern:
///
/// - self/this: the receiver of the method,the type whose Const-parameter will change.
///
/// - ConstMethod : the ConstMethod being called .
///
/// - Msg : the type of the parameter to the ConstMethod .
///
///
pub trait MutConstParam: GetConstConstructor_ {
    /// Mutates the Const-parameter of Self.
    #[inline(always)]
    fn mutparam<Op, Msg>(self, _const_method: Op, _msg: VariantPhantom<Msg>) -> Self::NextSelf
    where
        Self: Sized + MutConstParamConstraints<Op, Msg, fields_ac::by_val>,
        Self::NextSelf: Sized,
    {
        unsafe { transmute_ignore_size(self) }
    }

    /// Mutates the Const-parameter of Box<Self>.
    #[inline(always)]
    #[cfg(feature = "std")]
    fn mutparam_box<Op, Msg>(
        self: Box<Self>,
        _const_method: Op,
        _msg: VariantPhantom<Msg>,
    ) -> Box<Self::NextSelf>
    where
        Self: MutConstParamConstraints<Op, Msg, fields_ac::by_val>,
    {
        unsafe { transmute_ignore_size(self) }
    }

    /// Mutates the Const-parameter of &'a Self.
    #[inline(always)]
    fn mutparam_ref<'a, Op, Msg>(
        &'a self,
        _const_method: Op,
        _msg: VariantPhantom<Msg>,
    ) -> &'a Self::NextSelf
    where
        Self: MutConstParamConstraints<Op, Msg, fields_ac::by_ref>,
        Self::NextSelf: 'a,
    {
        unsafe { transmute_ignore_size(self) }
    }

    /// Mutates the Const-parameter of Rc<Self>.
    #[inline(always)]
    #[cfg(feature = "std")]
    fn mutparam_rc<Op, Msg>(
        this: Rc<Self>,
        _const_method: Op,
        _msg: VariantPhantom<Msg>,
    ) -> Rc<Self::NextSelf>
    where
        Self: MutConstParamConstraints<Op, Msg, fields_ac::by_ref>,
    {
        unsafe { transmute_ignore_size(this) }
    }

    /// Mutates the Const-parameter of Arc\<Self>.
    #[inline(always)]
    #[cfg(feature = "std")]
    fn mutparam_arc<Op, Msg>(
        this: Arc<Self>,
        _const_method: Op,
        _msg: VariantPhantom<Msg>,
    ) -> Arc<Self::NextSelf>
    where
        Self: MutConstParamConstraints<Op, Msg, fields_ac::by_ref>,
    {
        unsafe { transmute_ignore_size(this) }
    }

    /// Mutates the Const-parameter of &'a mut Self.
    #[inline(always)]
    fn mutparam_mut<'a, Op, Msg>(
        &'a mut self,
        _const_method: Op,
        _msg: VariantPhantom<Msg>,
    ) -> &'a mut Self::NextSelf
    where
        Self: MutConstParamConstraints<Op, Msg, fields_ac::by_mut>,
        Self::NextSelf: 'a,
    {
        unsafe { transmute_ignore_size(self) }
    }
}

impl<This> MutConstParam for This where This: GetConstConstructor_ {}

/// Trait used to alias the constraints for every MutConstParam method,
/// which are on the blanket impl of this trait.
pub trait MutConstParamConstraints<Op, Msg, Pointerness>: MCPBounds<Op, Msg> {}

impl<This: ?Sized, Op, Msg, Pointerness> MutConstParamConstraints<Op, Msg, Pointerness> for This
where
    This: MCPBounds<Op, Msg>,
    Op: OpAttrs,
    Op::Conversions: AllowedConversionsTrait,
    Op::Conversions: GetField_<Pointerness, Output = True>,
{}

/// The constraints for calling any MutConstParam methods.
pub trait MCPBounds<Op, Msg> {
    /// The mutated Const-parameter.
    type NextConst;

    /// The type of `Self` after changing its Const-parameter
    type NextSelf: ?Sized;
}

impl<This: ?Sized, NextConst, Op, Msg> MCPBounds<Op, Msg> for This
where
    This: GetConstConstructor_,
    Op: ComputeConstParam_<This::Const, Msg, Output = NextConst>,
    This::Constructor: AllowOp<Op>,
    This: SetConstParam_<NextConst>,
    This: ConstLayoutIndependent<SetConstParam<This, NextConst>>,
{
    type NextConst = NextConst;
    type NextSelf = SetConstParam<This, NextConst>;
}
