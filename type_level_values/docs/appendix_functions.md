/*!

This library provides type-level functions,which are implementors of the TypeFn_ trait,
providing a way to express and compose computations on the type-level.

For more information go:

- To the documentation of [the TypeFn_ trait](../../type_fn/trait.TypeFn_.html)

- To the documentation of [the type_fn macro](../../macro.type_fn.html)

- To the [appendix on control flow](../appendix_control_flow/index.html)

# Piped

Piped_ is an alternative syntax for calling a function,
where the parameters and the function are reversed,
most useful when composing multiple functions such that they span multiple lines.

An example of using it in a where clause:
```
# #[macro_use]
# extern crate type_level_values;
 
# use type_level_values::prelude::*;
use type_level_values::std_ops::*;
use type_level_values::ops::{MinMaxOp,SatSub1Op};
use type_level_values::type_fn::Piped_;



type_fn!{
    pub fn Example[L,R](L,R)
    where [
        (L,R):Piped_<(
            MinMaxOp, 
            SubRevOp,
            If<ConstLtMt<U10>,
                MulMt<U2>,
                (
                    DivMt<U2>,
                    SatSub1Op,
                ),
            >
        ),Output=Out >,
    ]{
        let Out;Out
    }
}

# fn main(){}

```
    
An example of using it in a type alias:
`type MulAdd<L,R>=Piped<L,(MutMt<R>,AddMt<R>)>;`

# Multiple branch function.

Functions can have multiple branches,where calling the function takes the 
branch that matches the parameters,this is most useful for emulating match expressions.

Note that because this is implemented as multiple impls of TypeFn_ for a struct,
the multiple branches can't specialize any other branch.

### Example

Say that we want to implement the `Option::map_or_else` method on the type level.

```
# #[macro_use]
# extern crate type_level_values;
 
# use type_level_values::prelude::*;
use type_level_values::std_types::cmp_ordering::*;
use type_level_values::ops::AssertEq;
use type_level_values::std_ops::*;
use type_level_values::fn_adaptors::*;


type_fn!{
    pub fn 
        MapOrElse[V,Mapper,Else](Some_<V>,Mapper,Else)
        where[ Mapper:TypeFn_<V> ]
        { Mapper::Output }

        MapOrElse[Mapper,Else](None_,Mapper,Else)
        where[ Else:TypeFn_<()> ]
        { Else::Output }
}

fn main(){
    let _:AssertEq< TypeFn<MapOrElse,(None_     , ApplyRhs<AddOp,U10> , Const<U7> )> , U7 >;
    let _:AssertEq< TypeFn<MapOrElse,(Some_<U100>, ApplyRhs<AddOp,U10> , Const<U7> )> , U110 >;
}

```

# Function composition

What this library calls function composition is taking 
multiple functions and producing a type which is itself a function.

# Ways to compose functions

The ways to compose functions are:

- Having a tuple/type-level-list entirely composed of TypeFn_,
    all elements of which take the return value of the previous TypeFn_.

- Using a function adaptor from type_level_value::fn_adaptors.

# Function adaptors

Function adaptors are generic types which implement TypeFn_,
taking other TypeFn as parameters (either as captures or as a function parameter).

Most of them are declared [in the fn_adaptors module](../../fn_adaptors/index.html).

The examples will use these adaptors:
 [ApplyRhs](../../fn_adaptors/struct.ApplyRhs.html)
/[ApplyLhs](../../fn_adaptors/struct.ApplyLhs.html)
/[ApplyNonSelf](../../fn_adaptors/type.ApplyNonSelf.html)

### Example 1

Creating a function which multiplies a number by 2.

```
# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;
use type_level_values::ops::AssertEq;
use type_level_values::fn_adaptors::*;
use type_level_values::std_ops::*;

fn main(){
    type Mul2=ApplyRhs<MulOp,U2>;

    let _:AssertEq< TypeFn<Mul2,U0> , U0 >;
    let _:AssertEq< TypeFn<Mul2,U1> , U2 >;
    let _:AssertEq< TypeFn<Mul2,U2> , U4 >;
    let _:AssertEq< TypeFn<Mul2,U3> , U6 >;

}

```


### Example 2

Creating a function which sets a field to U0.

```
# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;
use type_level_values::ops::AssertEq;
use type_level_values::fn_adaptors::*;
use type_level_values::std_ops::*;
use type_level_values::field_traits::{SetFieldOp};

fn main(){
    type ToU0=ApplyNth< SetFieldOp,U2,U0 >;

    let _:AssertEq< TypeFn<ToU0,((U10,U20,U30,U40),U0)> ,(U0 ,U20,U30,U40)>;
    let _:AssertEq< TypeFn<ToU0,((U10,U20,U30,U40),U1)> ,(U10,U0 ,U30,U40)>;
    let _:AssertEq< TypeFn<ToU0,((U10,U20,U30,U40),U2)> ,(U10,U20,U0 ,U40)>;
    let _:AssertEq< TypeFn<ToU0,((U10,U20,U30,U40),U3)> ,(U10,U20,U30,U0 )>;

}

```


### Example 3

Creating a function which divides a field by 2.

```
# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;
use type_level_values::ops::AssertEq;
use type_level_values::fn_adaptors::*;
use type_level_values::std_ops::*;
use type_level_values::field_traits::{MapFieldOp};

fn main(){
    type MapDiv2<Field>=
        ApplyNonSelf<
            MapFieldOp,
            (Field,ApplyRhs<DivOp,U2>)
        >;

    let _:AssertEq< TypeFn<MapDiv2<U0>,(U10,U20,U30,U40)> , (U5 ,U20,U30,U40) >;
    let _:AssertEq< TypeFn<MapDiv2<U1>,(U10,U20,U30,U40)> , (U10,U10,U30,U40) >;
    let _:AssertEq< TypeFn<MapDiv2<U2>,(U10,U20,U30,U40)> , (U10,U20,U15,U40) >;
    let _:AssertEq< TypeFn<MapDiv2<U3>,(U10,U20,U30,U40)> , (U10,U20,U30,U20) >;

}

```



### Example 4

Copying one field into another.

```
# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;
use type_level_values::ops::AssertEq;
use type_level_values::fn_adaptors::*;
use type_level_values::std_ops::*;
use type_level_values::field_traits::{GetFieldMt,MapIntoFieldOp};


fn main(){

    type CopyField<From,To>=
        ApplyNonSelf<
            MapIntoFieldOp,
            (To,GetFieldMt<From>)
        >;

    let _:AssertEq<
        TypeFn<CopyField<U2,U0>,(U20,U40,U60)>,
        (U60,U40,U60)
    >;

    let _:AssertEq<
        TypeFn<CopyField<U0,U1>,(U20,U40,U60)>,
        (U20,U20,U60)
    >;
}



```


# Sequencing

Sequences are tuples or type-level-lists where every element implements TypeFn_ and
the return value of every function is fed to the next function.


### Example 1

Creating a type-level function which wraps the type T in a `Arc<Mutex<Vec<Option<T>>>>`.

```
# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;

use type_level_values::ops::*;

use std::sync::{Mutex,Arc};

type_fn!{ pub fn OptionFn[T](T){Option<T>} }
type_fn!{ pub fn VecFn[T](T){Vec<T>} }
type_fn!{ pub fn MutexFn[T](T){Mutex<T>} }
type_fn!{ pub fn ArcFn[T](T){Arc<T>} }

fn main(){
    let _:AssertEq< 
        TypeFn< OptionFn ,u32>, 
        Option<u32> 
    >={
        None
    };
    
    let _:AssertEq< TypeFn<( OptionFn,VecFn ),u32>, Vec<Option<u32>> >= {
        vec![ Some(10) , None ]
    };
    
    let _:AssertEq< 
        TypeFn<( OptionFn,VecFn,MutexFn ),u32>, 
        Mutex<Vec<Option<u32>>> 
    >={
        Mutex::new( vec![ None , None ] )
    };
    
    let _:AssertEq< 
        TypeFn<( OptionFn,VecFn,MutexFn,ArcFn ), u8 >,
        Arc<Mutex<Vec<Option< u8 >>>>
    >={
        10
        .piped(Some)
        .piped(|x| vec![x;10] )
        .piped(Mutex::new)
        .piped(Arc::new)
    };
    
    let _:AssertEq< 
        TypeFn<( OptionFn,VecFn,MutexFn,ArcFn ), String >,
        Arc<Mutex<Vec<Option< String >>>>
    >={
        "what the"
        .to_string()
        .piped(Some)
        .piped(|x| vec![x;10] )
        .piped(Mutex::new)
        .piped(Arc::new)
    };
}

```

`AssertEq` is a type alias that assert that both types are the same,evaluating to the second one.

The `piped` method is defined like this:

```ignore
fn piped<F, U>(self, f: impl FnOnce(Self) -> U) -> U {
    f(self)
}
```



### Example 2

Implementing a multiply-add function.

```
# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;
use type_level_values::ops::AssertEq;
use type_level_values::fn_adaptors::*;
use type_level_values::std_ops::*;

fn main(){

    type_fn!{
        pub fn MulAdd[L,R](L,R)
        where[ (L,R):Piped_<(MulOp,AddMt<R>),Output=Out>  ]
        { let Out;Out }
    }

    let _:AssertEq<TypeFn<MulAdd,(U1,U1)>,U2>;
    let _:AssertEq<TypeFn<MulAdd,(U1,U2)>,U4>;
    let _:AssertEq<TypeFn<MulAdd,(U1,U3)>,U6>;
    let _:AssertEq<TypeFn<MulAdd,(U1,U4)>,U8>;
    let _:AssertEq<TypeFn<MulAdd,(U2,U1)>,U3>;
    let _:AssertEq<TypeFn<MulAdd,(U2,U2)>,U6>;
    let _:AssertEq<TypeFn<MulAdd,(U2,U3)>,U9>;
    let _:AssertEq<TypeFn<MulAdd,(U2,U4)>,U12>;
}

```


*/