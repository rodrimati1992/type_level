use syn::{self, NestedMeta, WherePredicate};

use quote::{ToTokens, TokenStreamExt};

use proc_macro2::{Span, TokenStream};

use syn::token::Comma;

use super::{MyMeta, MyNested};
use super::shared::{NotUpdated, UpdateWithMeta};

use ArenasRef;

#[derive(Debug, Clone, Default)]
pub(crate) struct ItemMetaData<'alloc,I> {
    pub(crate) inner: I,
    pub(crate) bounds: Vec<WherePredicate>,
    pub(crate) attrs: Vec<&'alloc NestedMeta>,
    pub(crate) docs: Vec<&'alloc str>,
}

impl<'alloc,I> ItemMetaData<'alloc,I> {
    pub(crate) fn new(inner: I) -> Self {
        Self {
            inner,
            attrs: Default::default(),
            bounds: Default::default(),
            docs: Default::default(),
        }
    }

    // pub(crate) fn map<F,U>(self,f:F)->ItemMetaData<'alloc,U>
    // where F:FnOnce(I)->U
    // {
    //     ItemMetaData{
    //         inner:f(self.inner),
    //         bounds:self.bounds,
    //         attrs:self.attrs,
    //         docs:self.docs,
    //     }
    // }

    pub(crate) fn impl_annotations(&self) -> AnnotationTokens {
        AnnotationTokens {
            attrs: &self.attrs,
            docs: &self.docs,
        }
    }

    pub(crate) fn chain_impl_annotations<'a>(
        &'a self,
        other: &'a Self,
    ) -> AnnotationTokensChain<'a> {
        AnnotationTokensChain {
            attrs_outer: &self.attrs,
            attrs_inner: &other.attrs,
            docs_inner: &self.docs,
            docs_outer: &other.docs,
        }
    }

    pub(crate) fn chain_impl_attrs<'a>(
        &'a self,
        other: &'a Self,
    ) -> AnnotationTokensChain<'a> {
        AnnotationTokensChain {
            attrs_outer: &self.attrs,
            attrs_inner: &other.attrs,
            docs_inner: &[],
            docs_outer: &[],
        }
    }

    pub(crate) fn bound_tokens(&self) -> BoundTokens {
        BoundTokens {
            bounds: &self.bounds,
        }
    }

    pub(crate) fn chain_bound_tokens<'a>(&'a self, other: &'a Self) -> BoundTokensChain<'a> {
        BoundTokensChain {
            bounds_outer: &self.bounds,
            bounds_inner: &other.bounds,
        }
    }
}

impl<'ar, I> UpdateWithMeta<'ar> for ItemMetaData<'ar,I>
where
    I: UpdateWithMeta<'ar>,
{
    fn update_with_meta(
        &mut self, meta: &MyMeta<'ar>, arenas: ArenasRef<'ar>
    ) -> Result<(), NotUpdated> {
        match (&*meta.word.str, &meta.value) {
            ("bound", &MyNested::Value(ref str_)) => 
                self.bounds.push(syn::parse_str(str_).unwrap()),
            ("doc", &MyNested::Value(str_)) => 
                self.docs.push(str_),
            ("attr", &MyNested::List(list)) => {
                for elem in list {
                    self.attrs.push(elem);
                }
            }
            (_, _value) => self.inner.update_with_meta(meta, arenas)?,
        }
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) struct AnnotationTokens<'a> {
    attrs: &'a [&'a NestedMeta],
    docs: &'a [&'a str],
}

impl<'a> ToTokens for AnnotationTokens<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let AnnotationTokens { attrs, docs } = *self;
        tokens.append_all(quote!{
            #( #[#attrs]    )*
            #( #[doc=#docs] )*
        });
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) struct AnnotationTokensChain<'a> {
    attrs_outer: &'a [&'a NestedMeta],
    attrs_inner: &'a [&'a NestedMeta],
    docs_inner: &'a [&'a str],
    docs_outer: &'a [&'a str],
}

impl<'a> ToTokens for AnnotationTokensChain<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let AnnotationTokensChain {
            attrs_outer,
            attrs_inner,
            docs_outer,
            docs_inner,
        } = *self;
        tokens.append_all(quote!{
            #( #[#attrs_outer]    )*
            #( #[#attrs_inner]    )*
            #( #[doc=#docs_outer] )*
            #( #[doc=#docs_inner] )*
        });
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub(crate) struct BoundTokens<'a> {
    bounds: &'a [WherePredicate],
}

impl<'a> ToTokens for BoundTokens<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let comma = Comma::new(Span::call_site());
        for bound in self.bounds {
            bound.to_tokens(tokens);
            comma.to_tokens(tokens);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub(crate) struct BoundTokensChain<'a> {
    bounds_outer: &'a [WherePredicate],
    bounds_inner: &'a [WherePredicate],
}

impl<'a> ToTokens for BoundTokensChain<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let comma = Comma::new(Span::call_site());
        for bound in self.bounds_outer {
            bound.to_tokens(tokens);
            comma.to_tokens(tokens);
        }
        for bound in self.bounds_inner {
            bound.to_tokens(tokens);
            comma.to_tokens(tokens);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
