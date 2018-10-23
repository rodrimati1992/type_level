// use std::cmp::max;
// use std::iter;

use std::collections::{HashSet};


use super::{
    // attribute_detection,
    Arenas,
    // ArenasRef,
};

use attribute_detection::const_constructor::{
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
// use core_extensions::iterators::ReplaceNth;

use proc_macro2::{TokenStream};

// use typed_arena::Arena;

use ::find_type_param::FindTypeParam;

use ::to_token_fn::{
    ToTokenFnMut,
};


use ::print_derive_tokens;


pub fn derive_from_derive_input(mut ast:DeriveInput) -> TokenStream {
    use syn::token::Comma;
    let comma=Comma::default();
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
    if ast.generics.params.empty_or_trailing() {
        ast.generics.params.push_punct(comma);
    }
    let ast=ast;

    let arenas=Arenas::default();
    let ds=DataStructure::new(&ast);
    let attrs=&CCAttributes::new(&ast.attrs,&arenas);
    let vis=&ast.vis;
    let name=&ds.name;

    if attrs.skip_derive {
        return quote!();
    }

    let new_ident=&|s:String|{
        &*arenas.idents.alloc(Ident::new(&s,name.span())) 
    };
    let help_message="\n\
        Help:\n\
        \n\
        Required parameters:\n\t\
            #[cconstructor(Type=\"Foo\",ConstParam=\"Const\")]\n\
        \n\
        Optional parameters:\n\t\
            #[cconstructor(ConstConstructor=\"FooCC\")]\n\
        \n\
    ";

    let ref attrs_cc_impl =attrs.impls.const_constructor.impl_annotations();
    let ref bounds_cc_impl =attrs.impls.const_constructor.bound_tokens();

    let ref attrs_acp=attrs.impls.apply_const_param.impl_annotations();
    let ref bounds_acp=attrs.impls.apply_const_param.bound_tokens();
    
    let ref attrs_gcc=attrs.impls.get_const_constructor.impl_annotations();
    let ref bounds_gcc=attrs.impls.get_const_constructor.bound_tokens();
    
    let ref attrs_gcp=attrs.impls.get_const_param.impl_annotations();
    let ref bounds_gcp=attrs.impls.get_const_param.bound_tokens();

    let ref attrs_cc_type=attrs.const_constructor.impl_annotations();
    let ref bounds_cc_type=attrs.const_constructor.bound_tokens();

    let ref attrs_cli =attrs.impls.const_layout_independent.impl_annotations();
    let ref bounds_cli=attrs.impls.const_layout_independent.bound_tokens();

    let ref attrs_type_alias=attrs.type_alias.impl_annotations();
    

    let created_module=format!("const_constructor_{}",name).piped(new_ident);

    let type_alias=&attrs.type_alias.inner.decl.unwrap_or_else(||{
        panic!("must pass the 'Type' parameter.\n{}",help_message);
    });
    let type_alias_ident=type_alias.ident();

    let const_constructor=&attrs.const_constructor.inner.decl.unwrap_or_else(||{
        TypeDeclVariant::Name(new_ident(format!("{}CC",type_alias_ident)))
    });
    let const_constructor_ident=const_constructor.ident();

    let const_param_for_alias=new_ident(format!("__ConstParam"));
    // let const_param_for_alias_rep=iter::repeat(const_param_for_alias);
    let (const_param_ident,const_param_default)=
        attrs.const_param.unwrap_or_else(||{
            panic!("must pass the 'ConstParam' parameter.\n{}",help_message);
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

    let field_tys:HashSet<&syn::Type>=ds.variants
        .iter()
        .flat_map(|v| &v.fields )
        .map(|f| f.ty )
        .collect();


    let ref field_tys_mentioning_const={
        let const_finder=FindTypeParam::new(const_param_ident);

        field_tys.iter().cloned()
            .filter(|ty| const_finder.in_(ty) )
            .collect::<Vec<_>>()
    };

    if field_tys_mentioning_const.is_empty() {
        panic!("Const-parameter '{}' is never used", const_param_ident);
    }


    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let where_clause=&where_clause.unwrap().predicates;



    let ext_methods_allowed=attrs.extension_methods.inner.is_allowed;
    let ext_methods_allowed_ty= new_ident(["False","True"][ext_methods_allowed as usize].into());
    
    let ref attrs_allowed_ops=attrs.extension_methods.impl_annotations();
    let ref bounds_allowed_ops=attrs.extension_methods.bound_tokens();

    
    let mut tokens=TokenStream::new();

    if let TypeDeclVariant::Name(_)=*type_alias {
        tokens.append_all(quote!{
            #attrs_type_alias
            #vis type #type_alias_ident< 
                #(#lifetimes,)*
                #(#type_alias_ty_params,)*
                #(#const_params,)*
            >=#name < #type_alias_gen_params >;
        });
    }
    if let TypeDeclVariant::Name(_)=*const_constructor {
        let const_constructor_doc=format!("The ConstConstructor for {}",name);
        tokens.append_all(quote!{
            #attrs_cc_type
            #[doc=#const_constructor_doc]
            #vis struct #const_constructor_ident< #remaining_generics >
            where 
                #bounds_cc_type
            {
                _marker: 
                    ::type_level_values::reexports::VariantPhantom<(
                        #( & #lifetimes () ,)*
                        #(::type_level_values::reexports::VariantPhantom<#truncated_type_params>,)*
                    )>
            }
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
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        mod #created_module { 
            use super::*;
            use type_level_values::reexports::*;
            use type_level_values::user_traits::const_traits::{
                AllowedOps,
                ApplyConstParam_,
                ConstConstructor,
                GetConstConstructor_,
                GetConstParam_,
                ConstLayoutIndependent,
                SameFieldLayout,
            };
            use type_level_values::const_wrapper::{
                WrapperTrait,
                ConstWrapper,
                UnwrapConst,
            };

            #[doc(hidden)]
            pub struct ConstDependentField<T>(T);

            #(#field_accessors)*

            #attrs_allowed_ops
            impl<#remaining_generics> AllowedOps 
            for #const_constructor_ident< #remaining_g_params > 
            where 
                #bounds_allowed_ops
            {
                type ExtensionMethods=type_level_bool::#ext_methods_allowed_ty;
            }

            #attrs_cc_impl
            impl< #remaining_generics > 
                ConstConstructor
            for #const_constructor_ident< #remaining_g_params > 
            where 
                #bounds_cc_impl
                #bounds_cc_type
            {}
            
            #attrs_gcc
            impl < #(#generic_params,)* > GetConstConstructor_ for #name #ty_generics 
            where 
                #(#where_clause,)*
                #bounds_gcc
                #bounds_cc_type
                Self:GetConstParam_,
            {
                type Constructor = #const_constructor_ident< #remaining_g_params >;
            }
            
            #attrs_cli
            unsafe impl < #(#generic_params,)* __Other:?Sized> ConstLayoutIndependent<__Other>
            for #name #ty_generics
            where 
                #(#where_clause,)*
                #bounds_cli
                #(
                    __Other:SameFieldLayout<
                        ConstDependentField<integer_reexports::#field_indices>,
                        Self
                    >,
                )*
            {}

            #attrs_gcp
            impl < #(#generic_params,)* #const_param_for_alias> GetConstParam_  
            for #name #ty_generics
            where 
                #(#where_clause,)*
                #bounds_gcp
                #const_param_ident:TypeIdentity<Type=ConstWrapper<#const_param_for_alias>>
            {
                type Const = #const_param_for_alias;
            }

            #attrs_acp
            impl < #(#generic_params,)* #const_param_for_alias, __Output,> 
                ApplyConstParam_<#const_param_for_alias>
            for #const_constructor_ident< #remaining_g_params > 
            where 
                #(#where_clause,)*
                #bounds_acp
                #bounds_cc_type
                ConstWrapper<#const_param_for_alias>:TypeIdentity<Type=#const_param_ident>,
                #name #ty_generics:TypeIdentity<Type=__Output>,
                __Output:
                    GetConstConstructor_<
                        Const=#const_param_for_alias,
                        Constructor=Self
                    >,
            {
                type Applied = __Output ;
            }

        }
    ));
    tokens.observe(|v|{
        if attrs.print_derive {
            print_derive_tokens(v);
        }
    })
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






