use super::item_metadata::ItemMetaData;
use super::my_meta::{MyMeta, MyNested};
use indexable_struct::GetEnumIndices;

use attribute_errors::mutconstvalue as attribute_errors;

use super::shared::{
    NotUpdated,
    UpdateWithMeta,
    // ident_from_nested,
    foreach_nestedmeta_index,
    parse_ident,
    typaram_from_nested,
};
use ArenasRef;

// use arrayvec::ArrayString;

#[allow(unused_imports)]
use core_extensions::*;
use ::*;

// use syn::punctuated::Punctuated;
// use syn::token::Comma;
use syn::{
    Attribute, 
    Ident,
    Meta,
    NestedMeta,
};

use quote::ToTokens;
// use std::str::FromStr;


#[derive(Default)]
pub(crate)struct CCAttributes<'alloc>{
    pub(crate) impls:IndexableImplA<ItemMetaData<'alloc,()>>,
    pub(crate) attrs:ItemMetaData<'alloc,()>,
    // pub(crate) const_constructor:ItemMetaData<'alloc,TypeDecl<'alloc>>,
    pub(crate) type_alias       :ItemMetaData<'alloc,TypeDecl<'alloc>>,
    pub(crate) const_param      :Option<(&'alloc Ident,Option<&'alloc syn::Type>)>,

    pub(crate) main_repr:Option<MainRepr>,
    // This is a Vec so that the user UnsafeRepr can be a list as well as an ident.
    pub(crate) additional_repr_attrs:Vec<&'alloc Ident>,
    
    pub(crate) print_derive: bool,
    pub(crate) skip_derive: bool,
    pub(crate) derive_str: bool,
}


#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum TypeOrCConstr{
    Type,
    // ConstConstructor,
}


////////////////////////////////////////////////////////////////////////////////


#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum MainRepr{
    Rust,
    C,
    Transparent,
}


////////////////////////////////////////////////////////////////////////////////


declare_indexable_struct!{
    enum index=ImplsIndex

    #[derive(Default)]
    struct indexable=IndexableImplA

    variants=[
        (const_layout_independent,ConstLayoutIndependent),
        (get_const_param         ,GetConstParam_),
    ]

    multi_indices=[]
}







////////////////////////////////////////////////////////////////////////////////


#[derive(Debug,Default,PartialEq)]
pub struct ExtMethodIA{
    pub is_allowed:bool,
}

impl ExtMethodIA{
    fn update_with_nested(&mut self,nested:&MyNested)->Result<(),NotUpdated>{
        match *nested {
            MyNested::Word=>{}
            MyNested::Value(value)=> {
                if ["true","True","Allow","allow"].contains(&value) {
                    self.is_allowed=true;
                }else if ["false","False"].contains(&value) {
                    self.is_allowed=false;
                }else{
                    return Err(NotUpdated)
                }
            }
            _=>return Err(NotUpdated),
        }
        Ok(())
    }
}


////////////////////////////////////////////////////////////////////////////////



#[derive(Debug,Default)]
pub(crate) struct TypeDecl<'alloc>{
    pub(crate) decl:Option<TypeDeclVariant<'alloc>>,

}

impl<'alloc> TypeDecl<'alloc>{
    fn alloc_ident(ident:&'alloc str,arenas:ArenasRef<'alloc>)->&'alloc Ident{
        arenas.idents.alloc(parse_ident(ident))
    }
    fn set_name(&mut self,ident:&'alloc str,arenas:ArenasRef<'alloc>){
        self.decl=Some(TypeDeclVariant::Name( Self::alloc_ident(ident,arenas) ));
    }
    fn set_use(&mut self,ident:&'alloc str,arenas:ArenasRef<'alloc>){
        self.decl=Some(TypeDeclVariant::Use( Self::alloc_ident(ident,arenas) ));
    }
}


impl<'alloc> UpdateWithMeta<'alloc> for TypeDecl<'alloc>{
    fn update_with_meta(
        &mut self,
        meta: &MyMeta<'alloc>,
        arenas: ArenasRef<'alloc>,
    ) -> Result<(), NotUpdated> {
        match (&*meta.word.str, &meta.value) {
            ("name",&MyNested::Value(ref name))=>
                self.set_name(name,arenas),
            ("use_",&MyNested::Value(ref name))=>
                self.set_use(name,arenas),
            _ => return Err(NotUpdated),
        }
        Ok(())
    }
}


