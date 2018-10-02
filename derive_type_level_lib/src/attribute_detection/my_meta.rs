use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{self, Ident, Lit, Meta, NestedMeta};

use std::borrow::Cow;
use std::fmt::{self, Debug, Display};

use core_extensions::*;
use ::*;

#[derive(Clone,Debug,PartialEq)]
pub(crate) struct MyMeta<'alloc> {
    pub(crate) word : MyWord<'alloc>,
    pub(crate) value: MyNested<'alloc>,
}

#[derive(Clone,Debug,PartialEq)]
pub(crate) enum MyNested<'alloc> {
    Word,
    List(&'alloc Punctuated<NestedMeta, Comma>),
    MyList(Vec<MyMeta<'alloc>>),
    Value(&'alloc str),
}



impl<'alloc> FromWith<&'alloc Meta,ArenasRef<'alloc>> for MyMeta<'alloc>{
    fn from_with(meta:&'alloc Meta,arenas:ArenasRef<'alloc>) -> Self {
        match meta {
            &Meta::Word(ref word) => MyMeta {
                word: word.into(),
                value: MyNested::Word,
            },
            &Meta::List(ref list) => MyMeta {
                word: (&list.ident).into(),
                value: MyNested::List(&list.nested),
            },
            &Meta::NameValue(ref n_v) => MyMeta {
                word: (&n_v.ident).into(),
                value: match n_v.lit {
                    syn::Lit::Str(ref s) => {
                        let str_=arenas.strings.alloc(s.value());
                        MyNested::Value(str_)
                    }
                    ref other => panic!("invalid literal,expected a string:{:?}", other),
                },
            },
        }
    }
}


impl<'alloc> FromWith<&'alloc NestedMeta,ArenasRef<'alloc>> for MyMeta<'alloc>{
    fn from_with(n_meta:&'alloc NestedMeta,arenas:ArenasRef<'alloc>) -> Self {
        match n_meta {
            &NestedMeta::Meta(ref meta) => {
                meta.into_with(arenas)
            }
            &NestedMeta::Literal(ref lit) => {
                let str_ = match lit {
                    &Lit::Str(ref x) => x.value(),
                    lit => panic!(
                        "Literals must be a string contain a valid identifier \
                         when converting to MyMeta:{:#?}",
                        lit
                    ),
                };
                let ident=syn::parse_str::<Ident>(&str_)
                    .unwrap_or_else(|x|panic!(
                        "Literals must be a string contain a valid identifier \
                         when converting to MyMeta:{:#?}",
                         x
                    ))
                    .piped(|x| arenas.idents.alloc(x) );

                MyMeta {
                    word: MyWord{ident,str:str_},
                    value: MyNested::Word,
                }
            }
        }
    }
}


//////////////////////////////////////////////////////////////////////////////////////////////////




impl<'alloc> MyNested<'alloc>{
    pub(crate) fn list_to_mylist(
            &mut self,
            arenas:ArenasRef<'alloc>
    )->Result<&mut [MyMeta<'alloc>],&mut Self>{
        match self {
            &mut MyNested::List(list)=>{
                let list=list.into_iter().map(|x|MyMeta::from_with(x,arenas)).collect();
                *self=MyNested::MyList(list);
            }
            _=>{}
        }
        match self {
            &mut MyNested::MyList(ref mut list)=>Ok(list),
            this=>Err(this),
        }
    }

    pub(crate) fn to_mylist<'s>(
            &'s self,
            arenas:ArenasRef<'alloc>
    )->Option<Cow<'s,[MyMeta<'alloc>]>>{
        match self {
            &MyNested::List(list)=>{
                Some(Cow::Owned(list.into_iter().map(|x|MyMeta::from_with(x,arenas)).collect()))
            }
            &MyNested::MyList(ref list)=>Some(Cow::Borrowed(list)),
            _=>None
        }
    }
}


////////////////////////////////////////////////////////////////////////////


#[derive(Clone,PartialEq)]
pub struct MyWord<'alloc> {
    pub ident: &'alloc Ident,
    pub str: String,
}

impl<'alloc> From<&'alloc Ident> for MyWord<'alloc> {
    fn from(ident: &'alloc Ident) -> Self {
        Self { 
            ident,
            str: ident.to_string() 
        }
    }
}

impl<'alloc> Display for MyWord<'alloc> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.str, f)
    }
}
impl<'alloc> Debug for MyWord<'alloc> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.str, f)
    }
}
