/*!

This appendix demonstrates how to do control flow on the type level.

# If expressions

If expressions have 2 type-level equivalents:

- [IfEager](../../ops/control_flow/struct.IfEager.html):
    which evaluates the then and else branches before they are taken,
    <br>
    Use this if computing both branches if cheap and does not cause a compile-time error
    (ie:attempting to divide by 0).

- [If](../../ops/control_flow/struct.If.html):
    which only evaluates the branch that was taken,
    but requires using type-level functions for either branch.
    <br>
    Use this if the branch that was not taken would have caused a compile-time error
    (ie:attempting to divide by 0).

### `IfEager` Example

Implementing a TypeFn_ which skips even numbers.

```
#[macro_use]
extern crate type_level_values;

use type_level_values::prelude::*;
use type_level_values::ops::*;

use std::ops::{Add,Rem};

type_fn!{
    pub fn SkipEven[N](N)
    where[
        N:Rem<U2,Output=tmp0>,
        N:Add<U1,Output=plus1>,
        tmp0:ConstEq_<U0,Output=is_even>,
         IfEager< is_even , plus1 , N >:TypeFn_<(),Output=Out>
    ]{ 
        let tmp0;let plus1;let is_even;
        let Out;
        Out 
    }
}

fn main(){
    let _:AssertEq< TypeFn<SkipEven,U0> , U1 >;
    let _:AssertEq< TypeFn<SkipEven,U1> , U1>;
    let _:AssertEq< TypeFn<SkipEven,U2> , U3 >;
    let _:AssertEq< TypeFn<SkipEven,U3> , U3 >;
    let _:AssertEq< TypeFn<SkipEven,U4> , U5 >;
    let _:AssertEq< TypeFn<SkipEven,U5> , U5 >;
    let _:AssertEq< TypeFn<SkipEven,U6> , U7 >;
}

```

### `If` Example

Implementing a function which returns 0 when dividing by 0.

This requires `If` instead of `ÌfEager` since the branch that performs the division must only 
be evaluated when the divisor is known not to be 0.

```
#[macro_use]
extern crate type_level_values;

use type_level_values::prelude::*;
use type_level_values::ops::*;
use type_level_values::fn_adaptors::*;
use type_level_values::fn_types::*;

type_fn!{
    pub fn SafeDiv[Dividend,Divisor](Dividend,Divisor)
    where[
        If<ApplyLhs<ConstEqOp,U0>,
            Const<U0>,
            ApplyLhs<DivOp,Dividend>,
        >:TypeFn_<Divisor,Output=Out>
    ]{ let Out;Out }
}

fn main(){
    let _:AssertEq< TypeFn<SafeDiv,(U10,U0)> , U0 >;
    let _:AssertEq< TypeFn<SafeDiv,(U10,U1)> , U10>;
    let _:AssertEq< TypeFn<SafeDiv,(U10,U2)> , U5 >;
    let _:AssertEq< TypeFn<SafeDiv,(U10,U3)> , U3 >;
}

```


# match expressions

Match expressions can be expressed through multi-branch TypeFn_.

TypeFn_ branches can match any values so long as they don't overlap.

### Example 

Let's implement some boolean operators.

```
#[macro_use]
extern crate type_level_values;
 
use type_level_values::prelude::*;
use type_level_values::ops::*;

type_fn!{
    pub fn Or[B](True ,B){ True }
           Or[B](False,B){ B }
}

type_fn!{
    pub fn And[B](True ,B){ B }
           And[B](False,B){ False }
}

type_fn!{
    pub fn Xor(True ,True ){ False }
           Xor(True ,False){ True  }
           Xor(False,True ){ True  }
           Xor(False,False){ False }
}

fn main(){
    let _:AssertEq< TypeFn<Or,(True ,True )> , True >;
    let _:AssertEq< TypeFn<Or,(True ,False)> , True >;
    let _:AssertEq< TypeFn<Or,(False,True )> , True >;
    let _:AssertEq< TypeFn<Or,(False,False)> , False>;
    
    let _:AssertEq< TypeFn<And,(True ,True )> , True  >;
    let _:AssertEq< TypeFn<And,(True ,False)> , False >;
    let _:AssertEq< TypeFn<And,(False,True )> , False >;
    let _:AssertEq< TypeFn<And,(False,False)> , False >;

    let _:AssertEq< TypeFn<Xor,(True ,True )> , False >;
    let _:AssertEq< TypeFn<Xor,(True ,False)> , True >;
    let _:AssertEq< TypeFn<Xor,(False,True )> , True >;
    let _:AssertEq< TypeFn<Xor,(False,False)> , False >;
}

```

# Iteration

While this library does not have an equivalent of `for` or `while`,
it does provide a variety of [operations on collections](../../collection_ops/index.html)
which serve a similar function.

These are some iterative operations defined on collections:

- FoldL/FoldR: 
    takes an initial value,and a function which incrementally consumes the collection 
    by mutating the initial value.

- ReduceL/ReduceR: 
    Similar to FoldL and FoldR,
    except that it takes the first/last element of the collection instead of an initial value.

- Map:
    Transforms all the elements of the collection with a function.

# Example 

```
#[macro_use]
extern crate type_level_values;
 
use type_level_values::prelude::*;
use type_level_values::ops::*;
use type_level_values::collection_ops::*;
use type_level_values::fn_adaptors::*;
use type_level_values::fn_types::*;

fn main(){

    type Val0= tlist![ U1,U2,U3 ] ;

    let _:AssertEq< FoldL  <Val0,U10,AddOp> , U16 >;
    let _:AssertEq< ReduceL<Val0    ,AddOp> , U6 >;
    
    let _:AssertEq< FoldR  <Val0,U10,SubOp> , U4 >;
    let _:AssertEq< ReduceR<Val0    ,SubOp> , U0 >;

    let _:AssertEq< Map<Val0,ApplyLhs<SubOp,U10>> , tlist![U9,U8,U7] >;
    let _:AssertEq< Map<Val0,ApplyLhs<AddOp,U1 >> , tlist![U2,U3,U4] >;
    
}

```


# Lazy evaluation

This library provides the [Lazy](../../ops/control_flow/struct.Lazy.html) 
helper type for lazily evaluating a function where all the parameters have been passed.
<br>
This is most useful in combination with 
[`If`](../../ops/control_flow/struct.If.html) to only evaluate the constraints of 
the function when that branch was taken.









*/