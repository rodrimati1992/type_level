#[macro_use]
extern crate derive_type_level;
#[allow(unused_imports)]
#[macro_use]
extern crate type_level_values;
#[macro_use]
extern crate derive_type_level_lib;
extern crate syn;
extern crate core_extensions;
extern crate quote;
#[macro_use]
extern crate lazy_static;



pub mod parsing;
mod check_variants;

// #[cfg(feature="passed_tests")]
mod test_reexports;

// #[cfg(feature="passed_tests")]
mod test_field_attrs;

// #[cfg(feature="passed_tests")]
mod typelevel_all_attrs;

// #[cfg(feature="passed_tests")]
mod typelevel_disabled_impls;

mod test_mutconstvalue;

#[path="../shared/mod.rs"]
mod shared;

#[allow(unused_imports)]
pub(crate) use self::shared::*;

#[allow(unused_imports)]
pub(crate) use self::shared::utils::{
    tokens_to_string,
    totoken_iter_to_string,
};


#[cfg(feature="passed_tests")]
include!(concat!(env!("OUT_DIR"), "/struct_enum_tests.rs"));

#[allow(unused_imports)]
use parsing::{
    Visiting,
    VisitItem,
    VisitItemsErrorKind as VIErrorKind,
    ModIndex,
    EnumOrStruct,
    type_level_modules,
    mut_const_value_modules,
    TLModIndex,
};

#[allow(unused_imports)]
use derive_type_level_lib::common_tokens::CommonTokens;

#[allow(unused_imports)]
use core_extensions::prelude::*;


mod type_level_shared{

    pub(crate)use type_level_values::ops::AssertEq;
    pub(crate)use type_level_values::field_traits::GetField;
    pub(crate)use type_level_values::prelude::*;

    pub(crate)use shared::traits::{Trivial};
    pub(crate)use check_variants::typelevel_field::{
        Field,
    };
    pub(crate)use derive_type_level_lib::parse_syn::parse_ident;
    pub(crate)use check_variants::datatype::{
        DataType,
        Variants,
        TLVariant,
        SHARED_FIELD_ATTR,
        test_items,
    };
    pub(crate)use check_variants::Exhaustiveness::{self,Exhaustive,Inexhaustive};
    pub(crate)use check_variants::{
        VariantKind,
        Privacy,
        PUB_DSUPER,
        UnparsedItemCheck,
        ToItemCheck,
        ItemToCheck,
    };

}