/// Whether to create a new type alias or use an old one.
#[derive(Debug,Copy,Clone)]
pub(crate) enum TypeDeclVariant<'alloc>{
    /// Uses a pre-existing type alias.
    Use(&'alloc Ident),
    /// Creates a new type alias.
    Name(&'alloc Ident),
}


impl<'alloc> TypeDeclVariant<'alloc>{
    pub(crate) fn ident(&self)->&'alloc Ident{
        match *self {
            TypeDeclVariant::Use(ident,..)=>ident,
            TypeDeclVariant::Name(ident,..)=>ident,
        }
    }
}



////////////////////////////////////////////////////////////////////////////////


impl<'alloc> CCAttributes<'alloc>{
    pub(crate)fn new(attributes:&'alloc [Attribute],arenas:ArenasRef<'alloc>)->Self{
        let mut this=Self::default();

        // for attr in attributes{
        //     println!("attribute:\n----------\n{}\n----------\n",attr.into_token_stream());
        // }

        for attr in attributes{
            constructor_inner(attr,&mut this,arenas);
        }

        this
    }
}


////////////////////////////////////////////////////////////////////////////////


fn constructor_inner<'alloc>(
    attrs: &'alloc Attribute,
    this:&mut CCAttributes<'alloc>, 
    arenas: ArenasRef<'alloc>
){
    let meta_list = match attrs.interpret_meta() {
        Some(Meta::List(meta_list)) => arenas.metalists.alloc(meta_list),
        Some(_)=>{ return }
        None=>{
            panic!("not a valid attribute:\n{}\n",attrs.into_token_stream() );
        }
    };

    // {
    //     for nested0_syn in &meta_list.nested {
    //         let nested0: MyMeta = nested0_syn.into_with(arenas);
    //         println!("word:{}", &nested0.word.str);
    //     }
    // }

    
    if meta_list.ident == "mcv" {
        let prev_attr_len=this.attrs.attrs.len();

        for nested0_syn in &meta_list.nested {
            let nested0: MyMeta = nested0_syn.into_with(arenas);
            let word = &*nested0.word.str;

            
            if let Ok(_)=this.attrs.update_with_meta(&nested0,arenas) {
                continue;
            }

            let is_reserved=word.chars().next().map_or(false,|c| c.is_uppercase());

            match word {
                "UnsafeRepr"=>{
                    let mut nested0=nested0.clone();
                    nested0.value.list_to_mylist(arenas).drop_();
                    match nested0.value {
                        MyNested::MyList(list)=>{
                            for elem in list {
                                this.main_repr=this.main_repr.or(
                                     match &*elem.word.str {
                                        "Rust"=>Some(MainRepr::Rust),
                                        "rust"=>Some(MainRepr::Rust),
                                        "C"=>Some(MainRepr::C),
                                        "c"=>Some(MainRepr::C),
                                        "Transparent"=>Some(MainRepr::Transparent),
                                        "transparent"=>Some(MainRepr::Transparent),
                                        _=>None,
                                    }
                                );
                                match this.main_repr {
                                     Some(MainRepr::Rust)=>{},
                                     Some(MainRepr::C)
                                    |Some(MainRepr::Transparent)
                                    |None
                                    =>{
                                        this.additional_repr_attrs.push(elem.word.ident);
                                    }
                                }
                            }
                        }
                        v=>{
                            panic!("\n\n\n\
                                Invalid attribute inside UnsafeRepr(..):\n\t{:?}\
                                {}\
                                ", 
                                v,
                                attribute_errors::unsafe_repr_attr()
                            )
                        }
                    }
                }
                "SkipDerive" => {
                    this.skip_derive = true;
                    return;
                }
                "PrintDerive" => {
                    this.print_derive = true;
                }
                "DeriveStr" => {
                    this.derive_str = true;
                }
                "Type" =>{
                    let item=&mut this.type_alias;
                    update_ident_and_metadata(
                        &nested0.value,
                        item,
                        TypeOrCConstr::Type,
                        arenas
                    );
                } 
                "ConstValue" => {
                    this.const_param = Some( typaram_from_nested(&nested0.value,arenas) );
                }
                "Items"=>{
                    let value = nested0.value;

                    let list_1=match &value {
                        &MyNested::List(list_1)=>list_1,
                        value=>panic!("Unsupported nested attribute:{:#?}", value),
                    };

                    foreach_nestedmeta_index(
                        list_1,
                        arenas,
                        |value, ind| {
                            let impl_=&mut this.impls[ind];
                            
                            for param in value.list_to_mylist(arenas).unwrap() {
                                impl_
                                    .update_with_meta(param, arenas)
                                    .unwrap_or_else(|_| panic!("Invalid parameter:{:#?}", param) );
                            }
                        },
                        |_,e|{
                            panic!(
                                "\n\nnot valid inside items( ... ):'{}'\n\nMust be one of:{}\n\n", 
                                e.0,
                                ImplsIndex::indices_message()
                            )
                        }
                    );

                }
                word if is_reserved =>{
                    panic!("\n\n\n\
                        Attributes starting with an uppercase character are reserved:'{}'\n\
                        {}\
                        ", 
                        word,
                        attribute_errors::mutconstvalue_attrs()
                    )
                }
                word =>{
                    this.attrs.attrs.push(nested0_syn);
                }
            }
        }

        for pushed_attrs in &this.attrs.attrs[prev_attr_len..] {
            match **pushed_attrs {
                NestedMeta::Meta(Meta::List(ref added_attrs))if added_attrs.ident=="repr" =>{
                    for added_attr in &added_attrs.nested {
                        if let NestedMeta::Meta(ref meta)=*added_attr {
                            let (ident,repr)=check_repr_attr(meta);
                            this.main_repr=Some(repr);
                        }
                    }
                },
                _=>{}
            }
        }

    }
}



