[![Build Status](https://travis-ci.org/rodrimati1992/type_level.svg?branch=master)](https://travis-ci.org/rodrimati1992/type_level)

Package for declaring and using type level values and functions.

**Work in progress.This will be ready for consumption in "0.1.0" .**
**Please create issues for any problem.**

### Documentation

For the api documentation go [here](https://docs.rs/type_level_values/).
Or use `cargo doc --open` if you included this in your crate.

For documentation outside of the API of type_level_values itself,
including the `TypeLevel` and `ConstConstructor` derive macros,
go to the `docs` submodule (in the documentation for type_level_values).

### Examples crate

For examples of using the type_level libraries look at 
[`type_level_examples`](https://crates.io/crates/type_level_examples)

### Minimum supported Rust version

This package support rust back to 1.20 .
Using build scripts to enable features after Rust 1.20.

### no-std support

To use `type_level_values` crate in no_std contexts disable the default-feature.

### Cargo Features for `type_level_values`

"std":Enables standard library support.Enabled by default.

"serde":Enables serde support.Enabled by default.


# Example 

Here we implement a type-safe builder which tracks the initialization of each field.

```rust

#[macro_use]
extern crate derive_type_level;

#[macro_use]
extern crate type_level_values;

use type_level_values::prelude::*;
use type_level_values::field_traits::{SetField,SetField_};

fn main(){    
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
}

///////////////////////////////////////////////////////////////////

macro_rules! declare_setter {( $($field:ident),* $(,)* ) => (


    #[derive(Clone, Debug,PartialEq)]
    pub struct TetrisPieces{
        $( $field:usize, )*
    }

    #[derive(TypeLevel)]
    #[typelevel(reexport(Variants))]
    pub enum FieldInitialization{
        InitField,
        UninitField,
    }

    #[derive(TypeLevel)]
    #[typelevel(
        derive(ConstEq,ConstOrd),
        reexport(Struct,Traits),
    )]
    pub struct InitializedFields{
        $( pub $field:FieldInitialization, )*
    }

    pub use self::type_level_InitializedFields::fields;

    pub type AllUninitialized=SetField<
        InitializedFields_Uninit,
        fields::All,
        UninitField
    >;

    pub type AllInitialized=SetField<
        InitializedFields_Uninit,
        fields::All,
        InitField
    >;


    #[derive(Clone, Debug, ConstConstructor)]
    #[cconstructor(Type = "TetrisBuilder",ConstParam = "C")]
    pub struct TetrisBuilderInner<C>{
        $( $field:Option<usize>, )*
        initialization:ConstWrapper<C>,
    }

    impl TetrisBuilder< AllUninitialized >{
        fn new()->Self {
            TetrisBuilder::default()
        }
    }

    impl<I> Default for TetrisBuilder< I >
    where AllUninitialized:TypeIdentity<Type=I>
    {
        fn default()->Self{
            Self{
                $( $field:None, )*
                initialization:ConstWrapper::NEW,
            }
        }
    }

    mod builder_internal{
        
        use super::*;
        
        const_method!{
            type ConstConstructor[]=( TetrisBuilderCC )
            type AllowedConversions=( allowed_conversions::ByVal )

            fn InitializeField[I,Field](I,Field)
            where [ I:SetField_<Field,InitField,Output=Out>, ]
            { let Out;Out }
        }


        impl<C> TetrisBuilder< C >{
            $(
                pub fn $field<__OutSelf>(mut self,value:usize)->__OutSelf
                where 
                    Self:MCPBounds<InitializeField,fields::$field,NextSelf=__OutSelf>
                {
                    self.$field=Some(value);
                    self.mutparam(InitializeField::new(),Default::default())
                }

            )*
        }
    }


    impl<C> TetrisBuilder< C >{

        fn build(self)->TetrisPieces
        where C:TypeIdentity<Type=AllInitialized>
        {
            TetrisPieces{
                $( $field:self.$field.unwrap(), )*
            }
        }

    }
}}


declare_setter!{ l_pieces,i_pieces,z_pieces,s_pieces,o_pieces }


```


# License

type_level is licensed under either of

    Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
    MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in type_level by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
