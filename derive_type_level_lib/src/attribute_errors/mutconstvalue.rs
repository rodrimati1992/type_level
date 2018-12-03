use attribute_detection::mutconstvalue::ImplsIndex;

use core_extensions::prelude::*;

use super::{new_items, shared_metadata, AttrKind, AttrShape, AttrVariant, CowStr, ValidAttrs};

pub fn type_attr() -> AttrShape {
    AttrShape {
        variants: vec![
            AttrVariant {
                kind: AttrKind::NameValue {
                    value: "ident".into(),
                },
                clarification: Some("the string must be a valid identifier".into()),
            },
            AttrVariant {
                kind: AttrKind::List {
                    value: " name=\"ident\" $(, <metadata_attribute> )* ".into(),
                },
                clarification: Some("the string must be a valid identifier".into()),
            },
            AttrVariant {
                kind: AttrKind::List {
                    value: " use_=\"ident\"  $(, <metadata_attribute> )* ".into(),
                },
                clarification: Some(use_clarification("<TypeAlias>")),
            },
        ],
        word: "Type",
        description: "(required attribute)\
                      Determines the name and other optional properties of <TypeAlias>.\
                      ".into(),
    }
}

pub fn items_attr() -> AttrShape {
    new_items(
        ImplsIndex::T,
        "Allows specifying the metadata attributes for the generated impls.".into(),
    )
}

pub fn const_param_attr() -> AttrShape {
    AttrShape {
        variants: vec![
            AttrVariant {
                kind: AttrKind::NameValue {
                    value: "ident".into(),
                },
                clarification: Some(
                    "\
                     ident must be the identifier of one of the type parameters.\
                     ".into(),
                ),
            },
            AttrVariant {
                kind: AttrKind::NameValue {
                    value: "ident = DefaultVal".into(),
                },
                clarification: Some(
                    "\
                     `ident` must be the identifier of one of the type parameters,\n\
                     and `DefaultVal` must be its default value.\
                     ".into(),
                ),
            },
        ],
        word: "ConstValue",
        description: "(required attribute) \
                      The identifier of the ConstValue-parameter of this type.\
                      ".into(),
    }
}
pub fn unsafe_repr_attr() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::List {
                value: "Repr0,Repr1,Repr2".into(),
            },
            clarification: Some(
                "\
                 The passed representation must be valid.\n\
                 Examples: C,transparent,Rust \
                 ".into(),
            ),
        }],
        word: "UnsafeRepr",
        description: "(optional attribute) \
            An unsafe attribute which allows using any repr attribute,
            even if it is not guaranteed to not change the layout of the type \
            when the ConstValue changes.\n
            It is plausible that the default \
            representation might change to be unsafe to use with MutConstValue,\
            which is why it this derive macro uses `#[repr(C)]` by default,\
            even though it is a terrible way to do it\
            (thank the people wanting repr(Rust) to not guarantee anything at all \
            about layout for this library having to use repr(C)).\
        ".into(),
    }
}

fn use_clarification(item_name: &str) -> CowStr {
    format!(
        "the string must be the identifier of a pre-existing {}.",
        item_name
    ).into()
}

fn use_attr(item_name: &str) -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::NameValue {
                value: "ident".into(),
            },
            clarification: Some(use_clarification(item_name)),
        }],
        word: "use_",
        description: format!(
            "(required attribute) Specifies which pre-existing {} to use.",
            item_name
        ).into(),
    }
}

fn name_subattr(item_name: &str) -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::NameValue {
                value: "ident".into(),
            },
            clarification: Some(use_clarification(item_name)),
        }],
        word: "name",
        description: format!(
            "(optional attribute) Specifies the name of the generated {}.",
            item_name
        ).into(),
    }
}

pub fn type_subattr(type_: &'static str) -> ValidAttrs {
    vec![use_attr(type_), name_subattr(type_)]
        .into_iter()
        .chain(shared_metadata())
        .collect::<Vec<_>>()
        .piped(ValidAttrs::new)
}

pub fn type_subattrs() -> ValidAttrs {
    type_subattr("<TypeAlias>")
}

pub fn mutconstvalue_attrs() -> ValidAttrs {
    vec![
        type_attr(),
        const_param_attr(),
        items_attr(),
        unsafe_repr_attr(),
    ].piped(ValidAttrs::new)
}
