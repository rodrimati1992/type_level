extern crate derive_type_level_lib;
extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(ConstConstructor, attributes(cconstructor))]
pub fn derive_const_constructor(input: TokenStream) -> TokenStream {
    // (quote!{}).into()
    derive_type_level_lib::const_constructor::derive_from_token_stream(input.into()).into()
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
