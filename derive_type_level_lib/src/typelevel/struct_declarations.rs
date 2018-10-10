use super::*;

use syn::{
    Type,
    Attribute,
    Generics,
    Visibility,
    // WherePredicate,
    Path,
    TypeParamBound,
};

use syn::token::{Add};

use syn::punctuated::Punctuated;

use core_extensions::BoolExt;

use std::borrow::Cow;
// use std::cmp::max;
use std::collections::{
    BTreeMap,
    HashSet,
    HashMap,
};

// use to_token_fn::ToTokenFnMut;

use common_tokens::CommonTokens;

use submod_visibility::{
    MyVisibility,
    IsPublic,
    DocHiddenAttr,
};

use attribute_detection::typelevel::ImplIndex;

use attribute_detection::shared::parse_type;


#[derive(Debug)]
pub(crate) struct FieldAccessor{
    /// The ammount of times the field identifier is used for a public field.
    pub(crate) public_instances:usize,
    pub(crate) is_tuple_field:bool,
}

impl FieldAccessor{
    pub(crate) fn doc_hidden_attr<'a>(&self,common_tokens:&'a CommonTokens)->DocHiddenAttr<'a>{
        DocHiddenAttr::new(
            IsPublic(self.public_instances!=0),
            common_tokens
        )
    }
}


#[derive(Debug)]
pub(crate) struct StructDeclarations<'a>{
    pub(crate) tokens:&'a CommonTokens,
    pub(crate) void_ident:&'a Ident,


    pub(crate) vis_kind:MyVisibility<'a>,

    /// The visibility of fields more private than the struct.
    /// This is the most public visibility of fields more private than the type itself.
    /// This is None if the type is an enum or all the fields are as public as the type.
    priv_field_vis:Option<MyVisibility<'a>>,


    pub(crate) type_:TokenStream,
    pub(crate) original_visibility:&'a Visibility,
    pub(crate) original_name:&'a Ident,
    pub(crate) original_path:Path,
    pub(crate) original_generics: &'a Generics,
    pub(crate) original_where_preds:TokenStream,
    pub(crate) original_gen_params:TokenStream,
    
    pub(crate) enum_path:Option<&'a TokenStream>,
    pub(crate) enum_attrs:&'a [Attribute],
    pub(crate) enum_trait:Option<&'a Ident>,
    pub(crate) enum_trait_doc:Option<&'a str>,

    pub(crate) type_marker_struct:&'a Ident,
    pub(crate) enum_or_struct:EnumOrStruct,
    pub(crate) all_types:Vec<FieldTyAndMod<'a>>,
    pub(crate) field_accessors:BTreeMap<&'a Ident,FieldAccessor>,
    
    pub(crate) attribute_settings:&'a TLAttributes<'a>,
    pub(crate) declarations:Vec<StructDeclaration<'a>>,
}


#[derive(Debug)]
pub(crate) struct StructDeclaration<'a>{
    pub(crate) name:&'a Ident,
    pub(crate) uninitialized_ident:&'a Ident,

    pub(crate) type_trait_docs  :&'a str,
    pub(crate) with_runtime_docs:&'a str,

    pub(crate) from_trait_ident:&'a Ident,
    pub(crate) trait_ident:&'a Ident,
    pub(crate) wr_trait_ident:&'a Ident,
    pub(crate) variant_marker_ident:&'a Ident,
    pub(crate) discriminant_ident:&'a Ident,

    pub(crate) generics:TokenStream,
    pub(crate) generics_2:TokenStream,
    

    pub(crate) variant:&'a Struct<'a>,
    pub(crate) attribute_settings:Cow<'a,TLAttributes<'a>>,
    pub(crate) fields:Vec<FieldDeclaration<'a>>
}

