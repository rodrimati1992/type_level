doc_code_snippets! {
    mod "guide_02",
    type_ident=Guide02,



    template=r##"


Here's an example of defining and 
using a type-level-enum as a type-parameter to a struct.


Trait/Type glossary:

- ConstValue: A trait for type-level values,also used here to refer to implementors of it.

- ConstWrapper:
    Zero-sized wrapper type for ConstValue that unconditionally implements 
    Eq/PartialEq/Ord/PartialOrd/Copy/Clone/Debug/etc ,
    delegating IntoRuntime/GetField/SetField/etc to the wrapped ConstValue.


- IntoRuntime : trait for converting a ConstValue to a runtime-value.


//@use_codeblock:declare-enum,ignore

This is the enum we will use for declaring the mutability of the wrapper.

//@use_codeblock:declare-struct,ignore

Here is the zero-overhead wrapper struct that determines the mutability of its contents based on 
the `Mut` parameter.

The `MutConstValue` derive macro generates the `Wrapper`
type alias which passes `Mut` wrapped inside a ConstWrapper
so that `Mut` does not have to implement Debug/PartialEq/etc.

Prefer using the type alias declared by MutConstValue everywhere,
except for Drop,instead use the generated `Wrapper_Ty` type to 
which all the attributes are delegated.

The reason why Mut is wrapped inside a ConstWrapper in the field type is 
because Rust does not allow type parameters to not be used.


//@use_codeblock:constructor,ignore

This is the constructor for Wrapper,which takes the `Mut` by value,
so as not to require using type inference.

When constructing a ConstWrapper inside a constructor function prefer using ConstWrapper::NEW.
<br>
When constructing a ConstWrapper whose type can't be inferred use Type::CW.
<br>
When there is a value that needs to be converted to a ConstWrapper 
either use value.to_cw() or value.into() (if the type is Copy),


//@use_codeblock:deref-impls,ignore

The impls of Deref for Wrapper,
note that Deref takes a generic Mut parameter because it doesn't matter what the mutability
is for shared references,
and that DerefMut requires the ConstValue parameter to be `Mutable` .


<br><br><br><br><br><br><br><br><br><br>
<hr>
<br><br><br><br><br><br><br><br><br><br>

# The entire thing

//@use_codeblock:all,rust

"##,


    code=r##"


//@codeblock-start:all

#[macro_use]
extern crate derive_type_level;
#[macro_use]
extern crate type_level_values;
 

use type_level_values::prelude::*;
 
use std::ops::{Deref,DerefMut};


//@codeblock-start:declare-enum


/// Creating a Mutability enum which reexports the 
/// ConstValue enum equivalent from type_level_Mutability
#[derive(TypeLevel)]
#[typelevel(reexport="pub")]
pub enum Mutability {
    Mutable,
    Immutable,
}

//@codeblock-end:declare-enum

//@codeblock-start:declare-struct

#[derive(MutConstValue)]
#[mcv(
    doc="
        A Wrapper type which allows configuring whether its contents 
        are mutable or not with the `Mut` type parameter.
    ",
    derive(Debug,Clone,PartialEq,PartialOrd,Eq,Ord),
    Type="Wrapper",ConstValue="Mut",
)]
pub struct ___Wrapper<T,Mut>{
    mutability:ConstWrapper<Mut>,
    value:T,
}

impl<T,Mut> Drop for Wrapper_Ty<T,Mut>{
    fn drop(&mut self){
        println!("Inside Drop imlp for Wrapper<T,Mut> ");
    }
}

//@codeblock-end:declare-struct

//@codeblock-start:constructor

impl<T,Mut> Wrapper<T,Mut>{
    fn new(value:T,mutability:Mut)->Self{
        Wrapper{
            value,
            mutability:ConstWrapper::NEW,
        }
    }
}

//@codeblock-end:constructor

//@codeblock-start:deref-impls

impl<T,Mut> Deref for Wrapper<T,Mut>{
    type Target=T;

    fn deref(&self)->&T{
        &self.value
    }
}

impl<T> DerefMut for Wrapper<T,Mutable>{
    fn deref_mut(&mut self)->&mut T{
        &mut self.value
    }
}

//@codeblock-end:deref-impls

/// This function mutates the contents of a Wrapper
fn mutate_wrapped_string(wrapper:&mut Wrapper<String,Mutable>){
    wrapper.push_str(" is");
}



fn main(){

    // the contents of this wrapper are mutable here
    {
        let mut wrapper_mut=Wrapper::new("what".to_string(),Mutable);
        mutate_wrapped_string(&mut wrapper_mut);
        wrapper_mut.push_str(" this thing.");

        assert_eq!(wrapper_mut.as_str(),"what is this thing.")
    }

    // the contents of this wrapper are immutable here
    {
        let mut wrapper_immut=Wrapper::new("what".to_string(),Immutable);
        // mutate_wrapped_string(&mut wrapper_immut);
        // wrapper_immut.push_str(" this thing.");

        assert_eq!(wrapper_immut.as_str(),"what");
    }

    
}






"##,
}

