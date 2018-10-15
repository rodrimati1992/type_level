pub(crate) mod test_tokens;
pub(crate) mod check_impl;

use self::test_tokens::TestTokens;
use self::check_impl::CheckImpl;

#[allow(unused_imports)]
use core_extensions::prelude::*;

use std::collections::{BTreeMap,HashMap,HashSet};
use std::borrow::Cow;
use std::mem;

use syn;
use syn::{Ident,ItemUse};
use syn::visit::{self,Visit};

use derive_type_level_lib::typelevel::derive_from_str;
use derive_type_level_lib::common_tokens::CommonTokens;
use derive_type_level_lib::parse_syn::*;

use derive_type_level_lib::indexable_struct::GetEnumIndices;

use utils::{    
    display_totokens_list,
    display_totokens,
    tokens_to_string,
};



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
    pub(crate)fn new(ident:&Ident,tokens:&TestTokens)->Option<ModIndex>{
        Some(if *ident==tokens.type_level_mod   { 
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


macro_rules! declare_declaration {
    ( $( $variant:ident  $( [ $($tparams:tt)* ] )* ),* $(,)*  ) => (
        
        pub(crate) enum Declaration<'a>{
            $( $variant( $variant < $($( $tparams )*)* > ), )*
        }
        $(
            impl<'a> From< $variant<$($( $tparams )*)*> > for Declaration<'a>{
                fn from(from: $variant<$($( $tparams )*)*> )-> Declaration<'a> {
                    Declaration::$variant(from)
                }
            }
        )*
    )
} 

declare_declaration!{
    ExpectedItem['a],
    CheckImpl['a],
    ItemUse,
}


#[derive(Debug,Default)]
pub struct Declarations<'a>{
    item_decls:ItemDecls<'a>,
    impl_decls:ImplDecls<'a>,
    use_decls:UseDecls,
}


pub(crate) type ItemDecls<'a>=IndexableByMod<BTreeMap<Ident,ExpectedItem<'a>>>;

pub(crate) type ImplDecls<'a>=IndexableByMod<HashSet<CheckImpl<'a>>>;

pub(crate) type UseDecls=IndexableByMod<HashSet<syn::ItemUse>>;

pub(crate) type ModExpectedVis=IndexableByMod<syn::Visibility>;


pub(crate)fn new_decls(mods:IndexableByMod<Vec<Declaration>>)->Declarations {
    let mut ret=Declarations::default();
    for (mod_ind , list ) in mods.to_vec(){
        let item_decls=&mut ret.item_decls[mod_ind];
        let impl_decls=&mut ret.impl_decls[mod_ind];
        let use_decls =&mut ret.use_decls[mod_ind];
        for expected in list {
            match expected {
                Declaration::ExpectedItem(expected)=>{
                    let ident=expected.ident().clone();
                    if let Some(previous)=item_decls.insert(ident,expected) {
                        panic!(
                            "attempting to insert with the same ident.\
                            \nprevious:\n\t{:#?}",
                            previous
                        );
                    }
                }
                Declaration::CheckImpl(expected)=>{
                    if impl_decls.contains(&expected) {
                        panic!(
                            "attempting to insert the same impl.\
                            \nprevious:\n\t{:#?}",
                            expected
                        );
                    }
                    impl_decls.insert(expected) ;
                }
                Declaration::ItemUse(expected)=>{
                    if use_decls.contains(&expected) {
                        panic!(
                            "attempting to insert with the same element.\
                            \nelement:\n\t{:#?}",
                            expected
                        );
                    }
                    use_decls.insert(expected);
                }
            }
        }   
    }
    ret
}


macro_rules! declare_expected_item {
    ( $( $variant:ident ( $variant_ty:ty ) ),* $(,)* ) => (

        #[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
        pub(crate)enum ExpectedItemRef<'a>{
            $( $variant(&'a $variant_ty), )*
        }


        #[derive(Debug,Clone,PartialEq,Eq,Hash)]
        pub(crate) enum ExpectedItem<'a>{
            $( $variant($variant_ty), )*
        }

        impl<'a> ExpectedItem<'a>{
            fn as_ref(&self)->ExpectedItemRef{
                match self {
                    $( 
                        &ExpectedItem::$variant(ref x)=>
                            ExpectedItemRef::$variant(x),
                    )*
                }
            }
        }
    )
}
declare_expected_item!{
    TypeAlias(CheckItem<'a>),
    Struct(syn::ItemStruct),
    Trait(syn::ItemTrait),
}

impl<'a> ExpectedItem<'a>{
    pub fn ident(&self)->&syn::Ident{
        use self::ExpectedItem as EI;
        match self {
            &EI::TypeAlias(ref check)=>&check.ident,
            &EI::Struct(ref check)=>&check.ident,
            &EI::Trait(ref check)=>&check.ident,
        }
    }

    pub fn new_typealias(attrs:&str,vis:&str,ident:&str)-> Self {
        CheckItem::new(attrs,vis,ident)
            .piped(ExpectedItem::TypeAlias)
    }

    pub fn new_struct(s:&str)->Self{
        ExpectedItem::Struct(
            syn::parse_str(s)
            .unwrap_or_else(|e| parse_error_msg("Invalid struct declaration",&s,e) )
        )
    }

    pub fn new_trait(s:&str)->Self{
        ExpectedItem::Trait(
            syn::parse_str(s)
            .unwrap_or_else(|e| parse_error_msg("Invalid trait declaration",&s,e) )
        )
    }
}


#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub(crate) struct CheckItem<'a>{
    pub(crate) attrs:Cow<'a,[syn::Attribute]>,
    pub(crate) vis:Cow<'a,syn::Visibility>,
    pub(crate) ident:Cow<'a,Ident>,
}


impl<'a> CheckItem<'a>{
    pub fn new<S>(attrs:&str,vis:&str,ident:S)-> Self 
    where S:Into<String>
    {
        Self{
            attrs:parse_syn_attributes(attrs).into(),
            vis:parse_visibility(vis).piped(Cow::Owned),
            ident:parse_ident(&ident.into()).piped(Cow::Owned),
        }
    }

    pub fn from_ref(typealias:&'a syn::ItemType)->Self{
        Self{
            attrs:Cow::Borrowed(&*typealias.attrs),
            vis:Cow::Borrowed(&typealias.vis),
            ident:Cow::Borrowed(&typealias.ident),
        }
    }
}


pub(crate) trait PushDeclarationExt<'a>:TypeIdentity<Type= Vec<Declaration<'a>> >{
    fn push_impl<'b,S>(&'b mut self,s:S)->&'b mut CheckImpl<'a>
    where 
        'a:'b,
        S:Into<String>,
    {
        let s=s.into();
        let this=self.into_type_mut();
        this.push(CheckImpl::<'a>::new(&s).into());
        match this.last_mut() {
            Some(&mut Declaration::CheckImpl(ref mut x))=>x,
            _=>unreachable!(),
        }
    }
    fn push_use<'b,S>(&'b mut self,s:S)->&'b mut syn::ItemUse 
    where 
        'a:'b,
        S:Into<String>,
    {
        let s=s.into();
        let this=self.into_type_mut();
        this.push( parse_syn_use(&s).into());
        match this.last_mut() {
            Some(&mut Declaration::ItemUse(ref mut x))=>x,
            _=>unreachable!(),
        }
    }
    
    fn push_typealias<'b,S>(&'b mut self,attrs:&str,vis:&str,ident:S)-> &'b mut CheckItem<'a>
    where 
        'a:'b,
        S:Into<String>,
    {
        let this=self.into_type_mut();
        let x=CheckItem::new(attrs,vis,ident);
        this.push(x.piped(ExpectedItem::TypeAlias).into());
        match this.last_mut() {
            Some(&mut Declaration::ExpectedItem(ExpectedItem::TypeAlias(ref mut x)))=>x,
            _=>unreachable!(),
        }
    }

    fn push_struct<'b,S>(&'b mut self,s:S)->&'b mut syn::ItemStruct
    where
        'a:'b,
        S:Into<String>,
    {
        let s=s.into();
        let this=self.into_type_mut();
        let x=syn::parse_str::<syn::ItemStruct>(&s)
            .unwrap_or_else(|e| parse_error_msg("Invalid struct declaration",&s,e) );
        this.push(x.piped(ExpectedItem::Struct).into());
        match this.last_mut() {
            Some(&mut Declaration::ExpectedItem(ExpectedItem::Struct(ref mut x)))=>x,
            _=>unreachable!(),
        }
    }

    fn push_trait<'b,S>(&'b mut self,s:S)->&'b mut syn::ItemTrait
    where
        'a:'b,
        S:Into<String>,
    {
        let s=s.into();
        let this=self.into_type_mut();
        let x=syn::parse_str::<syn::ItemTrait>(&s)
            .unwrap_or_else(|e| parse_error_msg("Invalid trait declaration",&s,e) );
        this.push(x.piped(ExpectedItem::Trait).into());
        match this.last_mut() {
            Some(&mut Declaration::ExpectedItem(ExpectedItem::Trait(ref mut x)))=>x,
            _=>unreachable!(),
        }
    }
}

