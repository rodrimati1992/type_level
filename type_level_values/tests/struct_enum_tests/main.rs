#![allow(non_camel_case_types)]


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

#[cfg(feature="passed_tests")]
mod test_reexports;

#[cfg(feature="passed_tests")]
mod test_field_attrs;

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
    VisitItemsError as VIError,
    VisitItemsErrorKind as VIErrorKind,
    ModIndex,
    EnumOrStruct,
};

#[allow(unused_imports)]
use derive_type_level_lib::common_tokens::CommonTokens;

#[allow(unused_imports)]
use core_extensions::prelude::*;


