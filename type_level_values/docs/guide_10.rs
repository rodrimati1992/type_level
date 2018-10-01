doc_code_snippets! {
    mod "guide_10",
    type_ident=Guide10,
    template=r##"

In this chapter we'll cover a ConstValue with private fields and how to construct it.

//@use_codeblock:non0_3d_point_module,ignore

This module is used to prevent accidentally depending on an 
implementation detail of NonZero3DPoint.

//@use_codeblock:non0_point_struct,ignore

Here we declare a point which cannot be initialized with a x/y/z with a value of 0.

The `typelevel(pub_trait_accessor)` attribute is used here to be able to access 
the value of the fields in the NonZero3DPointTrait trait,
otherwise they would be innacessible outside of this module.

//@use_codeblock:new_non0_point_struct,ignore

This is the constructor function for the `ConstNonZero3DPoint`,the only way to construct one.

This function checks that each parameter (x/y/z)  is not zero 
before constructing a `ConstNonZero3DPoint` with them.

The `construct` macro here ensures that we correctly initialize a ConstValue,
producing an error message mentioning which fields are not initialized 
if they are not.

The `NonZero3DPoint_Uninit` used by the `construct` macro is innacessible outside of the module
so as to prevent users from constructing an invalid `ConstNonZero3DPoint`.

//@use_codeblock:distance_struct,ignore

Here we declare a Movement struct,which describes how much a point moves in 3d space.

//@use_codeblock:add_distance_fn,ignore

This function calculates how much a `ConstNonZero3DPoint` is moved by a Movement.

Note that this function has to use `NewNonZero3DPoint` to construct a `ConstNonZero3DPoint`.

//@use_codeblock:main,ignore

Here we construct a `ConstNonZero3DPoint` and then move it in 3d space twice,
showing which values it has after each move using a let binding.

<br><br><br><br><br><br><br><br><br><br>
<hr>
<br><br><br><br><br><br><br><br><br><br>


# The entire thing

//@use_codeblock:all,rust

"##,

    code=r##"


//@codeblock-start:all




#[macro_use]
extern crate type_level_values;
#[macro_use]
extern crate derive_type_level;

use std::ops::Add;

use type_level_values::field_traits::*;
use type_level_values::ops::ConstNE_;
use type_level_values::prelude::*;



//@codeblock-start:non0_3d_point_module

mod non_zero_3d_point{
# /*
    ...
}
# */

//@codeblock-end:non0_3d_point_module

    //@codeblock-start:non0_point_struct
    
    use super::*;
    #[derive(TypeLevel)]
    #[typelevel(reexport(Struct,Traits))]
    #[allow(dead_code)]
    pub struct NonZero3DPoint{
        #[typelevel(pub_trait_accessor)]
        x:u64,
        #[typelevel(pub_trait_accessor)]
        y:u64,
        #[typelevel(pub_trait_accessor)]
        z:u64,
    }

    use self::type_level_NonZero3DPoint::fields;
    
    //@codeblock-end  :non0_point_struct

    
    //@codeblock-start:new_non0_point_struct

    type_fn!{
        pub fn NewNonZero3DPoint[x,y,z](x,y,z)
        where[
            x:ConstNE_<Z0,Output=True>,
            y:ConstNE_<Z0,Output=True>,
            z:ConstNE_<Z0,Output=True>,
        ]{
            construct!(NonZero3DPoint_Uninit=>
                fields::x=x,
                fields::y=y,
                fields::z=z,
            )
        }
    }

    //@codeblock-end  :new_non0_point_struct
}

use self::non_zero_3d_point::{NewNonZero3DPoint,NonZero3DPointTrait};


//@codeblock-start:distance_struct

#[derive(TypeLevel)]
#[typelevel(reexport(Struct,Traits))]
pub struct Movement{
    pub x:u64,
    pub y:u64,
    pub z:u64,
}

//@codeblock-end  :distance_struct



//@codeblock-start:add_distance_fn

type_fn!{
    pub fn AddMovement[Point,Movement](Point,Movement)
    where [
        Point:NonZero3DPointTrait,
        Movement:MovementTrait,
        Point::x :Add<Movement::x ,Output=new_x>,
        Point::y :Add<Movement::y ,Output=new_y>,
        Point::z :Add<Movement::z ,Output=new_z>,
        NewNonZero3DPoint:TypeFn_<(new_x,new_y,new_z),Output=new_point>,
    ]{
        let new_x;let new_y;let new_z;
        let new_point;
        new_point
    }
}

//@codeblock-end  :add_distance_fn


//@codeblock-start:main

fn main() {
    type Point0=TypeFn<NewNonZero3DPoint,(P1,P1,P1)>;

    type Point1=TypeFn<AddMovement,( Point0, ConstMovement<P10,P5,N5> )>;
    let _:TypeFn<NewNonZero3DPoint,(P11,P6,N4 )>=Point1::MTVAL;

    type Point2=TypeFn<AddMovement,( Point1, ConstMovement<N3,Z0,P100> )>;
    let _:TypeFn<NewNonZero3DPoint,(P8 ,P6,P96)>=Point2::MTVAL;
}

//@codeblock-end  :main




"##,
}