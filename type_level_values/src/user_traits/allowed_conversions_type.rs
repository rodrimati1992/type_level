//! Module containing the AllowedConversions type and aliases for its possible values.
//!

use prelude::*;

#[derive(Debug, Copy, Clone, Default, TypeLevel)]
#[typelevel(
    // skip_derive,
    // print_derive,
    reexport(Struct,Traits),
    derive(ConstEq, ConstOrd),
    items(runtime_conv(NoImpls)),
    doc = "\
        This is used when defining Const-methods to determine 
        the allowed self-types (value/reference/mutable reference) on 
        [MutConstParam](./mut_const_param_ext/struct.MutConstParam.html),
        limiting which methods on the trait can be called .
    "
)]
pub struct AllowedConversions {
    #[typelevel(doc="If True:enables `mutparam_ref`,`mutparam_rc`,`mutparam_arc` ")]
    pub by_ref: bool,
    #[typelevel(doc="If True:enables `mutparam_mut` ")]
    pub by_mut: bool,
    #[typelevel(doc="If True:enables `mutparam` , `mutparam_box` ")]
    pub by_val: bool,
}

//////////////////////////////////////////////////////////////////////////////////////////////////

/// Contains aliases for different values of
/// [ConstAllowedConversions](../struct.ConstAllowedConversions.html).
pub mod allowed_conversions {


    use super::type_level_AllowedConversions::{
        fields, AllowedConversionsType, ConstAllowedConversions,
    };
    use super::*;

    pub type All = construct!{
        AllowedConversionsType=>
        fields::by_ref=True,
        fields::by_mut=True,
        fields::by_val=True,
    };

    pub type NoConversions = construct!{
        AllowedConversionsType=>
        fields::by_ref=False,
        fields::by_mut=False,
        fields::by_val=False,
    };

    pub type ByRef = construct!{
        AllowedConversionsType=>
        fields::by_ref=True,
        fields::by_mut=False,
        fields::by_val=True,
    };

    pub type ByMut = construct!{
        AllowedConversionsType=>
        fields::by_ref=True,
        fields::by_mut=True,
        fields::by_val=False,
    };

    pub type ByVal = construct!{
        AllowedConversionsType=>
        fields::by_ref=False,
        fields::by_mut=False,
        fields::by_val=True,
    };

}
