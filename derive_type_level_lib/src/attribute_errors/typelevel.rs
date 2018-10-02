use attribute_detection::typelevel::ImplIndex;

use core_extensions::prelude::*;

use super::{
    ValidAttrs,
    AttrVariant,
    AttrShape,
    AttrKind,
    SHARED_METADATA,
    // SHARED_BOUND,
    // SHARED_ATTR,
    // SHARED_DOC,
};

pub static RENAME_CONSTTYPE:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::NameValue{value:"new_name"},
            clarification:Some("the string must be a valid identifier."),
        }
    ],
    word:"rename_statictype",
    description:"\
        Renames the ConstType generated for the Type.\n\
        ConstType is marker type used as the type of a ConstValue,\n\
        in which ConstValue is the compiletime equivalent of a value.\n\
    ",
};



pub static REEXPORT:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::NameValue{value:"visibility"},
            clarification:Some("where the string has to be a valid visibility"),
        },
        AttrVariant{
            kind:AttrKind::List{value:" $(<reexport_kind>),* "},
            clarification:Some("\
Where <reexport_kind> enables re-exporting a group of items , one/many of:
- Traits:
    For structs \\<DerivingType>Trait and \\<DerivingType>IntoRuntime.
    For enums \\<DerivingType>Trait,<Variant>Trait and \\<DerivingType>IntoRuntime.

- Variants/Struct:
    For structs Const\\<DerivingType> .
    For enums types of the same name as the variants.

- Discriminants:the `variants` module

- Fields:the `fields` module.
            ")
        }
    ],
    word:"reexport",
    description:"\
        Reexports the generated items outside of the generated module ,\n\
        reexported to the module of the deriving type.\n\
    ",
};



pub static RENAME:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::NameValue{value:"new_name"} ,
            clarification:Some("the string has to be a valid identifier."),
        }
    ],
    word:"rename",
    description:"Renames the ConstValue equivalent of the derived Type/Variant.",
};


pub static RENAME_TRAIT:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::NameValue{value:"new_name"} ,
            clarification:Some("the string has to be a valid identifier."),
        }
    ],
    word:"rename_trait",
    description:"\
        Renames the trait used to access the fields of the ConstValue equivalent \n\
        for the derived Type/Variant.\
    ",
};


pub static DERIVE:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::List{value:"ConstEq|ConstOrd| OtherTraits "} ,
            clarification:Some("the string has to be a valid identifier."),
        }
    ],
    word:"derive",
    description:"\
        Derives all Built-in traits,\n\
        delegating all unsupported traits to the #[derive(...)] attribute.\
    ",
};


lazy_static!{
    pub static ref ITEMS:AttrShape<'static>={
        lazy_static!{
            static ref ITEMS_CLARIFICATION:String=
                format!("NameOfImpls can be one of:{}",ImplIndex::indices_message());
            
            static ref ITEMS_VARIANTS:Vec<AttrVariant<'static>>=vec![
                AttrVariant{
                    kind:AttrKind::List{value:" NameOfImpls0(..),NameOfImpls1(..), ... "} ,
                    clarification:Some(&*ITEMS_CLARIFICATION),
                }
            ];
        }

        AttrShape{
            variants:&ITEMS_VARIANTS,
            word:"derive",
            description:"\
                Allows specifying Metadata for the generated impls and how/whether \
                they are implemented.\n\
                The generated impls are for the Built-in traits and \
                all the Automatically implementd Traits.\
            ",
        }
    };

}



pub static ITEMS_ATTR_NO_IMPLS:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::Word,
            clarification:None
        }
    ],
    word:"NoImpls",
    description:"Disables this implementation.",
};


pub static ITEMS_ATTR_DEFAULT_IMPLS:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::Word,
            clarification:None
        }
    ],
    word:"DefaultImpls",
    description:"Generates the default implementation.",
};


