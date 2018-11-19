doc_code_snippets! {
    mod "guide_05",
    type_ident=Guide05,
    template=r##"

Here is an example of using a ConstValue struct to enable/disable access to a field 
(through an accessor method).

This chapter demonstrates:
    -how to set fields on a ConstValue struct.
    -how to use a ConstValue struct in generic contexts.
    -how to use enable/disable methods based on the value of a ConstValue struct's field .


//@use_codeblock:enum_decl,ignore

Here we declare an enum describes whether a something is mutable or immutable.

//@use_codeblock:const_struct_decl,ignore

Here we have a ConstValue struct which describes the mutability of every field individually.

//@use_codeblock:typealias,ignore

These 2 type aliases use the SetField type alias,
which allows setting a field of a type level struct.
<br>
In this case we are using the `All` accessor,which allows setting all fields of the struct
with a value.


//@use_codeblock:rect_decl,ignore


Declares a rectangle which uses a ConstValue parameter to determine the mutability of each field.

//@use_codeblock:constructor,ignore

The constructor for Rectangle,taking in a ConstValue describing the mutability of each field.


//@use_codeblock:set_mut,ignore


The mutability method allows getting back the ConstFieldsMutability ConstValue-parameter .
<br>
FieldsMutabilityTrait ensures that `C` is a ConstFieldsMutability,
and allows constructing it with the MTVAL associated constant(from MarkerType).

The set_mutability method allows changing the mutability of the fields by 
passing in another ConstFieldsMutability.
<br>
In a later chapter we'll see a more concise way of changing the ConstValue-parameter.


//@use_codeblock:getters,ignore

Declares getters for every field.
There are no constraints on the getter methods.

//@use_codeblock:setter_0,ignore

This is the setter method for the `x` field which requires that the same field on 
`C:FieldsMutabilityTrait` be `Mutable`.

//@use_codeblock:setter_1,ignore

These are the same as the setter for `x` but for other fields.


//@use_codeblock:main_0,ignore

This is a rectangle in which all setter methods are callable.

The MTVAL associated constant is defined in the ::core_extensions::MarkerType trait,
allowing any ConstValue to be instantiated.


//@use_codeblock:main_1,ignore


This is a rectangle in which only the setter methods for `w` and `h` are callable.

The SetFields type alias allows setting multiple fields on a ConstValue struct.
<br>
The fields are located in the type_level<TypeName>::fields module,
reexported here as fields_fm.


//@use_codeblock:main_2,ignore

This is a rectangle in which only the setter methods for `x` and `y` are callable.

The CW associated constant,
defined in ::type_level_values::const_wrapper::AsConstWrapper,
wraps the type in a ConstWrapper.
<br>
ConstWrapper has multiple inherent methods that allow manipulating ConstValue structs,
in this case using its `set_field_val` method.
<br> 
ConstWrapper can also dereference to its wrapped type if it is a MarkerType,
which all ConstValues are.


//@use_codeblock:main_3,ignore

This is an immutable rectangle.
Note that like every other rectangles.all getters are still callable.



<br><br><br><br><br><br><br><br><br><br>
<hr>
<br><br><br><br><br><br><br><br><br><br>


# The entire thing

//@use_codeblock:all,rust

"##,

    code=r##"


//@codeblock-start:all


#[macro_use]
extern crate derive_type_level;
#[macro_use]
extern crate type_level_values;


use type_level_values::prelude::*;

use type_level_values::field_traits::SetField;


//@codeblock-start:enum_decl

#[derive(TypeLevel)]
#[typelevel(
    derive(ConstEq,ConstOrd),
    reexport(Variants),
)]
pub enum Mutability{
    Mutable,
    Immutable,
}

//@codeblock-end:enum_decl



//@codeblock-start:const_struct_decl

#[derive(TypeLevel)]
#[typelevel(
    derive(ConstEq,ConstOrd),
    //print_attributes,
    reexport(Struct,Traits),
)]
pub struct FieldsMutability{
    pub x:Mutability,
    pub y:Mutability,
    pub w:Mutability,
    pub h:Mutability,
}

pub use self::type_level_FieldsMutability::fields as fields_fm;

//@codeblock-end:const_struct_decl


//@codeblock-start:typealias

type AllMutable=
    SetField< FieldsMutability_Uninit ,fields_fm::All , Mutable> ;

type AllImmutable=
    SetField< FieldsMutability_Uninit ,fields_fm::All , Immutable> ;

