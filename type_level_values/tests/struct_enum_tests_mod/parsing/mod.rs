#[allow(unused_imports)]
use core_extensions::prelude::*;

// use std::borrow::Cow;
use std::mem;
use std::cmp::{PartialEq,Eq,Ord,PartialOrd};
use std::hash::Hash;
use std::fmt;
#[allow(unused_imports)]
use std::fmt::{Write,Debug};
use std::rc::Rc;
use std::collections::{HashMap,HashSet};

use syn;
use syn::{Ident};
use syn::visit::{self,Visit};

use derive_type_level_lib::common_tokens::CommonTokens;
use derive_type_level_lib::parse_syn::*;

use derive_type_level_lib::indexable_struct::GetEnumIndices;

use shared::utils::{
    tokens_to_string,
};


declare_indexable_struct!{
    enum index=TLModIndex
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

declare_indexable_struct!{
    enum index=MCVModIndex
    #[derive(Default)]
    struct indexable=MCVIndexableByMod
    variants=[ 
        (deriving_mod         ,DerivingMod),
        (const_constructor_mod,ConstConstructorMod),
    ]
    multi_indices=[]
}

pub(crate) fn type_level_modules(tokens:&CommonTokens,type_level_mod:Ident)->Module<TLModIndex>{
    use self::TLModIndex as TLI;

    let tl_mod=Module::new(type_level_mod,TLI::TypeLevelMod)
        .add_submod(Module::new(parse_ident("variants")       ,TLI::VariantsMod))
        .add_submod(Module::new(tokens.dund_fields_mod.clone(),TLI::DunderFieldMod))
        .add_submod(Module::new(tokens.fields_mod.clone()     ,TLI::FieldsMod))
        .add_submod(Module::new(tokens.priv_mod.clone()       ,TLI::PrivateMod));

    Module::new(parse_ident("deriving"),TLI::DerivingMod)
        .add_submod(tl_mod)
}


pub(crate) fn mut_const_value_modules(
    tokens:&CommonTokens,
    const_constructor_mod:Ident
)->Module<MCVModIndex>{
    use self::MCVModIndex as MCVI;
    Module::new(parse_ident("deriving"),MCVI::DerivingMod)
        .add_submod(Module::new(const_constructor_mod,MCVI::ConstConstructorMod))
}


/////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Module<I>{
    pub name:Ident,
    pub index:I,
    pub nested:HashMap<Ident,Rc<Module<I>>>,
}

impl<I> Module<I>{
    pub fn new(name:Ident,index:I)->Self{
        Self{
            name,
            index,
            nested:HashMap::new(),
        }
    }
    pub fn add_submod(mut self,submod:Module<I>)->Self{
        match self.nested.insert( submod.name.clone(), Rc::new(submod) ) {
            Some(prev)=>
                panic!("\
                    Attempting to add '{}' submodule \
                    which collides with a pre-existing module of the same name.\
                "),
            _=>{}
        }
        self
    }

}


/////////////////////////////////////////////////////////////////////////////////////


pub trait ModIndex:'static+Debug+Copy+PartialEq+PartialOrd+Ord+Eq+Hash{}

impl<T> ModIndex for T
where T:'static+Debug+Copy+PartialEq+PartialOrd+Ord+Eq+Hash
{}



/////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug,Copy,Clone,PartialEq,PartialOrd,Ord,Eq,Hash)]
pub enum EnumOrStruct{
    Struct,
    Enum,
}


/////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug,Clone)]
pub enum VisitItem{
    Trait (syn::ItemTrait),
    Struct(syn::ItemStruct),
    Enum  (syn::ItemEnum),
    Type  (syn::ItemType),
    Impl  (syn::ItemImpl),
    Use   (syn::ItemUse),
    /// Signals that all the items in the module were visited.
    EndOfMod,
    /// Signals that all items have been visited,
    /// allowing the user to output errors that don't fit anywhere else.
    EndOfVisitor,
}


#[derive(Debug,Clone,Copy,PartialEq,Eq,Ord,PartialOrd)]
pub enum VisitItemsErrorKind{
    WrongDefinition,
    UnexpectedItem,
    ExpectedMoreItems,
}


#[derive(Debug,Clone)]
struct VisitItemsError{
    kind:VisitItemsErrorKind,
    message:String,
}


impl fmt::Display for VisitItemsError{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        use self::VisitItemsErrorKind as VIEK;
        write!(
            f,
            "{}:\n{}\n\n",
            match self.kind {
                VIEK::WrongDefinition=>"An item is defined wrong",
                VIEK::UnexpectedItem=>"Did not expect item",
                VIEK::ExpectedMoreItems=>"Expected more items",
            },
            self.message
        )
    }
}


/////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug,Clone)]
struct ItemErrors<I>{
    mod_index:I,
    item:String,
    errors:Vec<VisitItemsError>,
}

impl<I> ItemErrors<I>
where I:ModIndex
{
    fn new(mod_index:I,item:VisitItem,errors:Vec<VisitItemsError>)->Self{
        let item=match item {
            VisitItem::Trait (item_)=>{
                tokens_to_string(item_)
            }
            VisitItem::Struct(item_)=>{
                tokens_to_string(item_)
            }
            VisitItem::Enum  (item_)=>{
                tokens_to_string(item_)

            }
            VisitItem::Type  (item_)=>{
                tokens_to_string(item_)
            }
            VisitItem::Impl  (item_)=>{
                tokens_to_string(item_)
            }
            VisitItem::Use   (item_)=>{
                tokens_to_string(item_)
            }
            VisitItem::EndOfMod=>{
                format!("End of {:?} Module",mod_index)
            }
            VisitItem::EndOfVisitor=>{
                format!("Visiting Finished")
            }
        };
        Self{
            mod_index,
            item,
            errors,  
        }
    }
}

