use prelude::*;

use crate_::ops::{FoldL_, Map_, TypeFn, TypeFn_};
use crate_::ops::fn_adaptors::ApplyNth;

/////////////////////////////////////////////////////////////////////////////////////////

/// Gets the value of the fields of a ConstValue.
pub trait GetField_<Field>: Sized {
    /// The type of the field.
    type Output;

    /// Returns the ConstValue field.
    fn get_field() -> Self::Output
    where
        Self::Output: MarkerType,
    {
        MarkerType::markertype_val()
    }

    /// Returns the ConstValue field by reference.
    fn get_field_ref<'a>() -> &'a Self::Output
    where
        Self::Output: MarkerType + 'a,
    {
        MarkerType::markertype_ref()
    }

    /// Returns the ConstValue field.
    fn field(self) -> Self::Output
    where
        Self::Output: MarkerType,
    {
        MarkerType::markertype_val()
    }

    /// Returns the ConstValue field by reference.
    fn field_ref<'a>(self) -> &'a Self::Output
    where
        Self::Output: MarkerType + 'a,
    {
        MarkerType::markertype_ref()
    }
}

/// Gets the runtime value of a field of a ConstValue.
pub trait GetFieldRuntime_<Field, RuntimeType>: GetField_<Field> {
    /// The type of the runtime equivalent of `Field`.
    type Runtime;

    /// Returns the runtime value of the field.
    fn get_val() -> Self::Runtime
    where
        GetField<Self, Field>: IntoRuntime<Self::Runtime>,
    {
        Self::Output::to_runtime()
    }

    /// Returns a VariantPhantom wrapping the type of the runtime equivalent of `Field`.
    fn runtime_field_ty(
        self,
        _: Field,
        _: VariantPhantom<RuntimeType>,
    ) -> VariantPhantom<Self::Runtime> {
        PhantomData
    }
}


//////////////////////////////////////////////////////////////////////////////////////////

type_fn!{
    /// Equivalent to `|Struct,Field| Struct[Field] `.
    alias GetFieldOp[This,Field]=GetField_
}

/// Returns the compile-time value of a field.
pub type GetField<This, FieldName> = <This as GetField_<FieldName>>::Output;

/// Returns the runtime type of a field.
pub type GetFieldRuntime<This, FieldName, RuntimeTy> =
    <This as GetFieldRuntime_<FieldName, RuntimeTy>>::Runtime;

//////////////////////////////////////////////////////////////////////////////////////////


/// Allows setting a field of a type-level struct.
pub trait SetField_<Field, Value: ?Sized>: Sized {
    type Output;
}

/// Changes the compile-time value of a field,returning a new struct.
pub type SetField<This, FieldName, Value> = <This as SetField_<FieldName, Value>>::Output;

type_fn!{
    /// Equivalent to `|Struct,Field,Value|{ Struct[Field]=Value; Struct }`.
    alias SetFieldOp[This,Field,Value]=SetField_
}


/**

Sets the fields of Self with the `FVPairs` list of (FieldAccessor,Value) pairs.

`FVPairs` example:tlist![ (field::x,U10), (field::y,U5) ] .

# Example

This example uses the `generic type as type alias` pattern.

```

# #[macro_use]
# extern crate derive_type_level;

# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;
use type_level_values::field_traits::{SetField,SetFields_};

#[derive(TypeLevel)]
#[typelevel(reexport(Struct))]
pub struct Rectangle{
    pub x:u32,
    pub y:u32,
    pub w:u32,
    pub h:u32,
}
use self::type_level_Rectangle::fields;

type InitialRectangle=SetField<
    Rectangle_Uninit,
    fields::All,
    U50
>;

fn reset_width_height<Rect,__RectOut>(_:Rect)->__RectOut
where 
    Rect:SetFields_<tlist![
        (fields::w, U0 ),
        (fields::h, U0 ),
    ],Output=__RectOut>,
    __RectOut:ConstValue,
{
    __RectOut::MTVAL
}

fn main(){
    let initial:ConstRectangle<U50,U50,U50,U50>=
        InitialRectangle::MTVAL;

    let _=reset_width_height(initial) ;

    let _:ConstRectangle<U50,U50,U0,U0>= 
        reset_width_height(initial) ;

}






```

*/
pub trait SetFields_<FVPairs>{
    type Output;
}

impl<This,FVPairs,Out> SetFields_<FVPairs> for This
where 
    FVPairs:FoldL_<This,SetFieldValuePair,Output=Out>
{
    type Output=Out;
}


/// Sets the values of fields in This initializing all the fields with FVPairs.
///
/// `FVPairs` example:tlist![ (field::x,U10), (field::y,U5) ] .
pub type SetFields<This, FVPairs> = <This as SetFields_<FVPairs>>::Output;


//////////////////////////////////////////////////////////////////////////////////////////

