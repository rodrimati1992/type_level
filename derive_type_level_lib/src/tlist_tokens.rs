use quote::{ToTokens,TokenStreamExt};
use proc_macro2::{TokenStream};
use syn::token::Gt;


use std::iter;
use std::cell::Cell;


pub struct TListFrom<I>{
    iter:Cell<Option<I>>,
}

impl<I> TListFrom<I>{
    pub fn new(iter:I)->Self{
        Self{iter:Cell::new(Some(iter)) }
    }
}


impl<I> ToTokens for TListFrom<I>
where 
    I:Iterator+ExactSizeIterator,
    I::Item:ToTokens,
{
    fn to_tokens(&self,tokens:&mut TokenStream){
        let iter=self.iter.take().expect("TListFrom::to_tokens must be called only once");
        let gt=Gt::default();
        let tlist_end=iter::repeat(&gt).take(iter.len());
        tokens.append_all(quote!{
            #(TList<#iter,)* TNil #(#tlist_end)*
        });
    }
}