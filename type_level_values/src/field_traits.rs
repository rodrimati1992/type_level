use prelude::*;

use crate_::ops::{FoldL_, Map_, TypeFn, TypeFn_,};

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

/// Trait for field accessors.
pub trait Field_ {
    /// What type this field is stored inside of.
    type Inside: ConstType;
}

//////////////////////////////////////////////////////////////////////////////////////////

type_fn!{alias GetFieldFn[This,Field]=GetField_}
type_fn!{alias SetFieldFn[This,Field,Value]=SetField_}

//////////////////////////////////////////////////////////////////////////////////////////

/// Set all the fields mentioned in the `Fields` type-level-list to the `Value` value .
pub type SetFieldsTo<Struct, Fields, Value> = TypeFn<SetFieldsToOp<Value>, (Struct, Fields)>;

type_fn!{
    captures(Value)
    /// Sets the fields listed in the `Fields` type-level-list to the `Value` value.
    pub fn SetFieldsToOp[Struct,Fields](Struct,Fields)
    where[
        Fields:FoldL_< Struct , SetFieldToOp<Value> >
    ]{
        Fields::Output
    }
}

type_fn!{
    captures(Value)
    #[doc(hidden)]
    /// Type-level equivalent of `|Struc,Field|{ Struc[Field]=Value; Struc }`
    pub fn SetFieldToOp[Struct,Field](Struct,Field)
    where [ SetFieldOp<Field,Value>:TypeFn_<Struct,Output=Out> ]
    { let Out;Out }
}


type_fn!{
    /// Sets the fields of Struc with the `FVPairs` list of (FieldAccessor,Value) pairs.
    pub fn SetFieldValuePairs[Struc,FVPairs](Struc,FVPairs)
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

type_fn!{
    captures(Field,Value)
    /// Type-level equivalent to `|Struct|{ Struct[Field]=Value; Struct }`.
    pub fn SetFieldOp[Struct](Struct)
    where[
        Struct:SetField_<Field,Value>
    ]{
        Struct::Output
    }
}

type_fn!{
    captures(Field)
    /// Equivalent to `|Struct| Struct[Field] `.
    pub fn GetFieldOp[Struct](Struct)
    where[
        Struct:GetField_<Field>
    ]{
        Struct::Output
    }
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

/// Trait used by the `constructor` macro to construct a fully initialized version of a value.
/// 
/// This is automatically implemented by the `TypeLevel` derive macro.
pub trait InitializationValues {
    /// Each field of this must be IsInitField< a type containing the field name >.
    type Uninitialized;

    /// Each field of this must be UninitField< a type containing the field name >.    
    type Initialized;
}

/// Constructs a fully initialized value from `Self::Uninitialized`,
/// initializing all the fields with FVPairs.
///
/// `FVPairs` example:tlist![ (field::x,U10), (field::y,U5) ] .
///
pub trait Construct_<FVPairs>: InitializationValues {
    type Output;
}

/// Constructs a fully initialized value ,
/// initializing all the fields with FVPairs.
pub type Construct<Type, FVPairs> = <Type as Construct_<FVPairs>>::Output;

#[doc(inline)]
pub use self::construct_helpers::ConstructFn;

/// Represents an initialized field
pub struct IsInitField<FieldAccessor>(FieldAccessor);

/// Represents an uninitialized field
pub struct UninitField<FieldAccessor>(FieldAccessor);

/// All the helper functions/for `Construct`.
pub mod construct_helpers {
    use super::*;

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
            SetFieldValuePairs:TypeFn_<(Type::Uninitialized,FVPairs),Output=Out>,
            SetFieldValuePairs:TypeFn_<(Type::Uninitialized,InitFVPairs),Output=InitOut>,
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

/// Represents a `field=value` pair.
pub trait FieldValue {
    type Field;
    type Value;
}

impl<FieldName, Value> FieldValue for (FieldName, Value) {
    type Field = FieldName;
    type Value = Value;
}

/// Allows setting a field of a type-level struct.
pub trait SetField_<Field, Value: ?Sized>: Sized {
    type Output;
}

/// Returns the compile-time value of a field.
pub type GetField<This, FieldName> = <This as GetField_<FieldName>>::Output;

/// Returns the runtime type of a field.
pub type GetFieldRuntime<This, FieldName, RuntimeTy> =
    <This as GetFieldRuntime_<FieldName, RuntimeTy>>::Runtime;

/// Changes the compile-time value of a field,returning a new struct.
pub type SetField<This, FieldName, Value> = <This as SetField_<FieldName, Value>>::Output;



/// Sets the values of fields in This initializing all the fields with FVPairs.
///
/// `FVPairs` example:tlist![ (field::x,U10), (field::y,U5) ] .
pub type SetFields<This, FVPairs> = TypeFn<SetFieldValuePairs,(This,FVPairs)>;

/// Macro for more conveniently setting the fields of a compile-time struct.
///
#[macro_export]
macro_rules! set_fields {
    ()=>{};
    ($this:ty) => { $this };
    ($this:ty => $($field_name:ty=$field_val:ty),* $(,)* ) => {
        $crate::field_traits::SetFields<
            $this,
            tlist![ $( ($field_name,$field_val) ),* ]
        >
    };
}