impl<'a> PushDeclarationExt<'a> for Vec<Declaration<'a>>{}


#[derive(Debug,Clone)]
pub(crate) struct Visiting<'a>{
    current:ModIndex,
    errors:Vec<String>,

    item_decls:ItemDecls<'a>,
    impl_decls:ImplDecls<'a>,
    use_decls:UseDecls,

    expected_vis:ModExpectedVis,

    ttokens:&'a TestTokens,
}

impl<'a> Visiting<'a>{
    pub fn new(
        declarations:Declarations<'a>,
        expected_vis:ModExpectedVis,
        ttokens:&'a TestTokens
    )->Self{
        Self{
            current:ModIndex::DerivingMod,
            errors:Vec::new(),
            item_decls:declarations.item_decls,
            impl_decls:declarations.impl_decls,
            use_decls :declarations.use_decls,
            expected_vis,
            ttokens,
        }
    }

    pub fn check_derive(&mut self,derive_output:&'a str){
        use std::fmt::Write;

        let derive_output:syn::File=syn::parse_str(derive_output).unwrap_or_else(|e|{
            panic!(
                "expected valid rust file.\nFile:\n\n{}\n\nerror:\n\t{:#?}",
                derive_output,
                e 
            )
        });
        self.visit_file(&derive_output);

        if self.errors.is_empty() {
            println!("\n\nno errors\n\n");
            return;
        }

        let mut output=String::new();
        output.push_str("\n\n\n");
        for error in self.errors.drain(..) {
            use std::fmt::Write;
            writeln!(output,"{S}{}{S}",error,S="\n\n----------------\n\n" ).drop_();
        }

        
        // output.push_str("\n\n\n");
        // writeln!(output,"{SEP}\t\t REMAINING UNMATCHED \t\t {SEP}",
        //     SEP="\n\n--------------------------------------------------------------------\n\n"
        // ).drop_();
        // writeln!(output,"item_decls:{:#?}\n\n\n",self.item_decls).drop_();
        // writeln!(output,"impl_decls:{:#?}\n\n\n",self.impl_decls).drop_();
        // writeln!(output,"use_decls :{:#?}\n\n\n",self.use_decls ).drop_();

        panic!("{}",output );
    }


