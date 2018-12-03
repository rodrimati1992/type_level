use syn::visit_mut::VisitMut;
// use syn::punctuated::Punctuated;
// use syn::token::Colon2;
use syn::{Ident, TypeParamBound, TypePath};

use proc_macro2::TokenStream;
use quote::ToTokens;

use std::mem;

#[derive(Debug, Clone)]
pub(crate) struct SelfRemovedBound {
    bound: TypeParamBound,
}

impl SelfRemovedBound {
    pub(crate) fn new<F>(mut bound: TypeParamBound, is_field_name: F) -> Self
    where
        F: FnMut(&Ident) -> Option<Ident>,
    {
        SelfRemover { is_field_name }.visit_type_param_bound_mut(&mut bound);
        Self { bound }
    }
    pub(crate) fn bound(&self) -> &TypeParamBound {
        &self.bound
    }
}

struct SelfRemover<F> {
    is_field_name: F,
}

impl<F> VisitMut for SelfRemover<F>
where
    F: FnMut(&Ident) -> Option<Ident>,
{
    fn visit_type_path_mut(&mut self, i: &mut TypePath) {
        if let Some(qself) = i.qself.as_mut() {
            self.visit_type_mut(&mut qself.ty);
            return;
        }
        let segments = &mut i.path.segments;
        if segments.len() < 2 || segments[0].ident != "Self" {
            return;
        }
        if let Some(field_ident) = (self.is_field_name)(&segments[1].ident) {
            let prev_segments = mem::replace(segments, Default::default());
            segments.push(field_ident.into());
            segments.extend(prev_segments.into_iter().skip(2));
        }
    }
}

impl ToTokens for SelfRemovedBound {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.bound().to_tokens(tokens);
    }
}
