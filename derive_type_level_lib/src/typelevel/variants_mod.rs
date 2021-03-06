use super::*;

use parse_syn::parse_ident;

pub(crate) struct VariantsMod<'a> {
    pub(crate) decls: &'a StructDeclarations<'a>,
}

impl<'a> VariantsMod<'a> {
    pub(crate) fn new(decls: &'a StructDeclarations<'a>) -> Self {
        Self { decls }
    }
}

impl<'a> ToTokens for VariantsMod<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let decls = self.decls;

        // let type_marker_struct=&decls.type_marker_struct;
        let const_type = iter::repeat(&decls.type_marker_struct);
        let discriminant_idents_b = decls.declarations.iter().map(|x| &x.name);

        let discriminant_idents_d = decls.declarations.iter().map(|x| x.variant_marker_ident);
        let discriminant_idents_e = decls.declarations.iter().map(|x| x.variant_marker_ident);
        let discriminant_idents_f = decls.declarations.iter().map(|x| x.variant_marker_ident);

        let priv_suffix = iter::repeat(decls.priv_param_suffix());

        let vis_submod2 = decls.vis_kind.submodule_level(2);
        let vis_submod2_rep_a = iter::repeat(vis_submod2);
        let vis_submod2_rep_b = iter::repeat(vis_submod2);

        let variant_generics_fn = || {
            decls
                .declarations
                .iter()
                .map(|x| x.fields.iter().map(|x| &x.generic))
        };
        let variant_generics_0 = variant_generics_fn();
        let variant_generics_1 = variant_generics_fn();
        let discriminant_name_0 = decls.declarations.iter().map(|x| &x.discriminant_ident);
        let discriminant_name_1 = decls.declarations.iter().map(|x| &x.discriminant_ident);

        let impl_ = &decls.attribute_settings.derived.get_discriminant;
        if impl_.inner.is_implemented() {
            let variant_indices_a = &(0..decls.declarations.len())
                .map(|index| parse_ident(&format!("U{}", index)))
                .collect::<Vec<::syn::Ident>>();

            let variant_indices_b = variant_indices_a;

            annotations_and_bounds!(inner;
                decls,ImplIndex::GetDiscriminant,let (annotations,bounds)
            );

            tokens.append_all(quote!(
                use self::variants::*;
                /**
                Contains discriminants/marker-types for each variant
                (Structs are implicitly enums with 1 variant).
                */
                pub mod variants{
                    use super::*;

                    #(
                        #vis_submod2_rep_b struct #discriminant_idents_e;

                        #vis_submod2_rep_a type #discriminant_name_0=
                            Discriminant<
                                #discriminant_idents_d,
                                #const_type,
                                type_level_values::prelude::#variant_indices_a
                            >;


                        #annotations
                        impl<#(#variant_generics_0),*>
                            GetDiscriminant
                        for #discriminant_idents_b<#(#variant_generics_1,)* #priv_suffix>
                        where #bounds
                        {
                            type Discriminant=#discriminant_name_1;
                            type UIntDiscr=type_level_values::prelude::#variant_indices_b;
                            type Variant=#discriminant_idents_f;
                        }
                    )*
                }
            ));
        }
    }
}
