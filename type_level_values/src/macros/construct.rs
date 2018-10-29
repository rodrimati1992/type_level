




/**
Initializes a ConstValue,ensuring that every field is initialized,
otherwise producing a compile-time error which mentions the fields that weren't initialized.

This is a type macro,for a macro that produces a runtime value please look at 
[construct_val](./macro.construct_val.html).

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
type_level_values::initialization::InitializationValues.

Valid constructors are:
    
- structs:\<DerivingType\>Type / \<DerivingType\>_Variant / \<DerivingTyoe\>_Uninit.
- variants:\<VariantName\>_Variant / \<VariantName\>_Uninit.

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

Constructing a struct with private fields requires that one uses the 
<DerivingType>_Uninit constructor,
<DerivingType>Type is not a constructor if any field is more private than the struct.

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
#[macro_export]
macro_rules! construct {
    ($name:ty)=>{
        $crate::initialization::Construct<$name,tlist![]>
    };
    (
        $name:ty =>
        $( $field_name:ty = $field_val:ty ),*
        $(,)*
    ) => (
        $crate::initialization::Construct<
            $name,
            tlist![
                $( ( $field_name,$field_val ) ,)*
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


# Example:Constructing a struct with no private fields.

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
