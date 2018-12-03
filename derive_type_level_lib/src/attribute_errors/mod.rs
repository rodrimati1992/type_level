//! This module contains data structures for error reporting when parsing attributes.

pub(crate) mod mutconstvalue;
pub(crate) mod typelevel;

use indexable_struct::GetEnumIndices;

#[allow(unused_imports)]
use core_extensions::prelude::*;

use std::borrow::Cow;
use std::fmt;

pub type CowStr = Cow<'static, str>;

#[derive(Debug)]
pub struct ValidAttrs {
    pub valid_attrs: Vec<AttrShape>,
}

#[derive(Debug)]
pub struct FilteredAttrs<'a, F> {
    pub valid_attrs: &'a [AttrShape],
    filter: F,
}

impl ValidAttrs {
    pub fn new(valid_attrs: Vec<AttrShape>) -> Self {
        Self { valid_attrs }
    }

    pub fn with_filter<'a, F>(&'a self, filter: F) -> FilteredAttrs<'a, F>
    where
        F: Fn(&str) -> bool,
    {
        FilteredAttrs {
            valid_attrs: &self.valid_attrs,
            filter,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AttrKind {
    Word,
    NameValue { value: CowStr },
    List { value: CowStr },
}

#[derive(Debug, Clone)]
pub struct AttrVariant {
    pub kind: AttrKind,
    pub clarification: Option<CowStr>,
}

#[derive(Debug, Clone)]
pub struct AttrShape {
    pub variants: Vec<AttrVariant>,
    pub word: &'static str,
    pub description: CowStr,
}

impl fmt::Display for ValidAttrs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\nMust be one of:\n")?;
        for attr in &self.valid_attrs {
            writeln!(f, "{}", attr)?;
        }
        Ok(())
    }
}

impl<'a, F> fmt::Display for FilteredAttrs<'a, F>
where
    F: Fn(&str) -> bool,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for attr in self
            .valid_attrs
            .iter()
            .filter(|as_| (self.filter)(as_.word))
        {
            write!(f, "{}", attr)?;
        }
        Ok(())
    }
}

impl fmt::Display for AttrShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write;

        writeln!(
            f,
            "\n{S}{S}\n'{}' attribute:",
            self.word,
            S = "--------------------"
        )?;
        write!(f, "{}\n", self.description)?;
        let mut buffer = String::new();
        for variant in &self.variants {
            write!(buffer, "\nusage `{}", self.word)?;
            match variant.kind {
                AttrKind::Word => Ok(()),
                AttrKind::NameValue { ref value } => write!(buffer, "=\"{}\"", value),
                AttrKind::List { ref value } => write!(buffer, "({})", value),
            }?;
            writeln!(buffer, "`.")?;
            if let Some(ref clarif) = variant.clarification {
                if clarif.chars().count() <= 60 && clarif.lines().count() <= 1 {
                    writeln!(buffer, "clarification:{}", clarif)?;
                } else {
                    writeln!(buffer, "clarification:\n{}", clarif.to_string().left_pad(2))?;
                }
            }
        }
        writeln!(f, "{}", buffer.left_pad(4))?;

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////

fn new_items<I>(_indices: VariantPhantom<I>, description: CowStr) -> AttrShape
where
    I: GetEnumIndices,
{
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::List {
                value: " NameOfImpls0(..),NameOfImpls1(..), ... ".into(),
            },
            clarification: Some(
                format!("NameOfImpls can be one of:{}", I::indices_message()).into(),
            ),
        }],
        word: "items",
        description,
    }
}

pub fn shared_metadata() -> Vec<AttrShape> {
    vec![shared_bound(), shared_attr(), shared_doc()]
}

pub fn shared_bound() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::NameValue {
                value: "Type:Bound".into(),
            },
            clarification: Some("the string has to be a single where predicate.".into()),
        }],
        word: "bound",
        description: "Bounds added to the generated item.".into(),
    }
}

pub fn shared_attr() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::List {
                value: " <attributes> ".into(),
            },
            clarification: Some(
                "\
                 <attributes> must be a valid attribute,eg:\"doc(hidden)\".\
                 ".into(),
            ),
        }],
        word: "attr",
        description: "Attributes that will be added to the generated item.".into(),
    }
}

pub fn shared_doc() -> AttrShape {
    AttrShape {
        variants: vec![AttrVariant {
            kind: AttrKind::NameValue {
                value: "documentation".into(),
            },
            clarification: Some("the string can span multiple lines".into()),
        }],
        word: "doc",
        description: "A documentation attribute the will be added to the generated item.".into(),
    }
}
