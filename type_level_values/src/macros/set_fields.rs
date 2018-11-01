
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
