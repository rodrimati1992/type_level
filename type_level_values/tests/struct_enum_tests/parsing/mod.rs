#[allow(unused_imports)]
use core_extensions::prelude::*;

// use std::collections::{BTreeMap,HashMap,HashSet};
// use std::borrow::Cow;
use std::mem;
use std::fmt;
#[allow(unused_imports)]
use std::fmt::Write;

use syn;
use syn::{Ident};
use syn::visit::{self,Visit};

use derive_type_level_lib::common_tokens::CommonTokens;
use derive_type_level_lib::parse_syn::*;

use derive_type_level_lib::indexable_struct::GetEnumIndices;

declare_indexable_struct!{
    enum index=ModIndex
    #[derive(Default)]
    struct indexable=IndexableByMod
    variants=[ 
        (variants_mod    ,VariantsMod),
        (deriving_mod    ,DerivingMod),
        (type_level_mod  ,TypeLevelMod),
        (dunder_field_mod,DunderFieldMod),
        (fields_mod      ,FieldsMod),
        (private_mod     ,PrivateMod),
    ]
    multi_indices=[]
}

impl ModIndex{
    pub(crate)fn new(ident:&Ident,tokens:&CommonTokens,type_level_mod:&Ident)->Option<ModIndex>{
        Some(if ident==type_level_mod   { 
            ModIndex::TypeLevelMod 
        }else if *ident==tokens.dund_fields_mod { 
            ModIndex::DunderFieldMod 
        }else if *ident==tokens.fields_mod       { 
            ModIndex::FieldsMod 
        }else if *ident==tokens.priv_mod      { 
            ModIndex::PrivateMod 
        }else if *ident=="variants"      { 
            ModIndex::VariantsMod
        }else{ 
            return None 
        })
    }
}

impl Default for ModIndex{
    fn default()->Self{
        ModIndex::DerivingMod
    }
}


/////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug,Copy,Clone,PartialEq,PartialOrd,Ord,Eq,Hash)]
pub enum EnumOrStruct{
    Struct,
    Enum,
}


