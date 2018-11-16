[![Build Status](https://travis-ci.org/rodrimati1992/type_level.svg?branch=master)](https://travis-ci.org/rodrimati1992/type_level)

Package for declaring and using type level values and functions.

**Work in progress.This will be ready for consumption in "0.1.0" .**
**Please create issues for any problem.**

### Documentation

For the api documentation go [here](https://docs.rs/type_level_values/).
Or use `cargo doc --open` if you included this in your crate.

For documentation outside of the docs for individual items and modules,
including documentation for the `TypeLevel` and `MutConstValue` derive macros,
go to the `docs` submodule (in the documentation for type_level_values).

### Examples crate

For examples of using the type_level libraries,
//TODO:link to the guide in docs.rs
read [`the guide`]() (also available with cargo doc --open), 
or look at the [`type_level_examples`](https://crates.io/crates/type_level_examples)
crate.

### Minimum supported Rust version

This package support rust back to 1.20 .
Using build scripts to enable features after Rust 1.20.

### no-std support

To use type_level_values in no_std contexts disable the default-feature.

This crate has few features that require the standard library (instead of core),
it is required by default so that users that are not aware of the core library don't have 
to pass a feature to enable std-requiring items.

# Cargo Features

"std":Enables standard library support,otherwise uses the core library.Enabled by default.

"serde":Enables serde support.Enabled by default.

"large_tlist":to enable fixed-size impls for type-lists of 
up to 32 elements instead of 16 elements,


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

    // This creates the type-level equivalent of FieldInitialization in the 
    // type_level_FieldInitialization module,requiring us to reexport what we need.
    #[derive(TypeLevel)]
    //This reexports the type-level equivalents of InitField/UninitField
    #[typelevel(reexport(Variants))]
    pub enum FieldInitialization{
        InitField,
        UninitField,
    }

    // This creates the type-level equivalent of InitializedFields in the 
    // type_level_InitializedFields module,requiring us to reexport what we need.
    #[derive(TypeLevel)]
    #[typelevel(
        derive(ConstEq,ConstOrd),
        //This reexports ConstInitializeFields
        reexport(Struct), 
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


    #[derive(MutConstValue)]
    #[mcv(
        doc="These are the docs for TetrisBuilder_Ty.",
        derive(Clone, Debug),
        Type = "TetrisBuilder",
        ConstValue = "C",
    )]
    pub struct __TetrisBuilder<C>{
        $( $field:Option<usize>, )*
        initialization:ConstWrapper<C>,
    }

    impl TetrisBuilder< AllUninitialized >{
        fn new()->Self {
            TetrisBuilder::default()
        }
    }

    // implementing this on TetrisBuilder<AllUninitialized> caused an internal compiler error,
    // so I just use TypeIdentity to assert type equality.
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
        
        mutator_fn!{
            type This[C]=(TetrisBuilder<C>)
            type AllowedSelf=(allowed_self_constructors::ByVal)

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
                    // The `::T` is an associated constant defined in core_extensions::SelfOps.
                    self.mutparam(InitializeField::NEW,fields::$field::T)
                }

            )*
        }
    }


    impl<C> TetrisBuilder< C >{
        fn build(self)->TetrisPieces
        // I am doing this so that the compiler will print the type parameters that differ.
        // If this impl block were on TetrisBuilder<AllInitialized>
        // it would just say that the method does not exist
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