pub static ITEMS_ATTR_REMOTE:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::NameValue{value:"type_identifier"},
            clarification:Some("where the string is a valid identifier.")
        },
        AttrVariant{
            kind:AttrKind::List{value:"Type=\"type_identifier\",Manual"},
            clarification:Some("\
                The string must be a valid identifier.\n\
                The trait must be manually implemented\n\
            ")
        }
    ],
    word:"Remote",
    description:"Generates an implementation of the trait for usage with the delegate attribute.",
};


pub static ITEMS_ATTR_INTERNAL:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::NameValue{value:"type_identifier"},
            clarification:Some("where the string is a valid identifier.")
        },
        AttrVariant{
            kind:AttrKind::List{value:"Type=\"type_identifier\",Manual"},
            clarification:Some("\
                The string must be a valid identifier.\n\
                The trait must be manually implemented\n\
            ")
        }
    ],
    word:"Internal",
    description:"\
        Generates an implementation for a different type,instead of the type being derived.\n\
        Has no effect on traits that do not involve the deriving type.\n\
    ",
};



pub static FIELD_PUB_TRAIT_ACCESSOR:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::Word,
            clarification:None,
        },
    ],
    word:"pub_trait_accessor",
    description:"\
        Allows accessing the value of a private field through the <DerivingType>Trait.\n\
        Does not allow using GetField to access the value of the field.\n\
    ",
};


pub static FIELD_BOUND:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::NameValue{value:"bound"},
            clarification:Some("bound must be a valid constraint."),
        },
    ],
    word:"bound",
    description:"\
        Allows adding a bound to the associated type of the \\<DerivingType>Trait\
        representing this field.\
    ",
};


pub static FIELD_BOUND_RUNT:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::NameValue{value:"bound"},
            clarification:Some("bound must be a valid constraint."),
        },
    ],
    word:"bound_runt",
    description:"\
        Allows adding a bound to the associated type of the \\<DerivingType>WithRuntime\
        representing this field.\
    ",
};


pub static FIELD_RENAME:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::NameValue{value:"name"},
            clarification:Some("the string must be a valid identifier."),
        },
    ],
    word:"rename",
    description:"\
        Renames the field in the generated code.\
        Currently only possible for Struct/Struct Variants.\
    ",
};


pub static FIELD_ACCESSOR:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::NameValue{value:"name"},
            clarification:Some("the string must be a valid identifier."),
        },
    ],
    word:"accessor",
    description:"\
        The name of the field accessor,declared in the fields submodule.\n\
        This accessor is used to access the contents of the field in GetField/SetField.\n\
    ",
};


pub static FIELD_DELEGATE:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::List{value:" runtime_conv/IntoConstType_/IntoRuntime =\"const_type\" "},
            clarification:Some("\
                The string must be a type which implements \
                IntoConstType_< FieldType > and/or IntoRuntime<FieldType,C>.\n\
            "),
        },
    ],
    word:"delegate",
    description:"\
        The type to which the implementations of IntoRuntime and IntoConstType are delegated to.\n\
    ",
};






lazy_static!{
    pub static ref VARIANT_ATTRS:ValidAttrs<'static>={
        [
            RENAME,
            RENAME_TRAIT,
            DERIVE,
            *ITEMS,
        ].iter().chain( SHARED_METADATA.iter() ).cloned().collect::<Vec<_>>().piped(ValidAttrs::new)
    };

    pub static ref TYPE_VARIANT:ValidAttrs<'static>={
        VARIANT_ATTRS.valid_attrs.iter().chain(&[
            RENAME_CONSTTYPE,
            REEXPORT,
        ]).cloned().collect::<Vec<_>>().piped(ValidAttrs::new)
    };

    pub static ref FIELD_ATTRS:ValidAttrs<'static>=vec![
        FIELD_PUB_TRAIT_ACCESSOR,
        FIELD_BOUND,
        FIELD_BOUND_RUNT,
        FIELD_RENAME,
        FIELD_ACCESSOR,
        FIELD_DELEGATE,
    ].piped(ValidAttrs::new);
}