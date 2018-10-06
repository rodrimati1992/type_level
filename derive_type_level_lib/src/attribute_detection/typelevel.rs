use super::item_metadata::ItemMetaData;
use super::my_meta::{MyMeta, MyNested};
use super::shared::{
    NotUpdated,
    UpdateWithMeta,
    ident_from_nested,
    foreach_nestedmeta_index,
    bounds_from_str,
    parse_syn_path,
    parse_ident,
    parse_visibility,
};

use super::indexable_struct::GetEnumIndices;

use attribute_errors::typelevel as attribute_errors;

use ArenasRef;

use arrayvec::ArrayString;


use core_extensions::*;
#[allow(unused_imports)]
use ::void_like::VoidLike as UsedVoid;
// #[allow(unused_imports)]
// use core_extensions::Void as UsedVoid;
use ::*;

use syn::punctuated::Punctuated;
use syn::token::{Comma,Add};
use syn::{
    self, Attribute, Ident, Meta, MetaList, NestedMeta, Path as SynPath, Visibility,TypeParamBound,
};

use std::marker::PhantomData;
use std::str::FromStr;

////////////////////////////////////////////////////////////////////////////

pub(crate) type ImplMetaData<'a> = ItemMetaData<'a,ImplVariant<'a>>;

////////////////////////////////////////////////////////////////////////////

/// Settings derived from a sequence of annotations on an item/variant/field.
#[derive(Debug, Clone, Default)]
pub(crate) struct TLAttributes<'a> {
    pub(crate) renames: Renames<'a>,
    pub(crate) reexports:ReExportCfg<'a>,
    /// Traits derived for the generated item.
    pub(crate) derived: DerivedTraits<'a>,
    /// The derives applied to the generated item that don't come built into this macro.
    pub(crate) additional_derives: Vec<Ident>,
    pub(crate) attrs:ItemMetaData<'a,()>,
    pub(crate) print_derive: bool,
    pub(crate) skip_derive: bool,
    pub(crate) print_debug: bool,
    pub(crate) print_attributes:bool,
    pub(crate) _marker: PhantomData<&'a ()>,
}

////////////////////////////////////////////////////////////////////////////

#[derive(Debug,Clone,Default)]
pub(crate) struct ReExportCfg<'a>{
    pub(crate) reexported:ReExports,
    pub(crate) visibility:ReExportVis<'a>,
}

macro_rules! declare_reexports {
    (   
        $( ($items:ident,$index:ident) ),* $(,)* 
        multi_indices=[ $($multi_indices:tt)* ]
    ) => (
        declare_indexable_struct!{
            enum index=ReExportIndex
            struct indexable=ReExports__
            variants=[ $( ( $items , $index ) ),* ]
            
            multi_indices=[ $($multi_indices)* ]
        }

        pub(crate) type ReExports=
            ReExports__<bool>;

        impl Default for ReExports{
            fn default()->Self{
                Self::none_reexported()
            }
        }

        impl ReExports{
            pub(crate) fn all_reexported()->Self{
                Self{ $( $items : true, )* }
            }
            pub(crate) fn none_reexported()->Self{
                Self{ $( $items : false, )* }
            }
        }
    )
}


declare_reexports!{
    (traits       ,Traits       ),
    (variants     ,Variants     ),
    (discriminants,Discriminants),
    (fields       ,Fields       ),

    multi_indices=[
        ("Struct",[Variants])
    ]
    
}

////////////////////////////////////////////////////////////////////////////

/// Determines whether and how the items inside of
/// the generated module are re-exported outside of it.
#[derive(Debug,Copy, Clone)]
pub(crate) enum ReExportVis<'a> {
    NoReexport,
    /// The visibility of the type being annotated
    WithDeriveVis,
    WithVis(&'a Visibility),
}

impl<'a> Default for ReExportVis<'a> {
    fn default() -> Self {
        ReExportVis::NoReexport
    }
}


////////////////////////////////////////////////////////////////////////////


