use super::*;

use std::fmt;

use syn;
use syn::{
    Visibility,
    Generics,
};




const DEFAULT_ATTRIBUTES_EXHAUS:Exhaustiveness =Inexhaustive;
const DEFAULT_WHERE_PREDS_EXHAUS:Exhaustiveness=Inexhaustive;


#[derive(Debug,Clone)]
pub struct UnparsedItemCheck<'a>{
    existence:Existence,

    vis:Option<Cow<'a,str>>,
    
    /// Each Cow may be multiple attributes each
    attributes:Vec<Cow<'a,str>>,
    attributes_exhaus:Exhaustiveness,

    where_preds:Vec<Cow<'a,str>>,
    where_preds_exhaus:Exhaustiveness,

    key:UnparsedKey<'a>,
}

#[derive(Debug,Clone)]
pub enum UnparsedKey<'a>{
    Impl{
        trait_ :Option<Cow<'a,str>>,
        self_ty:Cow<'a,str>,
    },
    TypeAlias{
        name:Cow<'a,str>,
    },
    TypeDecl{
        kind:EnumOrStruct,
        name:Cow<'a,str>,
    },
    Trait{
        name:Cow<'a,str>,
    },
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum ItemKey{
    Impl{
        trait_:Option<SynPath>,
        self_ty:SynType,
    },
    TypeAlias{
        name:Ident,
    },
    TypeDecl{
        kind:EnumOrStruct,
        name:Ident,
    },
    Trait{
        name:Ident,
    },
}



#[derive(Debug)]
pub struct ItemCheck<K=ItemKey>{
    pub(crate) existence:Existence,


    pub(crate) vis:Option<Visibility>,

    pub(crate) attributes:HashSet<Attribute>,
    pub(crate) attributes_exhaus:Exhaustiveness,
    
    pub(crate) where_preds:HashSet<WherePredicate>,
    pub(crate) where_preds_exhaus:Exhaustiveness,
    
    pub(crate) key:K,

    _priv:(),
}



impl<'a> UnparsedItemCheck<'a>{
    pub fn new(key:UnparsedKey<'a>)->Self{
        Self{
            existence:Exists,

            vis:None,

            attributes:Vec::new(),
            attributes_exhaus:DEFAULT_ATTRIBUTES_EXHAUS,
            
            where_preds:Vec::new(),
            where_preds_exhaus:DEFAULT_WHERE_PREDS_EXHAUS,
            
            key,
        }
    }

    pub fn inherent_impl<S1>(self_ty:S1)->Self
    where 
        S1:Into<Cow<'a,str>>,
    {
        UnparsedKey::Impl{
            trait_ :None,
            self_ty:self_ty.into(),
        }.piped(Self::new)
    }



    pub fn trait_impl<S0,S1>(trait_:S0,self_ty:S1)->Self
    where 
        S0:Into<Cow<'a,str>>,
        S1:Into<Cow<'a,str>>,
    {
        UnparsedKey::Impl{
            trait_ :Some(trait_.into()),
            self_ty:self_ty.into(),
        }.piped(Self::new)
    }

    pub fn trait_<S1>(name:S1)->Self
    where 
        S1:Into<Cow<'a,str>>,
    {
        UnparsedKey::Trait{name:name.into()}.piped(Self::new)
    }

    pub fn type_alias<S1>(name:S1)->Self
    where 
        S1:Into<Cow<'a,str>>,
    {
        UnparsedKey::TypeAlias{name:name.into()}.piped(Self::new)
    }

    pub fn struct_<S1>(name:S1)->Self
    where 
        S1:Into<Cow<'a,str>>,
    {
        UnparsedKey::TypeDecl{
            kind:EnumOrStruct::Struct,
            name:name.into(),
        }.piped(Self::new)
    }

    pub fn enum_<S1>(name:S1)->Self
    where 
        S1:Into<Cow<'a,str>>,
    {
        UnparsedKey::TypeDecl{
            kind:EnumOrStruct::Enum,
            name:name.into(),
        }.piped(Self::new)
    }


}


impl<'a> UnparsedItemCheck<'a>{
    pub fn set_exhaustive(mut self)->Self{
        self.attributes_exhaus=Exhaustive;
        self.where_preds_exhaus=Exhaustive;
        self
    }
    pub fn set_nonexistant(mut self)->Self{
        self.existence=NotExists;
        self
    }

    pub fn set_vis<S>(mut self,vis:S)->Self
    where S:Into<Cow<'a,str>>,
    {
        self.vis=Some(vis.into());
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

    pub fn parse(self)->ItemCheck{
        ItemCheck{
            existence:self.existence,

            vis:self.vis.map(|x| parse_visibility(&x) ),

            attributes:self.attributes.iter().flat_map(|x| parse_syn_attributes(x) ).collect(),
            attributes_exhaus:self.attributes_exhaus,
            
            where_preds:self.where_preds.iter().map(|x| parse_where_pred(x) ).collect(),
            where_preds_exhaus:self.where_preds_exhaus,

            key:self.key.parse(),
            
            _priv:(),
        }
    }
}


impl ItemCheck{

