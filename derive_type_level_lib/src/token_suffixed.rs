#![allow(dead_code)]

use quote::ToTokens;
use proc_macro2::TokenStream;
// use syn::token::Gt;

// use std::iter;
use std::cell::Cell;


/// ToTokens implementor which outputs every element of the iterator suffixed 
/// by the `suffixed` token.
pub struct TokenSuffixed<'a,I,Token:'a>{
    iter:Cell<Option<I>>,
    suffixed:&'a Token,
}

impl<'a,I,Token:'a> TokenSuffixed<'a,I,Token>
{
    pub fn new(iter:I,suffixed:&'a Token)->Self{
        Self{
            iter:Cell::new(Some(iter)),
            suffixed,
        }
    }
}


impl<'a,I,Token:'a> ToTokens for TokenSuffixed<'a,I,Token>
where 
    I:Iterator+ExactSizeIterator,
    I::Item:ToTokens,
    Token:ToTokens,
{
    fn to_tokens(&self,tokens:&mut TokenStream){
        let iter=self.iter.take().expect("TokenSuffixed::to_tokens must be called only once");
        for elem in iter {
            elem.to_tokens(tokens);
            self.suffixed.to_tokens(tokens)
        }
    }
}