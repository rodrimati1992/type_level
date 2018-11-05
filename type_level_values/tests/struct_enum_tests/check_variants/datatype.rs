use super::*;


pub(crate) use super::item_check::{
    ToItemCheck,
    ItemToCheck,
};

use super::typelevel_field::Field;


///
/// The I parameter is the module index enum.
pub(crate) struct DataType<'a,I>{
    pub(crate) name:&'a str,
    pub(crate) variants:Variants<'a>,
    pub(crate) item_checks:HashMap<ItemKey,ItemCheck<()>>,
    pub(crate) reexported:HashMap<I,HashSet<ItemUse>>,
    pub(crate) modules:Rc<Module<I>>,
}


impl<'a,I> DataType<'a,I>
where I:ModIndex
{
    fn priv_default(modules:Rc<Module<I>>)->Self{
        Self{
            name:"",
            variants:Variants::no_checking(),
            item_checks:Default::default(),
            reexported :Default::default(),
            modules,
        }
    }
    pub fn new<M>(name:&'a str,modules:M,variants:Variants<'a>)->Self
    where M:Into<Rc<Module<I>>>,
    {
        let mut this=Self::priv_default( modules.into() );
        this.name=name;
        this.variants=variants;
        this
    }
    pub fn add_tl_variant(mut self,variant:TLVariant<'a>)->Self{
        match &mut self.variants {
            &mut Variants::TypeLevel(ref mut tl)=>{
                tl.list.push(variant);
            }
            x=>panic!("attempting to add typelevel variant to a {} variant", x.variant()),
        }
        self
    }
    pub fn add_check<IC>(mut self,item:IC)->Self
    where IC:ToItemCheck
    {
        {
            let (key,val)=item.to_item_check().split_key();
            if let Some(_)=self.item_checks.insert(key.clone(),val) {
                panic!("\n\nAttempting to insert the same item twice:\n\t{}\n\n",key);
            }
        }
        self
    }
    
    pub fn add_reexport<S>(mut self,index:I,use_:S)->Self
    where S:AsRef<str>
    {
        self.reexported
            .entry(index)
            .or_insert_with(Default::default)
            .insert(parse_syn_use(use_.as_ref()));
        self
    }
    pub fn add_reexports<II>(mut self,index:I,reexports:II)->Self
    where 
        II:IntoIterator,
        II::Item:AsRef<str>
    {
        for reexp in reexports {
            self=self.add_reexport(index.clone(),reexp);
        }
        self
    }
}


pub(crate) enum Variants<'a>{
    NoChecking,
    TypeLevel(TLVariants<'a>)
}

impl<'a> Variants<'a>{
    pub fn no_checking()->Self{
        Variants::NoChecking
    }
    pub fn typelevel()->Self{
        Variants::TypeLevel(TLVariants{
            list:Vec::new(),
        })
    }

    pub fn variant(&self)->&'static str{
        match *self {
            Variants::NoChecking=>"no checking",
            Variants::TypeLevel{..}=>"type level",
        }
    }
}

pub(crate) struct TLVariants<'a>{
    list:Vec<TLVariant<'a>>,
}

pub(crate) struct TLVariant<'a>{
    pub(crate) const_value:&'a str,
    pub(crate) dt_trait:&'a str,
    pub(crate) wr_trait:&'a str,
    #[allow(dead_code)]
    pub(crate) kind:VariantKind,
    pub(crate) fields:Option<Vec< Field<'a> >>,

}


pub(crate) static SHARED_FIELD_ATTR:&str=r###"
    # [ derive ( Clone , Copy ) ]
    # [ doc = r" This is the accessor for the field of the same name." ]
"###;

pub(crate) static FIELD_ALL_ATTR:&str=r###"
    # [ doc = r" This is the accessor for all the fields." ]
    # [ derive ( Clone , Copy ) ]
"###;



