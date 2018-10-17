use super::*;

use ::attribute_detection::typelevel::{ReExports,ReExportCfg,ImplIndex};

use syn::Visibility;


#[derive(Debug,Clone,Copy)]
pub(crate) struct ReExportPrinter<'a>{
    pub(crate) struct_decls:&'a StructDeclarations<'a>,
    pub(crate) derive_vis:&'a Visibility,
    pub(crate) created_module:&'a Ident,
    pub(crate) cfg:&'a ReExportCfg<'a>,
    pub(crate) c_tokens:&'a CommonTokens,
}


impl<'a> ToTokens for ReExportPrinter<'a>{
    fn to_tokens(&self,tokens:&mut TokenStream){
        let cfg=self.cfg;
        let c_t=self.c_tokens;
        let sd=self.struct_decls;

        let priv_field_vis=sd.priv_field_vis().submodule_level(0);

        let vis=match cfg.visibility {
            ReExportVis::NoReexport=>return,
            ReExportVis::WithDeriveVis=>self.derive_vis,
            ReExportVis::WithVis(reexport_vis)=>reexport_vis,
        };

        let mut pub_top_reexports   =Vec::<&'a Ident>::new();
        let mut priv_top_reexports  =Vec::<&'a Ident>::new();
        // let mut pub_variants_reexports =Vec::<&'a Ident>::new();
        let mut priv_variants_reexports=Vec::<&'a Ident>::new();

        if ReExports::none_reexported()!=cfg.reexported {
            pub_top_reexports.push(sd.type_marker_struct);
        }

        if cfg.reexported.traits{
            if let Some(v)=sd.enum_trait {
                pub_top_reexports.push(v);
            }
            for struct_ in &sd.declarations {
                // pub_top_reexports.push(struct_.from_trait_ident );
                pub_top_reexports.push(struct_.trait_ident );
                
                if struct_.is_item_declared(sd,ImplIndex::IntoRuntime)  {
                    pub_top_reexports.push(struct_.wr_trait_ident );
                }
            }
        }
        if cfg.reexported.variants{
            for struct_ in &sd.declarations {
                priv_top_reexports.push(struct_.name);
                priv_top_reexports.push(struct_.uninitialized_ident);
            }
        }
        if cfg.reexported.fields {
            pub_top_reexports.push(&c_t.fields_mod);
        }

        if cfg.reexported.discriminants {
            for struct_ in &sd.declarations {
                priv_variants_reexports.push(struct_.variant_marker_ident);
                priv_variants_reexports.push(struct_.discriminant_ident);
            }
        }

        fn use_mod_with_vis<V:ToTokens>(
            vis:V,
            mods:&[&Ident],
            idents:Vec<&Ident>,
            c_t:&CommonTokens,
            tokens:&mut TokenStream
        ){
            if idents.is_empty() { return; }
            to_stream!(tokens ; c_t.allow_unused_imports, vis,c_t.use_,c_t.low_self,c_t.colon2, );
            for submod in mods{
                submod.to_tokens(tokens);
                c_t.colon2.to_tokens(tokens);
            }
            c_t.brace.surround(tokens,|tokens|{
                for ident in idents {
                    ident.to_tokens(tokens);
                    c_t.comma.to_tokens(tokens);
                }
            });
            c_t.semicolon.to_tokens(tokens);
        }

        use_mod_with_vis( vis,&[self.created_module],pub_top_reexports,c_t,tokens );
        use_mod_with_vis( priv_field_vis,&[self.created_module],priv_top_reexports,c_t,tokens );

        let field_mod_tokens=&[self.created_module,&c_t.variants_mod][..];
        // use_mod_with_vis( vis           ,field_mod_tokens,pub_variants_reexports,c_t,tokens );
        use_mod_with_vis( priv_field_vis,field_mod_tokens,priv_variants_reexports,c_t,tokens );
    }
}