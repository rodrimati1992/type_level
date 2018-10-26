doc_code_snippets! {
    mod "guide_04",
    type_ident=Guide04,


    template=r##"


Here is an example of using a ConstValue-parameter to chose between different implementations 
of std::fmt::Debug .


//@use_codeblock:enum_decl,ignore

This declares and reexports the variants of a type-level enum which describes which
impl is used,either ::std::fmt::Debug or ::std::fmt::Display.


//@use_codeblock:struct_decl,ignore

This is a wrapper struct used to chose how T is printed with the Debug formatter,
either using its Debug or Display implementation.


//@use_codeblock:constructor,ignore

This is the constructor function for the wrapper struct,
which requires passing both the wrapped value and thet constant.

//@use_codeblock:impl_usedebug,ignore

This is the Debug implementation used if the Const-parameter is UseDebug.
Notice that it requires `T:fmt::Debug`.

//@use_codeblock:impl_usedisplay,ignore

This is the Debug implementation used if the Const-parameter is UseDisplay.
Notice that it requires `T:fmt::Display`.

//@use_codeblock:main,ignore

This example shows how the wrapper type allows customizing which impl is selected
simply by passing a different variant of the enum.
The usage of raw string literals here is to avoid mixing escaped and unescaped strings,
since Debug unescapes strings and Display does not.

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

use std::fmt;


//@codeblock-start:enum_decl

#[derive(TypeLevel)]
#[typelevel(
    derive(ConstEq,ConstOrd),
    reexport(Variants),
)]
pub enum DebugImpl{
    UseDebug,
    UseDisplay,
}

//@codeblock-end:enum_decl



//@codeblock-start:struct_decl

#[derive(MutConstValue)]
#[mcv(
    derive(Copy,Clone),
    Type="DebugWrapper",Param="C" ,
)]
pub struct DebugWrapperInner<T,C>{
    pub value:T,
    impl_:ConstWrapper<C>,
}

//@codeblock-end:struct_decl



//@codeblock-start:constructor

impl<T,C> DebugWrapper<T,C>{
    fn new(value:T,_debug_impl:C)->Self{
        Self{
            value,
            impl_:ConstWrapper::NEW
        }
    }
}

//@codeblock-end:constructor



//@codeblock-start:impl_usedebug

impl<T> fmt::Debug for DebugWrapper<T,UseDebug>
where T:fmt::Debug
{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        fmt::Debug::fmt(&self.value,f)
    }
}

//@codeblock-end:impl_usedebug


//@codeblock-start:impl_usedisplay


impl<T> fmt::Debug for DebugWrapper<T,UseDisplay>
where T:fmt::Display
{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        fmt::Display::fmt(&self.value,f)
    }
}

//@codeblock-end:impl_usedisplay

fn main(){





    //@codeblock-start:main
    
    {
        let wrapper_debug  =DebugWrapper::new(r#"Hello \ world."#,UseDebug);
        assert_eq!(
            format!("{:?}",wrapper_debug),
            r#""Hello \\ world.""# 
        );
    }

    {
        let wrapper_display=DebugWrapper::new(r#"Hello \ world."#,UseDisplay);
        assert_eq!(
            format!("{:?}",wrapper_display),
            r#"Hello \ world."# 
        );
    }

    //@codeblock-end:main

}







"##,

}
