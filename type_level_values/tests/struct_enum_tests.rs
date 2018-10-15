#![allow(non_camel_case_types)]

pub mod parsing;
mod shared;

pub(crate) use self::shared::*;

include!(concat!(env!("OUT_DIR"), "/struct_enum_tests.rs"));

use parsing::test_tokens::TestTokens;
use parsing::check_impl::CheckImpl;
use parsing::{
    ExpectedItem,
    IndexableByMod,
    new_decls,
    Visiting,
    Declaration,
    PushDeclarationExt,
};

use derive_type_level_lib::common_tokens::CommonTokens;
use derive_type_level_lib::parse_syn::{
    parse_visibility,
    parse_ident,
    parse_syn_use,
};

use core_extensions::prelude::*;


use std::fmt::Write;




