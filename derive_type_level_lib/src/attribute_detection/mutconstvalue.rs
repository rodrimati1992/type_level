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
};

// use std::str::FromStr;


#[derive(Default)]
pub(crate)struct CCAttributes<'alloc>{
    pub(crate) impls:IndexableImplA<ItemMetaData<'alloc,()>>,
    pub(crate) attrs:ItemMetaData<'alloc,()>,
    // pub(crate) const_constructor:ItemMetaData<'alloc,TypeDecl<'alloc>>,
    pub(crate) type_alias       :ItemMetaData<'alloc,TypeDecl<'alloc>>,
    pub(crate) const_param      :Option<(&'alloc Ident,Option<&'alloc syn::Type>)>,
    
    /// Whether extension Const-methods are allowed.
    pub(crate) extension_methods:ExtMethodIA,
    pub(crate) print_derive: bool,
    pub(crate) skip_derive: bool,
}


#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum TypeOrCConstr{
    Type,
    // ConstConstructor,
}


////////////////////////////////////////////////////////////////////////////////


declare_indexable_struct!{
    enum index=ImplsIndex

    #[derive(Default)]
    struct indexable=IndexableImplA

    variants=[
        (const_layout_independent,ConstLayoutIndependent),
        (apply_const_param       ,ApplyConstParam_),
        (get_const_constructor   ,GetConstConstructor_),
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
        _ => return,
    };

    if meta_list.ident == "mcv" {
        for nested0_syn in &meta_list.nested {
            let nested0: MyMeta = nested0_syn.into_with(arenas);
            let word = &*nested0.word.str;

            if let Ok(_)=this.attrs.update_with_meta(&nested0,arenas) {
                continue;
            }

            let is_reserved=word.chars().next().map_or(false,|c| c.is_uppercase());

            match word {
                "SkipDerive" => {
                    this.skip_derive = true;
                    return;
                }
                "PrintDerive" => {
                    this.print_derive = true;
                }
                "ExtensionMethods" => {
                    if let Err(_)=this.extension_methods.update_with_nested(&nested0.value) {
                        panic!(
                            "invalid value for ExtensionMethods={:?}\n{}",
                            nested0.value,
                            attribute_errors::extension_methods_attr()
                        );
                    }
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
                "Param" => {
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
                            
                            //for param in value.list_to_mylist(arenas).into_iter().flat_map(|v|v) {
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
    }
}



////////////////////////////////////////////////////////////////////////////////


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



