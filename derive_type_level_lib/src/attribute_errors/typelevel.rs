use attribute_detection::typelevel::ImplIndex;

use core_extensions::prelude::*;

use super::{
    new_items,
    shared_metadata,
    // SHARED_BOUND,
    // SHARED_ATTR,
    // SHARED_DOC,
    AttrKind,
    AttrShape,
    AttrVariant,
    ValidAttrs,
};

pub fn type_attrs() -> ValidAttrs {
    vec![
        rename(),
        rename_consttype(),
        rename_constvalue(),
        rename_trait(),
        rename_withruntime(),
        derive(),
        items(),
        reexport(),
    ].into_iter()
    .chain(shared_metadata())
    .collect::<Vec<_>>()
    .piped(ValidAttrs::new)
}

pub fn field_attrs() -> ValidAttrs {
    vec![
        field_pub_trait_getter(),
        field_bound(),
        field_bound_runt(),
        field_rename(),
        field_accessor(),
    ].piped(ValidAttrs::new)
}

pub fn item_attrs() -> ValidAttrs {
    vec![
        items_attr_no_impls(),
        items_attr_default_impls(),
        items_attr_internal(),
    ].into_iter()
    .chain(shared_metadata())
    .collect::<Vec<_>>()
    .piped(ValidAttrs::new)
}

pub fn rename_consttype() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::NameValue {
                value: "new_name".into(),
            },
            clarification: Some("the string must be a valid identifier.".into()),
        }],
        word: "rename_consttype",
        description: "\
                      Renames the ConstType generated for the Type.\n\
                      ConstType is the compile-time equivalent of a type,\
                      and ConstValue is the compile-time equivalent of a value of that type.\
                      ".into(),
    }
}

pub fn reexport() -> AttrShape {
    AttrShape {
        variants: vec![
            AttrVariant {
                kind: AttrKind::NameValue {
                    value: "visibility".into(),
                },
                clarification: Some("where the string has to be a valid visibility".into()),
            },
            AttrVariant {
                kind: AttrKind::List {
                    value: " $(<reexport_kind>),* ".into(),
                },
                clarification: Some(
                    "\
}
Where <reexport_kind> enables re-exporting a group of items , one/many of:
- Traits:
    For structs <DerivingType>Trait and <DerivingType>IntoRuntime.
    For enums <DerivingType>Trait,<Variant>Trait and <DerivingType>IntoRuntime.

- Variants/Struct:
    For structs Const<DerivingType> .
    For enums types of the same name as the variants.

- Discriminants:the `variants` module

- Fields:the `fields` module.
                ".into(),
                ),
            },
        ],
        word: "reexport",
        description: "\
                      Reexports the generated items outside of the generated module ,\n\
                      reexported to the module of the deriving type.\
                      ".into(),
    }
}

pub fn rename() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::NameValue {
                value: "new_name".into(),
            },
            clarification: Some("the string has to be a valid identifier.".into()),
        }],
        word: "rename",
        description: "\
                      Changes the base name used in identifiers from `<DerivingType>` \
                      to the passed identifier\
                      ".into(),
    }
}

pub fn rename_constvalue() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::NameValue {
                value: "new_name".into(),
            },
            clarification: Some("the string has to be a valid identifier.".into()),
        }],
        word: "rename_constvalue",
        description: "Renames the ConstValue equivalent of the derived Type/Variant.".into(),
    }
}

pub fn rename_trait() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::NameValue {
                value: "new_name".into(),
            },
            clarification: Some("the string has to be a valid identifier.".into()),
        }],
        word: "rename_trait",
        description: "\
                      Renames the trait used to access the fields of the ConstValue equivalent \n\
                      for the derived Type/Variant.\
                      ".into(),
    }
}

pub fn rename_withruntime() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::NameValue {
                value: "new_name".into(),
            },
            clarification: Some("the string has to be a valid identifier.".into()),
        }],
        word: "rename_withruntime",
        description: "\
                      Renames the <DerivingType>WithRuntime trait.\
                      ".into(),
    }
}

pub fn derive() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::List {
                value: "ConstEq|ConstOrd| OtherTraits ".into(),
            },
            clarification: Some("the string has to be a valid identifier.".into()),
        }],
        word: "derive",
        description: "\
                      Derives all Built-in traits,\n\
                      delegating all unsupported traits to the #[derive(...)] attribute.\
                      ".into(),
    }
}

pub fn items() -> AttrShape {
    new_items(
        ImplIndex::T,
        "\
         Allows specifying Metadata for the generated impls and how/whether \
         they are implemented.\n\
         The generated impls are for the Built-in traits and \
         all the Automatically implementd Traits.\
         ".into(),
    )
}

pub fn items_attr_no_impls() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::Word,
            clarification: None,
        }],
        word: "NoImpls",
        description: "Disables this implementation.".into(),
    }
}

pub fn items_attr_default_impls() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::Word,
            clarification: None,
        }],
        word: "DefaultImpls",
        description: "Generates the default implementation.".into(),
    }
}

pub fn items_attr_internal() -> AttrShape {
    AttrShape {
        variants: vec![
            AttrVariant {
                kind: AttrKind::NameValue {
                    value: "type_identifier".into(),
                },
                clarification: Some("where the string is a valid identifier.".into()),
            },
            AttrVariant {
                kind: AttrKind::List {
                    value: "Type=\"type_identifier\",Manual".into(),
                },
                clarification: Some(
                    "\
                     The string must be a valid identifier.\n\
                     The trait must be manually implemented.\
                     ".into(),
                ),
            },
        ],
        word: "Internal",
        description:
            "\
             Generates an implementation for a different type,instead of the type being derived.\n\
             Has no effect on traits that do not involve the deriving type.\
             ".into(),
    }
}

pub fn field_pub_trait_getter() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::Word,
            clarification: None,
        }],
        word: "pub_trait_getter",
        description:
            "\
             Allows accessing the value of a private field through the <DerivingType>Trait.\n\
             Does not allow using GetField to access the value of the field.\
             ".into(),
    }
}

pub fn field_bound() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::NameValue {
                value: "bound".into(),
            },
            clarification: Some("bound must be a valid constraint.".into()),
        }],
        word: "bound",
        description: "\
                      Allows adding a bound to the associated type of the <DerivingType>Trait\
                      representing this field.\
                      ".into(),
    }
}

pub fn field_bound_runt() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::NameValue {
                value: "bound".into(),
            },
            clarification: Some("bound must be a valid constraint.".into()),
        }],
        word: "bound_runt",
        description: "\
                      Allows adding a bound to the associated type of the <DerivingType>WithRuntime\
                      representing this field.\
                      ".into(),
    }
}

pub fn field_rename() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::NameValue {
                value: "name".into(),
            },
            clarification: Some("the string must be a valid identifier.".into()),
        }],
        word: "rename",
        description: "\
                      Renames the field in the generated code.\
                      Currently only possible for Struct/Struct Variants.\
                      ".into(),
    }
}

pub fn field_accessor() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::NameValue {
                value: "name".into(),
            },
            clarification: Some("the string must be a valid identifier.".into()),
        }],
        word: "accessor",
        description:
            "\
             The name of the field accessor,declared in the fields submodule.\n\
             This accessor is used to access the contents of the field in GetField/SetField.\
             ".into(),
    }
}
