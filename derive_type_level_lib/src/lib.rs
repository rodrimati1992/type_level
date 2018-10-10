#![deny(unused_imports)]
#![deny(unused_attributes)]
#![recursion_limit = "512"]

extern crate arrayvec;
#[macro_use]
extern crate core_extensions;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate lazy_static;
extern crate proc_macro2;
extern crate syn;
extern crate typed_arena;
extern crate regex;

#[macro_use]
pub(crate) mod macros;
#[macro_use]
pub mod indexable_struct;
pub(crate) mod attribute_errors;
pub(crate) mod attribute_detection;
pub(crate) mod to_token_fn;
pub(crate) mod find_type_param;
pub(crate) mod tlist_tokens;
pub(crate) mod token_suffixed;
pub(crate) mod submod_visibility;
pub(crate) mod self_removed_bound;
pub(crate) mod data_structure;
pub mod common_tokens;
pub mod typelevel;
pub mod const_constructor;
pub mod doc_code_snippets;
pub mod parse_syn;
pub(crate) mod void_like;


////////////////////////////////////////////////////////////////////////////////


pub(crate) trait FromWith<T,W>{
    fn from_with(from:T,with:W)->Self;
}

pub(crate) trait IntoWith<To,W>{
    fn into_with(self,with:W)->To;
}


impl<To,W,This> IntoWith<To,W> for This
where To:FromWith<Self,W>,
{
    #[inline]
    fn into_with(self,with:W)->To{
        To::from_with(self,with)
    }
}


////////////////////////////////////////////////////////////////////////////////

use typed_arena::Arena;


macro_rules! declare_arenas {
    (
        $( $field_name:ident : $arena_type:ty , )*
    ) => {
        pub(crate) struct Arenas {
            $(pub(crate) $field_name : Arena<$arena_type>, )*
        }

        impl Default for Arenas{
            fn default()->Self{
                Arenas{
                    $( $field_name:Arena::new(), )*
                }
            }
        }

    }
}


declare_arenas!{
    idents: syn::Ident,
    paths: syn::Path,
    types: syn::Type,
    metalists: syn::MetaList,
    visibilities: syn::Visibility,
    strings: String,
}




pub(crate) type ArenasRef<'alloc>=
    &'alloc Arenas;


use proc_macro2::{TokenStream};

pub(crate) fn print_derive_tokens(stream:&TokenStream)-> ! {
    use regex::Regex;

    let str_=format!("{}",stream);
    let regex= Regex::new(r#" (impl|#|pub|fn|use|for|where|\{|\})"#).unwrap();
    panic!("{}", regex.replace_all(&str_, "\n$1"));
}