//@codeblock-end:typealias



mod rectangle{

    use super::*;

    //@codeblock-start:rect_decl

    #[derive(MutConstValue)]
    #[mcv(
        derive(Default,Debug,Copy,Clone),
        Type="Rectangle",ConstValue="C",
    )]
    pub struct __Rectangle<C>{
        x:u32,
        y:u32,
        w:u32,
        h:u32,
        mutability:ConstWrapper<C>
    }

    //@codeblock-end:rect_decl


    //@codeblock-start:constructor

    impl<C> Rectangle<C>{
        pub fn new(x:u32,y:u32,w:u32,h:u32,_mutability:C)->Rectangle<C>{
            Rectangle{ x, y, w, h, mutability:ConstWrapper::NEW }
        }
    }

    //@codeblock-end:constructor

    

    //@codeblock-start:set_mut

    impl<C> Rectangle<C>{
        pub fn mutability(&self)->C
        where C:FieldsMutabilityTrait
        {
            C::MTVAL
        }

        pub fn set_mutability(self,mutability:C)->Rectangle<C>{
            Rectangle{
                x:self.x,
                y:self.y,
                w:self.w,
                h:self.h,
                mutability:ConstWrapper::NEW,
            }
        }
    }
    
    //@codeblock-end:set_mut



    //@codeblock-start:getters

    impl<EF> Rectangle<EF>{
        pub fn x(&self) -> u32{
            self.x
        }
        pub fn y(&self) -> u32{
            self.y
        }
        pub fn w(&self) -> u32{
            self.w
        }
        pub fn h(&self) -> u32{
            self.h
        }
    }
    
    //@codeblock-end:getters

    
    
    //@codeblock-start:setter_0

    impl<EF> Rectangle<EF>{
        pub fn set_x(&mut self, x: u32)
        where
            EF: FieldsMutabilityTrait<x = Mutable>,
        {
            self.x = x;
        }
    }
    
    //@codeblock-end:setter_0



    //@codeblock-start:setter_1

    impl<EF> Rectangle<EF>{

        pub fn set_y(&mut self, y: u32)
        where
            EF: FieldsMutabilityTrait<y = Mutable>,
        {
            self.y = y;
        }

        pub fn set_w(&mut self, w: u32)
        where
            EF: FieldsMutabilityTrait<w = Mutable>,
        {
            self.w = w;
        }
        
        pub fn set_h(&mut self, h: u32)
        where
            EF: FieldsMutabilityTrait<h = Mutable>,
        {
            self.h = h;
        }
    }
    
    //@codeblock-end:setter_1
}

use rectangle::Rectangle;



fn main(){
    {   
        //@codeblock-start:main_0

        let mut rect=Rectangle::new(10,20,30,40,AllMutable::MTVAL);

        rect.set_x(100);
        rect.set_y(50);
        rect.set_w(25);
        rect.set_h(12);

        println!("{:?}", rect );

        //@codeblock-end:main_0

    }

    {    
        //@codeblock-start:main_1

        type XYMutable=SetFields<AllMutable,(
            (fields_fm::x , Immutable),
            (fields_fm::y , Immutable),
        )>;

        let mut rect=Rectangle::new(10,20,30,40,XYMutable::MTVAL);

        // this methods are disabled
        // rect.set_x(100);
        // rect.set_y(50);
        rect.set_w(25);
        rect.set_h(12);

        println!("{:?}", rect );

        //@codeblock-end:main_1
        
    }

    {
        //@codeblock-start:main_2

        let mutability=AllMutable::CW
            .set_field_val( fields_fm::x , Mutable )
            .set_field_val( fields_fm::y , Mutable )
            .set_field_val( fields_fm::w , Immutable )
            .set_field_val( fields_fm::h , Immutable );

        let mut rect=Rectangle::new(10,20,30,40, *mutability );

        rect.set_x(100);
        rect.set_y(50 );

        // this methods are disabled
        // rect.set_w(25);
        // rect.set_h(12);

        println!("{:?}", rect );

        //@codeblock-end:main_2
        
    }


    {
        //@codeblock-start:main_3
        
        let mut rect=Rectangle::new(10,20,30,40, AllImmutable::MTVAL );

        // this methods are disabled
        // rect.set_x(100);
        // rect.set_y(50 );
        // rect.set_w(25);
        // rect.set_h(12);


        println!("{:?}", rect );

        //@codeblock-end:main_3
    }
}




"##,
}
