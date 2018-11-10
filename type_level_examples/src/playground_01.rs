use type_level_values::prelude::*;


#[derive(TypeLevel)]
#[typelevel(reexport(Struct))]
struct Rectangle{
    x:u32,
    y:u32,
    w:u32,
    h:u32,
}


use self::type_level_Rectangle::fields;


pub fn main(){

    let _:Construct<Rectangle_Uninit,(
        (fields::x,U0),
        (fields::y,U1),
        (fields::w,U2),
        (fields::h,U3),
    )>;

}
