use super::*;

use attribute_detection::shared::{
    parse_type,
};

pub(crate)struct FieldTraits<'a>{
    pub(crate)decls:&'a StructDeclarations<'a>,
    // The identifier "NewValue"
    value_ident:syn::Type,
}


impl<'a> FieldTraits<'a>{
    pub(crate)fn new(decls:&'a StructDeclarations<'a>)->Self{
        Self{
            decls,
            value_ident:parse_type("NewValue"),
        }

    }
}


impl<'a> ToTokens for FieldTraits<'a>{
    fn to_tokens(&self,tokens:&mut TokenStream){
        let value_ident=&self.value_ident;
        let original_generics=&self.decls.original_generics.params;
        // let original_name=&self.decls.original_name;
        let where_preds=&self.decls.original_where_preds;

        let priv_suffix=self.decls.priv_param_suffix();
        
        for struct_ in &self.decls.declarations {
            let struct_name=&struct_.name;
            let generics_fn=||struct_.fields.iter().map(|x|x.generic);
            let generics=&struct_.generics;

            let derived=&self.decls.attribute_settings.derived;
            let specified=derived.into_consttype.inner.to_specified();
            let into_=match specified {
                ImplVariant::Unspecified(_)=>unreachable!("because of Void"),
                ImplVariant::NoImpls|ImplVariant::DefaultImpls=>&self.decls.original_path,
                ImplVariant::Internal{type_,..}=>type_,
                ImplVariant::Remote{type_,..} =>type_,
            };
            
            for (field_i,field) in struct_.fields.iter().enumerate(){
                let field_name_struct =&field.accessor_ident;
                let doc_hidden_attr=field.doc_hidden_attr();
                let original_ty=&field.original_ty;
                let generic    =&field.generic;
                
                let generics_set=ReplaceNth::new(generics_fn(),field_i,value_ident);
                // let generics_0=generics_fn();
                
                tokens.append_all(quote!{

                    #doc_hidden_attr
                    impl<#generics #value_ident> 
                        SetField_<self::fields::#field_name_struct,#value_ident>
                    for #struct_name<#generics #priv_suffix>
                    {
                        type Output=#struct_name<#(#generics_set,)* #priv_suffix>;
                    }

                    #doc_hidden_attr
                    impl<#generics> GetField_<self::fields::#field_name_struct>
                    for #struct_name<#generics #priv_suffix>
                    {
                        type Output=#generic;
                    }

                    #doc_hidden_attr
                    impl<#(#original_generics,)* #generics > 
                        GetFieldRuntime_<
                            self::fields::#field_name_struct,
                            #into_<#(#original_generics,)*>
                        >
                    for #struct_name<#generics #priv_suffix>
                    where #where_preds
                    {
                        type Runtime= #original_ty;
                    }

                    
                });


            } 

            let values=iter::repeat(value_ident).take(struct_.fields.len());

            tokens.append_all(quote!{

                impl<#generics Field,FieldVal> std_::ops::Index<Field> 
                for #struct_name<#generics #priv_suffix>
                where 
                    Self:GetField_<Field,Output=FieldVal>,
                {
                    type Output=PhantomWrapper<FieldVal>;

                    #[inline(always)]
                    fn index(&self,_:Field)->&Self::Output{
                        PhantomWrapper::markertype_ref()
                    }
                }

                
                impl<#generics #value_ident> 
                    SetField_<self::fields::All,#value_ident>
                for #struct_name<#generics #priv_suffix>
                {
                    type Output=#struct_name<
                        #(#values,)*
                        #priv_suffix
                    >;
                }
            })
        }
    }
}


