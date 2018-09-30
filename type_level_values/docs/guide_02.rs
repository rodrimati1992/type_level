doc_code_snippets! {
    mod "guide_02",
    type_ident=Guide02,



    template=r##"


Here's an example of defining and 
using a type-level-enum as a type-parameter to a struct.


Trait/Type glossary:

- ConstValue: A trait for type-level values,also used here to refer to implementors of it.

- PhantomWrapper:
    Zero-sized wrapper type for type-level-values that unconditionally implements 
    Eq/PartialEq/Ord/PartialOrd/Copy/Clone/Debug/etc ,
    delegating IntoRuntime/GetField/SetField/etc to the wrapped type-level-value.


- IntoRuntime : trait for converting a type-level-value to a runtime-value.


//@use_codeblock:declare-enum,ignore

This is the enum that derives TypeLevel,whose type-level version will be used as a 
ConstValue parameter

//@use_codeblock:declare-struct,ignore

Here is the zero-overhead wrapper struct that determines the mutability of its contents based on 
the `Mut` parameter.

The `ConstConstructor` derive macro generates the `Wrapper`
type alias which passes `Mut` wrapped inside a PhantomWrapper
so that `Mut` does not have to implement Debug/PartialEq/etc.

Prefer using the type alias declared by ConstConstructor everywhere,
except the ::std::ops::Drop trait since it requires 
that the generic parameters stay generic and not be wrapped inside another type.

The reason why Mut is wrapped inside a PhantomWrapper in the field type is because
this allows using the repr(transparent) attribute,which is used to guarantee
that a wrapper type has the same representation as its only non-zero-sized field,
and because Rust does not allow type parameters to not be used in a field.


//@use_codeblock:constructor,ignore

This is the constructor for Wrapper,which takes the `Mut` by value,
so as not to require using type inference.

When constructing a PhantomWrapper inside a constructor function prefer using PhantomWrapper::NEW.
<br>
When constructing a PhantomWrapper whose type can't be inferred use Type::PW.
<br>
When there is a value that needs to be converted to a PhantomWrapper 
either use value.to_pw() or value.into() (if the type is Copy),


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

///
/// A Wrapper type which allows configuring whether its contents 
/// are mutable or not with the `Mut` type parameter.
///
#[derive(Debug,Copy,Clone,PartialEq,PartialOrd,Eq,Ord,ConstConstructor)]
#[cconstructor(Type="Wrapper",ConstParam="Mut")]
pub struct WrapperInner<T,Mut>{
    mutability:PhantomWrapper<Mut>,
    value:T,
}

//@codeblock-end:declare-struct

//@codeblock-start:constructor

impl<T,Mut> Wrapper<T,Mut>{
    fn new(value:T,mutability:Mut)->Self{
        Wrapper{
            value,
            mutability:PhantomWrapper::NEW,
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

