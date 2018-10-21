/*!
Traits and `TypeFn`s for manipulating fields.

*/


use prelude::*;

use crate_::collection_ops::{FoldL_, Map_};
use crate_::fn_types::{SubOp};
use crate_::fn_adaptors::{ApplyNth,ApplyRhs};

/////////////////////////////////////////////////////////////////////////////////////////

/// Gets the value of a field of a ConstValue.
pub trait GetField_<Field> {
    /// The type of the field.
    type Output;
}

/// Gets the runtime value of a field of a ConstValue.
pub trait GetFieldRuntime_<Field, RuntimeType>: GetField_<Field> {
    /// The type of the runtime equivalent of `Field`.
    type Runtime;

    /// Returns the runtime value of the field.
    fn get_field_runtime() -> Self::Runtime
    where
        GetField<Self, Field>: IntoRuntime<Self::Runtime>,
    {
        Self::Output::to_runtime()
    }
}


//////////////////////////////////////////////////////////////////////////////////////////

type_fn!{
    /// Equivalent to `|This,Field| This[Field] `.
    alias GetFieldOp[This,Field]=GetField_
}

/// Equivalent to `|This,Field| This[Field] `.
pub type GetField<This, FieldName> = <This as GetField_<FieldName>>::Output;

type_fn!{
    /// Equivalent to `|This| This[Field] `.
    captures(Field)
    pub fn GetFieldMt[This](This)
    where[ This:GetField_<Field> ]
    { This::Output }
}

type_fn!{
    pub fn GetFieldRuntimeOp[This,Field,RuntimeTy](This,Field,RuntimeTy)
    where[ This:GetFieldRuntime_<Field,RuntimeTy> ]
    { This::Runtime }
}

/// Returns the runtime type of a field.
pub type GetFieldRuntime<This, FieldName, RuntimeTy> =
    <This as GetFieldRuntime_<FieldName, RuntimeTy>>::Runtime;

//////////////////////////////////////////////////////////////////////////////////////////


/// Sets the value of a field of a ConstValue.
pub trait SetField_<Field, Value: ?Sized>: Sized {
    type Output;
}

/// Changes the compile-time value of a field,returning a new struct.
pub type SetField<This, FieldName, Value> = <This as SetField_<FieldName, Value>>::Output;

type_fn!{
    /// Equivalent to `|This,Field,Value|{ This[Field]=Value; This }`.
    alias SetFieldOp[This,Field,Value]=SetField_
}

type_fn!{
    captures(Field,Value)
    /// Equivalent to `|This|{ This[Field]=Value; This }`.
    pub fn SetFieldMt[This](This)
    where[ This:SetField_<Field,Value> ]
    { This::Output }
}


