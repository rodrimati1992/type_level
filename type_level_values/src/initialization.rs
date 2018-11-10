/*!
Traits,types,`TypeFn_`s related to ConstValue construction.
*/

use field_traits::*;
use prelude::*;
use crate_::collection_ops::{Map_};

/// Trait used by the `Construct` type alias to construct a fully initialized version of a value.
///
/// This is automatically implemented by the `TypeLevel` derive macro.
pub trait InitializationValues {
    /// Each field of this must be IsInitField< a type containing the field name >.
    type Uninitialized;

    /// Each field of this must be UninitField< a type containing the field name >.    
    type Initialized;
}

/// Constructs a fully initialized value,initializing all the fields with FVPairs.
///
/// `FVPairs` example:tlist![ (field::x,U10), (field::y,U5) ] .
///
pub trait Construct_<FVPairs>: InitializationValues {
    type Output;
}

/**
Initializes a ConstValue,ensuring that every field is initialized,
otherwise producing a compile-time error which mentions the fields that weren't initialized.

# Usage

Usage of this type alias takes this form:

```text
Construct< <constructor> , (
    $( 
        <field_accessor> = <field_value> ,
    )*
)>
```

You need to use a type-list if you have more than 16 elements,
or are writing a macro which allows any ammount of elements:

```text
Construct< <constructor> , tlist!(
    $(
        <field_accessor> = <field_value>,
    )*
))
```



\<constructor> is a type implementing 
type_level_values::initialization::InitializationValues.

Valid constructors are:
    
- structs:\<DerivingType\>Type / \<DerivingTyoe\>_Uninit.
- variants:\<VariantName\>_Uninit.

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

    let _:Construct<RectangleType,(
        (fields::x,U0),
        (fields::y,U0),
        (fields::w,U0),
        (fields::h,U0),
    )>;

}

```


# Example 2:Constructing a struct with private fields.

Constructing a struct with private fields requires that one uses the 
\<DerivingType>_Uninit constructor,
\<DerivingType\>Type is not a constructor if any field is more private than the struct.

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

    let _:Construct<Game_Uninit,(
        (fields::points , U10),
        (fields::winner , None_),
    )>;

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

    let transfer:Construct<Transfer_Uninit,(
        (fields::type_ , u32),
    )>=Transfer::MTVAL;

    let _:Transfer<u32>=transfer;

    let _:Construct<Loop_Uninit,(
        (fields::repetitions , U10),
        (fields::sequence , tlist![
            Transfer< () >,
            Transfer< Vec<u64> >,
        ]),
    )>;

    let _:Construct< Close_Uninit ,()>;

}

```





*/
pub type Construct<Type, FVPairs> = <Type as Construct_<FVPairs>>::Output;


#[derive(Clone, Copy, Debug, PartialEq, Eq, TypeLevel)]
#[typelevel(
    reexport(Variants),
    items(runtime_conv(NoImpls)),
)]
pub enum FieldInit {
    #[typelevel(doc="Represents an initialized field.Used by the TypeLevel macro.")]
    IsInitField(()),
    #[typelevel(doc="Represents an uninitialized field.Used by the TypeLevel macro.")]
    UninitField(()),
}


impl<F> IntoRuntime<FieldInit> for IsInitField<F>{
    fn to_runtime()->FieldInit{
        FieldInit::IsInitField(())
    }
}

impl<F> IntoRuntime<FieldInit> for UninitField<F>{
    fn to_runtime()->FieldInit{
        FieldInit::UninitField(())
    }
}




impl<Type, FVPairs, Out> Construct_<FVPairs> for Type
where
    Self: InitializationValues,
    ConstructFn: TypeFn_<(Type, FVPairs), Output = Out>,
{
    type Output = Out;
}

type_fn!{
    /// Constructs a fully initialized value from `Type::Uninitialized`,
    /// initializing all the fields with `FVPairs`.
    pub fn ConstructFn[Type,FVPairs](Type,FVPairs)
    where [
        Type:InitializationValues,
        FVPairs:Map_< SetInitialized ,Output=InitFVPairs>,
        SetFieldsOp:TypeFn_<(Type::Uninitialized,FVPairs),Output=Out>,
        SetFieldsOp:TypeFn_<(Type::Uninitialized,InitFVPairs),Output=InitOut>,
        InitOut:TypeIdentity<Type= Type::Initialized >,
    ]{
        let InitFVPairs;
        let Out;
        let InitOut;
        Out
    }
}

type_fn!{
    #[doc(hidden)]
    pub fn SetInitialized[Field,Value]((Field,Value))
    { (Field,IsInitField<Field>) }
}