macro_rules! derived_traits {
    (
        variants=[
            $( ($field:ident,$index:ident,$default_impl:expr $(,)* ) ),*
            $(,)*
        ]

        multi_indices=[
            $( ($mul_ind_name:expr , [ $($index_alias:ident),* $(,)* ]) ),*
            $(,)*
        ]

    ) => {

        declare_indexable_struct!{
            enum index=ImplIndex
            struct indexable=TraitImpls
            variants=[ $( ( $field , $index ) ),* ]
            multi_indices=[$(
                ( $mul_ind_name ,  [ $( $index_alias ),* ] ),
            )*]
        }

        /// The traits derived on a type/enum-variant
        pub(crate) type DerivedTraits<'a>=
            TraitImpls<ImplMetaData<'a>>;

        impl<'a> Default for DerivedTraits<'a>{
            fn default()->Self{
                Self{
                    $( $field :ImplMetaData::new($default_impl), )*
                }
            }
        }

    }
}

derived_traits!{
    variants=[
        (const_eq        ,ConstEq        ,ImplVariant::Unspecified(&UNSPEC_NO_IMPLS)     ),
        (const_ord       ,ConstOrd       ,ImplVariant::Unspecified(&UNSPEC_NO_IMPLS)     ),
        (get_discriminant,GetDiscriminant,ImplVariant::Unspecified(&UNSPEC_DEFAULT_IMPLS)),
        (into_consttype  ,IntoConstType  ,ImplVariant::Unspecified(&UNSPEC_DEFAULT_IMPLS)),
        (into_runtime    ,IntoRuntime    ,ImplVariant::Unspecified(&UNSPEC_DEFAULT_IMPLS)),
        (as_t_list       ,AsTList        ,ImplVariant::Unspecified(&UNSPEC_DEFAULT_IMPLS)),
    ]

    multi_indices=[
        ("runtime_conv",[IntoConstType, IntoRuntime])
    ]
}

////////////////////////////////////////////////////////////////////////////

/// Determines whether and how an impl is derived.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ImplVariant<
    'a, 
    Priv = &'static ImplVariant<'static, UsedVoid,UsedVoid> , 
    Path:'a=&'a SynPath 
> 
{
    Unspecified(Priv),
    NoImpls,
    DefaultImpls,
    Internal { type_:Path,manual:bool,_marker:PhantomData<&'a ()>},
}

static UNSPEC_NO_IMPLS:ImplVariant<UsedVoid, UsedVoid>= 
    ImplVariant::NoImpls;

static UNSPEC_DEFAULT_IMPLS:ImplVariant<UsedVoid, UsedVoid>= 
    ImplVariant::DefaultImpls;



pub(crate) trait ImplVariantMethods{
    /// Whether the trait is derived or not.Being derived implies being implemented.
    fn is_derived(self)->bool;
}


impl<'a> ImplVariant<'a,UsedVoid,UsedVoid> {
    fn void_to<A,B>(self)->ImplVariant<'a,A,B>{
        match self {
            ImplVariant::Unspecified{..} => unreachable!(),
            ImplVariant::NoImpls => ImplVariant::NoImpls,
            ImplVariant::DefaultImpls => ImplVariant::DefaultImpls,
            ImplVariant::Internal {..} => unreachable!(),
        }
    }
}


impl<'a,Priv> ImplVariantMethods for  ImplVariant<'a,Priv> 
where Priv:ImplVariantMethods
{
    fn is_derived(self) -> bool {
        match self {
            ImplVariant::Unspecified(v) => v.is_derived(),
            ImplVariant::NoImpls => false,
            ImplVariant::DefaultImpls => true,
            ImplVariant::Internal { manual , ..} => !manual,
        }
    }
}


impl ImplVariantMethods for UsedVoid {
    fn is_derived(self)->bool{ false }
}