    pub fn from_item(
        attributes:&[Attribute],
        vis:Option<&Visibility>,
        gens:&Generics,
    )->ItemCheck<()>{
        ItemCheck{
            existence:Exists,
            vis:vis.cloned(),
            attributes:attributes.iter().cloned().collect(),
            attributes_exhaus:DEFAULT_ATTRIBUTES_EXHAUS,
            where_preds:match gens.where_clause.as_ref() {
                Some(where_)=>where_.predicates.iter().cloned().collect(),
                None=>Default::default(),
            },
            where_preds_exhaus:DEFAULT_WHERE_PREDS_EXHAUS,
            key:(),
            _priv:(),
        }
    }
}

impl<K> ItemCheck<K>{
    pub fn split_key(self)->(K,ItemCheck<()>){
        self.replace_key(())
    }
    pub fn set_key<K2>(self,key:K2)->ItemCheck<K2>{
        self.replace_key(key).1
    }
    fn replace_key<K2>(self,key:K2)->(K,ItemCheck<K2>){
        let check=ItemCheck{
            existence:self.existence,
            vis:self.vis,
            attributes:self.attributes,
            attributes_exhaus:self.attributes_exhaus,
            where_preds:self.where_preds,
            where_preds_exhaus:self.where_preds_exhaus,
            key,
            _priv:(),
        };
        (self.key,check)
    }

}


////////////////////////////////////////////////////////////////////////////////////////


/// Converts Self to ItemCheck,possibly panicking on conversion failure.
/// 
/// This is a new trait instead of using From,since this convertion can panic.
pub trait ToItemCheck{
    fn to_item_check(self)->ItemCheck;
}

impl<'a> ToItemCheck for UnparsedItemCheck<'a>{
    fn to_item_check(self)->ItemCheck{
        self.parse()
    }
}

impl ToItemCheck for ItemCheck{
    fn to_item_check(self)->ItemCheck{
        self
    }
}


//////////////////////////////////////////////////////////////////////////////////////


/// Converts a reference to Self to Option<ItemCheck>.
pub trait ItemToCheck{
    fn item_to_check(&self)->Option<ItemCheck>;
}


impl<'a> ItemToCheck for VisitItem<'a> {
    fn item_to_check(&self)->Option<ItemCheck>{
        match *self {
            VisitItem::Trait (item)=>item.item_to_check(),
            VisitItem::Struct(item)=>item.item_to_check(),
            VisitItem::Enum  (item)=>item.item_to_check(),
            VisitItem::Type  (item)=>item.item_to_check(),
            VisitItem::Impl  (item)=>item.item_to_check(),
            VisitItem::Use   (item)=>None,
            VisitItem::EndOfMod=>None,
            VisitItem::EndOfVisitor=>None,
        }
    }
}


impl ItemToCheck for syn::ItemImpl {
    fn item_to_check(&self)->Option<ItemCheck>{
        let key=ItemKey::Impl{
            trait_ :self.trait_.as_ref().map(|x|&x.1).cloned(),
            self_ty:(*self.self_ty).clone(),
        };
        ItemCheck::from_item(&self.attrs,None,&self.generics)
            .set_key(key)
            .piped(Some)
    }
}

impl ItemToCheck for syn::ItemType {
    fn item_to_check(&self)->Option<ItemCheck>{
        let key=ItemKey::TypeAlias{
            name:self.ident.clone(),
        };
        ItemCheck::from_item(&self.attrs,Some(&self.vis),&self.generics)
            .set_key(key)
            .piped(Some)
    }
}
impl ItemToCheck for syn::ItemEnum {
    fn item_to_check(&self)->Option<ItemCheck>{
        let key=ItemKey::TypeDecl{
            kind:EnumOrStruct::Enum,
            name:self.ident.clone(),
        };
        ItemCheck::from_item(&self.attrs,Some(&self.vis),&self.generics)
            .set_key(key)
            .piped(Some)
    }
}
impl ItemToCheck for syn::ItemStruct {
    fn item_to_check(&self)->Option<ItemCheck>{
        let key=ItemKey::TypeDecl{
            kind:EnumOrStruct::Struct,
            name:self.ident.clone(),
        };
        ItemCheck::from_item(&self.attrs,Some(&self.vis),&self.generics)
            .set_key(key)
            .piped(Some)
    }
}
impl ItemToCheck for syn::ItemTrait {
    fn item_to_check(&self)->Option<ItemCheck>{
        let key=ItemKey::Trait{
            name:self.ident.clone(),
        };
        ItemCheck::from_item(&self.attrs,Some(&self.vis),&self.generics)
            .set_key(key)
            .piped(Some)
    }
}



////////////////////////////////////////////////////////////////////////////////////////


impl<'a> UnparsedKey<'a>{
    pub fn parse(self)->ItemKey{
        match self {
            UnparsedKey::Impl{trait_,self_ty}=>
                ItemKey::Impl{
                    trait_:trait_.map(|x|parse_syn_path(&x)) ,
                    self_ty:parse_type(&self_ty)
                },
            UnparsedKey::TypeAlias{ref name}=>
                ItemKey::TypeAlias{
                    name:parse_ident(name)
                },
            UnparsedKey::TypeDecl{kind,ref name}=>
                ItemKey::TypeDecl{
                    kind,
                    name:parse_ident(name)
                },
            UnparsedKey::Trait{ref name}=>
                ItemKey::Trait{
                    name:parse_ident(name)
                },
        }
    }
}
impl fmt::Display for ItemKey{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        use shared::utils::{
            tokens_to_string as ttstr,
        };

        match *self {
            ItemKey::Impl{ref trait_,ref self_ty}=>
                write!(f,"impl {} for {}",ttstr(trait_),ttstr(self_ty)),
            ItemKey::TypeAlias{ref name}=>
                write!(f,"{} type alias",ttstr(name)),
            ItemKey::TypeDecl{kind:EnumOrStruct::Enum,ref name}=>
                write!(f,"enum {}",ttstr(name)),
            ItemKey::TypeDecl{kind:EnumOrStruct::Struct,ref name}=>
                write!(f,"{} struct",ttstr(name)),
            ItemKey::Trait{ref name}=>
                write!(f,"trait {}",ttstr(name)),
        }
    }
}