#[derive(Debug)]
pub(crate) struct FieldDeclaration<'a>{
    pub(crate) docs:Vec<&'a str>,
    pub(crate) original_name:&'a FieldIdent<'a>,
    pub(crate) common_tokens:&'a CommonTokens,
    /// An attribute override allowing one to access the field through the \<DerivingType>Trait.
    pub(crate) pub_trait_accessor:bool,
    pub(crate) vis_kind:MyVisibility<'a>,
    pub(crate) name_ident:FieldName<'a>,
    pub(crate) accessor_ident:&'a Ident,
    pub(crate) assoc_type:&'a Ident,
    pub(crate) original_ty:&'a Type,
    pub(crate) relative_priv:RelativePriv,
    pub(crate) generic  :&'a Type,
    // used when generating conversions between the struct and another type.
    pub(crate) generic_2:&'a Type, 

    /// Associated type in the <Trait>WithRuntime trait.
    pub(crate) rt_assoc_type:&'a Ident,

    pub(crate) const_bound:Punctuated<TypeParamBound, Add>,
    pub(crate) runt_bound:Punctuated<TypeParamBound, Add>,
}


#[derive(Debug)]
pub(crate) struct FieldTyAndMod<'a>{
    pub(crate) field_ty:&'a Type,
    pub(crate) mod_ty:&'a Type,
}


#[derive(Copy,Clone,Debug,Ord,PartialOrd,Eq,PartialEq)]
pub enum RelativePriv{
    Inherited,
    MorePrivate,
}