impl<'a> ImplVariant<'a> {
    pub(crate) fn specified_or(self, or: ImplVariant<'a>) -> Self {
        match self {
            ImplVariant::Unspecified(_) => or,
            this => this,
        }
    }

    pub(crate) fn is_implemented(self) -> bool {
        self.to_specified() != ImplVariant::NoImpls
    }

    pub(crate) fn to_specified(self) -> ImplVariant<'a, UsedVoid,&'a SynPath> {
        match self {
            ImplVariant::Unspecified(v) =>  {
                let ret:ImplVariant<'a, UsedVoid,&'a SynPath>=v.void_to();
                ret
            }
            ImplVariant::NoImpls => ImplVariant::NoImpls,
            ImplVariant::DefaultImpls => ImplVariant::DefaultImpls,
            ImplVariant::Internal { type_,manual,_marker } => 
                ImplVariant::Internal { type_,manual,_marker },
        }
    }
}

impl<'a> Default for ImplVariant<'a> {
    fn default() -> Self {
        ImplVariant::Unspecified(&UNSPEC_NO_IMPLS)
    }
}

impl<'ar> UpdateWithMeta<'ar> for ImplVariant<'ar> {
    fn update_with_meta(
        &mut self, meta: &MyMeta<'ar>, arenas: ArenasRef<'ar>
    ) -> Result<(), NotUpdated> {
        let new_type=|str_| arenas.paths.alloc(parse_syn_path(str_)) ;

        *self = match (&*meta.word.str, &meta.value) {
            ("NoImpls", _) => ImplVariant::NoImpls,
            ("DefaultImpls", _) => ImplVariant::DefaultImpls,
            ("Internal", _) => {
                let mut type_=None::<&'ar SynPath> ;
                let mut manual=false;
                
                if let MyNested::Value(str_)=meta.value {
                    type_ = Some(new_type(str_));
                }else if let Some(list)=meta.value.to_mylist(arenas) {
                    for nested in &*list {
                        match (&*nested.word.str, &nested.value) {
                            ("Type", &MyNested::Value(str_))=>{
                                type_ = Some(new_type(str_));
                            }
                            ("Manual",_)=>{
                                manual=true;
                            }
                            (attr_,_)=>panic!(
                                " inside {},attribute {} not recognized,\
                                 must be either type_ or manual", 
                                meta.word.str,
                                attr_
                            )
                        }
                    }
                }
                let type_=type_.unwrap_or_else(||{
                    panic!("must specify the type for Internal derive.");
                });

                ImplVariant::Internal{type_,manual,_marker:PhantomData}
            }
            (_, _) => return Err(NotUpdated),
        };
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, Clone)]
pub(crate) struct Renames<'a> {
    /// The struct representing either an enum variant or a struct.
    pub(crate) variant_type: Option<&'a Ident>,
    
    /// The trait used to access the enum variant/struct in generic contexts
    pub(crate) trait_  : Option<&'a Ident>,

    /// The trait used to access the enum variant/struct in generic contexts,
    /// that can have bounds mentioning for every field mentioning the runtime type .
    pub(crate) wr_trait: Option<&'a Ident>,

    /// The ConstType.
    pub(crate) const_type: Option<&'a Ident>,
}

////////////////////////////////////////////////////////////////////////////

impl<'a> TLAttributes<'a> {
    pub(crate) fn new(attrs: &'a [Attribute], arenas: ArenasRef<'a>) -> Self {
        let mut settings = TLAttributes::default();
        for attr in attrs {
            attr_settings_new_attr(attr, &mut settings, arenas);
        }
        settings
    }
}

////////////////////////////////////////////////////////////////////////////

