#[macro_use]
extern crate derive_type_level;

#[macro_use]
extern crate type_level_values;

use type_level_values::prelude::*;
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
    let _:ConstRectangle<U50,U50,U50,U50>=
        InitialRectangle::MTVAL;

    let _= 
        reset_width_height(InitialRectangle::MTVAL) ;

}
