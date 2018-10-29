pub(crate) mod typelevel_field;

pub(crate) mod typelevel;

pub(crate) mod impl_block;

use syn;

pub(crate) use self::impl_block::{
    UnparsedImplBlock,
    ImplBlock,
    ImplHeader,
    ToImplBlock,
};

use type_level_values::prelude::*;

use core_extensions::OptionExt;
use core_extensions::BoolExt;

use shared::utils::{
    totoken_iter_to_string,
    tokens_to_string,
    display_totokens,
    AlwaysDisplay,
};

use shared::traits::{OptIdent};

use std::iter;
use std::borrow::Cow;
use std::collections::{HashMap,HashSet};
use std::rc::Rc;

use derive_type_level_lib::submod_visibility::{MyVisibility};
use derive_type_level_lib::parse_syn::*;

#[allow(unused_imports)]
pub(crate) use ::shared::*;



#[cfg(feature="passed_tests")]
include!(concat!(env!("OUT_DIR"), "/struct_enum_tests.rs"));

#[allow(unused_imports)]
use parsing::{
    Visiting,
    VisitItem,
    VisitItemsErrorKind as VIEK,
    ModIndex,
    EnumOrStruct,
    Module,
};

#[allow(unused_imports)]
use derive_type_level_lib::common_tokens::CommonTokens;

#[allow(unused_imports)]
use core_extensions::prelude::*;



use syn::{
    Attribute,
    Fields,
    Ident,
    ItemUse,
    Path as SynPath,
    TraitItem,
    Type as SynType,
    WherePredicate,
};


#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[allow(dead_code)]
pub(crate) enum VariantKind{
    Braced,
    Tupled,
    Unit,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub(crate) enum Privacy{
    Private,
    Inherited,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub(crate) enum Exhaustiveness{
    /// Means that a list is exhaustive.
    Exhaustive,
    /// Means that a list is inexhaustive,listing only the things we want to check.
    Inexhaustive,
}
use self::Exhaustiveness::{ Exhaustive,Inexhaustive };

/// Whether this must exist or must not exist.
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub enum Existence{
    /// Means that this item must exist.
    Exists,
    /// Means that this item must not exist.
    NotExists,
}
use self::Existence::{Exists,NotExists};



#[derive(Copy,Clone,PartialEq,Eq)]
pub(crate) enum AccessorKind{
    Struct,
    Integer,
}



pub(crate) static PUB_DSUPER:&str="pub(in super::super)";


pub(crate) fn empty_slice<'a,T>()->&'a [T]{
    &[]
}