pub(crate) fn test_items<'a,I>(
    mut variants:DataType<'a,I>,
    ctokens:&CommonTokens,
    derive_str:&str,
)
where 
    I:ModIndex
{
    let mut accessor_exhaus;

    let accessor_structs:HashMap<Ident,Vec<syn::Attribute>>=match variants.variants {
        Variants::TypeLevel(ref tl)=>{
            accessor_exhaus=Exhaustiveness::Exhaustive;
            tl.list.iter()
                .flat_map(|variant|{
                    if variant.fields.is_none() {
                        accessor_exhaus=Exhaustiveness::Inexhaustive;
                    }
                    variant.fields.as_ref().map_or_else(empty_slice,|x|x)
                })
                .filter_map(|f|{
                    (f.accessor_kind==AccessorKind::Struct)
                        .if_true(|| (parse_ident(&f.accessor),&*f.attributes) )
                })
                .chain( iter::once( (parse_ident("All"),FIELD_ALL_ATTR) ) )
                .map(|(a,b)| (a,parse_syn_attributes(b)) )
                .collect()
        }
        _=>{
            accessor_exhaus=Exhaustiveness::Inexhaustive;
            HashMap::default()
        },
    };

    
    test_non_variants(
        ctokens,
        variants.modules.clone(),
        variants.item_checks,
        accessor_exhaus,
        accessor_structs,
        variants.reexported,
        derive_str,
    );


    let type_level_mod=format!("type_level_{}",variants.name);

    match variants.variants {
        Variants::TypeLevel(ref tl)=>{
            for variant in &tl.list {
                println!("\nIn variant:{}\n",variant.const_value);

                test_items_typelevel_variant(
                    ctokens,
                    variant,
                    variants.modules.clone(),
                    derive_str,
                );
            }
        }
        Variants::NoChecking=>{},
    };

}