impl<'a> StructDeclarations<'a>{
    pub fn new(
        ds:&'a DataStructure<'a>,
        outer_attr_sett:&'a TLAttributes<'a>,
        arenas:ArenasRef<'a>,
        c_tokens:&'a CommonTokens,
    )->Self{
        let name=ds.name;


        let alloc_ident=|ident:Ident|->&'a Ident{
            arenas.idents.alloc(ident)
        };
        let ident_from=|ident:&str|->&'a Ident{
            alloc_ident(Ident::new(ident,name.span()))
        };

        let mut declarations=Vec::new();
        let mut all_types=HashMap::<&'a Type,Option<&'a Type>>::new();
        let mut field_accessors=BTreeMap::<&'a Ident,FieldAccessor>::new();

        let vis_kind=MyVisibility::new(ds.vis,c_tokens);

        let mut priv_field_vis=None::<MyVisibility<'a>>;

        let original_where_preds=&ds.generics.where_clause.as_ref()
            .expect("where clause must be initialized before calling StructDeclarations::new")
            .predicates;

        let original_gen_params={
            let params=&ds.generics.params;
            quote!(#(#params,)*)
        };


        let type_marker_struct=outer_attr_sett.renames.const_type.clone()
            .unwrap_or_else(|| ident_from(&format!("{}Type",name)) );

        let type_trait_docs:&'a str=format!("A type-level version of `{}`.",name)
            .piped(|s| arenas.strings.alloc(s) );

        let enum_trait:Option<&'a Ident>=ds.enum_.as_ref().map(|_|{
            outer_attr_sett.renames.trait_
                .unwrap_or_else(|| ident_from(&format!("{}Trait",name)) )
        });

        let enum_trait_doc:Option<&'a str>=ds.enum_.as_ref().map(|_| type_trait_docs );


        for variant in &ds.variants {
            let inner_attr_sett=match ds.enum_or_struct {
                EnumOrStruct::Enum  =>
                    Cow::Owned(TLAttributes::new(variant.attrs,arenas)),
                EnumOrStruct::Struct=>
                    Cow::Borrowed(outer_attr_sett),
            };
            let fields=variant.fields.iter()
                .map(|v|{
                    let mut get_tuple_field_ident={
                        let mut tuple_fields=Vec::<&'a Ident>::new();
                        
                        let alloc_ident=&alloc_ident;
                        move|i|{
                            while tuple_fields.len() <= i {
                                let new_ident=variant
                                    .new_ident( format!("U{}",tuple_fields.len()) )
                                    .piped(alloc_ident);
                                tuple_fields.push( new_ident );
                            }
                            tuple_fields[i]
                        }
                    };

                    let field_attrs=FieldAttrs::new(&v.attrs,arenas);

                    let name_ident=match (FieldName::new(&v.ident),field_attrs.rename) {
                        (fieldname          ,None)=>
                            fieldname,
                        (FieldName::Index(_),Some(_))   =>
                            panic!("cannot rename positional(tuple struct/variant) fields."),
                        (FieldName::Named(_),Some(ident))=>
                            FieldName::Named(ident),
                    };

                    let accessor_ident=match (name_ident,field_attrs.accessor) {
                        (FieldName::Index(i)    ,None)=>
                            get_tuple_field_ident(i),
                        (FieldName::Named(ident),None)=>
                            ident,
                        (_                      ,Some(ident))=>
                            ident,
                    };

                    let parse_alloc=|suffix:&str|->&'a Type{
                        let x=parse_type(&format!("{}{}",v.pattern_ident,suffix));
                        arenas.types.alloc(x)
                    };

                    let original_ty=v.ty;

                    let generic  =parse_alloc("");
                    let generic_2=parse_alloc("_TyB");

                    use self::RelativePriv as RP;

                    let field_vis_kind;
                    let relative_priv;
                    match ds.enum_or_struct {
                        EnumOrStruct::Enum  =>{
                            field_vis_kind=vis_kind;
                            relative_priv=RP::Inherited;
                        }
                        EnumOrStruct::Struct=>{
                            field_vis_kind=MyVisibility::new(&v.vis,c_tokens);
                            relative_priv=if field_vis_kind < vis_kind {
                                match priv_field_vis {
                                    Some(ref mut pfv) => 
                                        if *pfv < field_vis_kind { *pfv=field_vis_kind; } ,
                                    ref mut pfv => *pfv=Some(field_vis_kind) ,
                                }

                                RP::MorePrivate
                            }else{
                                RP::Inherited
                            };
                        }
                    };

                    let pub_trait_accessor=
                        field_attrs.pub_trait_accessor|| 
                        relative_priv==RP::Inherited;

                    {
                        let accessor=field_accessors.entry(accessor_ident).or_insert_with(||{
                            FieldAccessor{
                                public_instances:0,
                                is_tuple_field:matches!(FieldName::Index{..}= name_ident),
                            }
                        });
                        if relative_priv==RP::Inherited{
                            accessor.public_instances+=1;
                        }
                    }

                    let assoc_type=match name_ident {
                        FieldName::Index(_    )=>&v.pattern_ident,
                        FieldName::Named(ident)=>ident,
                    };

                    let assoc_type=if pub_trait_accessor {
                        assoc_type
                    }else{
                        ident_from(&format!("priv_{}",assoc_type))
                    };


                    FieldDeclaration{
                        common_tokens:c_tokens,
                        docs:field_attrs.docs,
                        original_name:&v.ident,
                        vis_kind:field_vis_kind,
                        pub_trait_accessor,
                        relative_priv,
                        name_ident,
                        accessor_ident,
                        assoc_type,
                        original_ty,
                        generic,
                        generic_2,
                        rt_assoc_type:ident_from(&format!("rt_{}",assoc_type)) ,
                        runt_bound :field_attrs.runt_bound,
                        const_bound:field_attrs.const_bound,
                    }
                })
                .collect::<Vec<_>>();
            let generics;
            let generics_2;
            {
                let generics_iter =fields.iter().map(|x|&x.generic);
                let generics_2_iter=fields.iter().map(|x|&x.generic_2);
                generics  =quote!{ #(#generics_iter,)* };
                generics_2=quote!{ #(#generics_2_iter,)* };
            }

            // the name of the variant after an explicit rename
            let variant_name=inner_attr_sett.renames.variant_type.unwrap_or(&variant.name);

            let name:&'a Ident=match ds.enum_or_struct {
                EnumOrStruct::Enum  =>variant_name,
                EnumOrStruct::Struct=>
                    outer_attr_sett.renames.variant_type
                        .unwrap_or_else(|| ident_from(&format!("Const{}",variant.name))),
            };
            
            let type_str=match ds.enum_or_struct {
                EnumOrStruct::Enum  =>format!("the `{}::{}` variant",ds.name,variant.name),
                EnumOrStruct::Struct=>format!("the `{}` type",variant.name),
            };

            let type_trait_docs=format!("A type-level version of {}.",type_str)
                .piped(|s| arenas.strings.alloc(s) );

            let with_runtime_docs=format!(
                "A type-level version of {} with access to its generic parameters.",
                type_str
            ).piped(|s| arenas.strings.alloc(s) );

            let uninitialized_ident=
                ident_from(&format!("{}_Uninit",variant_name));

            let trait_ident=inner_attr_sett.renames.trait_
                .unwrap_or_else(|| ident_from(&format!("{}Trait",variant_name)) );
            let wr_trait_ident=inner_attr_sett.renames.wr_trait
                .unwrap_or_else(|| ident_from(&format!("{}WithRuntime",variant_name)) );
            let discriminant_ident=
                ident_from(&format!("{}_Discr",variant_name));
            let variant_marker_ident=
                ident_from(&format!("{}_Variant",variant_name));
            let from_trait_ident=ident_from(&format!("{}FromTrait",variant_name));
            declarations.push(StructDeclaration{
                name,
                type_trait_docs,
                with_runtime_docs,
                uninitialized_ident,
                trait_ident,
                wr_trait_ident,
                variant_marker_ident,
                discriminant_ident,
                variant,
                from_trait_ident,
                attribute_settings:inner_attr_sett,
                generics  ,
                generics_2,
                fields,
            })
        }


        let all_types=all_types
            .into_iter()
            .map(|(field_ty,mod_ty)|{
                let mod_ty=mod_ty.unwrap_or(field_ty);
                FieldTyAndMod{ field_ty, mod_ty }
            })
            .collect::<Vec<FieldTyAndMod<'a>>>();

        

        Self{
            tokens:c_tokens,
            void_ident:ident_from("_core_Void"),
            vis_kind,
            priv_field_vis,
            type_:quote!{ #name <#original_gen_params> },
            original_visibility:ds.vis,
            original_name:name,
            original_path:name.clone().into(),
            type_marker_struct,
            enum_or_struct:ds.enum_or_struct,
            original_generics:ds.generics,
            original_gen_params,
            original_where_preds:quote!{#(#original_where_preds,)*},
            all_types,
            field_accessors,

            enum_trait,
            enum_trait_doc,
            enum_path:ds.enum_.as_ref().map(|x| &x.path ),
            enum_attrs:ds.enum_.as_ref().map_or(&[],|x| x.attrs ),
            declarations,
            attribute_settings:outer_attr_sett,
        }
    }


    /// Returns a type which outputs `Priv,` if there are any fields more private than the type.
    pub(crate) fn priv_param_suffix(&self)->PrivParam<'a>{
        PrivParam{
            priv_field_vis:self.priv_field_vis.is_some(),
            c_tokens:self.tokens,
        }
    }

    /// Returns a type which outputs the visibility of the 
    /// most public field more private than the type.
    pub(crate) fn priv_field_vis(&self)->MyVisibility<'a>{
        self.priv_field_vis.unwrap_or(self.vis_kind)
    }

    pub(crate) fn opt_priv_field_vis(&self)->Option<MyVisibility<'a>>{
        self.priv_field_vis
    }

    pub(crate) fn relative_field_priv(&self)->RelativePriv {
        if self.priv_field_vis.map_or(true,|p|self.vis_kind <= p) {
            RelativePriv::Inherited
        }else{
            RelativePriv::MorePrivate
        }
    }

}
    

