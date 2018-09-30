use syn::visit_mut::VisitMut;
use syn::punctuated::Punctuated;
use syn::token::Colon2;
use syn::{Ident,TypeParamBound,TypePath,PathSegment};

use quote::ToTokens;
use proc_macro2::TokenStream;

use std::mem;
use std::fmt::Write;


#[derive(Debug,Clone)]
pub(crate) struct SelfRemovedBound{
    bound:TypeParamBound,
}

impl SelfRemovedBound{
    pub(crate) fn new<F>(mut bound:TypeParamBound,is_field_name:F)->Self
    where F:FnMut(&Ident)->bool,
    {
        SelfRemover{is_field_name}.visit_type_param_bound_mut(&mut bound);
        Self{ bound }
    }
    pub(crate) fn bound(&self)->&TypeParamBound{
        &self.bound
    }
}

struct SelfRemover<F>{
    is_field_name:F,
}


impl<F> VisitMut for SelfRemover<F>
where F:FnMut(&Ident)->bool,
{
    fn visit_type_path_mut(&mut self, i: &mut TypePath){
        if let Some(qself)=i.qself.as_mut() { 
            self.visit_type_mut(&mut qself.ty);
            return; 
        }
        let segments=&mut i.path.segments;
        if 2 <= segments.len() && 
           segments[0].ident=="Self" &&
           (self.is_field_name)(&segments[1].ident)
        {
            *segments=mem::replace(segments,Default::default())
                .into_iter()
                .skip(1)
                .collect::<Punctuated<PathSegment, Colon2>>();
        }
    }
}


impl ToTokens for SelfRemovedBound{
    fn to_tokens(&self,tokens:&mut TokenStream){
        self.bound().to_tokens(tokens);
    }
}