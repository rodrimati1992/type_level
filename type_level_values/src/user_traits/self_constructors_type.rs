//! Contains the AllowedConstructors type and aliases for its possible values.
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
        This is used when defining Mutator Functions to determine 
        the methods that are enabled on the MutConstParam trait.
    "
)]
pub struct AllowedConstructors {
    #[typelevel(doc = "If True:enables `mutparam_ref`,`mutparam_rc`,`mutparam_arc` ")]
    pub by_ref: bool,
    #[typelevel(doc = "If True:enables `mutparam_mut` ")]
    pub by_mut: bool,
    #[typelevel(doc = "If True:enables `mutparam` , `mutparam_box` ")]
    pub by_val: bool,
}

//////////////////////////////////////////////////////////////////////////////////////////////////

/// Contains aliases for different values of
/// [ConstAllowedConstructors
/// ](../type_level_AllowedConstructors/struct.ConstAllowedConstructors.html).
pub mod allowed_self_constructors {

    use super::type_level_AllowedConstructors::{
        fields, AllowedConstructorsType, ConstAllowedConstructors,
    };
    use super::*;

    /// Allows all methods on MutConstParam to be called
    pub type All = Construct<AllowedConstructorsType,(
        (fields::by_ref,True),
        (fields::by_mut,True),
        (fields::by_val,True),
    )>;

    /// Allows methods taking &This/Rc<This>/Arc<This>/This/Box<This> 
    /// on MutConstParam to be called
    pub type ByRef = Construct<AllowedConstructorsType,(
        (fields::by_ref,True),
        (fields::by_mut,False),
        (fields::by_val,True),
    )>;

    /// Allows methods taking &This/Rc<This>/Arc<This>/&mut This
    /// on MutConstParam to be called
    pub type ByMut = Construct<AllowedConstructorsType,(
        (fields::by_ref,True),
        (fields::by_mut,True),
        (fields::by_val,False),
    )>;

    /// Allows methods taking This/Box<This> on MutConstParam to be called
    pub type ByVal = Construct<AllowedConstructorsType,(
        (fields::by_ref,False),
        (fields::by_mut,False),
        (fields::by_val,True),
    )>;

}
