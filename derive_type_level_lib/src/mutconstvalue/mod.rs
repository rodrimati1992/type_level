// use std::cmp::max;
// use std::iter;

use parse_syn;
use std::collections::{HashSet};


use super::{
    // attribute_detection,
    Arenas,
    // ArenasRef,
};

use attribute_detection::mutconstvalue::{
    CCAttributes,
    TypeDeclVariant,
};

use data_structure::DataStructure;

use syn::{
    self,
    DeriveInput,
    Ident,
    GenericParam,
};


use quote::{
    TokenStreamExt,
    ToTokens,
};

// use core_extensions::IterCloner;
use core_extensions::SelfOps;
use core_extensions::iterators::IteratorExt;

use proc_macro2::{TokenStream};

// use typed_arena::Arena;

use ::find_type_param::FindTypeParam;

use ::to_token_fn::{
    ToTokenFnMut,
};


use ::print_derive_tokens;

use std::mem;

pub fn derive_from_derive_input(mut ast:DeriveInput) -> TokenStream {
    // println!("{S}{S}{S}\n{}\n",ast.ident,S="-------------------------");


    use syn::token::Comma;
    let comma=Comma::default();
    let attrs=mem::replace(&mut ast.attrs,Vec::new());
    ast.generics.make_where_clause();
    for generic in &mut ast.generics.params {
        match *generic {
            GenericParam::Type(ref mut t_param)=>{
                t_param.eq_token=None;
                t_param.default=None;
            },
            _=>{}
        }
    }
    if !ast.generics.params.trailing_punct() {
        ast.generics.params.push_punct(comma);
    }
    
    let arenas=Arenas::default();
    let attrs=&CCAttributes::new(&attrs,&arenas);
    let vis =ast.vis.clone();
    let original_name=ast.ident.clone();


    let field_tys:HashSet<syn::Type>;

    {
        let ds=DataStructure::new(&ast);
        
        field_tys=ds.variants
            .iter()
            .flat_map(|v| &v.fields )
            .map(|f| f.ty.clone() )
            .collect();
    }
    
    let new_ident=&|s:String|{
        &*arenas.idents.alloc(parse_syn::parse_ident(&s))
    };

    let created_module=format!("const_constructor_{}",original_name).piped(new_ident);

    if attrs.skip_derive {
        // println!("skipping derive");
        return quote!();
    }

    let help_message="\n\
        Help:\n\
        \n\
        Required parameters:\n\t\
            #[mcv(Type=\"Foo\",ConstValue =\"Const\")]\n\
        \n\
    ";

    // let ref attrs_cc_impl =attrs.impls.const_constructor.impl_annotations();
    // let ref bounds_cc_impl =attrs.impls.const_constructor.bound_tokens();

    // let ref attrs_acp=attrs.impls.apply_const_param.impl_annotations();
    // let ref bounds_acp=attrs.impls.apply_const_param.bound_tokens();
    
    // let ref attrs_gcc=attrs.impls.get_const_constructor.impl_annotations();
    // let ref bounds_gcc=attrs.impls.get_const_constructor.bound_tokens();
    
    let ref attrs_gcp=attrs.impls.get_const_param.impl_annotations();
    let ref bounds_gcp=attrs.impls.get_const_param.bound_tokens();

    // let ref attrs_cc_type=attrs.const_constructor.impl_annotations();
    // let ref bounds_cc_type=attrs.const_constructor.bound_tokens();

    let ref attrs_cli =attrs.impls.const_layout_independent.impl_annotations();
    let ref bounds_cli=attrs.impls.const_layout_independent.bound_tokens();

    let ref attrs_type_alias=attrs.type_alias.impl_annotations();
    

    let type_alias=&attrs.type_alias.inner.decl.unwrap_or_else(||{
        panic!("must pass the 'Type' parameter.\n{}",help_message);
    });
    let type_alias_ident=type_alias.ident();

    let const_constructor_ident=new_ident(format!("{}_CC",type_alias_ident));
    
    attrs.attrs.bounds.iter().cloned()
        .extending( &mut ast.generics.make_where_clause().predicates );

    let delegated_attrs=&attrs.attrs.attrs;
    let delegated_docs =&attrs.attrs.docs;
    let name=new_ident(format!("{}_Ty",type_alias_ident));

    // let typeconstr_ident=new_ident(format!("{}_T",type_alias_ident));
    // let typeconstr_doc=format!("\
    //     The TypeMarker for {0},used to talk about {0} in generic contexts.\
    // ",name);

    ast.ident=name.clone();

    let const_param_for_alias=new_ident(format!("__ConstParam"));
    // let const_param_for_alias_rep=iter::repeat(const_param_for_alias);
    let (const_param_ident,const_param_default)=
        attrs.const_param.unwrap_or_else(||{
            panic!("must pass the 'Param' parameter.\n{}",help_message);
        });


    let ref lifetimes   =ast.generics.lifetimes().collect::<Vec<_>>();
    let ref lifetime_idents=lifetimes.iter().map(|x| &x.lifetime ).collect::<Vec<_>>();
    
    let ref type_params =ast.generics.type_params().collect::<Vec<_>>();
    let ref type_param_idents=type_params.iter().map(|x| &x.ident ).collect::<Vec<_>>();
    
    let ref type_alias_ty_params=type_params.iter()
        .map(|x| {
            let ident=&x.ident;
            if x.ident==*const_param_ident { 
                quote!( #const_param_for_alias #( = #const_param_default )* )
            }else{ 
                quote!( #ident )
            }
        })
        .collect::<Vec<_>>();

    let ref truncated_type_params=type_param_idents.iter()
        .filter(|x| **x!=const_param_ident )
        .collect::<Vec<_>>();

    let ref const_params=ast.generics.const_params().collect::<Vec<_>>();
    let ref const_param_idents=const_params.iter().map(|x| &x.ident ).collect::<Vec<_>>();
    
    let ref remaining_generics=quote!{
        #(#lifetimes,)* #(#truncated_type_params:?Sized,)* #(#const_params,)*
    };

    let ref remaining_g_params=quote!{
        #(#lifetime_idents,)* #(#truncated_type_params,)* #(#const_param_idents,)*
    };


    let type_params_wrapped=ToTokenFnMut::new(|t_s|{
        for tparam in type_param_idents {
            if *tparam==const_param_ident{
                t_s.append_all(quote!{
                    ::type_level_values::reexports::ConstWrapper<#const_param_for_alias>
                });
            }else{
                tparam.to_tokens(t_s);
            }
            comma.to_tokens(t_s);
        }
    });

    let ref type_alias_gen_params=quote!{
        #(#lifetimes,)*
        #type_params_wrapped
        #(#const_params,)*
    };


    let ref generics=ast.generics;
    
    let ref generic_params=generics.params;

    
    let ref field_tys_mentioning_const={
        let const_finder=FindTypeParam::new(const_param_ident);

        field_tys.iter().cloned()
            .filter(|ty| const_finder.in_(ty) )
            .collect::<Vec<_>>()
    };

    if field_tys_mentioning_const.is_empty() {
        panic!("ConstValue-parameter '{}' is never used", const_param_ident);
    }

    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let where_clause=&where_clause.unwrap().predicates;

    let mut tokens=TokenStream::new();
    
    let ty_doc   =format!(
        "\n# Remaining Impls\nThe remaining impls for this are on [{0}](./type.{0}.html)",
        type_alias_ident,
    );

    tokens.append_all(quote!{
        #(#[#delegated_attrs])*
        #(#[doc=#delegated_docs])*
        #[doc=#ty_doc]
        #ast
    });

    let ty_html_prefix=match ast.data {
        syn::Data::Struct{..}=>"struct",
        syn::Data::Enum{..}=>"enum",
        syn::Data::Union{..}=>"union",
    };

    let alias_doc=format!(
        "The type alias for [{0}](./{pre}.{0}.html).
        Use this instead for everything,except for implementing Drop.",
        name,
        pre=ty_html_prefix,
    );

    if let TypeDeclVariant::Name(_)=*type_alias {
        tokens.append_all(quote!{
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            #attrs_type_alias
            #[doc=#alias_doc]
            #vis type #type_alias_ident< 
                #(#lifetimes,)*
                #(#type_alias_ty_params,)*
                #(#const_params,)*
            >=#name < #type_alias_gen_params >;
        });
    }
    
    let field_indices=&(0..field_tys_mentioning_const.len())
        .map(|i| new_ident(format!("U{}",i)) )
        .collect::<Vec<&Ident>>();


    let field_accessors=field_indices.iter().cloned().zip(field_tys_mentioning_const)
        .map(|(field_i,field_tmc)|{
            quote!{
                impl < #(#generic_params,)*> 
                    GetField_<ConstDependentField<integer_reexports::#field_i>> 
                for #name #ty_generics
                where 
                    #(#where_clause,)*
                {
                    type Output=PhantomData<#field_tmc>;
                }
            }
        })
        .collect::<Vec<TokenStream>>();


    tokens.append_all(quote!(
        // pub use self::#created_module::#typeconstr_ident;

        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        mod #created_module { 
            #[doc(hidden)]
            pub struct #const_constructor_ident< #remaining_generics > {
                _marker: 
                    ::type_level_values::reexports::VariantPhantom<(
                        #( & #lifetimes () ,)*
                        #(::type_level_values::reexports::VariantPhantom<#truncated_type_params>,)*
                    )>
            }
            use super::*;
            use type_level_values::reexports::*;
            use type_level_values::user_traits::const_traits as _const_traits;
            use type_level_values::const_wrapper::ConstWrapper;

            #[doc(hidden)]
            pub struct ConstDependentField<T>(T);

            #(#field_accessors)*

            // #attrs_cc_impl
            impl< #remaining_generics > 
                _const_traits::ConstConstructor
            for #const_constructor_ident< #remaining_g_params > 
            where 
                // #bounds_cc_impl
                // #bounds_cc_type
            {}
            
            // #attrs_gcc
            impl < #(#generic_params,)* > _const_traits::GetConstConstructor_ for #name #ty_generics 
            where 
                #(#where_clause,)*
                // #bounds_gcc
                // #bounds_cc_type
                Self:_const_traits::GetConstParam_,
            {
                type Constructor = #const_constructor_ident< #remaining_g_params >;
            }
            
            #attrs_cli
            unsafe impl < #(#generic_params,)* __Other:?Sized> 
                _const_traits::ConstLayoutIndependent<__Other>
            for #name #ty_generics
            where 
                #(#where_clause,)*
                #bounds_cli
                #(
                    __Other:_const_traits::SameFieldLayout<
                        ConstDependentField<integer_reexports::#field_indices>,
                        Self
                    >,
                )*
            {}

            #attrs_gcp
            impl < #(#generic_params,)* #const_param_for_alias> _const_traits::GetConstParam_  
            for #name #ty_generics
            where 
                #(#where_clause,)*
                #bounds_gcp
                #const_param_ident:TypeIdentity<Type=ConstWrapper<#const_param_for_alias>>
            {
                type Const = #const_param_for_alias;
            }

            impl < #(#generic_params,)* #const_param_for_alias, __Output,> 
                _const_traits::ApplyConstParam_<#const_param_for_alias>
            for #const_constructor_ident< #remaining_g_params > 
            where 
                #(#where_clause,)*
                ConstWrapper<#const_param_for_alias>:TypeIdentity<Type=#const_param_ident>,
                #name #ty_generics:TypeIdentity<Type=__Output>,
                __Output:
                    _const_traits::GetConstConstructor_<
                        Const=#const_param_for_alias,
                        Constructor=Self
                    >,
            {
                type Applied = __Output ;
            }


            // #[doc=#typeconstr_doc]
            // #vis struct #typeconstr_ident;

            // impl _const_traits::TypeMarker for #typeconstr_ident{}

            // impl < #(#generic_params,)* > _const_traits::TypeMarkerOf_ for #name #ty_generics 
            // where 
            //     #(#where_clause,)*
            // {
            //     type Marker = #typeconstr_ident ;
            // }

        }
    ));
    
    if attrs.print_derive {
        print_derive_tokens(&tokens);
    }

    if attrs.derive_str {
        let derive_str=format!("{}",tokens);
        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
        
        tokens.append_all(quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                pub const TYPELEVEL_DERIVE:&'static str=#derive_str;
            }
        });
    }

    tokens
}

pub fn derive_from_str(input:&str) -> TokenStream {
    let ast :DeriveInput = syn::parse_str(input).unwrap();
    derive_from_derive_input(ast)
}


pub fn derive_from_token_stream(input: TokenStream) -> TokenStream {
    let ast :DeriveInput = syn::parse2(input).unwrap();
    derive_from_derive_input(ast)
} 




////////////////////////////////////////////////////////////////////////////////




////////////////////////////////////////////////////////////////////////////////










////////////////////////////////////////////////////////////////////////////////










////////////////////////////////////////////////////////////////////////////////










////////////////////////////////////////////////////////////////////////////////










////////////////////////////////////////////////////////////////////////////////