/////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug,Copy,Clone)]
pub enum VisitItem<'a>{
    Trait (&'a syn::ItemTrait),
    Struct(&'a syn::ItemStruct),
    Type  (&'a syn::ItemType),
    Impl  (&'a syn::ItemImpl),
    Use   (&'a syn::ItemUse),
    /// Signals that all the items in the module were visited.
    EndOfMod,
}


#[derive(Debug,Clone,Copy,PartialEq,Eq,Ord,PartialOrd)]
pub enum VisitItemsErrorKind{
    WrongDefinition,
    UnexpectedItem,
    ExpectedMoreItems,
}


#[derive(Debug,Clone)]
pub struct VisitItemsError{
    kind:VisitItemsErrorKind,
    mod_ind:ModIndex,
    message:String,
}

impl VisitItemsError{
    pub fn new<S>(kind:VisitItemsErrorKind,message:S)->Self
    where S:Into<String>
    {
        Self{
            kind,
            mod_ind:ModIndex::default(),
            message:message.into(),
        }
    }
}


impl fmt::Display for VisitItemsError{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        use self::VisitItemsErrorKind as VIEK;
        writeln!(
            f,
            "{} in module {:?} :\n{}",
            match self.kind {
                VIEK::WrongDefinition=>"An item is defined wrong",
                VIEK::UnexpectedItem=>"Did not expect item",
                VIEK::ExpectedMoreItems=>"Expected more items",
            },
            self.mod_ind,
            self.message
        )
    }
}


/////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug,Clone)]
pub(crate) struct Visiting<'a>{
    current:ModIndex,
    errors:Vec<VisitItemsError>,
    ctokens:&'a CommonTokens,
    type_level_mod:Ident,
}


pub struct VisitingCheck<'a:'b,'b,F>{
    inner:&'b mut Visiting<'a>,
    visitor:F,
}


impl<'a> Visiting<'a>{
    pub fn new(ctokens:&'a CommonTokens,type_level_mod:&str)->Self{
        Self{
            current:ModIndex::DerivingMod,
            errors:Vec::new(),
            ctokens,
            type_level_mod:parse_ident(type_level_mod),
        }
    }

    pub fn check_derive<F>(&mut self,derive_output:&'a str,visitor:F)
    where
        F:FnMut(ModIndex,VisitItem)->Result<(),VisitItemsError>
    {
        let derive_output:syn::File=syn::parse_str(derive_output).unwrap_or_else(|e|{
            panic!(
                "expected valid rust file.\nFile:\n\n{}\n\nerror:\n\t{:#?}",
                derive_output,
                e 
            )
        });

        VisitingCheck{ inner:&mut *self,visitor }.visit_file(&derive_output);

        if self.errors.is_empty() {
            println!("\n\nno errors\n\n");
            return;
        }

        let mut output=String::new();
        output.push_str("\n\n\n");
        for error in self.errors.drain(..) {
            writeln!(output,"{S}{}{S}",error,S="\n\n----------------\n\n" ).drop_();
        }
        panic!("{}",output );
    }


    fn check_module(
        &self,
        modind:ModIndex,
    )->ModIndex{
        use self::ModIndex as MI;

        let current=match ( self.current, modind ) {
             ( MI::DerivingMod  , ind@MI::TypeLevelMod )
            |( MI::TypeLevelMod , ind@MI::DunderFieldMod )
            |( MI::TypeLevelMod , ind@MI::FieldsMod )
            |( MI::TypeLevelMod , ind@MI::PrivateMod )
            |( MI::TypeLevelMod , ind@MI::VariantsMod )
            =>ind,
            (from,to)=>
                panic!("cannot go from:{:?} to:{:?}", from,to),
        };
        current
    }

}


impl<'a,'b,F> VisitingCheck<'a,'b,F> 
where
    F:FnMut(ModIndex,VisitItem)->Result<(),VisitItemsError>
{
    fn call_closure(&mut self,item:VisitItem){
        let current=self.inner.current;

        if let Err(mut e)=(self.visitor)(current,item) {
            e.mod_ind=current;
            self.inner.errors.push(e);
        }
    }
    fn nested_frame<NF>(&mut self,mod_:ModIndex,f:NF)
    where
        NF:FnOnce(&mut Self)
    {
        let replaced=mem::replace(&mut self.inner.current,mod_);
        f(self);
        self.inner.current=replaced;
    }
}


impl<'a,'b,'ast,F> Visit<'ast> for VisitingCheck<'a,'b,F> 
where
    F:FnMut(ModIndex,VisitItem)->Result<(),VisitItemsError>
{
    fn visit_item(&mut self, item: &'ast syn::Item){
        use syn::Item;
        match item {
            &Item::Mod(ref v)=> self.visit_item_mod(v),
            &Item::Impl(ref v)=>self.call_closure(VisitItem::Impl(v)),
            &Item::Use(ref v)=>self.call_closure(VisitItem::Use(v)),
            &Item::Trait (ref v)=>self.call_closure(VisitItem::Trait(v)),
            &Item::Struct(ref v)=>self.call_closure(VisitItem::Struct(v)),
            &Item::Type  (ref v)=>self.call_closure(VisitItem::Type(v)),
            v=>panic!("unsupported item type:{:#?}",v)
        }
    }
    fn visit_item_mod(&mut self, mod_: &'ast syn::ItemMod){
        let mod_ident=&mod_.ident;
        let mod_ind=ModIndex::new(mod_ident,self.inner.ctokens,&self.inner.type_level_mod)
            .unwrap_or_else(||{
                panic!("invalid module:\n\t'{}'\n\nexpected one of:\n\t{}",
                    mod_ident,
                    ModIndex::indices_message(),
                )
            });
        let inner_module=self.inner.check_module(mod_ind);
        self.nested_frame(inner_module,|this|{
            visit::visit_item_mod(this,mod_);
            this.call_closure(VisitItem::EndOfMod);
        });
    }
}