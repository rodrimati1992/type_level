# Context

These are the definitions of items defined in either 
core_extensions/type_level_values that you'll see in the rest of the document.

```

pub trait TypeIdentity{
    type This:?Sized
}
impl<This:?Sized> TypeIdentity for This{
    type This=This;
}


pub type VariantPhantom<T>=
    ::std::marker::PhantomData<fn()->T>;


pub trait TypeFn_<Param:?Sized>{
    type Output;
}
pub type TypeFn<Func,Params>=
    <Func as TypeFn_<Params>>::Output;




```

# Syntactic constructs

### Captures

You can capture generics(lifetimes,types,constants) outside of closures
using  `captures( )` above the function definition.

All closures automatically capture the variables they reference.

### Example

This example assumes that const generics will be available in nightly by 
the time this macro is implemented(it's not a blocker for implementing this macro).

```
type_fn2!{
    captures('a,T,const SIZE:usize)
    pub fn 
        ArrayType(u8 ){ T }
        ArrayType(u16){ &'a T }
        ArrayType(u32){ [&'a T;SIZE] }
        ArrayType(u64){ &'a [&'a T;SIZE] }
}

type Nope=
    ArrayType<'static,usize,300>;

let _:AssertEq< TypeFn<Nope,u8 > ,  usize >;
let _:AssertEq< TypeFn<Nope,u16> ,  &'a usize >;
let _:AssertEq< TypeFn<Nope,u32> ,  [&'a usize;SIZE] >;
let _:AssertEq< TypeFn<Nope,u64> ,  &'a [&'a usize;SIZE] >;
```

# Generic parameters

We can have branches with generic parameters by using the 
`function_name< generics >()` syntax,the same one as Rust.

The limitation of generic parameters in TypeFn_ is that you must use all of them
as part of types in the parameter list,
since you can't specify the generic parameters at the call site

Generic parameters on the generated struct (which implements TypeFn_) 
represents the captured variables,not the generic parameters to the function.

### Example


```
type_fn2!{
    captures(T:?Sized)
    pub fn 
        ReplaceRef<U:?Sized>(Box<U>){ Box<T> }
        
        ReplaceRef<U:?Sized>(Rc<U>){ Rc<T> }
        
        // `#U:?Sized` is equivalent to declaring U 
        // inside the generic parameter list with the `?Sized` bound.
        // It is mostly a matter of taste which one you use
        ReplaceRef(Arc<#U:?Sized>){ Arc<T> }
        
        ReplaceRef<'a,U:?Sized>(&'a U){ &'a T }
        
        ReplaceRef<'a,U:?Sized>(&'a mut U){ &'a mut T }
}

type WTF=ReplaceRef<()>;

let _:AssertEq< TypeFn<WTF,Box<usize>        > , Box<()>         >;
let _:AssertEq< TypeFn<WTF,Rc<usize>         > , Rc<()>          >;
let _:AssertEq< TypeFn<WTF,Arc<usize>        > , Arc<()>         >;
let _:AssertEq< TypeFn<WTF,&'static usize    > , &'static ()     >;
let _:AssertEq< TypeFn<WTF,&'static mut usize> , &'static mut () >;

```

# Where clause 

You cannot use where clauses,
you can however use bound ascription in let bindings to declare bounds.

# Branching on the function level 

It will still be possible to have function level branching.

# Destructuring

Use the '#' character to create new variables (eg: #foo ),
instead of refering to existing ones,
except for `let <ident>=<expr>;`
(that one always creates the ident without needing to use '#').

Use the `_` character to ignore part or all of a type.

Examples:

```
type_fn2!{
    pub fn 
        MaybeAdd(#l,#r,Some_<_>){
            Some_<l.AddOp(r)>
        }
        MaybeAdd(#l,#r,None_){
            None_
        }
}

```

# Bound Ascription

One can ascribe a bound to a pattern by using the `pattern:bound` syntax.

Examples:

```
type_fn2!{
    pub fn TransferOp(Open_Discr,#chan:OpenTrait){
        //Using the if function which takes a state as a function parameter
        let ret:ChannelTrait=
            If<Const< chan::Remaining.EqOp(U0) >,
                NewClosed,
                (Sub1Op,|x| Open<x> )
            >.(chan::remaining);
        ret
    }
}
```
In this example chan is constrained to be `OpenTrait`,and ret is constrained to be `ChannelTrait`.


Doing the equivalent of where clauses:
```
type_fn2!{
    pub fn AssertUsizeInto(#what){ 
        let _:Into<what>=usize; 
        () 
    }
}

```
This just asserts that usize implements `Into<what>`.
We must return `()` explicitly since type-level functions cannot have side-effects,
which is what most `()` returning functions do.

```
type_fn2!{
    pub fn AssertAdded(#l,#r){
        let _:Piped<Add1Op>=l.AddOp(r);
        () 
    }
}

```



### Function calls

Allow doing function calls by using the  <type>.( parameters ) syntax,
`.()` is used here to disambiguate from group expressions.

Examples:
    
    `AddOp.(U2,U3)`,
    
    `SetFieldOp.(AllInitialized,fields::x,IsUninitialized)`
    
    `AssertEqMt<init>.(ÀllUninitialized)`

### Method calls

Allow method calls where the "method" is a TypeFn_ type,
the receiver of the method call is passed  before all others,
exclude parenthesised expressions as part of method types
( Foo.(AddMt<U3>,SubMt<U3>)() would interpret Foo as a function to which we pass 2 parameters),
with a special case for macros 
(eg: foo.tlist!(AddMt<U3>,SubMt<U3>)() 
    would unambiguously be a method call with the type-list as a function type).

Examples:
    
    `U2.AddOp(U3)`,
    
    `AllInitialized.SetFieldOp(fields::x,IsUninitialized)`,
    
    `ÀllUninitialized.AssertEqMt<init>()`


### Pattern matching 

Use pattern matching with 
```
match( <matched_expression> ) {
    <pattern_0> => <block_expression0>
    <pattern_1> => <block_expression1>
    <pattern_2> => <block_expression2>
    <pattern_3> => <block_expression3>
}
```
The parenthesis for match are required to simplify parsing.

match is implemented as an anonymous multi-branch function ,
which captures the variables it references,
and in which every pattern we match on becomes its own function branch.

Examples:
```
let val=match(foo){
    Some_<#bar>=>{
        let dec_1=bar.SatSub1Op();
        OtherFunction.(dec_1,U4);
    }
    None_=>{
        // the type parameter was declared outside the function
        Panic< this_cannot_be_None >.();
    }
};
```

```
let val=match((a,b)){
    (False,#second)=>{ second }
    (True ,#second)=>{ second.NotOp() }
};
```

```
let expected1=ConstructOp.(Rectangle_Uninit,tlist!(
    (rect_field::x,U10),
    (rect_field::y,U20),
    (rect_field::w,U30),
    (rect_field::h,U40),
));
let expected2=ConstructOp.(Circle_Uninit,tlist!(
    (circle_field::x,U10),
    (circle_field::y,U20),
    (circle_field::diameter,U40),
));

// The patterns are matching against the expected1/expected2,not shadowing them.
let val=match( foo ){
    expected1 =>{ U0 }
    expected2 =>{ U1 }
    Square<_,#sy,U10> =>{ sy }
};
```

### Lambdas (AKA Closures)

You can declare lambdas using these syntaxes
```
|param0,param1| <expression>
```
or
```
|param0,param1| <block_expression>
```

lambdas can only be declared at the start of an [sub-]expression and 
everything in that [sub-]expression is interpreted as being part of the lambda.

Examples:

```
let (func,operands)=(|params|AddOp(params),(U2,U3));
AssertEqOp(func.(operands),U5);
U5
```

```
let val1:RectangleTrait = val0.MapFieldOp(field::x,|x|x.AddOp(U10));
AssertEqOp.(val1::x,U20);
```
```
let val1=(|x|x.AddOp(U10)).(U10);
AssertEqOp.(val1,U20);
```

```
let add2=|x|x.AddOp(U2);
add2.(U10)
```

Invalid examples(These examples are errors):


```
foo.(|x|x.AddOp(U10))
   .(|y|y.MulOp(U5))
```
This interprets foo as being a function which takes a closure and 
returns a function which takes a closure.

```
foo.|x|x.AddOp(U10)
   .|y|y.MulOp(U5)
```
This is ambiguous as to where the first closure starts 
and ends
.
To fix both examples use PipedOp:
```
foo.PipedOp(|x|x.AddOp(U10))
   .PipedOp(|y|y.MulOp(U5))
```
or just call AddOp and MulOp directly
```
foo.AddOp(U10)
   .MulOp(U5)
```

# Shadowing

Shadowing is allowed in any scope.

Be aware that patterns in match don't create a new variable unless you prefix it with '#',
otherwise it just matches against it.

Let bindings have the special case of `let ident=` for ergonomics,
this implicitly declares a new variable.


### Reserved syntax

Reserve the `(:ident(optional-parameter-list)<a-space-or-newline><anything_here>)` 
syntax for extensions to the macro which don't use language keywords.

Examples(no promises of new features here):

Say that someone wants to implement async-await on the type-level:
```
(:async(initial_val)
    let val1=initial_val.Add1Op();
    //yield returns what this is passed as parameter when called again as a function.
    let val2=(yield val1).Add1Op(); 
    val2
)
```
Every time we yield this would return a new function with the remaining code.



# Generated code 

### Initial optimizations

Optimize closures of the form `|x|match(x){...}` to only create TypeFn_ impls
for the match statement itself.

Optimize expressions containing only a type(generic or not) (with no function calls)
so that expression is used directly in the assigned variable or the `TypeFn_::Output`.


### Variables

Every variable declared outside a closure (every control flow structure is a closure)
using `let <varname>` or `#<varname>` is declared in the impl header.

Every temporary is translated to a variable in the impl header named like this:
```
__temporary_S<statement_number>_V<num_in_statement>
```

Every `_` used as an ignore pattern is translated to:
```
__ignore_S<statement_number>_V<num_in_statement>
```

For the purposes of generating identifiers the parameter list is considered the 'P' statement.


When a variable is shadowed,the generated identifier of the new variable is :
```
__shadowing_<number_of_shadowing>_<variable_name>
```

If a closure (this includes match) shadows a variable without capturing 
the variable it shadowed,there is no need to create a shadowed name for 
the new variable in the generated code of that closure.

### Generating closures

Because closures can capture named variables,
we generate structs in which each generic parameter is the same name as the variables they capture.

Then we generate an impl block with the contents of the closure,
following the same rules as a top-level function for names.

The generated code for the function passes the captures for the closure explicitly.

One important detail is that if we have nested closures,
every closure most pass the captured variables that their inner closures need to them.


##### Closure naming:

Closures are named like this:
```
__<name_of_top_level_type_fn>_C<closure_nesting>_S<statement_number>_V<closure_in_statement>
```

<name_of_top_level_type_fn> is the name of the top-level function .

<closure_nesting> 
    is how many closures is the declared closure nested inside of,
    starting at 0 for the top_level function,1 inside of a closure,2 inside 2 closures,etc.





### Example:how variables and temporaries and named.

```
type_fn2!{
    pub fn 
        function(Some_<_>){
            let a=U2;
            let b=U2;
            let ret=a
                .AddOp(b)
                .MulOp(b);
            // This is a silly thing to write
            let Some_<_>=Some_<U10>;
            ret
        }
}
```

The generated impl block is:

```
impl<
    a,
    b,
    ret,
    __ignore_SP_V0,
    __ignore_S0_V0,
    __temporary_S2_V0,
    __temporary_S2_V1,
> 
    TypeFn_<Some_<__ignore_SP_V0>> 
for function 
where
    AddOp:TypeFn_<(a,b),Output=__temporary_S2_V0>,
    MulOp:TypeFn_<(__temporary_S2_V0,b),Output=__temporary_S2_V1>,
    Some_<U10>:TypeIdentity<Type= Some_<__ignore_S0_V0> >
{
    type Output=__temporary_S2_V1;
}
```


### Example:nested closures

```
type_fn2!{
    pub fn foo(#nope){
        |#a|{
            let _:Debug=nope;
            let _=|_|();
            |#b|{
                a.SubOp(b)
            };
        }
    }
}

let _:AssertEq<TypeFn<TypeFn<foo,U10>,U0> ,U10>;
let _:AssertEq<TypeFn<TypeFn<foo,U10>,U4> ,U6>;
let _:AssertEq<TypeFn<TypeFn<foo,U6 >,U4> ,U2>;

```

The generated code is:

```

/// This is the pub fn foo(#nope){...}
struct foo;

impl<nope> TypeFn_<nope> for foo{
    type Output=__foo_C0_S0_V0<nope>;
}


// This is the `|#a|{ ... }` closure
//
#[doc(hidden)]
pub struct __foo_C0_S0_V0<nope>(
    VariantPhantom<(
        VariantPhantom<nope>,
    )> 
);

impl<a,nope,__ignore_S0_V0,__ignore_S1_V0,__temporary_S2_V0>
    TypeFn_<a> 
for __foo_C0_S0_V0<nope>
where
    nope:TypeIdentity<Type=__ignore_S0_V0>,
    nope:Debug,
    __foo_C1_S0_V0:TypeIdentity<Type=__ignore_S1_V0>,
    __foo_C1_S1_V0<a>:TypeIdentity<Type=__temporary_S2_V0>
{
    type Output=__temporary_S2_V0;
}



// This is the `|_|()` closure
//
#[doc(hidden)]
pub struct __foo_C1_S0_V0;

impl<__ignore_SP_V0> TypeFn_<__ignore_SP_V0> for __foo_C1_S0_V0{
    type Output=();
}



// This is the `|#b|{ ... }` closure
//
#[doc(hidden)]
pub struct __foo_C1_S1_V0<a>(
    VariantPhantom<(
        VariantPhantom<a>,
    )> 
);

impl<a,b,__temporary_S0_V0> TypeFn_<b> for __foo_C1_S0_V0<a>
where
    SubOp:TypeFn_<(a,b),Output=__temporary_S0_V0>
{
    type Output=__temporary_S0_V0;
}


```

# Example



```
type_fn2!{
    captures('a.'b:'a,T:'a+Send)

    pub fn 
        ClusterFasteriskck<B:Sync>(B){
            EqOp(T,B)
        }

        ClusterFasteriskck<B:Sync>((T,B)) {
            usize:Into<T>;

            let (#T2,#B2)=(T.Add1Op(),B.Add1Op());
            
            let _:Piped_<ApplyRhs<AddOp,U10>>=T2.AddOp(B2);

            // this is equivalent to the previous one
            let _:Piped_<ApplyRhs<AddOp,U10>>=T2.SubOp(B2);

            let (_:Debug,_:Display)=(T2.MulOp(U1),T2.MulOp(U2));

            // You can create nested scopes with closures
            let ret=(|#range:RangeTrait,#expected|{
                let ret=range
                    .FoldLOp(U0,AddOp)
                    .AssertEqOp(expected);

                |x|match(x){
                    Some_<ret>=>{ ret }
                    None_=>{ U0 }
                }
            }).();

            let ret=(ret,ret);

            ret
        }
}


```

That generates the following code

```

struct ClusterFasteriskck<'a,'b,T>(
    VariantPhantom<(
        VariantPhantom<&'a ()>,
        VariantPhantom<&'b ()>,
        VariantPhantom<T>,
    )>
);


impl<'a,'b:'a,T:'a+Send,B:Sync,__temporary_S0_V0> 
    TypeFn_<B> 
for ClusterFasteriskck<'a,'b,T>
where
    EqOp:TypeFn_<(T,B),Output=__temporary_S0_V0>
{
    type Output=__temporary_S0_V0;
}

impl<
    'a,
    'b:'a,
    T:'a+Send,
    B:Sync,
    T2,
    B2,
    ret,
    __shadowing_0_ret,
    __temporary_S1_V0,
    __temporary_S1_V1,
    __temporary_S4_V0,
    __temporary_S4_V1,
    __ignore_S2_V0,
    __ignore_S3_V0,
    __ignore_S4_V0,
    __ignore_S4_V1,
> 
    TypeFn_<(T,B)>
for ClusterFasteriskck<'a,'b,T>
where
    usize:Into<T>,
    
    Add1Op:TypeFn_<T,Output=__temporary_S1_V0>,
    Add1Op:TypeFn_<B,Output=__temporary_S1_V1>,
    (__temporary_S1_V0,__temporary_S1_V1):TypeIdentity<Type=(T2,B2)>,
    
    AddOp:TypeFn_<(T2,B2),Output=__ignore_S2_V0>,
    __ignore_S2_V0:Piped_<ApplyRhs<AddOp,U10>>,
    
    SubOp:TypeFn_<(T2,B2),Output=__ignore_S3_V0>;
    __ignore_S3_V0:Piped_<ApplyRhs<AddOp,U10>>,
    
    MulOp:TypeFn_<(T2,U0),Output=__temporary_S4_V0>,
    MulOp:TypeFn_<(T2,U1),Output=__temporary_S4_V1>,
    (__temporary_S4_V0,__temporary_S4_V1):
        TypeIdentity<Type=(__ignore_S4_V0,__ignore_S4_V1)>;
    __ignore_S4_V0:Debug,
    __ignore_S4_V1:Display,
    
    __foo_C0_S0_V0:TypeIdentity<Type=ret>,

    (ret,ret):TypeIdentity<Type=__shadowing_0_ret>,

{
    type Output=__shadowing_0_ret;
}

struct __foo_C0_S0_V0;

impl<
    range,
    expected,
    ret,
    __temporary_S0_V0,
> TypeFn_<(range,expected)> for __foo_C0_S0_V0 
where
    range:RangeTrait,
    FoldLOp:TypeFn_<(range,U0,AddOp),Output=__temporary_S0_V0>,
    AssertEqOp:TypeFn_<(__temporary_S0_V0,expected),Output=ret>,
{
    type Output=__foo_C1_S1_V0<ret>;
}


struct __foo_C1_S1_V0<ret>(
    VariantPhantom<(
        VariantPhantom<ret>,
    )>
);

impl<ret,x> TypeFn_<Some_<x>> __foo_C1_S1_V0<ret> {
    type Output=ret;
}

impl<ret> TypeFn_<None_> __foo_C1_S1_V0<ret> {
    type Output=U0;
}



``` 