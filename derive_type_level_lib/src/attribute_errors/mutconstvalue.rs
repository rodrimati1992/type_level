use attribute_detection::mutconstvalue::{
    ImplsIndex,
};

use core_extensions::prelude::*;

use super::{
    ValidAttrs,
    AttrVariant,
    AttrShape,
    AttrKind,
    shared_metadata,
    new_items,
    CowStr,
};



pub fn extension_methods_attr()->AttrShape{
    AttrShape{
        variants:vec![
            AttrVariant{ kind:AttrKind::NameValue{value:"false".into()} , clarification:None},
            AttrVariant{ kind:AttrKind::NameValue{value:"true".into()} , clarification:None},
        ],
        word:"ExtensionMethods",
        description:"\
            Determines whether extension ConstMethods are allowed to mutate 
            the Const-parameter of the derived type.\n\
            Default value is `false`.\
        ".into(),
    }
}


pub fn type_attr()->AttrShape{
    AttrShape{
        variants:vec![
            AttrVariant{ 
                kind:AttrKind::NameValue{value:"ident".into()} ,
                clarification:Some("the string must be a valid identifier".into())
            },
            AttrVariant{ 
                kind:AttrKind::List{value:" name=\"ident\" $(, <metadata_attribute> )* ".into()} , 
                clarification:Some("the string must be a valid identifier".into())
            },
            AttrVariant{ 
                kind:AttrKind::List{value:" use_=\"ident\"  $(, <metadata_attribute> )* ".into()} , 
                clarification:Some(use_clarification("<TypeAlias>"))
            },
        ],
        word:"Type",
        description:"(required attribute)\
             Determines the name and other optional properties of <TypeAlias>.\
        ".into(),
    }
}


pub fn items_attr()->AttrShape{
    new_items(
        ImplsIndex::T,
        "Allows specifying the metadata attributes for the generated impls.".into()
    )
}

pub fn const_param_attr()->AttrShape{
    AttrShape{
        variants:vec![
            AttrVariant{
                kind:AttrKind::NameValue{value:"ident".into()} , 
                clarification:Some("the string must be one of the type parameters.".into())
            },
            AttrVariant{
                kind:AttrKind::NameValue{value:"ident = DefaultType".into()} , 
                clarification:Some("\
                    `ident` must be the identifier of one of the type parameters,\n\
                    and `DefaultType` must be its default value.\
                ".into())
            },
        ],
        word:"Param",
        description:"(required attribute) \
            The identifier of the Const-parameter of this type.\
        ".into(),
    }
}

// pub fn constconstructor_attr()->AttrShape{
//     AttrShape{
//         variants:vec![
//             AttrVariant{ 
//                 kind:AttrKind::NameValue{value:"ident".into()} ,
//                 clarification:Some("the string must be a valid identifier".into())
//             },
//             AttrVariant{ 
//                 kind:AttrKind::List{
//                     value:" name=\"ident\" $(, <metadata_attribute> )* ".into()
//                 }, 
//                 clarification:Some("the string must be a valid identifier".into())
//             },
//             AttrVariant{ 
//                 kind:AttrKind::List{
//                     value:" use_=\"ident\"  $(, <metadata_attribute> )* ".into()
//                 },
//                 clarification:Some(use_clarification("<ConstConstructor>"))
//             },
//         ],
//         word:"ConstConstructor",
//         description:"(optional attribute)\
//              Determines the name and other optional properties of the ConstConstructor.\
//         ".into(),
//     }
// }



fn use_clarification(item_name:&str)->CowStr{
    format!("the string must be the identifier of a pre-existing {}.",item_name).into()
}

fn use_attr(item_name:&str)->AttrShape{
    AttrShape{
        variants:vec![
            AttrVariant{
                kind:AttrKind::NameValue{value:"ident".into()} , 
                clarification:Some(use_clarification(item_name))
            }
        ],
        word:"use_",
        description:format!(
            "(required attribute) Specifies which pre-existing {} to use.",
            item_name
        ).into(),
    }
}


fn name_subattr(item_name:&str)->AttrShape{
    AttrShape{
        variants:vec![
            AttrVariant{
                kind:AttrKind::NameValue{value:"ident".into()} , 
                clarification:Some(use_clarification(item_name))
            }
        ],
        word:"name",
        description:format!(
            "(optional attribute) Specifies the name of the generated {}.",
            item_name
        ).into(),
    }
}


pub fn type_subattr(type_:&'static str)->ValidAttrs{
    vec![
        use_attr(type_),
        name_subattr(type_),
    ].into_iter()
        .chain( shared_metadata() )
        .collect::<Vec<_>>()
        .piped(ValidAttrs::new)
}


pub fn type_subattrs()->ValidAttrs{
    type_subattr("<TypeAlias>")
}

// pub fn constconstructor_subattrs()->ValidAttrs{
//     type_subattr("<ConstConstructor>")
// }

pub fn mutconstvalue_attrs()->ValidAttrs{
    vec![
        type_attr(),
        const_param_attr(),
        // constconstructor_attr(),
        extension_methods_attr(),
        items_attr(),
    ].piped(ValidAttrs::new)
}








