doc_code_snippets! {
    mod "guide_11",
    type_ident=Guide11,
    template=r##"

This chapter demonstrates creating a `type module`,
which is a way used to pass many types around with just 1 generic parameter by using a ConstValue.


Say that we want to read some type from csv file,in this case a Point2D.


//@use_codeblock:point_struct,ignore

This is the definition of Point2D,which we are parsing from the file.

//@use_codeblock:point_from_str,ignore

This is how we parse Point2D from a string.

//@use_codeblock:module_struct,ignore

This is the `type module`,used to pass multiple types within a single type parameter.

These are the types in `ConstModule`:

- `type_`:
    is the type being parsed.

- `collection`:
    is the collection we are collecting into.

- `get_default`:
    is closure type which returns the default value in case that parsing failed.


The `bound` attribute on every field gets passed to the associated types in `ModuleTrait`.
<br>
These bounds are then enforced in `impl ModuleTrait for ConstModule< ... >`.


//@use_codeblock:new_module,ignore

This constructs a ConstModule,
requiring the caller to pass the types for 
the type being parsed and the collection it will be stored into.

This function uses the `construct` type macro in its return type to ensure that it
initializes every ConstModule field.

Examples of calling this function:

- `new_module(u32::T  ,Vec::T)` 

- `new_module(<bool>::T ,Vec::<u32>::T)` 

- `new_module(0u8.ty_(),<Vec<u32>>::T)` 

- `new_module(String::T, BTreeSet::T )` 

//@use_codeblock:start_lines,ignore

This function takes the multiline string `lines` ,
creates a lines iterator,
then parses each line as an `M::type_`,
using `get_default` to get the default value for that type if parsing fails,
and finally collects all parsed elements into an `M::collection`.


//@use_codeblock:main,ignore

Here we start by instantiating a ConstModule with 
`type_=Point2D` and `collection=BTreeSet<Point2D>`.
<br>
Then we parse the text using that module,
providing the closure which returns the default value for each line that couldn't be parsed.
<br>
Then we declare a BTreeSet ,
asserting that the return value of calling parse_lines must be equal to it.


<br><br><br><br><br><br><br><br><br><br>
<hr>
<br><br><br><br><br><br><br><br><br><br>


# The entire thing

//@use_codeblock:all,rust

"##,

    code=r##"


//@codeblock-start:all




#[macro_use]
extern crate type_level_values;
#[macro_use]
extern crate derive_type_level;

use std::collections::BTreeSet;
use std::fmt;
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::str::FromStr;

use type_level_values::field_traits::*;
use type_level_values::prelude::*;


//@codeblock-start:point_struct

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Point2D {
    pub x: u32,
    pub y: u32,
}

//@codeblock-end  :point_struct



//@codeblock-start:point_from_str

impl FromStr for Point2D {
    type Err = InvalidPoint2DStr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ints=s.split(",").map(u32::from_str);
        let x=ints.next().ok_or_else(|| InvalidPoint2DStr::InvalidString(s.into()) )??;
        let y=ints.next().ok_or_else(|| InvalidPoint2DStr::InvalidString(s.into()) )??;
        Ok(Point2D { x, y })
    }
}

#[derive(Debug)]
pub enum InvalidPoint2DStr {
    InvalidInteger(ParseIntError),
    InvalidString(String),
}

impl From<ParseIntError> for InvalidPoint2DStr{
    fn from(e:ParseIntError)->Self{
        InvalidPoint2DStr::InvalidInteger(e)
    }
}

//@codeblock-end  :point_from_str



//@codeblock-start:module_struct

#[derive(TypeLevel)]
#[typelevel(reexport(Struct, Traits))]
#[allow(dead_code)]
pub struct Module {
    #[typelevel(bound = "FromStr+fmt::Debug")]
    pub type_: (),

    #[typelevel(bound = "FromIterator<Self::type_>+Default+Extend<Self::type_>")]
    pub collection: (),

    #[typelevel(bound = "FnMut(<Self::type_ as FromStr>::Err)->Self::type_")]
    pub get_default: (),
}

use self::type_level_Module::fields as module_type;

//@codeblock-end:module_struct




//@codeblock-start:new_module

pub fn new_module<T, C, GD>(
    _type: VariantPhantom<T>,
    _collection: VariantPhantom<C>,
) -> construct!(Module_Uninit=>
    module_type::type_=T,
    module_type::collection=C,
    module_type::get_default=GD,
) {
    ConstModule::MTVAL
}

//@codeblock-end:new_module



//@codeblock-start:start_lines

/// Parses every line of `lines` as a `M::type_` and collects them into a `M::collection`.
pub fn parse_lines<M: ModuleTrait>(
    lines: &str,
    _module: M,
    mut get_default: M::get_default,
) -> M::collection {
    lines
        .lines()
        .map(|s| s.parse::<M::type_>().unwrap_or_else(&mut get_default))
        .collect()
}

//@codeblock-end  :start_lines


//@codeblock-start:main

fn main() {
    let module = new_module(Point2D::T, BTreeSet::T);
    let text="\
        10,20\n\
        asdasd\n\
        20,40\
    ";
    let value = parse_lines(text, module, |_| Point2D { x: 0, y: 100 });
    let set = BTreeSet::new().mutated(|v| {
        v.insert(Point2D { x: 10, y: 20 });
        v.insert(Point2D { x: 0, y: 100 });
        v.insert(Point2D { x: 20, y: 40 });
    });
    assert_eq!(value, set);
}

//@codeblock-end  :main




"##,
}