fn attr_settings_new_attr<'alloc>(
    attr: &'alloc Attribute,
    settings: &mut TLAttributes<'alloc>,
    arenas: ArenasRef<'alloc>,
) {
    let meta_list:&'alloc MetaList = match attr.interpret_meta() {
        Some(Meta::List(meta_list)) => arenas.metalists.alloc(meta_list),
        _ => return,
    };

    if meta_list.ident == "typelevel" {
        for nested0 in &meta_list.nested {
            let nested0: MyMeta = nested0.into_with(arenas);
            
            if let Ok(_)=settings.attrs.update_with_meta(&nested0,arenas) {
                continue;
            }

            let value=&nested0.value;

            match &*nested0.word.str {
                "skip_derive" => {
                    settings.skip_derive = true;
                    return;
                }
                "print_derive" => {
                    settings.print_derive = true;
                }
                "print_debug" => {
                    settings.print_debug = true;
                }
                "print_attributes" => {
                    settings.print_attributes = true;
                }
                "reexport" => {
                    reexport_attribute(
                        value,
                        &mut settings.reexports,
                        arenas,
                    );
                }
                "rename" => {
                    settings.renames.variant_type = Some(ident_from_nested(&value,arenas));
                }
                "rename_trait" => {
                    settings.renames.trait_ = Some(ident_from_nested(&value,arenas));
                }
                "rename_wr_trait" => {
                    settings.renames.wr_trait = Some(ident_from_nested(&value,arenas));
                }
                "rename_consttype" => {
                    settings.renames.const_type = Some(ident_from_nested(&value,arenas));
                }
                _ => {
                    if let &MyNested::List(ref list) = value {
                        new_attr_nested_meta(attr, settings, &nested0.word.str, list, arenas);
                    } else {
                        panic!("\
                            attribute 'typelevel({})' not recognized.{}", 
                            nested0.word,
                            attribute_errors::type_attrs());
                    }
                }
            }
        }
    }
}



fn new_attr_nested_meta<'alloc>(
    _attr: &'alloc Attribute,
    settings: &mut TLAttributes<'alloc>,
    word: &str,
    list: &'alloc Punctuated<NestedMeta, Comma>,
    arenas: ArenasRef<'alloc>,
) {
    match word {
        "derive" => {
            let derived = &mut settings.derived;
            let additional_derives = &mut settings.additional_derives;
            foreach_nestedmeta_index(
                list,
                arenas,
                move |_, impl_index| {
                    let variant = &mut derived[impl_index].inner;
                    *variant = variant.specified_or(ImplVariant::DefaultImpls);
                },
                move |_,word| {
                    additional_derives.push(parse_ident(&word.0));
                },
            );
        }
        "items" => foreach_nestedmeta_index(
            list,
            arenas,
            move |value, impl_index| {
                value.list_to_mylist(arenas).unwrap();

                let impl_ = &mut settings.derived[impl_index];
                impl_.inner = impl_.inner.specified_or(ImplVariant::DefaultImpls);

                if let MyNested::MyList(ref list_1) = *value {
                    for param in list_1 {
                        impl_
                            .update_with_meta(param, arenas)
                            .unwrap_or_else(|_|{
                                panic!("Invalid parameter:{:#?} {}", 
                                    param,
                                    attribute_errors::item_attrs()
                                )
                            });
                    }
                }
            },
            |_,e| panic!("\n\nnot valid inside items( ... ):'{}'\n\nMust be one of:{}\n\n", 
                e.0,
                ImplIndex::indices_message()
            ),
        ),
        word => panic!("Unsupported nested attribute:{:#?}{}", 
            word,
            attribute_errors::type_attrs()
        ),
    }
}


