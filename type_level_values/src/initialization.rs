/*!
Traits,types,`TypeFn_`s related to ConstValue construction.
*/

use field_traits::*;
use prelude::*;
use crate_::ops::{Map_};

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
    #[doc(hidden)]
    pub fn SetInitialized[Field,Value]((Field,Value))
    { (Field,IsInitField<Field>) }
}