/**

Sets the value of fields of a ConstValue with a list of (FieldAccessor,Value) pairs.

Example of the list:`tlist![ (field::x,U10), (field::y,U5) ]` .

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
    captures(FVPairs)
    /// Sets the values of fields in FVPairs.
    ///
    /// `FVPairs` example:tlist![ (field::x,U10), (field::y,U5) ] .
    pub fn SetFieldsMt[This](This)
    where[ This:SetFields_<FVPairs> ]
    { This::Output }
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

type_fn!{
    captures(Field, Mapper)
    /// Type-level equivalent of "|This|{ This[Field]=Mapper(This[Field]); This }".
    pub fn MapFieldMt[This](This)
    where[ MapFieldOp:TypeFn_<(This, Field, Mapper),Output=Out> ]
    { let Out;Out }
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


type_fn!{
    captures(Field, Mapper)
    /// Type-level equivalent of "|This|{ This[Field]=Mapper(This); This }".
    pub fn MapIntoFieldMt[This](This)
    where[ MapIntoFieldOp:TypeFn_<(This, Field, Mapper),Output=Out> ]
    { let Out;Out }
}
//////////////////////////////////////////////////////////////////////////////////////////



/**
Macro for setting the fields of a compile-time struct.

When constructing a ConstValue prefer using [the construct macro](./macro.construct.html)
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






// #[cfg(all(test,feature="passed_tests"))]
#[cfg(test)]
mod tests{
    use super::*;

    use std_types::range::{fields as range_f,ConstRange};

    #[allow(dead_code)]
    #[derive(TypeLevel)]
    #[typelevel(reexport(Struct))]
    struct Tuple2(
        (),
        (),
    );

    #[allow(dead_code)]
    #[derive(TypeLevel)]
    #[typelevel(reexport(Struct))]
    struct Tuple3(
        (),
        (),
        (),
    );

    #[allow(dead_code)]
    #[derive(TypeLevel)]
    #[typelevel(reexport(Struct))]
    struct Rectangle{
        x:u32,
        y:u32,
        w:u32,
        h:u32,
    }
    use self::type_level_Rectangle::fields as rect_f;

    #[test]
    fn test_get_field(){
        type Test<This,Index,Value>=(
            AssertEq<GetField<This,Index>,Value>,
            AssertFnRet<GetFieldMt<Index>,This,Value>,
        );

        let _:Test<Some_<True>,U0,True>;
        let _:Test<Some_<False>,U0,False>;


        let _:Test<Ok_<True>,U0,True>;
        let _:Test<Ok_<False>,U0,False>;


        let _:Test<Err_<True>,U0,True>;
        let _:Test<Err_<False>,U0,False>;


        let _:Test<ConstTuple2<U10,U20>,U0,U10>;
        let _:Test<ConstTuple2<U10,U20>,U1,U20>;


        let _:Test<ConstTuple3<U10,U20,U30>,U0,U10>;
        let _:Test<ConstTuple3<U10,U20,U30>,U1,U20>;
        let _:Test<ConstTuple3<U10,U20,U30>,U2,U30>;

        
        let _:Test<ConstRange<U10,U20>,range_f::start,U10>;
        let _:Test<ConstRange<U10,U20>,range_f::end  ,U20>;

        let _:Test<ConstRectangle<U0,U10,U20,U30>,rect_f::x,U0>;
        let _:Test<ConstRectangle<U0,U10,U20,U30>,rect_f::y,U10>;
        let _:Test<ConstRectangle<U0,U10,U20,U30>,rect_f::w,U20>;
        let _:Test<ConstRectangle<U0,U10,U20,U30>,rect_f::h,U30>;
    }

    #[test]
    fn test_set_field(){
        type Test<This,Index,Value,NewValue>=(
            AssertEq<SetField<This,Index,Value>,NewValue>,
            AssertFnRet<SetFieldMt<Index,Value>,NewValue>,
        );

        let _:Test<Some_<True> ,U0,False,Some_<False> >;
        let _:Test<Some_<False>,U0,True,Some_<True>>;


        let _:Test<Ok_<True >,U0,False,Ok_<False>>;
        let _:Test<Ok_<False>,U0,True,Ok_<True>>;


        let _:Test<Err_<True>,U0,True,Err_<True>>;
        let _:Test<Err_<False>,U0,False,Err_<False>>;


        let _:Test<ConstTuple2<U10,U20>,U0,(),ConstTuple2<() ,U20>>;
        let _:Test<ConstTuple2<U10,U20>,U1,(),ConstTuple2<U10,() >>;


        let _:Test<ConstTuple3<U10,U20,U30>,U0,(),ConstTuple3<() ,U20,U30>>;
        let _:Test<ConstTuple3<U10,U20,U30>,U1,(),ConstTuple3<U10,() ,U30>>;
        let _:Test<ConstTuple3<U10,U20,U30>,U2,(),ConstTuple3<U10,U20,() >>;

        
        let _:Test<ConstRange<U10,U20>,range_f::start,(),ConstRange<() ,U20>>;
        let _:Test<ConstRange<U10,U20>,range_f::end  ,(),ConstRange<U10,() >>;

        let _:Test<ConstRectangle<U0,U10,U20,U30>,rect_f::x,(),ConstRectangle<(),U10,U20,U30>>;
        let _:Test<ConstRectangle<U0,U10,U20,U30>,rect_f::y,(),ConstRectangle<U0,() ,U20,U30>>;
        let _:Test<ConstRectangle<U0,U10,U20,U30>,rect_f::w,(),ConstRectangle<U0,U10,() ,U30>>;
        let _:Test<ConstRectangle<U0,U10,U20,U30>,rect_f::h,(),ConstRectangle<U0,U10,U20,() >>;
    }

    #[test]
    fn test_set_fields(){
        let _:AssertEq<
            set_fields!{ConstTuple2<(),()> =>
                U0=U10,
                U1=U20,
            },
            ConstTuple2<U10,U20>
        >;
        
        let _:AssertEq<
            set_fields!{ConstTuple3<(),(),()> =>
                U0=U10,
                U1=U20,
                U2=U30,
            },
            ConstTuple3<U10,U20,U30>
        >;
        
        let _:AssertEq<
            set_fields!{ConstRange<(),()> =>
                range_f::start=U10,
                range_f::end  =U20,
            },
            ConstRange<U10 ,U20>
        >;
        let _:AssertEq<
            set_fields!{ConstRange<(),()> =>
                range_f::start=U10,
                range_f::end  =U20,
            },
            ConstRange<U10,U20>
        >;


        type TestSetFields<This,FVPairs,Expected>=(
            AssertFnRet<SetFieldsOp,(This,FVPairs),Expected>,
            AssertFnRet<SetFieldsMt<FVPairs>,This,Expected>,
            AssertEq<<This as SetFields_<FVPairs>>::Output,Expected>,
        );


        let _:TestSetFields<
            ConstRectangle<(),(),(),()>,
            tlist![  
                (rect_f::x,U0),
                (rect_f::y,U10),
                (rect_f::w,U20),
                (rect_f::h,U30),
            ],
            ConstRectangle<U0,U10,U20,U30>
        >;
        let _:TestSetFields<
            ConstTuple2<(),()>,
            tlist![
                (U0,U10),
                (U1,U20),
            ],
            ConstTuple2<U10,U20>
        >;
        
        let _:TestSetFields<
            ConstTuple3<(),(),()>,
            tlist![
                (U0,U10),
                (U1,U20),
                (U2,U30),
            ],
            ConstTuple3<U10,U20,U30>
        >;
        
        let _:TestSetFields<
            ConstRange<(),()>,
            tlist![
                (range_f::start,U10),
                (range_f::end  ,U20),
            ],
            ConstRange<U10 ,U20>
        >;
        let _:TestSetFields<
            ConstRange<(),()>,
            tlist![
                (range_f::start,U10),
                (range_f::end  ,U20),
            ],
            ConstRange<U10,U20>
        >;

        let _:TestSetFields<
            ConstRectangle<(),(),(),()>,
            tlist![
                (rect_f::x,U0),
                (rect_f::y,U10),
                (rect_f::w,U20),
                (rect_f::h,U30),
            ],
            ConstRectangle<U0,U10,U20,U30>
        >;  

        
    }

    #[test]
    fn test_set_fields_to(){
        type Test<This,Fields,To,Equals>=(
            AssertEq<TypeFn<SetFieldsToOp<To>,(This,Fields)>,Equals>,
            AssertEq<SetFieldsTo<This,Fields,To>,Equals>,
        );
        let _:Test<
            ConstRectangle<U100,U100,U100,U100>,
            tlist![ rect_f::x,rect_f::w ],
            U0,
            ConstRectangle<U0,U100,U0,U100>,
        >;

        let _:Test<
            ConstRectangle<U100,U100,U100,U100>,
            tlist![  ],
            U0,
            ConstRectangle<U100,U100,U100,U100>,
        >;

        let _:Test<
            ConstTuple2<(),()>,
            tlist![U0,U1],
            False,
            ConstTuple2<False,False>
        >;
        
        let _:Test<
            ConstRange<U100,U100>,
            tlist![range_f::start,range_f::end],
            U0,
            ConstRange<U0,U0>
        >;

        let _:Test<
            ConstTuple3<(),(),()>,
            tlist![U0,U2],
            False,
            ConstTuple3<False,(),False>
        >;
        
    }

    type Sub1Op=ApplyRhs<SubOp,U1>;


    #[test]
    fn map_field(){
        type Test<This,Field,Mapper,Equals>=(
            AssertFnRet<MapFieldOp,(This,Field,Mapper),Equals>,
            AssertFnRet<MapFieldMt<Field,Mapper>,This,Equals>,
            AssertEq<MapField<This,Field,Mapper>,Equals>,
        );
        

        let _:Test<
            ConstRectangle<U100,U100,U100,U100>,
            rect_f::x,
            Sub1Op,
            ConstRectangle<U99,U100,U100,U100>,
        >;

        let _:Test<
            ConstRectangle<U100,U100,U100,U100>,
            rect_f::w,
            Sub1Op,
            ConstRectangle<U100,U100,U99,U100>,
        >;

        let _:Test<
            ConstTuple2<U100,U100>,
            U0,
            Sub1Op,
            ConstTuple2<U99,U100>,
        >;
        
        let _:Test<
            ConstTuple2<U100,U100>,
            U1,
            Sub1Op,
            ConstTuple2<U100,U99>,
        >;
        
        let _:Test<
            ConstRange<U100,U100>,
            range_f::start,
            Sub1Op,
            ConstRange<U99,U100>,
        >;
        
        let _:Test<
            ConstRange<U100,U100>,
            range_f::end,
            Sub1Op,
            ConstRange<U100,U99>,
        >;

        let _:Test<
            ConstTuple3<U100,U100,U100>,
            U0,
            Sub1Op,
            ConstTuple3<U99,U100,U100>,
        >;

        let _:Test<
            ConstTuple3<U100,U100,U100>,
            U1,
            Sub1Op,
            ConstTuple3<U100,U99,U100>,
        >;

        let _:Test<
            ConstTuple3<U100,U100,U100>,
            U2,
            Sub1Op,
            ConstTuple3<U100,U100,U99>,
        >;
        
    }

    #[test]
    fn map_into_field(){
        type Test<This,Field,Mapper,Equals>=(
            AssertFnRet<MapIntoFieldOp,(This,Field,Mapper),Equals>,
            AssertFnRet<MapIntoFieldMt<Field,Mapper>,This,Equals>,
            AssertEq<MapIntoField<This,Field,Mapper>,Equals>,
        );
        type SubField<Field>=tlist![ GetFieldMt<Field>,Sub1Op ];

        let _:Test<
            ConstRectangle<U10,U20,U30,U40>,
            rect_f::x,
            SubField<rect_f::h>,
            ConstRectangle<U39,U20,U30,U40>,
        >;

        let _:Test<
            ConstRectangle<U10,U20,U30,U40>,
            rect_f::y,
            SubField<rect_f::w>,
            ConstRectangle<U10,U29,U30,U40>,
        >;

        let _:Test<
            ConstTuple3<U10,U20,U30>,
            U1,
            SubField<U0>,
            ConstTuple3<U10,U9,U30>,
        >;

        let _:Test<
            ConstTuple3<U10,U20,U30>,
            U0,
            SubField<U2>,
            ConstTuple3<U29,U20,U30>,
        >;

    }
}