impl<'alloc> StructDeclaration<'alloc>{

    pub(crate) fn is_item_declared(
        &self,
        declarations:&StructDeclarations<'alloc>,
        item:ImplIndex
    )->bool{
        let decls_is_impld=declarations.attribute_settings.derived[item].inner.is_implemented();
        if matches!(ImplIndex::IntoConstType|ImplIndex::IntoRuntime = item) {
            return decls_is_impld;
        }
        decls_is_impld||
        self.attribute_settings.derived[item].inner.is_implemented()
    }
}


impl<'a> ToTokens for StructDeclarations<'a>{
    fn to_tokens(&self,tokens:&mut TokenStream){
        let type_marker_struct=&self.type_marker_struct;
        let type_marker_struct_rep_a=iter::repeat(&self.type_marker_struct);
        let type_marker_struct_rep_b=iter::repeat(&self.type_marker_struct);
        // let type_marker_struct_rep_c=iter::repeat(&self.type_marker_struct);
        let enum_trait=self.enum_trait.as_ref();
        let enum_trait_doc=self.enum_trait_doc;

        let type_docs=&self.attribute_settings.attrs.docs;

        let priv_suffix=self.priv_param_suffix();

        let fields_doc_hidden=self.field_accessors.values()
            .map(|acc|acc.doc_hidden_attr(self.tokens));
        
        let mut fields_1a=Vec::new();
        let mut fields_1b=Vec::new();
        for (k,acc) in &self.field_accessors {
            match acc.is_tuple_field {
                false=>&mut fields_1a,
                true =>&mut fields_1b,
            }.push(k);
        }
        let fields_1a=&fields_1a;
        let fields_1b=&fields_1b;

        let fields_2=self.field_accessors.keys();
        let pub_fields=self.field_accessors.iter()
            .filter(|&(_,v)| v.public_instances != 0 )
            .map(|t|t.0);
        let priv_fields=self.field_accessors.iter()
            .filter(|&(_,v)| v.public_instances == 0 )
            .map(|t|t.0);

        let vis= self.vis_kind.submodule_level(1);        
        let vis_rep_a=iter::repeat(vis);
        
        let vis_kind_submod=self.vis_kind.submodule_level(2);
        let vis_kind_submod_rep=iter::repeat(vis_kind_submod);
        
        let priv_field_vis_submod2 =self.priv_field_vis().submodule_level(2);
        let priv_field_vis_submod =self.priv_field_vis().submodule_level(1);
        let opt_priv_field_vis =self.opt_priv_field_vis().map(|v| v.submodule_level(1) );
        let has_priv_fields=self.opt_priv_field_vis().map(|_| &self.tokens.priv_struct );


        let priv_struct_reexport=self.opt_priv_field_vis().map(|_|{
            quote!(
                #priv_field_vis_submod use self::__private_mod::{
                    __PrivTrait,
                    __IsPriv,
                };
            )
        });

            

