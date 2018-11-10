doc_code_snippets! {
    mod "guide_06",
    type_ident=Guide06,
    template=r##"


This chapter demonstrates how to mutate the ConstValue parameter with a Mutator Function.

Mutator Functions are TypeFn which can mutate the ConstValue parameter of a type,
declared using the mutator_fn macro.


Here is an example of a builder which checks that all fields have been initialized 
at compile-time.

//@use_codeblock:tetris_pieces_struct,ignore

This is the type created by the builder.
It is the ammound of tetrominos (tetris pieces) used for a special for of tetris where 
one must fill as many lines as possible before the pieces run out.

//@use_codeblock:field_init_enum,ignore

This is the enum describing whether a field is initialized.

//@use_codeblock:init_struct,ignore

This is the struct describing describing the initialization state of all fields.

//@use_codeblock:init_aliases,ignore

These are aliases for ConstInitializedFields where all the fields are either 
initialized (AllInitialized) or uninitialized (AllUninitialized).

//@use_codeblock:builder_struct_decl,ignore

This is the builder struct itself,
taking the initialization of each field as the ConstValue-parameter `C`.


//@use_codeblock:mutator_fn,ignore

This declares a Mutator Function for TetrisBuilder,
I is the current value of the ConstValue parameter,and Field is the field we want to initialize.

The `AllowedSelf` associated type allows us to control which methods of 
MutConstParam we can call,by value/reference/mutable_reference/all.

the `let` syntax here is used to declare a variable in the impl of TypeFn_ 
for InitializeField.

//@use_codeblock:declare_setter_macro,ignore

Here we use a macro to declare each setter method of the builder 
since it is such a repetitive task.

The mutparam method comes from the MutConstParam trait and allows changing the 
ConstValue-parameter of a value.

MCPBounds is a trait alias for the constraints required by most MutConstParam methods.

The `NextSelf=_Out` is an example of the 
[`generic type as type alias`
](../../appendix_patterns/index.html#patterngeneric-type-as-type-alias) 
pattern,
which allows us to return whatever the trait's associated type is without
repeating the constraint in the return type.

//@use_codeblock:setter_impls,ignore

This decares the setter methods for each field in the builder.

//@use_codeblock:build_fn,ignore

This constructs the TetrisPieces,the unwraps are fine since this 
type already checks whether all the fields are initialized with the ConstValue-parameter.

The `C:TypeIdentity<Type= AllInitialized >` constraint in the method 
is an equality constraint,requiring that the ConstValue-parameter be AllInitialized,
and produces a better error message than if the impl was for TetrisBuilder< AllInitialized >.

//@use_codeblock:main,ignore

This looks like a typical builder type.
if any of the setters are commented out this will result in a compiletime error.


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
use type_level_values::field_traits::{SetField,SetField_};


//@codeblock-start:tetris_pieces_struct

#[derive(Clone, Debug,PartialEq)]
pub struct TetrisPieces{
    l_pieces:usize,
    i_pieces:usize,
    z_pieces:usize,
    s_pieces:usize,
    o_pieces:usize,
}

//@codeblock-end  :tetris_pieces_struct



//@codeblock-start:field_init_enum

#[derive(TypeLevel)]
#[typelevel(
    derive(ConstEq,ConstOrd),
    reexport(Variants),
)]
pub enum FieldInitialization{
    InitField,
    UninitField,
}

//@codeblock-end  :field_init_enum



//@codeblock-start:init_struct

#[derive(TypeLevel)]
#[typelevel(
    derive(ConstEq,ConstOrd),
    reexport(Struct,Traits),
)]
pub struct InitializedFields{
    pub l_pieces:FieldInitialization,
    pub i_pieces:FieldInitialization,
    pub z_pieces:FieldInitialization,
    pub s_pieces:FieldInitialization,
    pub o_pieces:FieldInitialization,
}

pub use self::type_level_InitializedFields::fields as if_f;


//@codeblock-end  :init_struct



//@codeblock-start:init_aliases

pub type AllUninitialized=SetField<
    InitializedFields_Uninit,
    if_f::All,
    UninitField
>;

pub type AllInitialized=SetField<
    InitializedFields_Uninit,
    if_f::All,
    InitField
>;

//@codeblock-end:init_aliases




//@codeblock-start:builder_struct_decl


#[derive(MutConstValue)]
#[mcv(
    derive(Clone,Default, Debug),
    Type = "TetrisBuilder",ConstValue = "C",
)]
pub struct TetrisBuilderInner<C>{
    l_pieces:Option<usize>,
    i_pieces:Option<usize>,
    z_pieces:Option<usize>,
    s_pieces:Option<usize>,
    o_pieces:Option<usize>,
    initialization:ConstWrapper<C>,
}


impl TetrisBuilder< AllUninitialized >{
    fn new()->Self{
        TetrisBuilder::default()
    }
}

//@codeblock-end:builder_struct_decl





//@codeblock-start:mutator_fn


mutator_fn!{
    type This[I]=(TetrisBuilder<I>)
    type AllowedSelf=(allowed_self_constructors::ByVal)

    fn InitializeField[I,Field](I,Field)
    where [ I:SetField_<Field,InitField,Output=Out>, ]
    { let Out;Out }
}

//@codeblock-end  :mutator_fn




//@codeblock-start:declare_setter_macro

macro_rules! declare_setter {
    ( $field:ident ) => (

        fn $field<_Out>(mut self,value:usize)->_Out
        where 
            Self:MCPBounds<InitializeField,if_f::$field,NextSelf=_Out>,
        {
            self.$field=Some(value);
            self.mutparam(InitializeField::NEW,if_f::$field::T)
        }

    )
}


//@codeblock-end  :declare_setter_macro



//@codeblock-start:setter_impls

impl<C> TetrisBuilder< C >{

    declare_setter!{ l_pieces }
    declare_setter!{ i_pieces }
    declare_setter!{ z_pieces }
    declare_setter!{ s_pieces }
    declare_setter!{ o_pieces }

}

//@codeblock-end:setter_impls



//@codeblock-start:build_fn

impl<C> TetrisBuilder< C >{

    fn build(self)->TetrisPieces
    where C:TypeIdentity<Type=AllInitialized>
    {
        TetrisPieces{
            l_pieces:self.l_pieces.unwrap(),
            i_pieces:self.i_pieces.unwrap(),
            z_pieces:self.z_pieces.unwrap(),
            s_pieces:self.s_pieces.unwrap(),
            o_pieces:self.o_pieces.unwrap(),
        }
    }

}

//@codeblock-end:build_fn




fn main(){
    
    //@codeblock-start:main

    let pieces=TetrisBuilder::new()
        .l_pieces(10)
        .i_pieces(20)
        .z_pieces(30)
        .s_pieces(40)
        .o_pieces(50)
        .build();

    assert_eq!(
        pieces,
        TetrisPieces{
            l_pieces:10,
            i_pieces:20,
            z_pieces:30,
            s_pieces:40,
            o_pieces:50,
        }
    )

    //@codeblock-end  :main
}







"##,

}
