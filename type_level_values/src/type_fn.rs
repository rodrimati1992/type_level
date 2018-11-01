/*!
Contains the TypeFn_ trait,for type-level-functions.

# type_fn macro

The most convenient way to define a type-level function is with the type_fn macro.

[type_fn](../macro.type_fn.html):
    is a macro for declaring type-level-functions.



*/


/**
A type-level function.

Type-level functions is what this library calls every implementor of this trait.

# Implementations

The preferred way to implement this trait is using the type_fn macro.

# Parameters

This is the convention how TypeFn_ gets implemented 
depending on the ammount of parameters it takes:

- 0 parameters: `TypeFn_<()>`.
- 1 parameter : `TypeFn_<Param0>`.
- 2 parameters: `TypeFn_<(Param0,Param1)>`.
- 3 parameters: `TypeFn_<(Param0,Param1,Param2)>`.
- 4 parameters: `TypeFn_<(Param0,Param1,Param2,Param3)>`.
- etc

This has the slight downside that function adaptors have to account for 
functions that take 0 and 1 parameters.


### Example

Implementing a multiply add operation.

```
# #[macro_use]
# extern crate type_level_values;

use type_level_values::std_ops::{MulOp,AddOp};
use type_level_values::prelude::*;


use std::ops::{Mul,Add};

type_fn!{
    pub fn MulAdd[L,R](L,R)
    where[
        L:Mul<R,Output=res0>,
        res0:Add<R,Output=res1>,
    ]{
        let res0;
        let res1;
        res1
    }
}

fn main(){
    let _:U20=TypeFn::<MulAdd,(U4,U4)>::MTVAL;
    let _:U50=TypeFn::<MulAdd,(U4,U10)>::MTVAL;
}


```

A more advanced version,using function composition

```

# #[macro_use]
# extern crate type_level_values;

use type_level_values::std_ops::{MulOp,AddMt};
use type_level_values::prelude::*;


type_fn!{
    pub fn MulAdd[L,R](L,R)
    where[ tlist![ MulOp, AddMt<R> ]:TypeFn_<(L,R),Output=out> ]
    { let out;out }
}

fn main(){
    let _:U20=TypeFn::<MulAdd,(U4,U4)>::MTVAL;
    let _:U50=TypeFn::<MulAdd,(U4,U10)>::MTVAL;
}

```


### Example

Emulating type constructors with a TypeFn_.

```
# #[macro_use]
# extern crate type_level_values;

use type_level_values::prelude::*;


use std::collections::{BTreeSet,BTreeMap};

type_fn!{pub fn VecFn[T](T){ Vec<T> }}

type_fn!{pub fn BTreeSetFn[T](T){ BTreeSet<T> }}

type_fn!{pub fn BTreeMapFn[K,V](K,V){ BTreeMap<K,V> }}

fn main(){
    let _:TypeFn<VecFn,usize> =
        Vec::<usize>::new();

    let _:TypeFn<BTreeSetFn,usize> =
        BTreeSet::<usize>::new();

    let _:TypeFn<BTreeMapFn,(String,usize)> =
        BTreeMap::<String,usize>::new();
}

```

### Example 

Implementing a function which unwraps a type-level Option<_>
or else returns the value from the `default` function.

The `default` capture gets translated to the type parameter of `UnwrapOrElse`.

```
# #[macro_use]
# extern crate type_level_values;

use type_level_values::fn_adaptors::{Const};
use type_level_values::prelude::*;



type_fn!{
    captures(default)

    pub fn
        UnwrapOrElse[T](Some_<T>)
        { T }
        
        UnwrapOrElse(None_)
        where[ default:TypeFn_<()> ]
        { default::Output }

}

fn main(){
    let _:U10=TypeFn::<UnwrapOrElse<Const<U0>>, Some_<U10>>::MTVAL;
    let _:U5 =TypeFn::<UnwrapOrElse<Const<U5>>, None_     >::MTVAL;
}

```



*/
pub trait TypeFn_<Params:?Sized> {
    /// The return value of the function
    type Output;
}

type_fn!{use_trait
    trait=TypeFn_ [Params]
    /// Calls the TypeFn_ `__Self` with the `Params` function parameters.
    type=TypeFn
    /// Calls the TypeFn_ `__Self` with the `Params` function parameters.
    fn_type=TypeFnOp
    /// Calls the TypeFn_ `__Self` with the `Params` function parameters.
    method_like=TypeFnMt
}




/// Calls the TypeFn_ `Func` with `__Self` as the function parameters.
pub trait Piped_<Func>{
    type Output;
}

impl<__Self,Func> Piped_<Func> for __Self
where Func:TypeFn_<__Self>
{
    type Output=Func::Output;
}


/// Calls the TypeFn_ `Func` with `__Self` as the function parameters.
pub type Piped<__Self,Func>=TypeFn<Func,__Self>;


type_fn!{
    /// Calls the TypeFn_ `Func` with `__Self` as the function parameters.
    pub fn PipedOp[__Self,Func](__Self,Func)
    where[ Func:TypeFn_<__Self> ]
    { Func::Output }
}