    fn check_module(
        &self,
        modind:ModIndex,
        vis:&syn::Visibility,
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
        assert_eq!(
            vis,&self.expected_vis[current],
            "\n\nexpected privacy:\n\t{:?}\nfound privacy:\n\t{:?}\n\n",
            vis,&self.expected_vis[current]
        );
        current
    }

    fn nested_frame<F>(&mut self,mod_:ModIndex,f:F)
    where
        F:FnOnce(&mut Self)
    {
        let replaced=mem::replace(&mut self.current,mod_);
        f(self);
        self.current=replaced;
    }

    fn visit_expected_item<'ast>(&mut self,found:ExpectedItemRef){
        let mod_ind= self.current;
        let mod_items=&mut self.item_decls[mod_ind];

        let ident=match found {
            ExpectedItemRef::TypeAlias(found)=>&found.ident,
            ExpectedItemRef::Struct(found)   =>&found.ident,
            ExpectedItemRef::Trait(found)    =>&found.ident,
        };
        let contains=mod_items.get(ident).map_or(false,|check| check.as_ref()==found );

        if contains {
            mod_items.remove(ident);
        }else{
            self.errors.push(format!(
                "item '{}' is not expected (in module {:?}):\n\n{}\n\n",
                ident,
                mod_ind,
                match found {
                    ExpectedItemRef::TypeAlias(found)=>format!("{:?}",found),
                    ExpectedItemRef::Struct(found)   =>tokens_to_string(found),
                    ExpectedItemRef::Trait(found)    =>tokens_to_string(found),
                }
            ));
        }
    }
}


