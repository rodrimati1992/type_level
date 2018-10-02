use super::item_metadata::ItemMetaData;
use super::my_meta::{MyMeta, MyNested};
use super::indexable_struct::GetEnumIndices;

use attribute_errors::const_constructor as attribute_errors;

use super::shared::{
    NotUpdated,
    UpdateWithMeta,
    ident_from_nested,
    foreach_nestedmeta_index,
    parse_ident,
};
use ArenasRef;

use arrayvec::ArrayString;

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

use std::str::FromStr;


#[derive(Default)]
pub(crate)struct CCAttributes<'alloc>{
    pub(crate) impls:IndexableImplA<ItemMetaData<'alloc,()>>,
    pub(crate) attrs:ItemMetaData<'alloc,()>,
    pub(crate) const_constructor:ItemMetaData<'alloc,TypeDecl<'alloc>>,
    pub(crate) type_alias       :ItemMetaData<'alloc,TypeDecl<'alloc>>,
    pub(crate) const_param      :Option<&'alloc Ident>,
    
    /// Whether extension Const-methods are allowed.
    pub(crate) extension_methods:ItemMetaData<'alloc,ExtMethodIA>,
    pub(crate) print_derive: bool,
    pub(crate) skip_derive: bool,
}


#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum TypeOrCConstr{
    Type,
    ConstConstructor,
}


////////////////////////////////////////////////////////////////////////////////


declare_indexable_struct!{
    enum index=ImplsIndex

    #[derive(Default)]
    struct indexable=IndexableImplA

    variants=[
        (const_constructor       ,ConstConstructor),
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
        self.is_allowed=match *nested {
             MyNested::Word
            |MyNested::Value("true")
            |MyNested::Value("True")
            =>true,
             MyNested::Value("false")
            |MyNested::Value("False")
            =>false,
            _=>return Err(NotUpdated),
        };
        Ok(())
    }
}


impl<'alloc> UpdateWithMeta<'alloc> for ExtMethodIA{
    fn update_with_meta(
        &mut self,
        meta: &MyMeta<'alloc>,
        _arenas: ArenasRef<'alloc>,
    ) -> Result<(), NotUpdated> {
        if meta.word.str=="allow" {
            self.update_with_nested(&meta.value)
        }else{
            Err(NotUpdated)
        }
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

    if meta_list.ident == "cconstructor" {
        for nested0 in &meta_list.nested {
            let nested0: MyMeta = nested0.into_with(arenas);
            let word = &*nested0.word.str;

            if let Ok(_)=this.attrs.update_with_meta(&nested0,arenas) {
                continue;
            }

            match word {
                "skip_derive" => {
                    this.skip_derive = true;
                    return;
                }
                "print_derive" => {
                    this.print_derive = true;
                }
                "extension_methods" => {
                    if let Err(_)=this.extension_methods.inner.update_with_nested(&nested0.value){
                        let mut value=nested0.value;

                        let list=value.list_to_mylist(arenas).unwrap_or_else(|e|{
                            panic!("Unsupported nested attribute:{:#?}", e)
                        });
                        for param in list {
                            let _=this.extension_methods.update_with_meta(param,arenas);
                        }
                    }
                }
                "ConstConstructor" =>{
                    let item=&mut this.const_constructor;
                    update_ident_and_metadata(
                        &nested0.value,
                        item,
                        TypeOrCConstr::ConstConstructor,
                        arenas
                    );
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
                "ConstParam" => {
                    this.const_param = Some(ident_from_nested(&nested0.value,arenas));
                }
                "items"=>{
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
                word=>{
                    panic!("Unsupported attribute:'{}'{}", 
                        word,
                        &*attribute_errors::CCONSTRUCTOR_ATTRS
                    );
                }
            }
            
            // match word {
            //     "rename" => {
            //         settings.rename = Some(ident_from_nested(&value,arenas));
            //     }
            //     "accessor" => {
            //         settings.accessor = Some(ident_from_nested(&value,arenas));
            //     }
            //     "delegate" => {
            //     }
            //     word => {
            //         panic!("Unsupported nested attribute:{:#?}", word);
            //     }
            // }
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
                            &*attribute_errors::TYPE_SUBATTRS,
                        TypeOrCConstr::ConstConstructor=>
                            &*attribute_errors::CONSTCONSTRUCTOR_SUBATTRS,
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
                TypeOrCConstr::Type=> &*attribute_errors::TYPE_ATTR,
                TypeOrCConstr::ConstConstructor=>&attribute_errors::CONSTCONSTRUCTOR_ATTR,
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