fn test_non_variants<'a,I>(
    ctokens:&CommonTokens,
    modules:Rc<Module<I>>,
    mut item_checks:HashMap<ItemKey,ItemCheck<()>>,
    accessor_exhaus:Exhaustiveness,
    mut accessor_structs:HashMap<Ident,Vec<syn::Attribute>>,
    mut reexported:HashMap<I,HashSet<ItemUse>>,
    derive_str:&str,
)
where 
    I:ModIndex
{
    let pub_vis=parse_visibility("pub");
    let pub_vis=MyVisibility::new(&pub_vis,ctokens);

    let mut visiting=Visiting::new(modules);

    visiting.check_derive(derive_str,|params|{
        let x=reexported.get_mut(&params.mod_index);

        if let Some(check)=params.item.item_to_check() {
            let (key,item)=check.split_key();
            
            match item_checks.remove(&key).map(|x| (x.existence,x)  ) {
                Some((Exists,mut e_item))=>{

                    let mut unexp_attrs=Vec::new();
                    for attr in &item.attributes {
                        if !e_item.attributes.remove(attr) {
                            unexp_attrs.push(attr);
                        }
                    }

                    let mut unexp_where_preds=Vec::new();
                    for predicate in &item.where_preds {
                        if !e_item.where_preds.remove(predicate) {
                            unexp_where_preds.push(predicate);
                        }
                    }


                    /////////////



                    if !unexp_attrs.is_empty() && e_item.attributes_exhaus==Exhaustive {
                        params.push_err(
                            VIEK::WrongDefinition,
                            format!(
                                "Unexpected attributes present in definition:\n{}",
                                totoken_iter_to_string(&unexp_attrs)
                            )
                        );
                    }
                    if !unexp_where_preds.is_empty() && e_item.where_preds_exhaus==Exhaustive{
                        params.push_err(
                            VIEK::WrongDefinition,
                            format!(
                                "Unexpected where predicated present in definition:\n{}",
                                totoken_iter_to_string(&unexp_where_preds)
                            )
                        );
                    }

                    if !e_item.attributes.is_empty() {
                        params.push_err(
                            VIEK::WrongDefinition,
                            format!(
                                "Expected more attributes in definition:\n{}",
                                totoken_iter_to_string(&e_item.attributes)
                            )
                        );
                    }

                    if !e_item.where_preds.is_empty() {
                        params.push_err(
                            VIEK::WrongDefinition,
                            format!(
                                "Expected more where predicates in definition:\n{}",
                                totoken_iter_to_string(&e_item.where_preds)
                            )
                        );
                    }

                }
                Some((NotExists,e_item))=>{
                    params.push_err(
                        VIEK::UnexpectedItem,
                        format!("{} must not exist.",key),
                    );
                }
                None=>{
                    println!("unexpected item:\n{}\n",key);
                    // do nothing for now since item_checks are considered NonExhaustive
                }
            }
        }

        match (x,params.item) {
            (Some(reexports),VisitItem::Use(use_))=>{
                if !reexports.remove(use_) {
                    return params.push_err(
                        VIEK::UnexpectedItem,
                        format!(
                            "{}\n\nRemaining Items:{}",
                            tokens_to_string(use_),
                            totoken_iter_to_string(&*reexports)
                        )
                    );
                }
            }
            (Some(_),VisitItem::Struct(struct_))=>{
                match accessor_structs.remove( &struct_.ident ) {
                    Some(attrs)=>{
                        if pub_vis!=MyVisibility::new(&struct_.vis,ctokens){
                            params.push_err(VIEK::WrongDefinition,format!(
                                "visibility is '{}' instead of 'pub'",
                                tokens_to_string(&struct_.vis),
                            ));
                        }
                        if attrs!=struct_.attrs {
                            params.push_err(VIEK::WrongDefinition,format!(
                                "accessor struct has unexpected attributes:\
                                 \n{:#?}\nexpected:\n{:#?}",
                                struct_.attrs,
                                attrs
                            ));
                        }
                    }
                    None if accessor_exhaus==Exhaustiveness::Exhaustive =>{
                        params.push_err(VIEK::UnexpectedItem,format!(
                            "accessor struct not in the list of accessor structs:{:#?}",
                            accessor_structs.keys().collect::<Vec<_>>()
                        ));
                    }
                    None=>{}
                }
            }
            (Some(ref reexports),VisitItem::EndOfMod) if !reexports.is_empty() =>{
                return params.push_err(
                    VIEK::ExpectedMoreItems,
                    format!("expected item reexports:\n{}",totoken_iter_to_string(&**reexports))
                );
            }
            (_,VisitItem::EndOfVisitor)=>{

                if !accessor_structs.is_empty() {
                    params.push_err(
                        VIEK::ExpectedMoreItems,
                        format!(
                            "Did not define these accessor structs:\n{:#?}.",
                            accessor_structs.values()
                                .flat_map(|x|x)
                                .map(|x|display_totokens(x))
                                .collect::<Vec<_>>()
                        )
                    );
                }
            

                item_checks.retain(|_,impl_| impl_.existence==Exists );

                if !item_checks.is_empty() {
                    params.push_err(
                        VIEK::ExpectedMoreItems,
                        format!(
                            "Did not define these impls:\n{:#?}.",
                            item_checks.keys()
                                .map(|x|AlwaysDisplay(x))
                                .collect::<Vec<_>>()
                        )
                    );
                }

            }
            _=>{}
        };
    });

}
fn test_items_typelevel_variant<'a,I>(
    ctokens:&CommonTokens,
    variant:&TLVariant<'a>,
    modules:Rc<Module<I>>,
    derive_str:&str,
)
where 
    I:ModIndex
{
    let pub_vis=parse_visibility("pub");
    let pub_vis=MyVisibility::new(&pub_vis,ctokens);
    
    let fields:Option<_>=variant.fields.as_ref();

    let mut trait_tys_map:HashMap<Ident,&Field>=Default::default();
    let mut wr_tys_map   :HashMap<Ident,&Field>=Default::default();

    if let Some(fields)=fields {
        for field in fields {
            let trait_name=match field.assoc_ty_privacy() {
                Privacy::Inherited=>"",
                Privacy::Private=>"priv_",
            }.piped(|x| format!("{}{}",x,field.trait_base) );

            let rt_name=format!("rt_{}",trait_name);

            trait_tys_map.insert( parse_ident(&trait_name    ) , field);
            wr_tys_map   .insert( parse_ident(&rt_name       ) , field);
        }
    }

    let mut visited_const_value=false;
    let mut visited_dt_trait=false;
    let mut visited_wr_trait=false;

    let mut visiting=Visiting::new(modules);

    visiting.check_derive(derive_str,|params|{
        match params.item {
            VisitItem::Struct(struct_)=> {
                if struct_.ident!=variant.const_value {
                    return;
                }
                visited_const_value=true;

                if let Some(fields)=fields {
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
                            params.push_err(VIEK::WrongDefinition,format!(
                                "expected field {}.{} found field {}",
                                struct_.ident ,field.name ,s_field.ident.or_index(i)
                            ));
                        }

                        let expected_vis =parse_visibility(field.visibility);

                        if  expected_vis!= s_field.vis {
                            params.push_err(VIEK::WrongDefinition,format!(
                                "visibility of {}.{} is {} when it should be {}",
                                struct_.ident ,s_field.ident.or_index(i) ,
                                tokens_to_string(&s_field.vis) ,
                                tokens_to_string(expected_vis)
                            ));
                        }
                    }
                }
            }
            VisitItem::Trait(trait_)=> {
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
                    return;
                };
                for item in &trait_.items {
                    let assoc_ty=match *item {
                        TraitItem::Type(ref assoc_ty)=>assoc_ty,
                        _=>continue,
                    };
                    let field=match assoc_item_map.get(&assoc_ty.ident) {
                        Some(x)=>{ x }
                        None=>{
                            params.push_err(VIEK::WrongDefinition,format!(
                                "{}::{} doesn't map to a field.\n",
                                trait_.ident,assoc_ty.ident
                            ));
                            continue;
                        }
                    };
                    let bounds=match visiting_trait {
                        VisitingTrait::VT_Trait      =>field.bound,
                        VisitingTrait::VT_WithRuntime=>field.bound_runt,
                    }.map(|x| parse_bounds(x) );

                    if let Some(bounds)=bounds.filter_(|b| *b!=assoc_ty.bounds) {
                        params.push_err(VIEK::WrongDefinition,format!(
                            "Bounds of {}::{} are:\n{}\nexpecting:\n{}\n",
                            trait_.ident,assoc_ty.ident,
                            tokens_to_string(&assoc_ty.bounds),
                            tokens_to_string(&bounds)
                        ));
                    }

                    let has_doc_hidden_attr=assoc_ty.attrs.iter()
                        .any(|s| tokens_to_string(s)==ctokens.doc_hidden.to_string() );

                    if has_doc_hidden_attr != ( field.assoc_ty_privacy()==Privacy::Private ) {
                        params.push_err(VIEK::WrongDefinition,format!(
                            "Privacy of {}::{} is {:?} when {:?} was expected \n",
                            trait_.ident,assoc_ty.ident,
                            field.assoc_ty_privacy(),
                            if has_doc_hidden_attr {Privacy::Private}else{Privacy::Inherited} ,
                        ));
                    }
                }
            }
            VisitItem::EndOfVisitor=>{
                if !visited_const_value {
                    params.push_err(
                        VIEK::ExpectedMoreItems ,
                        format!("did not find the {} struct.",variant.const_value)
                    );
                }
                if !visited_dt_trait {
                    params.push_err(
                        VIEK::ExpectedMoreItems ,
                        format!("did not find the {} trait.",variant.dt_trait)
                    );
                }
                if !visited_wr_trait {
                    params.push_err(
                        VIEK::ExpectedMoreItems ,
                        format!("did not find the {} trait.",variant.wr_trait)
                    );
                }

            }
            _=>{}
        };
    });

}

