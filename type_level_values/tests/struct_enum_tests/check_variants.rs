use super::*;

use type_level_values::prelude::*;

use core_extensions::OptionExt;

use shared::utils::{
    totoken_iter_to_string,
    tokens_to_string,
};

use shared::traits::{OptIdent};

use std::iter;
use std::borrow::Cow;
use std::collections::{HashMap,HashSet};

use derive_type_level_lib::submod_visibility::{MyVisibility};
use derive_type_level_lib::parse_syn::*;


use syn::{
    TraitItem,
    Fields,
    Ident,
    ItemUse,
};


#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[allow(dead_code)]
pub(crate) enum VariantKind{
    Braced,
    Tupled,
    Unit,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub(crate) enum Privacy{
    Private,
    Inherited,
}


pub(crate) struct Variants<'a>{
    pub(crate) name:&'a str,
    pub(crate) variants:Vec<Variant<'a>>
}


pub(crate) struct Variant<'a>{
    pub(crate) const_value:&'a str,
    pub(crate) dt_trait:&'a str,
    pub(crate) wr_trait:&'a str,
    #[allow(dead_code)]
    pub(crate) kind:VariantKind,
    pub(crate) fields:Vec< Field<'a> >,
}


pub(crate) struct Field<'a>{
    pub(crate) attributes:Cow<'a,str>,
    pub(crate) privacy:Privacy,
    pub(crate) name:Cow<'a,str>,
    pub(crate) trait_base:Cow<'a,str>,
    pub(crate) accessor:Cow<'a,str>,
    pub(crate) accessor_kind:AccessorKind,
    pub(crate) bound:Option<&'a str>,
    pub(crate) bound_runt:Option<&'a str>,
    pub(crate) pub_assoc_ty:bool,
    pub(crate) visibility:&'a str,
}

impl<'a> Field<'a>{
    pub(crate) fn named(
        attributes:&'a str,
        privacy:Privacy,
        name:&'a str,
        visibility:&'a str,
    )->Self{
        Self{
            attributes:attributes.into(),
            privacy,
            name:name.into(),
            trait_base:name.into(),
            accessor  :name.into(),
            accessor_kind:AccessorKind::Struct,
            bound:None,
            bound_runt:None,
            pub_assoc_ty:false,
            visibility,
        }
    }
    pub(crate) fn positional(
        attributes:&'a str,
        privacy:Privacy,
        name:&'a str,
        visibility:&'a str,
    )->Self{
        let (acc,ak)=match privacy {
            Privacy::Inherited=>(format!("U{}"     ,name),AccessorKind::Integer),
            Privacy::Private  =>(format!("field_{}",name),AccessorKind::Struct),
        };
        Self{
            attributes:attributes.into(),
            privacy,
            name:name.into(),
            trait_base:format!("field_{}",name).into(),
            accessor:acc.into(),
            accessor_kind:ak,
            bound:None,
            bound_runt:None,
            pub_assoc_ty:false,
            visibility,
        }
    }
    pub(crate) fn ren_acc<S>(
        attributes:&'a str,
        privacy:Privacy,
        name:&'a str,
        rename:S,
        visibility:&'a str,
    )->Self
    where S:Into<Cow<'a,str>>,
    {
        let rename=rename.into();
        Field{
            attributes:attributes.into(),
            privacy,
            name:name.into(),
            trait_base:rename.clone(),
            accessor:rename.clone(),
            accessor_kind:AccessorKind::Struct,
            bound:None,
            bound_runt:None,
            pub_assoc_ty:false,
            visibility,
        }
    }

    pub(crate) fn assoc_ty_privacy(&self)->Privacy{
        if self.pub_assoc_ty {
            Privacy::Inherited
        }else{
            self.privacy
        }
    }
}

#[derive(Copy,Clone,PartialEq,Eq)]
pub(crate) enum AccessorKind{
    Struct,
    Integer,
}


pub(crate) static SHARED_FIELD_ATTR:&str=r###"
    # [ derive ( Clone , Copy ) ]
    # [ doc = r" This is the accessor for the field of the same name." ]
"###;

pub(crate) static FIELD_ALL_ATTR:&str=r###"
    # [ doc = r" This is the accessor for all the fields." ]
    # [ derive ( Clone , Copy ) ]
"###;

pub(crate) static PUB_DSUPER:&str="pub(in super::super)";



