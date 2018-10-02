use attribute_detection::const_constructor::{
    ImplsIndex,
};

use utils::{leak_string,leak_vec};

use core_extensions::prelude::*;

use super::{
    ValidAttrs,
    AttrVariant,
    AttrShape,
    AttrKind,
    SHARED_METADATA,
    new_items,
};



pub static EXTENSION_METHODS_ATTR:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{ kind:AttrKind::NameValue{value:"false"} , clarification:None },
        AttrVariant{ kind:AttrKind::NameValue{value:"true"} , clarification:None },
    ],
    word:"extension_methods",
    description:"\
        Determines whether extension ConstMethods are allowed to mutate 
        the Const-parameter of the derived type.\n\
        Default value is `false`.\
    ",
};


lazy_static!{
    pub static ref TYPE_ATTR:AttrShape<'static>=AttrShape{
        variants:vec![
            AttrVariant{ 
                kind:AttrKind::NameValue{value:"ident"} ,
                clarification:Some("the string must be a valid identifier")
            },
            AttrVariant{ 
                kind:AttrKind::List{value:" name=\"ident\" $(, <metadata_attribute> )* "} , 
                clarification:Some("the string must be a valid identifier")
            },
            AttrVariant{ 
                kind:AttrKind::List{value:" use_=\"ident\"  $(, <metadata_attribute> )* "} , 
                clarification:Some(use_clarification("<TypeAlias>"))
            },
        ].piped(leak_vec),
        word:"Type",
        description:"(required attribute)\
             Determines the name and other optional properties of <TypeAlias>.\
        ",
    };
}


lazy_static!{
    pub static ref ITEMS_ATTR:AttrShape<'static>=new_items(
        ImplsIndex::T,
        "Allows specifying the metadata attributes for the generated impls."
    );
    
    pub static ref CONST_PARAM_ATTR:AttrShape<'static>=AttrShape{
        variants:vec![
            AttrVariant{
                kind:AttrKind::NameValue{value:"ident"} , 
                clarification:Some("the string must be one of the type type parameters.")
            }
        ].piped(leak_vec),
        word:"ConstParam",
        description:"(required attribute) The identifier of the Const-parameter of this type.",
    };

    pub static ref CONSTCONSTRUCTOR_ATTR:AttrShape<'static>=AttrShape{
        variants:vec![
            AttrVariant{ 
                kind:AttrKind::NameValue{value:"ident"} ,
                clarification:Some("the string must be a valid identifier")
            },
            AttrVariant{ 
                kind:AttrKind::List{value:" name=\"ident\" $(, <metadata_attribute> )* "} , 
                clarification:Some("the string must be a valid identifier")
            },
            AttrVariant{ 
                kind:AttrKind::List{value:" use_=\"ident\"  $(, <metadata_attribute> )* "} , 
                clarification:Some(use_clarification("<ConstConstructor>"))
            },
        ].piped(leak_vec),
        word:"ConstConstructor",
        description:"(optional attribute)\
             Determines the name and other optional properties of the ConstConstructor.\
        ",
    };
    
}



fn use_clarification(item_name:&str)->&'static str{
    format!("the string must be the identifier of a pre-existing {}.",item_name)
        .piped(leak_string)
}

fn use_attr(item_name:&str)->AttrShape{
    AttrShape{
        variants:vec![
            AttrVariant{
                kind:AttrKind::NameValue{value:"ident"} , 
                clarification:Some(use_clarification(item_name))
            }
        ].piped(leak_vec),
        word:"use_",
        description:format!(
            "(required attribute) Specifies which pre-existing {} to use.",
            item_name
        ).piped(leak_string),
    }
}


fn name_subattr(item_name:&str)->AttrShape<'static>{
    AttrShape{
        variants:vec![
            AttrVariant{
                kind:AttrKind::NameValue{value:"ident"} , 
                clarification:Some(use_clarification(item_name))
            }
        ].piped(leak_vec),
        word:"name",
        description:format!(
            "(optional attribute) Specifies the name of the generated {}.",
            item_name
        ).piped(leak_string),
    }
}


pub fn type_subattr(type_:&'static str)->ValidAttrs<'static>{
    [
        use_attr(type_),
        name_subattr(type_),
    ].iter()
        .chain( SHARED_METADATA.iter() ).cloned()
        .collect::<Vec<_>>()
        .piped(ValidAttrs::new)
}


lazy_static!{
    pub static ref TYPE_SUBATTRS:ValidAttrs<'static>=
        type_subattr("<TypeAlias>");

    pub static ref CONSTCONSTRUCTOR_SUBATTRS:ValidAttrs<'static>=
        type_subattr("<ConstConstructor>");

    pub static ref CCONSTRUCTOR_ATTRS:ValidAttrs<'static>={
        vec![
            *TYPE_ATTR,
            *CONST_PARAM_ATTR,
            *CONSTCONSTRUCTOR_ATTR,
            EXTENSION_METHODS_ATTR,
            *ITEMS_ATTR,
        ].piped(ValidAttrs::new)
    };
}








