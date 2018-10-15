use super::*;
use super::struct_declarations::RelativePriv;
use self_removed_bound::SelfRemovedBound;


#[allow(unused_imports)]
use core_extensions::{
    BoolExt,
    OptionExt,
};
#[allow(unused_imports)]
use core_extensions::Void;

use attribute_detection::typelevel::ImplVariantMethods;
#[allow(unused_imports)]
use ::void_like::VoidLike;

// use token_suffixed::TokenSuffixed;

use syn::token;




pub(crate)struct CompiletimeTraits<'a>{
    pub(crate)decls:&'a StructDeclarations<'a>,
}


impl<'a> CompiletimeTraits<'a>{
    pub(crate)fn new(decls:&'a StructDeclarations<'a>)->Self{
        Self{decls}
    }
}

impl<'a> ToTokens for CompiletimeTraits<'a>{
    fn to_tokens(&self,tokens:&mut TokenStream){
        let span=self.decls.original_name.span();

        let token=&self.decls.tokens;
        let original_generics=&self.decls.original_generics.params;
        let original_generics_b=&self.decls.original_generics.params;
        let original_generics_c=&self.decls.original_generics.params;
        let original_gen_params=&self.decls.original_gen_params;
        let original_type=&self.decls.type_;
        let original_path=&self.decls.original_path;
        let type_marker_struct=&self.decls.type_marker_struct;
        // let type_marker_struct_rep=iter::repeat(type_marker_struct);
        // let all_types=&self.decls.all_types;
        let where_preds=&self.decls.original_where_preds;

        let priv_suffix=self.decls.priv_param_suffix();
        
        let vis=self.decls.vis_kind.submodule_level(1);;

        let derived=&self.decls.attribute_settings.derived;

        let enum_trait=&self.decls.enum_trait;

        let supertrait=match self.decls.enum_or_struct {
            EnumOrStruct::Enum  =>
                quote!{ #enum_trait },
            EnumOrStruct::Struct=>
                quote!{ DerivedTraits<Type=#type_marker_struct> },
        };

        {
            let self_ty=ToTokenFnMut::new(|tstream|{
                match derived.into_consttype.inner.to_specified() {
                    ImplVariant::Unspecified(_)|ImplVariant::NoImpls=>{}
                    ImplVariant::DefaultImpls=>
                        original_type.to_tokens(tstream),
                    ImplVariant::Internal{type_,..}=>
                        to_stream!( tstream; type_,token.lt, original_gen_params ,token.gt ),
                }
            });
            let derived=derived.into_consttype.inner.to_specified();
            let tmp=match derived {
                ImplVariant::Unspecified(_)=>
                    false,
                ImplVariant::NoImpls=>
                    false,
                ImplVariant::DefaultImpls|ImplVariant::Internal{..}=>
                    true,
            };
            if tmp && derived.is_derived() {
                annotations_and_bounds!(outer;
                    self.decls,ImplIndex::IntoConstType,let (from_runtime_attrs,from_runtime_bounds)
                );
                let field_ty=self.decls.all_types.iter().map(|x| x.field_ty );
                let mod_ty  =self.decls.all_types.iter().map(|x| x.mod_ty );
                tokens.append_all(quote!{
                    #from_runtime_attrs
                    impl<#original_gen_params> IntoConstType_ for #self_ty
                    where 
                        #where_preds
                        #( #mod_ty:IntoConstType_<#field_ty>,)*
                        #from_runtime_bounds
                    {
                        type ToConst=#type_marker_struct;
                    }
                });
            }
        }


        for struct_ in &self.decls.declarations {

            let original_variant_name=struct_.variant.name;
            let struct_name=&struct_.name;
            // let struct_name_rep=iter::repeat(&struct_.name);
            let trait_ident=&struct_.trait_ident;
            // let trait_ident=struct_.fields.is_empty().if_false(||&struct_.trait_ident);
            let trait_ident_rep=iter::repeat(&struct_.trait_ident);
            let from_trait_ident=&struct_.from_trait_ident;
            let generics_fn=||struct_.fields.iter().map(|x|x.generic);


            let enum_path=self.decls.enum_path;
            // let original_where=self.decls.original_generics.where_clause.as_ref()
                // .map(|x| &x.predicates );


            // let generics_a=generics_fn();
            // let generics_b=generics_fn();
            let generics_c=generics_fn();
            let generics_d=generics_fn();
            let generics_e=generics_fn();
            let generics_e_0=generics_fn();
            let generics_f=generics_fn();
            // let generics_g=generics_fn();
            let generics_j0=generics_fn();
            let generics_j1=generics_fn();
            // let generics_k=generics_fn();

            let generics  =&struct_.generics;
            // let generics_rep=iter::repeat(generics);
            // let field_names_b=struct_.fields.iter().map(|x|&x.name_ident);

            let field_mod_c=struct_.fields.iter().map(|x|{
                match x.relative_priv {
                    RelativePriv::Inherited=>&token.fields_mod ,
                    RelativePriv::MorePrivate=>&token.dund_fields_mod ,
                }
            });
            let field_names_c=struct_.fields.iter().map(|x|&x.accessor_ident);

            let field_name_vis_fn=||{
                struct_.fields.iter().map(|f|f.doc_hidden_attr())
            };

            let field_name_vis_a=field_name_vis_fn();
            let field_name_vis_b=field_name_vis_fn();
            let field_name_vis_d=field_name_vis_fn();
            let field_name_vis_e=field_name_vis_fn();

            let field_docs_a=struct_.fields.iter().map(|x|&x.docs);
            let field_docs_b=struct_.fields.iter().map(|x|&x.docs);

            let field_names_d=struct_.fields.iter().map(|x|&x.assoc_type);
            let field_names_e=struct_.fields.iter().map(|x|&x.assoc_type);
            let field_names_f=struct_.fields.iter().map(|x|&x.assoc_type);

            let field_names_z=struct_.fields.iter().map(|x|&x.original_name);

            let original_types_a=struct_.fields.iter().map(|x|&x.original_ty);
            let original_types_b=struct_.fields.iter().map(|x|&x.original_ty);

            // let original_types_e=struct_.fields.iter().map(|x|&x.original_ty);
            // let original_types_f=struct_.fields.iter().map(|x|&x.original_ty);

            let specified_ir_iv=derived.into_runtime.inner.to_specified();


            let ir_type_ident=match specified_ir_iv {
                ImplVariant::Unspecified(_v)=>{
                    // let _:VoidLike=v;
                    panic!("WTF:file:{} line:{}",file!(),line!())
                },
                ImplVariant::NoImpls       =>None,
                ImplVariant::DefaultImpls  =>Some(original_path),
                ImplVariant::Internal{type_,..}=>Some(type_),
            };

            let constructor=ToTokenFnMut::new(|tstream|{
                match ( specified_ir_iv , self.decls.enum_or_struct ){
                    (ImplVariant::Unspecified(_),_)|(ImplVariant::NoImpls,_)=>{}
                    (ImplVariant::DefaultImpls,_)=>{
                        enum_path.to_tokens(tstream);
                        original_variant_name.to_tokens(tstream);
                    }
                    (ImplVariant::Internal{type_,..},EnumOrStruct::Enum)
                    =>{
                        type_.to_tokens(tstream);
                        token::Colon2::default().to_tokens(tstream);
                        original_variant_name.to_tokens(tstream);
                    }
                    (ImplVariant::Internal{type_,..},EnumOrStruct::Struct)
                    =>{
                        type_.to_tokens(tstream);
                    }
                }
            });

            if let Some(ir_type_ident)=ir_type_ident.filter_(|_| specified_ir_iv.is_derived() )  {
                annotations_and_bounds!(outer;
                    self.decls,ImplIndex::IntoRuntime,let (variant_attrs,variant_bounds)
                );

                tokens.append_all(quote!{
                    #variant_attrs
                    impl<#(#original_generics,)* #generics > 
                        IntoRuntime<#ir_type_ident< #(#original_generics_b,)* >>
                    for #struct_name<#generics #priv_suffix>
                    where 
                        #where_preds
                        #( #generics_e_0:IntoRuntime<#original_types_a> ,)*
                        #variant_bounds
                    {
                        fn to_runtime()->#ir_type_ident < #(#original_generics_c,)* > {
                            #constructor {
                                #(
                                    #field_names_z:#generics_e::to_runtime(),
                                )*
                            }
                        }
                    }
                });

                #[cfg(rust_1_22)]
                {
                    let original_generics_d=&self.decls.original_generics.params;
                    let generics_h=generics_fn();
                    let field_names_y=struct_.fields.iter().map(|x|&x.original_name);
                    let original_types_d=struct_.fields.iter().map(|x|&x.original_ty);
                    
                    tokens.append_all(quote!{
                        #variant_attrs
                        impl<#(#original_generics,)* #generics > 
                            IntoConstant<#ir_type_ident< #(#original_generics_d,)* >>
                        for #struct_name<#generics #priv_suffix>
                        where 
                            #where_preds
                            #( #generics_j0:IntoConstant<#original_types_d,#generics_h> ,)*
                            #variant_bounds
                        {
                            const VALUE: #ir_type_ident < #(#original_generics_c,)* > =
                                #constructor {
                                    #(
                                        #field_names_y:#generics_j1::VALUE,
                                    )*
                                };
                        }

                    });
                }
            }

            
            let discriminant_ident=&struct_.discriminant_ident;

            #[derive(Copy,Clone,PartialEq)]
            enum ConstOrRuntime{
                Const,
                Runtime,
            }

            let c_or_r_bound_wclause=|c_or_r|{
                ToTokenFnMut::new(move|t_s|{
                    for field in &struct_.fields {
                        let bounds=match c_or_r{
                            ConstOrRuntime::Const  => &field.const_bound,
                            ConstOrRuntime::Runtime=> &field.runt_bound,
                        };
                        if bounds.empty_or_trailing() {
                            continue;
                        }
                        field.generic.to_tokens(t_s);
                        token.colon.to_tokens(t_s);
                        match c_or_r{
                            ConstOrRuntime::Const  => for bound in bounds{
                                let removed_self=SelfRemovedBound::new(
                                    bound.clone(),
                                    |i:&Ident| self.decls.field_accessors.contains_key(i) 
                                );
                                removed_self.to_tokens(t_s);
                                token.add.to_tokens(t_s);
                            },
                            ConstOrRuntime::Runtime=> 
                                bounds.to_tokens(t_s),
                        }
                        token.comma.to_tokens(t_s);
                    }
                })
            };

            
            let original_generics_ir_a=&self.decls.original_generics.params;
            let original_generics_ir_b=&self.decls.original_generics.params;

            let field_names_rt_a=struct_.fields.iter().map(|x|&x.rt_assoc_type);
            let field_names_rt_b=struct_.fields.iter().map(|x|&x.rt_assoc_type);
            // let field_names_rt_c=struct_.fields.iter().map(|x|&x.rt_assoc_type);
            // let field_names_rt_d=struct_.fields.iter().map(|x|&x.rt_assoc_type);

            let const_bounds_wclause=c_or_r_bound_wclause(ConstOrRuntime::Const);
            let const_bounds_b=struct_.fields.iter().map(|f| &f.const_bound );
            let const_bounds_opt_colon=struct_.fields.iter()
                .map(|f| f.const_bound.is_empty().if_false(|| token.colon ) );
            // let const_bounds_c=struct_.fields.iter().map(|f| &f.const_bound );

            let runt_bounds_wclause=c_or_r_bound_wclause(ConstOrRuntime::Runtime);
            let runt_bounds_b=struct_.fields.iter().map(|f| &f.runt_bound );
            let runt_bounds_opt_colon=struct_.fields.iter()
                .map(|f| f.runt_bound.is_empty().if_false(|| token.colon ) );
            // let runt_bounds_c=struct_.fields.iter().map(|f| &f.runt_bound );

            let field_accessor_a=struct_.fields.iter().map(|f| &f.accessor_ident );
            let field_accessor_b=struct_.fields.iter().map(|f| &f.accessor_ident );

            let uninitialized_value=&struct_.uninitialized_ident;

            let wr_trait_ident=&struct_.wr_trait_ident;
            let variant_marker_ident=&struct_.variant_marker_ident;
            let type_trait_docs=&struct_.type_trait_docs;

            tokens.append_all(quote!{

                impl<#generics> ConstTypeOf_ for #struct_name<#generics #priv_suffix>{
                    type Type=#type_marker_struct;
                }

                #[doc=#type_trait_docs]
                #vis trait #trait_ident:
                    Sealed+
                    #supertrait+
                    GetDiscriminant<Discriminant=variants::#discriminant_ident>+
                    #(GetField_<#field_mod_c::#field_names_c> + )*
                {
                    #(
                        #field_name_vis_a
                        #(#[doc=#field_docs_a])*
                        type #field_names_d
                            #const_bounds_opt_colon
                            #const_bounds_b ;
                    )*
                }
                impl<#generics> #trait_ident for #struct_name<#generics #priv_suffix>
                where 
                    #const_bounds_wclause
                {
                    #( 
                        #field_name_vis_b
                        type #field_names_e=#generics_c ;
                    )*
                }

                #vis type #from_trait_ident<This>=
                    IgnoreFirst<This,
                        #struct_name<
                            #(
                                <This as #trait_ident_rep>::#field_names_f ,
                            )*
                            #priv_suffix
                        >
                    >;

                impl<#generics> Sealed for #struct_name<#generics #priv_suffix> {}
                
                
                impl __initialization::InitializationValues for variants::#variant_marker_ident{
                    type Uninitialized=
                        <#uninitialized_value as 
                            __initialization::InitializationValues
                        >::Uninitialized;
                    type Initialized=
                        <#uninitialized_value as 
                            __initialization::InitializationValues
                        >::Initialized;
                }

                impl<#generics> __initialization::InitializationValues 
                for #struct_name<#generics #priv_suffix> 
                {
                    type Uninitialized=#struct_name<
                        #( __initialization::UninitField<
                                fields::#field_accessor_a 
                            >, 
                        )*
                        #priv_suffix
                    >;
                   
                    type Initialized=#struct_name<
                        #( __initialization::IsInitField< 
                                fields::#field_accessor_b 
                            >, 
                        )*
                        #priv_suffix
                    >;
                }
                
            });


            if self.decls.enum_or_struct==EnumOrStruct::Struct &&
                self.decls.relative_field_priv()==RelativePriv::Inherited
            {
                tokens.append_all(quote!{
                    impl __initialization::InitializationValues 
                    for #type_marker_struct
                    {
                        type Uninitialized=
                            <#uninitialized_value as 
                                __initialization::InitializationValues
                            >::Uninitialized;
                        type Initialized=
                            <#uninitialized_value as 
                                __initialization::InitializationValues
                            >::Initialized;
                    }                
                });
            }

            if let Some(_)=ir_type_ident {
                let with_runtime_docs=&struct_.with_runtime_docs;

                tokens.append_all(quote!{
                    #[doc=#with_runtime_docs]
                    #vis trait #wr_trait_ident< #(#original_generics_ir_a,)* >:#trait_ident{
                        #(
                            #field_name_vis_d
                            #(#[doc=#field_docs_b])*
                            type #field_names_rt_a
                                #runt_bounds_opt_colon
                                #runt_bounds_b ;
                        )*
                    }


                    impl<#(#original_generics,)* #generics> 
                        #wr_trait_ident< #(#original_generics_ir_b,)* >
                    for #struct_name<#generics #priv_suffix>
                    where
                        Self:#trait_ident,
                        #runt_bounds_wclause
                    {
                        #(
                            #field_name_vis_e
                            type #field_names_rt_b=#generics_f;
                        )*
                    }
                });
            }


        }
    }
}

