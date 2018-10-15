use syn;
use syn::WhereClause;

use core_extensions::prelude::*;

use std::borrow::Cow;
use std::fmt;

use utils::{    
    display_totokens_list,
    display_totokens,
};

#[derive(Clone,Hash,Eq,PartialEq)]
pub(crate) struct CheckImpl<'a>{
    pub(crate) attrs  :Cow<'a,[syn::Attribute]>,
    pub(crate) trait_ :Cow<'a,syn::Path>,
    pub(crate) self_ty:Cow<'a,syn::Type>,
    pub(crate) where_clause:Cow<'a,WhereClause>,
}

impl<'a> CheckImpl<'a>{
    /**
    The input must be a string parsable as an empty impl block without 
    declaring generics explicitly .
    
    # Example 

    ```
    impl IntoRuntime<Rectangle> for ConstRectangle<X,Y,W,H> 
    ẁhere
        X:IntoRuntime<u32>,
        Y:IntoRuntime<u32>,
        W:IntoRuntime<u32>,
        H:IntoRuntime<u32>,
    {} 
    ```

    ```
    /// Some docs
    impl IntoConstType_ for Rectangle {} 
    ```


    */
    pub fn new(impl_block:&str)->Self{
        let impl_:syn::ItemImpl=syn::parse_str(impl_block).unwrap_or_else(|e|{
            panic!("could not parse as impl:\n\n{}\n\nerror:{:#?}",impl_block,e)
        });
        impl_.into()
    }

    // pub fn from_ref(impl_:&'a syn::ItemImpl)->Self{
    //     CheckImpl{
    //         attrs:impl_.attrs.piped_ref(|v| Cow::Borrowed(&**v) ),
    //         trait_:impl_.trait_.as_ref().expect("must be a trait impl").1.piped_ref(Cow::Borrowed),
    //         self_ty:impl_.self_ty.piped_ref(|v| Cow::Borrowed(&**v) ),
    //         where_clause:impl_.generics.where_clause.as_ref().map(Cow::Borrowed),
    //     }
    // }

}

impl<'a> fmt::Debug for CheckImpl<'a>{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        f.debug_struct("CheckImpl")
            .field("attrs",&display_totokens_list(&*self.attrs)) 
            .field("trait_",&display_totokens(&self.trait_)) 
            .field("self_ty",&display_totokens(&self.self_ty)) 
            .field("where_clause",&display_totokens(&self.where_clause)) 
            .finish()
    }
}


impl<'a> From<syn::ItemImpl> for CheckImpl<'a>{
    fn from(impl_:syn::ItemImpl)->Self{
        CheckImpl{
            attrs:impl_.attrs.piped(Cow::Owned),
            trait_:impl_.trait_.expect("must be a trait impl").1.piped(Cow::Owned),
            self_ty:impl_.self_ty.piped(|v|Cow::Owned(*v)),
            where_clause:impl_.generics.where_clause
                .unwrap_or_else(default_where_clause)
                .piped(Cow::Owned),
        }
    }
}


fn default_where_clause()->WhereClause{
    WhereClause{
        where_token:Default::default(),
        predicates:Default::default(),
    }
}