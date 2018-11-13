/*!

This appendix demonstrates how to do control flow on the type level.

<br><br>

For information on type-level-functions go:

- To the documentation of [the TypeFn_ trait](../../type_fn/trait.TypeFn_.html)

- To the documentation of [the type_fn macro](../../macro.type_fn.html)

- To the [appendix on functions](../appendix_functions/index.html)


# Sequences

Sequences are tuples or type-lists where every element implements TypeFn_ and
the result of evaluating every function if fed to the next function.
This,in combination with the Piped_ trait , \*Mt functions and unary \*Op functions,
allows emulating method chains.


### Example

Implementing a function which returns whether a number is even.

```
#[macro_use]
extern crate type_level_values;

use type_level_values::prelude::*;
use type_level_values::ops::*;
use type_level_values::std_ops::*;

# fn main(){

type IsEven=(
    BitAndMt<U1>,
    ConstEqMt<U0>,
);

let _:AssertEq< Piped<U0,IsEven>,True >;
let _:AssertEq< Piped<U1,IsEven>,False >;
let _:AssertEq< Piped<U2,IsEven>,True >;
let _:AssertEq< Piped<U3,IsEven>,False >;
let _:AssertEq< Piped<U4,IsEven>,True >;
let _:AssertEq< Piped<U5,IsEven>,False >;

# }

```

This example makes use of the \*Mt variants of the operators,
which apply every parameter except for the Self parameter (which is by convention the first),
emulating a method chain.

This example is equivalent to this:
```ignore
let is_even=|x| x.bit_and(1).eq(0) ;

assert_eq!(is_even(0),true);
assert_eq!(is_even(1),false);
assert_eq!(is_even(2),true);
assert_eq!(is_even(3),false);
```


# If expressions

If expressions take a predicate function ,a Then function,
and an Else  function (with a default value of IdentityFn) as type parameters.

If implements TypeFn_ taking some state ,first passing it to the predicate,
and if the predicate returns True it runs the Then function with the state,
if the predicate returns False it runs the Else function,
returning the result of whichever function ran.

If only evaluates the branch that was taken,meaning that if the branch 
was not taken the constraints of the function are not enforced.

### `If` Example 0

Implementing a function that skips even numbers:

```
#[macro_use]
extern crate type_level_values;

use type_level_values::prelude::*;
use type_level_values::ops::*;
use type_level_values::std_ops::*;

type SkipEven=If< ( BitAndMt<U1>, IsZeroOp ) , Add1Op >;

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
<
### `If` Example 1

Implementing a function which returns 0 when dividing by 0.

```
#[macro_use]
extern crate type_level_values;

use type_level_values::prelude::*;
use type_level_values::ops::*;
use type_level_values::fn_adaptors::*;
use type_level_values::std_ops::*;

type SafeDiv=
    If<(GetRhs,IsZeroOp),
        Const<U0>,
        DivOp,
    >;

fn main(){
    let _:AssertEq< TypeFn<SafeDiv,(U10,U0)> , U0 >;
    let _:AssertEq< TypeFn<SafeDiv,(U10,U1)> , U10>;
    let _:AssertEq< TypeFn<SafeDiv,(U10,U2)> , U5 >;
    let _:AssertEq< TypeFn<SafeDiv,(U10,U3)> , U3 >;
}

```

Function adaptors:

- Const always returns the value it captures.

- GetRhs returns the second parameter ,it is a binary function.


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

- TryFoldL/TryFoldR: 
    Like FoldR,with the ability to return early on a value that converts to TFBreak.

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
use type_level_values::std_ops::*;

fn main(){

    type Val0= tlist![ U1,U2,U3 ] ;

    let _:AssertEq< FoldL  <Val0,U10,AddOp> , U16 >;
    let _:AssertEq< ReduceL<Val0    ,AddOp> , U6 >;
    
    let _:AssertEq< FoldR  <Val0,U10,SubOp> , U4 >;
    let _:AssertEq< ReduceR<Val0    ,SubOp> , U0 >;

    let _:AssertEq< Map<Val0,SubRevMt<U10>> , tlist![U9,U8,U7] >;
    let _:AssertEq< Map<Val0,AddMt<U1>> , tlist![U2,U3,U4] >;
    
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