        tokens.append_all(quote!{
            mod __private_mod{
                #vis_kind_submod trait Sealed{}

                #vis_kind_submod trait __PrivTrait{}

                #vis_kind_submod struct __IsPriv;

                impl __PrivTrait for __IsPriv{}

            }
            use self::__private_mod::Sealed;

            #priv_struct_reexport

            #(#[doc= #type_docs ])*
            #[derive(Copy,Clone)]
            #vis struct #type_marker_struct;

            impl ConstType for #type_marker_struct{}
            

            #(
                #[doc= #enum_trait_doc]
                #vis_rep_a trait #enum_trait:DerivedTraits<Type=#type_marker_struct_rep_a>{

                } 
            )*

            mod __fields{
                #(
                    #fields_doc_hidden
                    #[derive(Clone,Copy)]
                    /// This is the accessor for the field of the same name.
                    #vis_kind_submod_rep struct #fields_1a;
                )*
                #vis_kind_submod use super::typenum_reexports::{
                    #( #fields_1b, )*
                };

                /// This is the accessor for all the fields.
                #[derive(Clone,Copy)]
                #vis_kind_submod struct All;
            }

            pub mod fields{
                #vis_kind_submod use super::__fields::{
                    #(#pub_fields,)*
                };

                #priv_field_vis_submod2 use super::__fields::{
                    #(#priv_fields,)*
                    All,
                };
            }
        });

        let mut additional_derives=HashSet::new();

        let additional_derives_outer=&self.attribute_settings.additional_derives;
        for declaration in &self.declarations {
            let trait_ident=&declaration.trait_ident;
            // let attrs=declaration.variant.attrs;
            let s_name=&declaration.name;
            // let s_name=&declaration.name;
            // let s_name_rep=iter::repeat(s_name);
            let uninitialized_ident=declaration.uninitialized_ident;
            let generics_fn=||declaration.fields.iter().map(|x|x.generic);
            let generics=&declaration.generics;
            let generics_0=generics_fn();
            let generics_1=generics_fn();
            
            let generics_voided=iter::repeat(self.void_ident).take(declaration.fields.len());
            let additional_derives_inner=&declaration.attribute_settings.additional_derives;
            
            let item_attrs=self.attribute_settings.attrs
                .chain_impl_attrs(&declaration.attribute_settings.attrs);
                
            let item_docs=&declaration.attribute_settings.attrs.docs;

            additional_derives.clear();
            additional_derives.extend(additional_derives_outer.iter());
            additional_derives.extend(additional_derives_inner.iter());
            
            let additional_derives=additional_derives.is_empty()
                .if_false(||additional_derives.iter());

            if let Some(enum_trait)=enum_trait{
                tokens.append_all(quote!{
                    impl<#generics> #enum_trait for #s_name<#generics>
                    where Self:#trait_ident,
                    {}
                });
            }
            let field_vis=declaration.fields.iter()
                .map(|x|x.vis_kind.submodule_level(1));

            tokens.append_all(quote!{
                
                #priv_field_vis_submod type #uninitialized_ident=
                    #s_name < #(#generics_voided,)* #priv_suffix> ;

                #item_attrs
                #( #[doc=#item_docs] )*
                #( #[derive(#(#additional_derives,)* )] )*
                #vis struct #s_name<
                    #(#generics_0,)* 
                    #priv_suffix
                >
                #(where 
                    #has_priv_fields:__PrivTrait,
                )*
            });

            tokens.append_all(match declaration.variant.kind {
                _ if declaration.fields.is_empty() =>{
                    quote!{ ; }
                }
                StructKind::Tuple=>{
                    quote!{ 
                        ( 
                            #(#field_vis ConstWrapper<#generics_1>,)* 
                            #(#opt_priv_field_vis ConstWrapper<__IsPriv>,)*
                        ); 
                    }
                }
                StructKind::Braced=>{
                    let names=declaration.fields.iter().map(|x| &x.name_ident );
                    quote!{ 
                        { 
                            #(#field_vis #names:ConstWrapper<#generics_1>,)* 
                            #(#opt_priv_field_vis priv_:ConstWrapper<__IsPriv>,)*
                        } 
                    }
                }
            });
        }
    }
}