pub(crate) fn test_reexport(
    variants:&Variants,
    ctokens:&CommonTokens,
    reexported_dunder:&[&str],
    reexported:&[&str],
    derive_str:&str,
){
    let accessor_structs:HashMap<Ident,Vec<syn::Attribute>>=variants.variants.iter()
        .flat_map(|x|&x.fields)
        .filter(|f| f.accessor_kind==AccessorKind::Struct )
        .map(|f| (parse_ident(&f.accessor),&*f.attributes) )
        .chain( iter::once( (parse_ident("All"),FIELD_ALL_ATTR) ) )
        .map(|(a,b)| (a,parse_syn_attributes(b)) )
        .collect();
    let mut accessor_structs=Some(accessor_structs);

    let reexported_dunder:Vec<_>=reexported_dunder.iter().map(|x| parse_syn_use(x) ).collect();
    let reexported:Vec<_>=reexported.iter().map(|x| parse_syn_use(x) ).collect();
    
    let type_level_mod=format!("type_level_{}",variants.name);

    let mut errors=Vec::new();

    for variant in &variants.variants {

        match test_reexport_inner(
            &type_level_mod,
            variant,
            ctokens,
            accessor_structs.take(),
            &reexported_dunder,
            &reexported,
            derive_str,
        ){
            Err(e)=>{
                errors.push(format!("\nIn variant:{}\n",variant.const_value));
                errors.extend(e);
            }
            Ok(())=>{}
        }
    }

    if errors.is_empty() {
        return;
    }
    
    let mut buffer=String::new();
    for error in errors {
        #[allow(unused_imports)]
        use std::fmt::Write;
        writeln!(
            buffer,
            "{S}{S}{S}{S}\n{}\n{S}{S}{S}{S}",
            error,
            S="--------------------" 
        );
    }
    panic!("{}",buffer);
}


