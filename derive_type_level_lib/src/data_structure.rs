use std::fmt::Write;
use std::{cmp,hash};

use arrayvec::{
    ArrayString
};

use syn::{
    self,
    Attribute,
    Data,
    DeriveInput,
    Field as SynField,
    Fields as SynFields,
    Generics,
    Ident,
    Type,
    Visibility,
};


use quote::{
    ToTokens,
};


use proc_macro2::{
    Span,
    TokenStream,
};


//////////////////////////////////////////////////////////////////////////////


#[derive(Clone,Debug,PartialEq,Hash)]
pub struct DataStructure<'a>{
    pub vis: &'a Visibility,
    pub name: &'a Ident,
    pub generics: &'a Generics,

    pub enum_or_struct:EnumOrStruct,
    pub enum_:Option<Enum<'a>>,
    pub variants:Vec<Struct<'a>>,
}


#[derive(Clone,Debug)]
pub struct Enum<'a>{
    pub name:&'a Ident,
    pub attrs:&'a [Attribute],
    pub path:TokenStream,
}


impl<'a> cmp::PartialEq for Enum<'a>{
    fn eq(&self,other:&Self)->bool{
        self.name ==other.name &&
        self.attrs==other.attrs
    }
}

impl<'a> hash::Hash for Enum<'a>{
    fn hash<H>(&self,hasher:&mut H)
    where H:hash::Hasher
    {
        self.name.hash(hasher);
        self.attrs.hash(hasher);
    }
}



impl<'a> DataStructure<'a>{
    pub fn new(ast:&'a DeriveInput)->Self{
        let name=&ast.ident;
        let enum_=match ast.data {
            Data::Enum(_)=>
                Some(Enum{
                    name,
                    attrs:&ast.attrs,
                    path:quote!{ #name:: },
                }),
            _=>None,
        };

        let mut variants=Vec::new();

        match ast.data {
            Data::Enum(ref enum_)=>
                for var in enum_.variants.iter() {
                    variants.push(Struct::new(&var.attrs,&var.ident,&var.fields));
                },
            Data::Struct(ref struct_)=>
                variants.push(Struct::new(&ast.attrs,name,&struct_.fields)),
            Data::Union{..}=>panic!("DataStructure can't be constructed from a union."),
        }

        Self{
            vis:&ast.vis,
            name,
            generics:&ast.generics,
            enum_or_struct:if enum_.is_some() { EnumOrStruct::Enum }else{ EnumOrStruct::Struct } ,
            enum_,
            variants,
        }
    }
}


//////////////////////////////////////////////////////////////////////////////

/// Whether the struct is tupled or not.
#[derive(Debug,Copy,Clone,PartialEq,PartialOrd,Ord,Eq,Hash)]
pub enum StructKind{
    /// structs declared using the `struct Name( ... ) syntax.
    Tuple,
    /// structs declared using the `struct Name{ ... }` or `struct name;` syntaxes
    Braced,
}

#[derive(Debug,Copy,Clone,PartialEq,PartialOrd,Ord,Eq,Hash)]
pub enum EnumOrStruct{
    Struct,
    Enum,
}


//////////////////////////////////////////////////////////////////////////////

#[derive(Clone,Debug,PartialEq,Hash)]
pub struct Struct<'a>{
    pub attrs: &'a [Attribute],
    pub name:&'a Ident,
    pub kind:StructKind,
    pub fields:Vec<Field<'a>>,
    _priv:(),
}

impl<'a> Struct<'a>{
    pub fn new(attrs: &'a [Attribute],name:&'a Ident,fields:&'a SynFields)->Self{
        let kind=match *fields {
            SynFields::Named{..}  =>StructKind::Braced,
            SynFields::Unnamed{..}=>StructKind::Tuple,
            SynFields::Unit{..}   =>StructKind::Braced,
        };
        let fields:Vec<Field<'a>>=match *fields {
            SynFields::Named(ref struct_)=>Some(&struct_.named),
            SynFields::Unnamed(ref struct_)=>Some(&struct_.unnamed),
            SynFields::Unit=>None,
        }.map(|v|v.iter().enumerate().map(|(i,f)| Field::new(i,f,name.span()) ).collect() )
         .unwrap_or_default();


            
        Self{
            attrs,
            name,
            kind,
            fields,
            _priv:(),
        }
    }

    pub fn new_ident<S>(&self,token:S)->Ident
    where S:AsRef<str>
    {
        Ident::new(token.as_ref(),self.name.span())
    }
}



//////////////////////////////////////////////////////////////////////////////


/// Represent a struct field
///
#[derive(Clone,Debug,PartialEq,Hash)]
pub struct Field<'a>{
    pub attrs: &'a [Attribute],
    pub vis: &'a Visibility,
    /// identifier for the field,which is either an index(in a tuple struct) or a name.
    pub ident: FieldIdent<'a>,
    /// name given to the variable when extracting the value of the field
    /// 
    /// # Example match branch
    /// 
    /// ```ignore
    ///     quote!{ #enum_path #struct_name{#ident : #pattern_ident , ..} =>{ ... }  }
    /// ```
    pub pattern_ident:Ident,
    pub ty: &'a Type,
}


impl<'a> Field<'a>{
    pub fn new(index:usize,field:&'a SynField,span:Span)->Self{
        let ident=match field.ident.as_ref() {
            Some(ident) => FieldIdent::Named(ident),
            None => FieldIdent::new_index(index,span),
        };
        let pattern_ident=ident.pattern_ident().clone();

        Self{
            attrs:&field.attrs,
            vis:&field.vis,
            ident,
            pattern_ident,
            ty:&field.ty,
        }
    }
}


//////////////////////////////////////////////////////////////////////////////



#[derive(Debug,Clone,PartialEq,Eq,Ord,PartialOrd,Hash)]
pub enum FieldIdent<'a>{
    Index(usize,Ident),
    Named(&'a Ident),
}


impl<'a> ToTokens for FieldIdent<'a>{
    fn to_tokens(&self, tokens: &mut TokenStream){
        match *self {
            FieldIdent::Index(ind,..)=>syn::Index::from(ind).to_tokens(tokens),
            FieldIdent::Named(name)=>name.to_tokens(tokens),
        }
    }
}



impl<'a> FieldIdent<'a>{
    fn new_index(index:usize,span:Span)->Self{
        let mut buff=ArrayString::<[u8;16]>::new();
        let _=write!(buff,"field_{}",index);
        FieldIdent::Index(index,Ident::new(&buff,span))
    }
    fn pattern_ident(&self)->&Ident{
        match *self{
            FieldIdent::Index(_,ref ident)=>ident,
            FieldIdent::Named(ident)=>ident,
        }
    }
}