impl<'a,'ast> Visit<'ast> for Visiting<'a>{
    fn visit_item(&mut self, item: &'ast syn::Item){
        use syn::Item;
        match item {
            &Item::Mod(ref v)=> self.visit_item_mod(v),
            &Item::Impl(ref v)=> self.visit_item_impl(v),
            &Item::Use(ref v)=> self.visit_item_use(v),
            &Item::Trait (ref v)=>self.visit_expected_item(ExpectedItemRef::Trait(v)),
            &Item::Struct(ref v)=>self.visit_expected_item(ExpectedItemRef::Struct(v)),
            &Item::Type  (ref v)=>{
                let check=CheckItem::from_ref(v);
                let item=ExpectedItemRef::TypeAlias(&check);
                self.visit_expected_item(item)
            },
            v=>panic!("unsupported item type:{:#?}",v)
        }
    }
    fn visit_item_mod(&mut self, mod_: &'ast syn::ItemMod){
        let mod_ident=&mod_.ident;
        let mod_ind=ModIndex::new(mod_ident,self.ttokens).unwrap_or_else(||{
            panic!("invalid module:\n\t'{}'\n\nexpected one of:\n\t{}",
                mod_ident,
                ModIndex::indices_message(),
            )
        });
        let inner_module=self.check_module(mod_ind,&mod_.vis);
        self.nested_frame(inner_module,|this|{
            visit::visit_item_mod(this,mod_);
        })   
    }

    fn visit_item_impl(&mut self, impl_: &'ast syn::ItemImpl){
        let check_impl_=CheckImpl::from(impl_.clone());
        let mod_ind=self.current;
        let mod_impls=&mut self.impl_decls[mod_ind];
        if mod_impls.contains(&check_impl_) {
            mod_impls.remove(&check_impl_);
        }else{
            self.errors.push(format!(
                "did not expect impl (in module {:?}):\n\n{:#?}\n\n{}\n\n", 
                mod_ind,
                check_impl_,
                tokens_to_string(impl_)
            ));
        }
    }
    fn visit_item_use(&mut self, use_: &'ast ItemUse){
        let mod_ind= self.current;
        let mod_uses=&mut self.use_decls[mod_ind];
        if mod_uses.remove(use_)==false {
            self.errors.push(format!(
                "did not expect use declaration (in module {:?})  :\n\n{}",
                mod_ind, 
                tokens_to_string(use_),
            ));
        }
    }
}