/// Set all the fields mentioned in the `Fields` list to the `Value` value .
pub type SetFieldsTo<Struct, Fields, Value> = TypeFn<SetFieldsToOp<Value>, (Struct, Fields)>;

type_fn!{
    captures(Value)
    /// Set all the fields mentioned in the `Fields` list to the `Value` value .
    pub fn SetFieldsToOp[Struct,Fields](Struct,Fields)
    where[
        Fields:FoldL_< Struct ,ApplyNth<SetFieldOp,U2,Value>>
    ]{
        Fields::Output
    }
}

type_fn!{
    /// Sets the fields of Struc with the `FVPairs` list of (FieldAccessor,Value) pairs.
    pub fn SetFieldsOp[Struc,FVPairs](Struc,FVPairs)
    where [ FVPairs:FoldL_<Struc,SetFieldValuePair,Output=Out> ]
    { let Out;Out }
}

type_fn!{
    /// Type-level equivalent of `|Struc,(Field,Value)|{ Struc[Field]=Value; Struc }`
    pub fn SetFieldValuePair[Struc,Field,Value](Struc,(Field,Value))
    where [ Struc:SetField_<Field,Value,Output=Out> ]
    { let Out;Out }
}

//////////////////////////////////////////////////////////////////////////////////////////

/// Type-level equivalent of "|This, Field, Mapper|{ This[Field]=Mapper(This[Field]); This }".
pub type MapField<This, Field, Mapper> = TypeFn<MapFieldOp, (This, Field, Mapper)>;

type_fn!{
    /// Type-level equivalent of "|This, Field, Mapper|{ This[Field]=Mapper(This[Field]); This }".
    pub fn MapFieldOp[This, Field, Mapper](This, Field, Mapper)
    where[
        This: GetField_<Field, Output = Res0>,
        Mapper: TypeFn_<Res0, Output = NewValue>,
        This: SetField_<Field, NewValue, Output = Res2>,
    ]{
        let Res0;let NewValue;let Res2;
        Res2
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////

/// Type-level equivalent of "|This, Field, Mapper|{ This[Field]=Mapper(This); This }".
pub type MapIntoField<This, Field, Mapper> = TypeFn<MapIntoFieldOp, (This, Field, Mapper)>;

type_fn!{
    /// Type-level equivalent of "|This, Field, Mapper|{ This[Field]=Mapper(This); This }".
    pub fn MapIntoFieldOp[This, Field, Mapper](This, Field, Mapper)
    where[
        Mapper: TypeFn_<This, Output = NewValue>,
        This: SetField_<Field, NewValue,Output=Out>,
    ]{
        let NewValue;let Out;
        Out
    }
}

//////////////////////////////////////////////////////////////////////////////////////////

/// Trait for field accessors.
pub trait Field_ {
    /// What type this field is stored inside of.
    type Inside: ConstType;
}

//////////////////////////////////////////////////////////////////////////////////////////


pub mod initialization{
    use super::*;

    /// Trait used by the `constructor` macro to construct a fully initialized version of a value.
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

    /// Constructs a fully initialized value ,
    /// initializing all the fields with FVPairs.
    pub type Construct<Type, FVPairs> = <Type as Construct_<FVPairs>>::Output;

    /// Represents an initialized field.Used by the TypeLevel macro.
    pub struct IsInitField<FieldAccessor>(FieldAccessor);

    /// Represents an uninitialized field.Used by the TypeLevel macro.
    pub struct UninitField<FieldAccessor>(FieldAccessor);

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
        pub fn SetInitialized[Field,Value]((Field,Value))
        { (Field,IsInitField<Field>) }
    }

}




//////////////////////////////////////////////////////////////////////////////////////////



/**
Macro for setting the fields of a compile-time struct.

When constructing a ConstValue prefer using the construct macro 
instead to ensure that all fields are initialized.

# Example 

```
# #[macro_use]
# extern crate derive_type_level;

# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;
# use type_level_values::field_traits::SetField;

#[derive(TypeLevel)]
#[typelevel(reexport(Struct))]
pub struct Rectangle{
    pub x:u32,
    pub y:u32,
    pub w:u32,
    pub h:u32,
}
use self::type_level_Rectangle::fields;

type InitialRectangle=SetField<
    Rectangle_Uninit,
    fields::All,
    U0
>;

type MovedRectangle=set_fields!{InitialRectangle=>
    fields::w=U10,
    fields::h=U5,
};

fn main(){
    let _:ConstRectangle<U0,U0,U0,U0>=InitialRectangle::MTVAL;

    let _:ConstRectangle<U0,U0,U10,U5>=MovedRectangle::MTVAL;

}



```

*/
#[macro_export]
macro_rules! set_fields {
    ()=>{};
    ($this:ty) => { $this };
    ($this:ty => $($field_name:ty=$field_val:ty),* $(,)* ) => {
        <$this as 
            $crate::field_traits::SetFields_< 
                tlist![ $( ($field_name,$field_val) ),* ] 
            >
        >::Output
    };
}
