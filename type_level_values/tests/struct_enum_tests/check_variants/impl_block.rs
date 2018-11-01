use super::*;

use std::fmt;

use syn::ItemImpl;

use shared::utils::{
    tokens_to_string,
};

pub struct UnparsedImplBlock<'a>{
    existence:Existence,
    
    /// Each Cow may be multiple attributes each
    attributes:Vec<Cow<'a,str>>,
    attributes_exhaus:Exhaustiveness,

    where_preds:Vec<Cow<'a,str>>,
    where_preds_exhaus:Exhaustiveness,

    trait_ :Option<Cow<'a,str>>,
    self_ty:Cow<'a,str>,
}

#[derive(Debug)]
pub struct ImplBlock{
    pub(crate) existence:Existence,

    pub(crate) attributes:HashSet<Attribute>,
    pub(crate) attributes_exhaus:Exhaustiveness,
    
    pub(crate) where_preds:HashSet<WherePredicate>,
    pub(crate) where_preds_exhaus:Exhaustiveness,
    
    pub(crate) trait_:Option<SynPath>,
    pub(crate) self_ty:SynType,

    _priv:(),
}



impl<'a> UnparsedImplBlock<'a>{
    fn priv_default()->Self{
        Self{
            existence:Exists,

            attributes:Vec::new(),
            attributes_exhaus:Inexhaustive,
            
            where_preds:Vec::new(),
            where_preds_exhaus:Inexhaustive,
            
            trait_:None,
            self_ty:"".into(),
        }
    }

    pub fn inherent<S1>(self_ty:S1)->Self
    where 
        S1:Into<Cow<'a,str>>,
    {
        Self{
            trait_ :None,
            self_ty:self_ty.into(),
            ..Self::priv_default()
        }
    }



    pub fn new<S0,S1>(trait_:S0,self_ty:S1)->Self
    where 
        S0:Into<Cow<'a,str>>,
        S1:Into<Cow<'a,str>>,
    {
        Self{
            trait_ :Some(trait_.into()),
            self_ty:self_ty.into(),
            ..Self::priv_default()
        }
    }

    pub fn set_exhaustive(mut self)->Self{
        self.attributes_exhaus=Exhaustive;
        self.where_preds_exhaus=Exhaustive;
        self
    }
    pub fn set_nonexistant(mut self)->Self{
        self.existence=NotExists;
        self
    }

    pub fn add_where_pred<S>(mut self,where_pred:S)->Self
    where S:Into<Cow<'a,str>>,
    {
        self.where_preds.push(where_pred.into());
        self
    }

    pub fn add_attribute<S>(mut self,attrs:S)->Self
    where S:Into<Cow<'a,str>>,
    {
        self.attributes.push(attrs.into());
        self
    }
    pub fn add_attributes<S>(mut self,attrs:S)->Self
    where S:Into<Cow<'a,str>>,
    {
        self.attributes.push(attrs.into());
        self
    }

    pub fn parse(self)->ImplBlock{
        ImplBlock{
            existence:self.existence,

            attributes:self.attributes.iter().flat_map(|x| parse_syn_attributes(x) ).collect(),
            attributes_exhaus:self.attributes_exhaus,
            
            where_preds:self.where_preds.iter().map(|x| parse_where_pred(x) ).collect(),
            where_preds_exhaus:self.where_preds_exhaus,
            
            trait_:self.trait_.as_ref().map(|x| parse_syn_path(x) ),
            self_ty:parse_type(&*self.self_ty),

            _priv:(),
        }
    }
}


////////////////////////////////////////////////////////////////////////////////////////


pub trait ToImplBlock{
    fn to_impl_block(self)->ImplBlock;
}

impl<'a> ToImplBlock for UnparsedImplBlock<'a>{
    fn to_impl_block(self)->ImplBlock{
        self.parse()
    }
}

impl ToImplBlock for ImplBlock{
    fn to_impl_block(self)->ImplBlock{
        self
    }
}


////////////////////////////////////////////////////////////////////////////////////////



#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct ImplHeader{
    pub(crate) trait_:Option<SynPath>,
    pub(crate) self_ty:SynType,
}


impl ImplHeader{
    pub fn new(impl_:&ImplBlock)->Self{
        Self{
            trait_ :impl_.trait_ .clone(),
            self_ty:impl_.self_ty.clone(),
        }
    }

    pub fn from_itemimpl(impl_:&ItemImpl)->Self{
        Self{
            trait_ :impl_.trait_.as_ref().map(|x|&x.1).cloned(),
            self_ty:(*impl_.self_ty).clone(),
        }   
    }
}


impl fmt::Display for ImplHeader{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        write!(
            f,
            "impl {} for {}",
            tokens_to_string(&self.trait_),
            tokens_to_string(&self.self_ty),
        )
    }
}


