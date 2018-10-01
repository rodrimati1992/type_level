

/**

Type macro for a type-level-list.

This macro uses takes these 2 forms:

- tlist![U0,U1,U2,U3] : 
Where one creates an ordered sequence of types,
this example is equivalent to TList<U0,TList<U1,TList<U2,TList<U3,Nil>>>>.

- tlist![False;3] : 
Repeats the same type 3 times,
this example is equivalent to TList<False,TList<False,TList<False,Nil>>>.

# Example 

```
# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;

use std::borrow::Cow;

type FirstPrimes=tlist![ U1,U2,U3,U5,U7,U11,U13,U17,U19,U23 ];

type Strings<'a>=tlist![ String,&'a str,Cow<'a,str> ];

# fn main(){}

```

*/
#[macro_export]
macro_rules! tlist {
    () => { $crate::new_types::TNil };
    (..$rest:ty) => { $rest };
    ($current:ty) => { tlist![$current,] };
    ($element:ty;$repeat:ty) => {
        $crate::ops::Repeat<
            $crate::new_types::type_list::TListType,
            $element,
            $repeat
        >
    };
    ($current:ty, $($rest:tt)*) => {
        $crate::new_types::TList<$current,tlist![$($rest)*]>
    };
}






/** 

Instantiates a type-level-list,
which is just a MarkerType and does not contain instances of the types it lists.

This macro uses takes these 2 forms:

- tlist_val![U0,U1,U2,U3] : 
this is equivalent to <tlist![U0,U1,U2,U3]>::MTVAL.

- tlist_val![False;3] : 
this is equivalent to <tlist![False;3]>::MTVAL.


# Example 

```
# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;

use std::borrow::Cow;

fn main(){

    let first_primes=tlist_val![ U1,U2,U3,U5,U7,U11,U13,U17,U19,U23 ];

    let strings=tlist_val![ String,&str,Cow<str> ];

}

```


*/
#[macro_export]
macro_rules! tlist_val {
    ($($all:tt)*) => {
        < tlist!($($all)*) as $crate::core_extensions::MarkerType >::MTVAL
    };
}







#[macro_export]
/**
Declares a ConstValue type,giving every field a type.

This macro ensures that every field is initialized,
otherwise producing a compile-time error which mentions the fields that weren't initialized.

# Usage

Usage of this type macro takes this form:

```text
construct!( <constructor> => 
    $( 
        <field_accessor> = <field_value> ,
    )*
)
```

\<constructor> is a type implementing 
type_level_values::field_traits::initialization::InitializationValues.

Valid constructors are:
    
- structs:<DerivingTyoe>Type/<DerivingTyoe>_Uninit.
- variants:<DerivingTyoe>_Uninit.

\<field_accessor> is the field accessor in the `type_level_<deriving_type>::fields` submodule.

\<field_value> is the value being assigned to the field.


# Example 1:Constructing a struct with only public fields.

```
# #[macro_use]
# extern crate derive_type_level;
# #[macro_use]
# extern crate type_level_values;


# use type_level_values::prelude::*;


#[derive(TypeLevel)]
#[typelevel(reexport(Struct))]
pub struct Rectangle{
    pub x:u32,
    pub y:u32,
    pub w:u32,
    pub h:u32,
}
use self::type_level_Rectangle::fields;

fn main(){

    let _:construct!{RectangleType=>
        fields::x=U0,
        fields::y=U0,
        fields::w=U0,
        fields::h=U0,
    };

}

```


# Example 2:Constructing a struct with private fields.

```
# #[macro_use]
# extern crate derive_type_level;
# #[macro_use]
# extern crate type_level_values;


# use type_level_values::prelude::*;

#[derive(TypeLevel)]
#[typelevel(reexport(Variants))]
pub enum Player{
    Player0,
    Player1,
}

#[derive(TypeLevel)]
#[typelevel(
    reexport(Struct),
    //print_derive,
)]
pub struct Game{
    points:u32,
    winner:Option<Player>,
}
use self::type_level_Game::fields;

fn main(){

    let _:construct!{Game_Uninit=>
        fields::points = U10,
        fields::winner = None_,
    };

}

```



# Example 3:Constructing enum variants.

```
# #[macro_use]
# extern crate derive_type_level;
# #[macro_use]
# extern crate type_level_values;


# use type_level_values::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, TypeLevel)]
#[typelevel(items(IntoConstType(NoImpls)), reexport(Variants,Discriminants, Traits),)]
// #[typelevel(skip_derive)]
// #[typelevel(print_derive)]
pub enum Operation {
    Transfer { type_: () },
    Loop {
        repetitions: u32,
        sequence: Vec<Operation>,
    },
    Close,
}

use self::type_level_Operation::fields;


fn main(){

    let transfer:construct!{Transfer_Variant=>
        fields::type_ = u32,
    }=Transfer::MTVAL;

    let _:Transfer<u32>=transfer;

    let _:construct!{Loop_Variant=>
        fields::repetitions = U10,
        fields::sequence = tlist![
            Transfer< () >,
            Transfer< Vec<u64> >,
        ],
    };

    let _:construct!{ Close_Variant };

}

```





*/
macro_rules! construct {
    ($name:ty)=>{
        $crate::field_traits::initialization::Construct<$name,tlist![]>
    };
    (
        $name:ty =>
        $( $field_name:ty = $field_ty:ty ),*
        $(,)*
    ) => (
        $crate::field_traits::initialization::Construct<
            $name,
            tlist![
                $( ( $field_name,$field_ty ) ,)*
            ]
        >
    )
}









#[macro_export]
/**
Constructs a ConstValue value,initializing every field.

[
Go to the documentation of the `construct` macro for more details on the syntax for this macro.
](./macro.construct.html)


# Example:Constructing a struct with only public fields.

```
# #[macro_use]
# extern crate derive_type_level;
# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;

#[derive(TypeLevel)]
#[typelevel(reexport(Struct))]
pub struct Rectangle{
    pub x:u32,
    pub y:u32,
    pub w:u32,
    pub h:u32,
}
use self::type_level_Rectangle::fields;

fn main(){

    let value=construct_val!{RectangleType=>
        fields::x=U0,
        fields::y=U0,
        fields::w=U0,
        fields::h=U0,
    };

}

```



*/
macro_rules! construct_val {
    ($($all:tt)*) => {
        < construct!($($all)*) as $crate::core_extensions::MarkerType >::MTVAL
    };
}
