use syn;
use syn::parse;
use syn::parse::Error as parseError;
use syn::punctuated::Punctuated;
use syn::token::Add;
use syn::TypeParamBound;

use attribute_detection::shared::bounds_from_str;

pub fn parse_error_msg<T>(invalid_msg: &str, str_: &str, e: parseError) -> T {
    panic!("\n\n{}:\n    '{}'\n\nerror:\n{}\n\n", invalid_msg, str_, e)
}

// pub fn parse_error_msg<T,E>(invalid_msg:&str,str_:&str,e:E)->T
// where
//     E: ::std::fmt::Debug
// {
//     panic!("\n\n{}:\n    '{}'\n\nerror:{:#?}\n\n",invalid_msg,str_,e )
// }

pub fn parse_where_pred(str_: &str) -> syn::WherePredicate {
    syn::parse_str(str_).unwrap_or_else(|e| parse_error_msg("Invalid where predicate", str_, e))
}

pub fn parse_ident(str_: &str) -> syn::Ident {
    syn::parse_str(str_).unwrap_or_else(|e| parse_error_msg("Invalid identifier", str_, e))
}

pub fn parse_type(str_: &str) -> syn::Type {
    syn::parse_str(str_).unwrap_or_else(|e| parse_error_msg("Invalid type", str_, e))
}

pub fn parse_visibility(str_: &str) -> syn::Visibility {
    syn::parse_str(str_).unwrap_or_else(|e| parse_error_msg("Invalid syn::Visibility", str_, e))
}

pub fn parse_syn_path(str_: &str) -> syn::Path {
    syn::parse_str(str_).unwrap_or_else(|e| parse_error_msg("Invalid syn::Path", str_, e))
}

pub fn parse_syn_use(str_: &str) -> syn::ItemUse {
    syn::parse_str(str_).unwrap_or_else(|e| parse_error_msg("Invalid syn::ItemUse", str_, e))
}

pub fn parse_syn_attributes(str_: &str) -> Vec<syn::Attribute> {
    syn::parse_str::<ParseOuter>(str_)
        .unwrap_or_else(|e| parse_error_msg("Invalid syn::Attribute", str_, e))
        .attributes
}

pub fn parse_bounds(str_: &str) -> Punctuated<TypeParamBound, Add> {
    let mut list = Punctuated::<TypeParamBound, Add>::new();
    bounds_from_str(str_, &mut list);
    list
}

struct ParseOuter {
    attributes: Vec<syn::Attribute>,
}

impl parse::Parse for ParseOuter {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        Ok(Self {
            attributes: syn::Attribute::parse_outer(input)?,
        })
    }
}
