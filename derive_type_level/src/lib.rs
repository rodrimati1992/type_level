//! 

extern crate derive_type_level_lib;
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(MutConstValue, attributes(mcv))]
pub fn derive_mutconstvalue(input: TokenStream) -> TokenStream {
    // (quote!{}).into()
    derive_type_level_lib::mutconstvalue::derive_from_token_stream(input.into()).into()
}

#[proc_macro_derive(TypeLevel, attributes(typelevel))]
pub fn derive_type_level(input: TokenStream) -> TokenStream {
    // (quote!{}).into()
    derive_type_level_lib::typelevel::derive_from_token_stream(input.into()).into()
}


#[proc_macro_derive(DocCodeSnippets, attributes(doccode))]
pub fn derive_doc_code_snippets(input: TokenStream) -> TokenStream {
    // (quote!{}).into()
    derive_type_level_lib::doc_code_snippets::derive_from_token_stream(input.into()).into()
}
