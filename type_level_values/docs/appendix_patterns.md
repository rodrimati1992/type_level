/*!

This appendix describes some of the patterns in this library.


# Pattern:Generic type as type alias.

This library uses generic types both on impl blocks and functions to alias types/associated types.

In the case of functions the convention is to prefix type aliases with an underscore `"_"`
,eg:`"_Out"`.

#### Impl Example 1

Let's manually implement TypeFn_ for a struct,which adds 3 numbers.

```

# #[macro_use]
# extern crate type_level_values;
# #[macro_use]
# extern crate derive_type_level;

# use type_level_values::prelude::*;
# use type_level_values::ops::*;

# use std::ops::Add;


struct Adds3;

impl<a,b,c,tmp0,Out> TypeFn_<(a,b,c)> for Adds3
where 
    a:Add<b,Output=tmp0>,
    tmp0:Add<c,Output=Out>,
{
    type Output=Out;
}

# fn main(){}

```

The `tmp0` and `Out` generic types here are an example of this pattern.


<br>

#### Impl Example 2

Let's manually implement TypeFn_ for a struct,comparing two 8-ary tuples.

```

# #[macro_use]
# extern crate type_level_values;
# #[macro_use]
# extern crate derive_type_level;

# use type_level_values::prelude::*;
# use type_level_values::std_types::cmp_ordering::OrderingTrait;
# use type_level_values::ops::*;
# use type_level_values::std_ops::*;
# use type_level_values::fn_adaptors::*;


struct Compare8Tuple;

impl<A0,B0,C0,D0,E0,F0,G0,H0,A1,B1,C1,D1,E1,F1,G1,H1,This,Other,Out> 
    TypeFn_<( (A0,B0,C0,D0,E0,F0,G0,H0) , (A1,B1,C1,D1,E1,F1,G1,H1) )> 
for Compare8Tuple
where 
    (A0,B0,C0,D0,E0,F0,G0,H0):TypeIdentity<Type=This>,
    (A1,B1,C1,D1,E1,F1,G1,H1):TypeIdentity<Type=Other>,
    This:ConstOrd_<Other,Output=Out>,
    Out:OrderingTrait,
{
    type Output=Out;
}

# fn main(){}


```

We use `OriginalType:TypeIdentity<Type=Alias>,` 
here to create an alias for `OriginalType`,
note that because TypeIdentity::Type has no constraints 
it forgets all traits implemented by `OriginalType`.

ConstOrd_ is an example where the `generic type as type alias` pattern has downsides,
since one has to repeat the OrderingTrait constraint  when aliasing its Output associated type.


#### Function example 1


```

# extern crate type_level_values;

# use type_level_values::prelude::*;

use std::ops::Add;

pub fn add<L,R,_Out>(l:VariantPhantom<L>,r:VariantPhantom<R>)->_Out
where 
    L:Add<R,Output=_Out>,
    _Out:ConstValue,
{
    _Out::MTVAL
}

fn main(){
    let _    =add(U10::T,U20::T);
    let _:U30=add(U10::T,U20::T);
}


```


#### Function example 2

```

# #[macro_use]
# extern crate derive_type_level;
# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;

use type_level_values::initialization::Construct_;

#[derive(TypeLevel)]
#[typelevel(reexport(Struct))]
pub struct Rectangle{
    pub x:u32,
    pub y:u32,
    pub w:u32,
    pub h:u32,
}

use self::type_level_Rectangle::fields;


pub fn construct<Constr,FieldValues,_Out>(_constr:Constr,_field_values:FieldValues)->_Out
where 
    Constr:Construct_<FieldValues,Output=_Out>,
    _Out:ConstValue,
{
    _Out::MTVAL
}

fn main(){
    let rectangle=construct(Rectangle_Uninit::MTVAL,tlist_val![
        (fields::x,U0),
        (fields::y,U1),
        (fields::w,U50),
        (fields::h,U100),
    ]);

    let _:ConstRectangle<U0,U1,U50,U100>=rectangle;
}


```




# Trait naming convention

The naming convention for traits declared alongside their TypeFn_ and type alias is :
    
- The trait:
    `<Operation>_`.Generally defined with a single associated type `type Output;`.

- The type alias:`<Operation>`.
    This type alias simple delegates to the trait,
    with the Self type as the first parameter of the alias.

- The TypeFn_:`<Operation>Op`.
    This type simple delegates to the trait in the where clause of its TypeFn_ impl,
    with the Self type as the first parameter of the function.

- The method-like TypeFn_ :`<Operation>Mt`.
    This type simple delegates to the trait in the where clause of its TypeFn_ impl,
    with the Self type as the only parameter of the function
    (the rest are generic parameters of the type).
    







*/
