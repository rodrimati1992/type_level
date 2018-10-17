pub(crate) mod compiletime_traits;
pub(crate) mod derived_traits;
pub(crate) mod field_traits;
pub(crate) mod struct_declarations;
pub(crate) mod variants_mod;
pub(crate) mod reexports;

// #[cfg(test)]
// pub(crate) mod test;


// use std::cmp::max;
use std::iter;


// use std::collections::{BTreeSet};


use super::{
    attribute_detection,
    Arenas,
    ArenasRef,
};

use data_structure::{
    DataStructure,
    FieldIdent,
    Struct,
    StructKind,
    EnumOrStruct,
};

use syn::{
    self,
    DeriveInput,
    Ident,
};
#[allow(unused_imports)]
use quote::TokenStreamExt;
use quote::ToTokens;

// use core_extensions::IterCloner;
use core_extensions::SelfOps;
use core_extensions::iterators::ReplaceNth;

use proc_macro2::{TokenStream};

// use typed_arena::Arena;

use self::attribute_detection::typelevel::{
    TLAttributes,
    FieldAttrs,
    ImplVariant,
    ImplIndex,
    ReExportVis,
    // ImplVariantMethods,
};

use self::compiletime_traits::CompiletimeTraits;
use self::derived_traits::DerivedTraits;
use self::field_traits::FieldTraits;
use self::struct_declarations::{
    StructDeclarations,
    // StructDeclaration,
    // FieldDeclaration,
};
use self::variants_mod::VariantsMod;

use ::to_token_fn::{
    ToTokenFnMut,
};

use common_tokens::CommonTokens;

// use submod_visibility::VisibilityKind;

use ::print_derive_tokens;

use self::reexports::ReExportPrinter;


pub fn derive_from_derive_input(mut ast:DeriveInput) -> TokenStream {

    ast.generics.make_where_clause();
    let ast=ast;

    let arenas=Arenas::default();
    let ref ds=DataStructure::new(&ast);

    let name = ds.name;

    let new_ident=|s:String| Ident::new(&s,name.span()) ;

    let ref created_module=format!("type_level_{}",name).piped(new_ident);

    if ds.variants.is_empty() {
        panic!("cannot derive Enumerable on empty enums");
    }


    
    let ref attribute_detected=TLAttributes::new(&ast.attrs,&arenas);

    if attribute_detected.skip_derive {
        return quote!();
    }

    let ref vis=ds.vis;
    
    let ref c_tokens = CommonTokens::new();
    let ref struct_decls=StructDeclarations::new(&ds,attribute_detected,&arenas,c_tokens);
    let ref field_traits=FieldTraits::new(&struct_decls);
    let ref ctime_traits=CompiletimeTraits::new(&struct_decls);
    let ref variants_mod_=VariantsMod::new(&struct_decls);
    let ref derived_traits_=DerivedTraits::new(&struct_decls);
    let reexport=ReExportPrinter{
        struct_decls,
        derive_vis:vis,
        created_module,
        cfg:&attribute_detected.reexports,
        c_tokens,
    };

    if attribute_detected.print_attributes{
        panic!("{:#?}", attribute_detected);
    }
    if attribute_detected.print_debug{
        panic!("{:#?}", struct_decls);
    }

    let mut output=quote!(
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        #[allow(unused_imports)]
        #vis mod #created_module { 
            use super::*;
            use type_level_values::reexports::*;

            #struct_decls

            #field_traits

            #ctime_traits

            #variants_mod_

            #derived_traits_

        }

        #reexport
    );
    if attribute_detected.print_derive {
        print_derive_tokens(&output);
    }
    if attribute_detected.derive_str {
        let derive_str=format!("{}",output);
        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
        
        output.append_all(quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                pub const TYPELEVEL_DERIVE:&'static str=#derive_str;
            }
        });
    }
    output
}

pub fn derive_from_str(input:&str) -> TokenStream {
    let ast :DeriveInput = syn::parse_str(input).unwrap();
    derive_from_derive_input(ast)
}


pub fn derive_from_token_stream(input: TokenStream) -> TokenStream {
    let ast :DeriveInput = syn::parse2(input).unwrap();
    derive_from_derive_input(ast)
} 