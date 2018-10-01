/*!

This library,which includes type_level_examples/derive_type_level,allows one to 
create type-level equivalents of regular values,and provides tools for using them.

From now on we will refer to type-level-values as ConstValue. 

The purpose of this library is to use Rusts' type system with a value-based approach.
This approach allows one to think about alternative solutions to type-level problem,
where as well as thinking in terms of type and traits one 
can think in terms of type-level functions and values.

# The Guide

[The guide starts here.](../guide_01/index.html)

# Defining a ConstValue.

To define ConstValue simply use the TypeLevel macro on a type definition.

Go to 
[attribute_typelevel](../attribute_typelevel/index.html) 
for more details on using the TypeLevel derive macro.

An example:

```

#[macro_use]
extern crate type_level_values;
#[macro_use]
extern crate derive_type_level;

#[derive(TypeLevel)]
#[typelevel(reexport(Variants))]
pub enum Direction{
    Left,
    Right,
}

# fn main(){}

```


# Defining a type using a ConstValue.

To define such a type we use the ConstConstructor macro

Go to [attribute_const_constructor](../attribute_const_constructor/index.html) for more details on 
the ConstConstructor derive macro.

This is an example example of a wrapper type 
in which the mutability of its contents is determined by a ConstValue parameter.

```

#[macro_use]
extern crate type_level_values;
#[macro_use]
extern crate derive_type_level;

use type_level_values::prelude::*;

use std::ops::{Deref,DerefMut};

fn main(){
    let mut wrapper_a=Wrapper::new("what ".to_string(),Mutable);

    wrapper_a.push_str("is this thing.");
    
    println!("{}",&*wrapper_a);


    let mut wrapper_b=Wrapper::new("what ".to_string(),Immutable);
    
    // Can't compile the next line.
    // wrapper_b.push_str("in the world.");

    println!("{}",&*wrapper_b);
}


#[derive(TypeLevel)]
#[typelevel(reexport(Variants))]
pub enum Mutability{
    Mutable,
    Immutable,
}


#[derive(Debug,Copy,Clone,ConstConstructor)]
#[cconstructor(Type="Wrapper",ConstParam="Mut")]
pub struct WrapperInner<T,Mut>{
    value:T,
    mutability:PhantomWrapper<Mut>,
}

impl<T, M> Wrapper<T, M> {
    pub fn new(value: T, _mutability: M) -> Self {
        Self {
            value,
            mutability: PhantomWrapper::NEW,
        }
    }
}

impl<T, M> Deref for Wrapper<T, M> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> DerefMut for Wrapper<T, Mutable> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}


```





*/