fn reexport_attribute<'alloc>(
    value:&MyNested<'alloc>,
    reexports:&mut ReExportCfg<'alloc>,
    arenas:ArenasRef<'alloc>,
){
    let r_reexported=&mut reexports.reexported;
    let r_visibility=&mut reexports.visibility;

    match value {
        &MyNested::Word | &MyNested::Value{..} => {
            let previous_vis=*r_visibility;

            *r_visibility=match value {
                &MyNested::Word=>
                    ReExportVis::WithDeriveVis,
                &MyNested::Value(ref val) => 
                    parse_visibility(&val)
                        .piped(|v| &*arenas.visibilities.alloc(v) )
                        .piped(ReExportVis::WithVis),
                _=>return,
            };

            if let ReExportVis::NoReexport= previous_vis {
                *r_reexported=ReExports::all_reexported();
            }

        }
        &MyNested::List (ref list) => {
            if let ReExportVis::NoReexport=*r_visibility {
                *r_visibility=ReExportVis::WithDeriveVis;
                *r_reexported=ReExports::none_reexported();
            }

            foreach_nestedmeta_index(
                list,
                arenas,
                |_,index| r_reexported[index]=true,
                |value,word|{
                    let value=&*value;
                    match (word.0,value) {
                        ( "Visibility" , &MyNested::Value(ref val) )=> {
                            let vis=arenas.visibilities.alloc(parse_visibility(&val));
                            *r_visibility = ReExportVis::WithVis(vis);
                        }
                        _=>panic!(
                            "inside reexports(...):\
                             subattribute not supported:{} {:?}\
                             {}",
                            word.0,
                            value,
                            attribute_errors::reexport()
                        ),
                    }
                }
            );
        }
        &MyNested::MyList { .. }=>
            panic!("unsupported mynested variant here:{:#?}",value),
    }
}



/////////////////////////////////////////////////////////////////////////////////


#[derive(Default)]
pub(crate) struct FieldAttrs<'a> {
    /// Renames field on Some.
    pub(crate) rename: Option<&'a Ident>,
    /// Renames the accessor struct on Some.
    pub(crate) accessor: Option<&'a Ident>,
    /// the bounds for the field in the <Type>Trait trait.
    pub(crate) const_bound:Punctuated<TypeParamBound, Add>,
    /// the bounds for the field in the <Type>IntoRuntime trait.
    pub(crate) runt_bound:Punctuated<TypeParamBound, Add>,
    pub(crate) pub_trait_accessor:bool,
    pub(crate) docs:Vec<&'a str>,
}

impl<'a> FieldAttrs<'a> {
    pub(crate) fn new(attrs: &'a [Attribute], arenas: ArenasRef<'a>) -> Self {
        let mut settings = FieldAttrs::default();
        for attr in attrs {
            field_attrs_helper(attr, &mut settings, arenas);
        }
        settings
    }
}

fn field_attrs_helper<'a>(
    attr: &'a Attribute, 
    settings: &mut FieldAttrs<'a>, 
    arenas: ArenasRef<'a>,
) {
    let meta_list :&'a syn::MetaList = match attr.interpret_meta() {
        Some(Meta::List(meta_list)) => arenas.metalists.alloc(meta_list),
        _ => return,
    };

    if meta_list.ident == "typelevel" {
        for nested0 in &meta_list.nested {
            let nested0: MyMeta = nested0.into_with(arenas);
            let word = &*nested0.word.str;
            let value = nested0.value;
            match word {
                "rename" => {
                    settings.rename = Some(ident_from_nested(&value,arenas));
                }
                "accessor" => {
                    settings.accessor = Some(ident_from_nested(&value,arenas));
                }
                "bound"|"bound_runt" => {
                    let str_=match value{
                        MyNested::Value(str_)=>str_,
                        v=>panic!("expected string literal found:{:#?}", v)
                    };

                    let bounds=match word {
                        "bound"=>&mut settings.const_bound,
                        "bound_runt"=>&mut settings.runt_bound,
                        _=>unreachable!(),
                    };

                    bounds_from_str(str_,bounds);
                }
                "pub_trait_accessor"=>{
                    settings.pub_trait_accessor=true;
                }
                "doc"=>{
                    match value {
                        MyNested::Value(str_)=>settings.docs.push(str_),
                        e=>panic!("\
                            doc subattribute expected string literal,instead found:{:#?}\
                        ",e)
                    }
                }
                word => {
                    panic!("Unsupported nested attribute:{:#?}{}", 
                        word,
                        attribute_errors::field_attrs()
                    );
                }
            }
        }
    }
}