fn test_reexport_inner<'a>(
    type_level_mod:&str,
    variant:&Variant<'a>,
    ctokens:&CommonTokens,
    mut accessor_structs:Option<HashMap<Ident,Vec<syn::Attribute>>>,
    reexported_dunder:&[ItemUse],
    reexported:&[ItemUse],
    derive_str:&str,
)->Result<(),Vec<String>>{
    let pub_vis=parse_visibility("pub");
    let pub_vis=MyVisibility::new(&pub_vis,ctokens);
    
    let mut reexported_dunder:HashSet<_>=reexported_dunder.iter().collect();
    let mut reexported       :HashSet<_>=reexported       .iter().collect();

    let fields=&variant.fields;

    let mut trait_tys_map:HashMap<Ident,&Field>=Default::default();
    let mut wr_tys_map   :HashMap<Ident,&Field>=Default::default();

    for field in fields {
        let trait_name=match field.assoc_ty_privacy() {
            Privacy::Inherited=>"",
            Privacy::Private=>"priv_",
        }.piped(|x| format!("{}{}",x,field.trait_base) );

        let rt_name=format!("rt_{}",trait_name);

        trait_tys_map.insert( parse_ident(&trait_name    ) , field);
        wr_tys_map   .insert( parse_ident(&rt_name       ) , field);
    }

    let mut errors=Vec::new();
    let mut visited_const_value=false;
    let mut visited_dt_trait=false;
    let mut visited_wr_trait=false;

    let mut visiting=Visiting::new(ctokens,&type_level_mod);

    visiting.check_derive(derive_str,|mod_ind,item|{
        let x=match mod_ind {
            ModIndex::DunderFieldMod=>Some(&mut reexported_dunder),
            ModIndex::FieldsMod     =>Some(&mut reexported),
            ModIndex::TypeLevelMod  =>None,
             ModIndex::DerivingMod
            |ModIndex::VariantsMod
            |ModIndex::PrivateMod
            =>return Ok(())
        };

        match (x,item) {
            (Some(reexports),VisitItem::Use(use_))=>{
                if !reexports.remove(use_) {
                    let s=format!(
                        "{}\n\nRemaining Items:{}",
                        tokens_to_string(use_),
                        totoken_iter_to_string(&*reexports)
                    );
                    return Err(VIError::new(VIErrorKind::UnexpectedItem,s))
                }
            }
            (Some(_),VisitItem::Struct(struct_))=>{
                let accessor_structs=match accessor_structs.as_mut() {
                    Some(x)=>x,
                    None=>return Ok(()) ,
                };
                if pub_vis!=MyVisibility::new(&struct_.vis,ctokens){
                    format!(
                        "visibility of {} is '{}' instead of 'pub'",
                        struct_.ident,
                        tokens_to_string(&struct_.vis),
                    );
                }
                match accessor_structs.remove( &struct_.ident ) {
                    Some(attrs)=>
                        if attrs!=struct_.attrs {
                            errors.push(format!(
                                "accessor struct '{}' has unexpected attributes:\
                                 \n{:#?}\nexpected:\n{:#?}",
                                struct_.ident,
                                struct_.attrs,
                                attrs
                            ));
                        }
                    None=>{
                        errors.push(format!(
                            "accessor struct '{}' not in the list of accessor structs:{:#?}",
                            struct_.ident,
                            accessor_structs.keys().collect::<Vec<_>>()
                        ));
                    }
                }
            }
            (Some(ref reexports),VisitItem::EndOfMod) if !reexports.is_empty() =>{
                return Err(VIError::new(
                    VIErrorKind::ExpectedMoreItems,
                    format!("expected item reexports:\n{}",totoken_iter_to_string(&**reexports))
                ));
            }
            (None,VisitItem::Struct(struct_))=> {
                if struct_.ident!=variant.const_value {
                    return Ok(());
                }
                visited_const_value=true;

                let s_fields=match struct_.fields {
                    Fields::Named(ref fields)=>Some(&fields.named),
                    Fields::Unnamed(ref fields)=>Some(&fields.unnamed),
                    Fields::Unit=>None
                };

                for (i,(s_field,field)) in
                    s_fields.into_iter().flat_map(|x|x).zip(fields).enumerate() 
                {
                    let same_field=match s_field.ident.as_ref() {
                        Some(fieldname)=>fieldname == &*field.name,
                        None=> field.name.parse().unwrap_or(!0usize)==i,
                    };

                    if !same_field {
                        errors.push(format!(
                            "expected field {}.{} found field {}",
                            struct_.ident ,field.name ,s_field.ident.or_index(i)
                        ));
                    }

                    let expected_vis =parse_visibility(field.visibility);

                    if  expected_vis!= s_field.vis {
                        errors.push(format!(
                            "visibility of {}.{} is {} when it should be {}",
                            struct_.ident ,s_field.ident.or_index(i) ,
                            tokens_to_string(&s_field.vis) ,
                            tokens_to_string(expected_vis)
                        ));
                    }
                }
            }
            (None,VisitItem::Trait(trait_))=> {
                #[derive(Copy,Clone,PartialEq,Eq)]
                enum VisitingTrait{
                    VT_Trait,
                    VT_WithRuntime,
                }

                let (visiting_trait,assoc_item_map)=if trait_.ident==variant.dt_trait{
                    visited_dt_trait=true;
                    (VisitingTrait::VT_Trait      ,&mut trait_tys_map)
                }else if trait_.ident==variant.wr_trait {
                    visited_wr_trait=true;
                    (VisitingTrait::VT_WithRuntime,&mut wr_tys_map)
                }else{
                    return Ok(());
                };
                for item in &trait_.items {
                    let assoc_ty=match *item {
                        TraitItem::Type(ref assoc_ty)=>assoc_ty,
                        _=>continue,
                    };
                    let field=match assoc_item_map.get(&assoc_ty.ident) {
                        Some(x)=>{ x }
                        None=>{
                            errors.push(format!(
                                "{}::{} doesn't map to a field.\n",
                                trait_.ident,assoc_ty.ident
                            ));
                            continue;
                        }
                    };
                    let bounds=match visiting_trait {
                        VisitingTrait::VT_Trait=>field.bound,
                        VisitingTrait::VT_WithRuntime=>field.bound_runt,
                    }.map(|x| parse_bounds(x) );

                    if let Some(bounds)=bounds.filter_(|b| *b!=assoc_ty.bounds) {
                        errors.push(format!(
                            "Bounds of {}::{} are:\n{}\nexpecting:\n{}\n",
                            trait_.ident,assoc_ty.ident,
                            tokens_to_string(&assoc_ty.bounds),
                            tokens_to_string(&bounds)
                        ));
                    }

                    let has_doc_hidden_attr=assoc_ty.attrs.iter()
                        .any(|s| tokens_to_string(s)==ctokens.doc_hidden.to_string() );

                    if has_doc_hidden_attr != ( field.assoc_ty_privacy()==Privacy::Private ) {
                        errors.push(format!(
                            "Privacy of {}::{} is {:?} when {:?} was expected \n",
                            trait_.ident,assoc_ty.ident,
                            field.assoc_ty_privacy(),
                            if has_doc_hidden_attr {Privacy::Private}else{Privacy::Inherited} ,
                        ));
                    }
                }
            }
            _=>{}
        };
        Ok(())
    });

    if !visited_const_value {
        errors.push(format!("did not find the {} struct.",variant.const_value));
    }
    if !visited_dt_trait {
        errors.push(format!("did not find the {} trait.",variant.dt_trait));
    }
    if !visited_wr_trait {
        errors.push(format!("did not find the {} trait.",variant.wr_trait));
    }
    if let Some(accessor_structs)=accessor_structs{
        if !accessor_structs.is_empty() {
            errors.push(format!(
                "Did not define these accessor structs:\n{:#?}.",
                accessor_structs
            ));
        }
    }

    if errors.is_empty(){
        Ok(())
    }else{
        Err(errors)
    }

}