////////////////////////////////////////////////////////////////////////////////


#[derive(Copy,Clone,Debug,Ord,PartialOrd,Eq,PartialEq)]
pub(crate) struct PrivParam<'a>{
    priv_field_vis:bool,
    c_tokens:&'a CommonTokens,
}


impl<'a> ToTokens for PrivParam<'a>{
    fn to_tokens(&self, tokens: &mut TokenStream){
        if self.priv_field_vis {
            self.c_tokens.priv_struct.to_tokens(tokens);
            self.c_tokens.comma.to_tokens(tokens);
        }
    }
}



////////////////////////////////////////////////////////////////////////////////


impl<'a> FieldDeclaration<'a> {
    pub(crate) fn doc_hidden_attr(&self)->DocHiddenAttr{
        DocHiddenAttr::new(
            IsPublic(self.pub_trait_accessor),
            self.common_tokens
        )
    }
}





////////////////////////////////////////////////////////////////////////////////


#[derive(Debug,Copy,Clone,PartialEq,Eq,Ord,PartialOrd,Hash)]
pub enum FieldName<'a>{
    Index(usize),
    Named(&'a Ident),
}

// impl<'a> FieldName<'a>{
//     fn map_index<F>(mut self,f:F)->Self
//     where F:FnOnce(usize)->usize
//     {
//         match self {
//             FieldName::Index(ref mut ind)=>
//                 *ind=f(*ind),
//             _=>{}
//         }
//         self
//     }

//     fn map_named<F>(mut self,f:F)->Self
//     where F:FnOnce(&'a Ident)->&'a Ident
//     {
//         match self {
//             FieldName::Named(ref mut name)=>
//                 *name=f(*name),
//             _=>{}
//         }
//         self
//     }
// }



impl<'a> FieldName<'a>{
    fn new(fi:&'a FieldIdent<'a>)->Self{
        match fi {
            &FieldIdent::Index(index,..)=>
                FieldName::Index(index),
            &FieldIdent::Named(name)=>
                FieldName::Named(name),
        }
    }
}




impl<'a> ToTokens for FieldName<'a>{
    fn to_tokens(&self, tokens: &mut TokenStream){
        match *self {
            FieldName::Index(ind)=>
                syn::Index::from(ind).to_tokens(tokens),
            FieldName::Named(name)=>
                name.to_tokens(tokens),
        }
    }
}
