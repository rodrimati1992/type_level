use super::*;
// use super::EnumOrStruct as EOS;
use tlist_tokens::TListFrom;


pub(crate)struct DerivedTraits<'a>{
    pub(crate)decls:&'a StructDeclarations<'a>,
}


impl<'a> DerivedTraits<'a>{
    pub(crate)fn new(decls:&'a StructDeclarations<'a>)->Self{
        Self{decls}
    }
}

impl<'a> ToTokens for DerivedTraits<'a>{
    fn to_tokens(&self,tokens:&mut TokenStream){
        let decls=self.decls;

        

        let d_attr_cfg=decls.attribute_settings;


        let priv_suffix=self.decls.priv_param_suffix();

        // let type_marker_struct=&self.decls.type_marker_struct;
        for struct_ in &self.decls.declarations {
            let struct_name=&struct_.name;
            
            

            let generics_1=&struct_.generics;
            
            let generics_1b=struct_.fields.iter().map(|x|&x.generic);

            tokens.append_all(quote!{
                impl<#generics_1> Copy for #struct_name<#generics_1 #priv_suffix>{}
                impl<#generics_1> Clone for #struct_name<#generics_1 #priv_suffix>{
                    #[inline(always)]
                    fn clone(&self)->Self{ *self }
                }
                unsafe impl<#generics_1> MarkerType for #struct_name<#generics_1 #priv_suffix>{}

                impl<#generics_1> Default for #struct_name<#generics_1 #priv_suffix>{
                    fn default()->Self{
                        MarkerType::MTVAL
                    }
                }

                
            });

            let s_attr_cfg=&struct_.attribute_settings;

            {
                let tlist=TListFrom::new(generics_1b);

                annotations_and_bounds!(outer;
                    self.decls,ImplIndex::AsTList,let (impl_attrs,impl_bounds)
                );

                if 
                    d_attr_cfg.derived.as_t_list.inner.is_implemented()||
                    s_attr_cfg.derived.as_t_list.inner
                        .specified_or(ImplVariant::NoImpls)
                        .is_implemented()
                {
                    tokens.append_all(quote!{
                        #impl_attrs
                        impl<#generics_1> AsTList_ for #struct_name<#generics_1 #priv_suffix>
                        where 
                            #impl_bounds
                        {
                            type Output=#tlist;
                        }
                    });
                }
            }


            {
                annotations_and_bounds!(outer;
                    self.decls,ImplIndex::ConstEq,let (impl_attrs,impl_bounds)
                );

                if 
                    d_attr_cfg.derived.const_eq.inner.is_implemented()||
                    s_attr_cfg.derived.const_eq.inner.is_implemented()
                {
                    if struct_.fields.is_empty() {
                        tokens.append_all(quote!(
                            #impl_attrs
                            impl<#generics_1 __Other,DiscrL,DiscrR> ConstEq_<__Other>
                            for #struct_name< #generics_1 #priv_suffix>
                            where 
                                #impl_bounds
                                Self   :GetDiscriminant<Discriminant=DiscrL>,
                                __Other:GetDiscriminant<Discriminant=DiscrR>,
                                DiscrL:ConstEq_<DiscrR>,
                            {
                                type Output=<DiscrL as ConstEq_<DiscrR>>::Output;
                            }
                        ));
                    }else{
                        tokens.append_all(quote!(
                            #impl_attrs
                            impl<#generics_1 __Other> ConstEq_<__Other>
                            for #struct_name< #generics_1 #priv_suffix>
                            where 
                                #impl_bounds
                                Self  : VariantAsTList_,
                                __Other: VariantAsTList_,
                                VariantAsTList<Self>: ConstEq_<VariantAsTList<__Other>>,
                            {
                                type Output=__CEq<VariantAsTList<Self>,VariantAsTList<__Other>>;
                            }
                        ));
                    }
                }
            }

            {
                annotations_and_bounds!(outer;
                    self.decls,ImplIndex::ConstOrd,let (impl_attrs,impl_bounds)
                );
                
                if d_attr_cfg.derived.const_ord.inner.is_implemented()||
                    s_attr_cfg.derived.const_ord.inner.is_implemented()
                {
                    if struct_.fields.is_empty() {
                        tokens.append_all(quote!(
                            #impl_attrs
                            impl<#generics_1 __Other,DiscrL,DiscrR> ConstOrd_<__Other>
                            for #struct_name< #generics_1 #priv_suffix>
                            where 
                                #impl_bounds
                                Self   :GetDiscriminant<Discriminant=DiscrL>,
                                __Other:GetDiscriminant<Discriminant=DiscrR>,
                                DiscrL:ConstOrd_<DiscrR>,
                            {
                                type Output=<DiscrL as ConstOrd_<DiscrR>>::Output;
                            }
                        ));
                    }else{
                        tokens.append_all(quote!(
                            #impl_attrs
                            impl<#generics_1 __Other> ConstOrd_<__Other>
                            for #struct_name< #generics_1 #priv_suffix>
                            where 
                                #impl_bounds
                                Self  : VariantAsTList_,
                                __Other: VariantAsTList_,
                                VariantAsTList<Self>: ConstOrd_<VariantAsTList<__Other>>,
                            {
                                type Output=__COrd<VariantAsTList<Self>,VariantAsTList<__Other>>;
                            }
                        ));
                    }
                }
            }

        }
    }
}