////////////////////////////////////////////////////////////////////////////////


fn check_repr_attr<'alloc>(meta:&'alloc Meta)-> (&'alloc Ident,MainRepr) {
    if let Meta::Word(ref ident)=*meta {
        if ident=="C"{
            (ident,MainRepr::C)
        }else if ident=="transparent" {
            (ident,MainRepr::Transparent)
        }else{
            panic!("\n\n\
                repr({0}) attribute not supported by this library,\n\t{:?}\n\n
                you must use #[mcv(UnsafeRepr( {0} ))] for a representation attribute \
                other than repr(C)/repr(transparent)\
                (other reprs are potentially unsafe in future versions of Rust).\n\
            ", ident)
        }
    }else{
        panic!("inside a repr attribute:{:?}",meta)
    }
}


fn update_ident_and_metadata<'alloc>(
    meta:&MyNested<'alloc>,
    item:&mut ItemMetaData<'alloc,TypeDecl<'alloc>>,
    type_or_cconstr:TypeOrCConstr,
    arenas: ArenasRef<'alloc>,
){
    match meta {
        &MyNested::List(list_2)=>{
            for param in list_2{
                let param: MyMeta = param.into_with(arenas);
                item.update_with_meta(&param,arenas).unwrap_or_else(|_|{
                    let error_msg=match type_or_cconstr {
                        TypeOrCConstr::Type=> 
                            attribute_errors::type_subattrs(),
                        // TypeOrCConstr::ConstConstructor=>
                        //     attribute_errors::constconstructor_subattrs(),
                    };
                    panic!("Invalid parameter:{:#?}{}", param,error_msg) 
                });
            }
        },
        &MyNested::Value(ref name)=>{
            item.inner.set_name(name,arenas);
        }
        word=>{
            let error_msg=match type_or_cconstr {
                TypeOrCConstr::Type=>attribute_errors::type_attr(),
                // TypeOrCConstr::ConstConstructor=>attribute_errors::constconstructor_attr(),
            };
            panic!("Unsupported nested attribute:{:#?}{}", word,error_msg);
        }
    }
}



////////////////////////////////////////////////////////////////////////////////







////////////////////////////////////////////////////////////////////////////////







////////////////////////////////////////////////////////////////////////////////







////////////////////////////////////////////////////////////////////////////////







////////////////////////////////////////////////////////////////////////////////



