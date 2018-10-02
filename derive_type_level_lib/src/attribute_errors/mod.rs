//! This module contains data structures for error reporting when parsing attributes.

pub(crate) mod typelevel;
pub(crate) mod const_constructor;


use attribute_detection::indexable_struct::GetEnumIndices;


#[allow(unused_imports)]
use core_extensions::prelude::*;

use std::fmt;



#[derive(Debug)]
pub struct ValidAttrs<'a>{
    pub valid_attrs:Vec<AttrShape<'a>>,
}


#[derive(Debug)]
pub struct FilteredAttrs<'a,F>{
    pub valid_attrs:&'a [AttrShape<'a>],
    filter:F
}


impl<'a> ValidAttrs<'a>{
    pub fn new(valid_attrs:Vec<AttrShape<'a>>)->Self{
        Self{
            valid_attrs,
        }
    }

    pub fn with_filter<F>(&'a self,filter:F)->FilteredAttrs<'a,F>
    where 
        F:Fn(&str)->bool
    {
        FilteredAttrs{
            valid_attrs:&self.valid_attrs,
            filter,
        }
    }
}


#[derive(Debug,Copy,Clone)]
pub enum AttrKind<'a>{
    Word,
    NameValue{
        value:&'a str,
    },
    List{
        value:&'a str,
    },
}


#[derive(Debug,Copy,Clone)]
pub struct AttrVariant<'a>{
    pub kind:AttrKind<'a>,
    pub clarification:Option<&'a str>,
}


#[derive(Debug,Copy,Clone)]
pub struct AttrShape<'a>{
    pub variants:&'a [AttrVariant<'a>],
    pub word:&'a str,
    pub description:&'a str,
}



impl<'a> fmt::Display for ValidAttrs<'a>{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        writeln!(f,"\nMust be one of:\n")?;
        for attr in &self.valid_attrs {
            writeln!(f,"{}",attr)?;
        }
        Ok(())
    }
}


impl<'a,F> fmt::Display for FilteredAttrs<'a,F>
where F:Fn(&str)->bool,
{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        for attr in self.valid_attrs.iter().filter(|as_| (self.filter)(as_.word) ) {
            write!(f,"{}",attr)?;
        }
        Ok(())
    }
}


impl<'a> fmt::Display for AttrShape<'a>{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        use std::fmt::Write;

        writeln!(f,"\n{S}{S}\n'{}' attribute:",self.word,S="--------------------")?;
        write!(f,"{}\n",self.description )?;
        let mut buffer=String::new();
        for variant in self.variants{
            write!(buffer,"\nusage `{}",self.word)?;
            match variant.kind {
                AttrKind::Word=>Ok(()),
                AttrKind::NameValue{value}=>write!(buffer,"=\"{}\"",value),
                AttrKind::List{value}=>write!(buffer,"({})",value),
            }?;
            writeln!(buffer,"`.")?;
            if let Some(clarif)=variant.clarification {
                if clarif.chars().count() <= 60 && clarif.lines().count()<=1 {
                    writeln!(buffer,"clarification:{}", clarif)?;
                }else{
                    writeln!(buffer,"clarification:\n{}", clarif.to_string().left_pad(2))?;
                }
            }
        }
        writeln!(f,"{}",buffer.left_pad(4))?;

        Ok(())
    }
}







////////////////////////////////////////////////////////////////////////


fn new_items<I>(
    _indices:VariantPhantom<I>,
    description:&'static str
)->AttrShape<'static>
where
    I:GetEnumIndices
{
    use utils::{leak_string,leak_vec};

    let items_clarification:&'static str=format!(
        "NameOfImpls can be one of:{}",
        I::indices_message()
    ).piped(leak_string);
        
    let item_variants:&'static [AttrVariant<'static>]=vec![
        AttrVariant{
            kind:AttrKind::List{value:" NameOfImpls0(..),NameOfImpls1(..), ... "} ,
            clarification:Some(items_clarification),
        }
    ].piped(leak_vec);

    AttrShape{
        variants:item_variants,
        word:"item",
        description,
    }
}


pub const SHARED_METADATA:&'static [AttrShape<'static>]=&[
    SHARED_BOUND,
    SHARED_ATTR,
    SHARED_DOC,
];


pub const SHARED_BOUND:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::NameValue{value:"Type:Bound"} ,
            clarification:Some("the string has to be a single where predicate."),
        }
    ],
    word:"bound",
    description:"Bounds added to the generated item.",
};


pub const SHARED_ATTR:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::List{ value:" <attributes> " },
            clarification:Some("<attributes> must be a valid attribute,eg:\"doc(hidden)\"."),
        }
    ],
    word:"attr",
    description:"Attributes that will be added to the generated item."
};


pub const SHARED_DOC:AttrShape<'static>=AttrShape{
    variants:&[
        AttrVariant{
            kind:AttrKind::NameValue{value:"documentation"},
            clarification:Some("the string can span multiple lines"),
        }
    ],
    word:"doc",
    description:"A documentation attribute the will be added to the generated item."
};

