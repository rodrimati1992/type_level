doc_code_snippets! {
    mod "guide_01",
    type_ident=Guide01,

    template=r##"


Defining a type-level-value (aka ConstValue) is done through the `TypeLevel` derive macro of  the 
derive_type_level crate.


//@use_codeblock:enum-decl,ignore

Here we define an enum with 2 variants,and a type-level equivalent with TypeLevel.


//@use_codeblock:enum-main,ignore

This shows how to construct a ConstValue enum.

//@use_codeblock:struct-decl,ignore

Here we declare and derive the type-level version of Rectangle.



//@use_codeblock:struct-main,ignore

Here we construct a type-level struct using both the struct literal syntax and 
using the MTVAL associated constant of ConstRectangle.

The CW associated constant comes from the AsConstWrapper trait,
part of this library's prelude,
wrapping Self in a ConstWrapper.
<br>
Every field of a type-level value is a ConstWrapper<_>.

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


// This prelude imports the type-level equivalent of the standard library prelude.


// This prelude is necessary because otherwise one would have to import a lot of traits/types.
use type_level_values::prelude::*;


//@codeblock-start:enum-decl

#[derive(TypeLevel)]
enum FieldAccessor {
    Enabled,
    Disabled,
}

//@codeblock-end:enum-decl

fn main_enum(){
    
    //@codeblock-start:enum-main
    
    use self::type_level_FieldAccessor::{Enabled,Disabled};

    let _:Enabled =Enabled;
    let _:Disabled=Disabled;

    //@codeblock-end:enum-main
}





//@codeblock-start:struct-decl

#[derive(TypeLevel)]
#[typelevel(reexport(Struct))]
struct Rectangle {
    x:u32,
    y:u32,
    w:u32,
    h:u32,
}

use self::type_level_Rectangle::fields as rect_f;



//@codeblock-end:struct-decl


fn main_struct(){
    //@codeblock-start:struct-main

    let rectangle1:ConstRectangle<U0,U1,U2,U3>=
        ConstRectangle{
            x:U0::CW, 
            y:U1::CW,
            w:U2::CW,
            h:U3::CW,
        };

    let rectangle2:Construct<Rectangle_Uninit,(
        (rect_f::x , U0),
        (rect_f::y , U1),
        (rect_f::w , U2),
        (rect_f::h , U3),
    )>=ConstRectangle::MTVAL;
    
    //@codeblock-end:struct-main

    // All type-level-values are zero-sized-types.
    assert_eq!(0 , ::std::mem::size_of_val(&rectangle1));
    assert_eq!(0 , ::std::mem::size_of_val(&rectangle2));
}




fn main(){
    main_enum();
    main_struct();
}





"##,


}