impl<I> fmt::Display for  ItemErrors<I>
where I:ModIndex
{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        writeln!(f,"--------------------")?;
        write!(f,"In Module {:?}:",self.mod_index)?;
        write!(f,"item:\n{}\n\n",self.item)?;
        for err in &self.errors {
            fmt::Display::fmt(err,f)?;
        }
        Ok(())
    }
}


/////////////////////////////////////////////////////////////////////////////////////


pub struct CheckDeriveParams<I>{
    pub mod_index:I,
    pub item:VisitItem,
    nested_errors:Vec<VisitItemsError>,
}

impl<I> CheckDeriveParams<I>
where I:ModIndex
{
    pub fn push_err<S>(&mut self,kind:VisitItemsErrorKind,error:S)
    where S:Into<String>
    {
        let e=VisitItemsError{ 
            kind,
            message:error.into() 
        };
        self.nested_errors.push(e);
    }
}



/////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug,Clone)]
pub(crate) struct Visiting<I>{
    current:Rc<Module<I>>,
    modules:Rc<Module<I>>,
    errors:Vec<ItemErrors<I>>,
}


pub struct VisitingCheck<'a,I:'static,F>{
    inner:&'a mut Visiting<I>,
    visitor:F,
}


impl<I> Visiting<I>
where I:ModIndex
{
    pub fn new(modules:Rc<Module<I>>)->Self{
        Self::check_module_indices(&modules);
        Self{
            current:modules.clone(),
            modules:modules.clone(),
            errors:Vec::new(),
        }
    }
    pub fn check_module_indices(modules:&Module<I>)->HashSet<I>{
        let mut set:HashSet<I>=Default::default();
        Self::check_module_indices_inner(modules,&mut set);
        set
    }
    pub fn check_module_indices_inner(module:&Module<I>,set:&mut HashSet<I>){
        if let Some(ind)=set.replace(module.index) {
            panic!("\n\nAttempting to inset module index twice:\n\t{:?}\n\n",ind);
        }
        for submod in module.nested.values() {
            Self::check_module_indices_inner(submod,set);
        }
    }

    pub fn check_derive<F>(&mut self,derive_output:&str,visitor:F)
    where
        F:FnMut(&mut CheckDeriveParams<I>)
    {
        let derive_output:syn::File=syn::parse_str(derive_output).unwrap_or_else(|e|{
            panic!(
                "expected valid rust file.\nFile:\n\n{}\n\nerror:\n\t{:#?}",
                derive_output,
                e 
            )
        });

        VisitingCheck{ 
            inner:&mut *self,
            visitor 
        }.start_visit(&derive_output);

        if self.errors.is_empty() {
            println!("\n\nno errors\n\n");
            return;
        }

        let mut output=String::new();
        output.push_str("\n\n\n");
        for error in self.errors.drain(..) {
            writeln!(output,"{S}{S}{}{S}{S}",error,S="\n\n----------------\n\n" ).drop_();
        }
        panic!("{}",output );
    }


    fn update_current_module(&mut self,ident:&Ident){
        if let Some(next)= self.current.nested.get(ident).cloned() {
            self.current=next;
            return;
        }

        let buffer=String::new().mutated(|buff|{
            for key in self.current.nested.keys() {
                write!(buff,"{},",key).drop_();
            }
        });
        panic!(
            "\n\nInvalid identifier for module:\n\t{}\nExpected one of:\n\t{}\n\n", 
            ident,
            buffer
        );
    }

}


impl<'a,I,F> VisitingCheck<'a,I,F> 
where
    I:ModIndex,
    F:FnMut(&mut CheckDeriveParams<I>)
{
    fn start_visit(&mut self,derive_output:&syn::File){
        self.visit_file(derive_output);
        self.call_closure(VisitItem::EndOfVisitor);
    }
    fn call_closure(&mut self,item:VisitItem){
        let current=self.inner.current.index;

        let mut params=CheckDeriveParams{
            mod_index:current,
            item:item.clone(),
            nested_errors:Vec::new(),
        };
        (self.visitor)(&mut params);

        if !params.nested_errors.is_empty() {
            let ierror=ItemErrors::new(current,item,params.nested_errors);
            self.inner.errors.push(ierror);
        }
    }
}


impl<'a,'ast,I,F> Visit<'ast> for VisitingCheck<'a,I,F> 
where
    I:ModIndex,
    F:FnMut(&mut CheckDeriveParams<I>)
{
    fn visit_item(&mut self, item: &'ast syn::Item){
        use syn::Item;
        match item {
            &Item::Mod   (ref mod_)=>{
                let replaced=self.inner.current.clone();
                let mod_ident=&mod_.ident;
                self.inner.update_current_module(mod_ident);
                visit::visit_item_mod(self,mod_);
                self.call_closure(VisitItem::EndOfMod);
                self.inner.current=replaced;
            },
            &Item::Impl  (ref v)=>self.call_closure(VisitItem::Impl(v.clone())),
            &Item::Use   (ref v)=>self.call_closure(VisitItem::Use(v.clone())),
            &Item::Trait (ref v)=>self.call_closure(VisitItem::Trait(v.clone())),
            &Item::Struct(ref v)=>self.call_closure(VisitItem::Struct(v.clone())),
            &Item::Enum  (ref v)=>self.call_closure(VisitItem::Enum(v.clone())),
            &Item::Type  (ref v)=>self.call_closure(VisitItem::Type(v.clone())),
            v=>panic!("unsupported item type:{:#?}",v)
        }
    }
}