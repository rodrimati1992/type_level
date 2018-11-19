doc_code_snippets! {
    mod "guide_09",
    type_ident=Guide09,
    template=r##"

This chapter demonstrates using TypeFn_ to represent type constructors,
allowing a function to construct a type with different type parameters .

Say that we need to parse a bunch of names (some of which have surnames) 
from a ';' separated string.

We also have the extra requirement that we have to store the string slices from which we parse 
the full names.

We will use a TypeFn_ to determine the kind of collection used to store both the 
parsed full name and string slices from which we parsed the names.

//@use_codeblock:fullname_struct,ignore

This is the struct representing the full name,
in which the surname is optional.

//@use_codeblock:fullname_new,ignore

This is the constructor for FullName.

//@use_codeblock:fullname_fromstr,ignore

This is how we parse a FullName,
note that because surname is optional we simply use the 
value returned from the second call to  iter.next().

//@use_codeblock:get_collection_trait,ignore

Here we declare a trait alias which ensures that the output of a TypeFn_
is a valid collection,
and we declare a type alias of that trait for convenience.

//@use_codeblock:parse_names_fn,ignore

The parse_names function parses the `text` into multiple values of type `T`,
outputs the &str from which each `T` was parsed and 
outputs the parse errors as a Vec\<T::Err\>.

This function takes the `T` type being parsed explicitly,
using the `TypeName::T`/`<Type>::T` syntax,
as described in the `core_extensions::SelfOps` trait.


//@use_codeblock:main_vec,ignore

Here we declare the VecFn TypeFn_ which `parse_names` will use to 
construct Vec<_> parameterized by different types.
<br>
Then we declare the text we'll parse,with 3 FullNames.
<br>
Then we call parse_names,
specifying the type being parsed with `FullName::T` 
and specifying the constructed collection with `VecFn::NEW`.
<br>
Then we check that the returned value is what we expect.

//@use_codeblock:main_set,ignore

This is basically identical to the previous example,except that it involves a BTreeSet.

The mutated method here is defined in the core_extensions::SelfOps trait,
allowing us to initialize an immutable variable with 
temporary mutable access to the value being initialize.

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

use std::collections::BTreeSet;
use std::iter;
use std::iter::FromIterator;
use std::str::FromStr;

use type_level_values::prelude::*;

//@codeblock-start:fullname_struct

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct FullName {
    name:String,
    surname:Option<String>,
}

//@codeblock-end  :fullname_struct



//@codeblock-start:fullname_new

impl FullName{
    fn new(name:&str,surname:Option<&str>)->Self{
        Self{
            name:name.into(),
            surname:surname.map(String::from),
        }
    }
}

//@codeblock-end  :fullname_new

//@codeblock-start:fullname_fromstr

impl FromStr for FullName {
    type Err = InvalidFullNameStr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter=s.split(",").map(String::from);
        let name=iter.next()
            .ok_or_else(|| InvalidFullNameStr::NoName(s.into()) )?;
        Ok(FullName { 
            name , 
            surname :iter.next(),
        })
    }
}

#[derive(Debug)]
pub enum InvalidFullNameStr {
    NoName(String),
}

//@codeblock-end  :fullname_fromstr



//@codeblock-start:get_collection_trait

pub trait GetCollection_<T>:TypeFn_<T>{
    type Collection:Default+FromIterator<T>+Extend<T>;
}

impl<Func,T,Out> GetCollection_<T> for Func
where 
    Func:TypeFn_<T,Output=Out>,
    Out:Default+FromIterator<T>+Extend<T>,
{
    type Collection=Out;
}

pub type GetCollection<This,T>=
    <This as GetCollection_<T>>::Collection;

//@codeblock-end  :get_collection_trait



//@codeblock-start:parse_names_fn

pub fn parse_names<'a,T,GC>(
    text: &'a str,
    _parsed_type:VariantPhantom<T>,
    _collection_fn:GC,
) -> (GetCollection<GC,T>,GetCollection<GC,&'a str>,Vec<T::Err>)
where 
    T:FromStr,
    GC:GetCollection_<T>+GetCollection_<&'a str>,
{
    let mut parsed=GetCollection::<GC,T>::default();
    let mut slices=GetCollection::<GC,&str>::default();
    let mut errors=Vec::<T::Err>::new();
    
    for slice_ in text.split(';') {
        match slice_.parse::<T>() {
            Ok(v)=>parsed.extend( iter::once(v) ),
            Err(e)=>errors.push(e),
        }
        slices.extend(iter::once(slice_));
    }

    (parsed,slices,errors)
}

//@codeblock-end  :parse_names_fn



fn main() {
    {
        //@codeblock-start:main_vec

        type_fn!{ fn VecFn[T](T){ Vec<T> } }

        let text="thomas,anderson;matt,parker;joe";

        let (ret,ret_slices,set_errors) = 
            parse_names(text, FullName::T , VecFn::NEW );
            
        let cmp = vec![
            FullName::new("thomas",Some("anderson")),
            FullName::new("matt",Some("parker")),
            FullName::new("joe",None),
        ];
        let cmp_slices = vec![
            "thomas,anderson",
            "matt,parker",
            "joe",
        ];
        assert_eq!(ret, cmp);
        assert_eq!(ret_slices,cmp_slices);
        assert!(set_errors.is_empty(),"{:?}", set_errors);
        
        //@codeblock-end  :main_vec
    }

    {
        //@codeblock-start:main_set

        type_fn!{ fn BTreeSetFn[T](T){ BTreeSet<T> } }

        let text="thomas,anderson;matt,parker;joe";

        let (ret,ret_slices,set_errors) = 
            parse_names(text, FullName::T , BTreeSetFn::NEW );
        let cmp = BTreeSet::new().mutated(|v| {
            v.insert(FullName::new("thomas",Some("anderson")));
            v.insert(FullName::new("matt",Some("parker")));
            v.insert(FullName::new("joe",None));
        });
        let cmp_slices = BTreeSet::new().mutated(|v| {
            v.insert("thomas,anderson");
            v.insert("matt,parker");
            v.insert("joe");
        });
        assert_eq!(ret, cmp);
        assert_eq!(ret_slices,cmp_slices);
        assert!(set_errors.is_empty(),"{:?}", set_errors);

        //@codeblock-end  :main_set
    }

}






"##,
}