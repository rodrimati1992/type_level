[![Build Status](https://travis-ci.org/rodrimati1992/type_level.svg?branch=master)](https://travis-ci.org/rodrimati1992/type_level)

Package for declaring and using type level values and functions.

**Work in progress.**
**This will be ready for consumption when the "0.1.0" is published on crates.io .**
**Please create issues for any problem.**

# Library Features

This library provides these (non-exhaustive list of) type-level features:

- Deriving TypeLevelness of enums and structs,and doing that for some core datatypes.

- Declaring Type-level functions,includes some equivalents to core datatypes methods.

- Collection traits/functions.

- Other Control flow operations:Panics/Assertions/If/Pattern-matching.

- Field traits/functions (setting/getting/mapping fields).

- Integer operations(aside from those provided in typenum).

- Conversion traits/functions both between type-level-values and into runtime values.

- Wrapper operations:unwrap/AndThen/OrElse/IntoInner/etc.

- Function adaptors/combinators.


This library provides these features for types using type-level-value parametesr:

- Mutating the type-level-value parameter using a (restricted by the type) type-level-function,
    even if the type is a reference.



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

This library supports back to 1.20 because other basic libraries do,
if this is unnecessary search for/create an issue arguing for what version to support back to.

### no-std support

To use type_level_values in no_std contexts disable the default-feature.

This crate has virtually no features that require the standard library (instead of core),
it is required by default so that users that are not aware of the core library don't have 
to pass a feature to enable std-requiring items.

# Cargo Features

"std":Enables standard library support,otherwise uses the core library.Enabled by default.

"serde":Enables serde support.Enabled by default.

"large_tlist":to enable fixed-size impls for type-lists of 
up to 32 elements instead of 16 elements,


# Example 

Let's say that we want to implement a special game of tetris where we have a 
limited ammount of tetris pieces(tetrominos),and the goal is to have the maximum score.

Here we implement a type-safe builder which tracks the initialization of each field
in the type system.

This is just an example,this library does not provide a derive for builders
(though a dependent crate could do that).

Cargo.toml:
```
type_level_values={version = "0.1"}
derive_type_level={version = "0.1"}

```

main.rs:
```rust

// For the 2018 edition uncomment the next lines.
// use derive_type_level::{TypeLevel,MutConstValue};
// use type_level_values::{tlist,mutator_fn};

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

macro_rules! declare_setter {( $($field:ident),* $(,)* ) => {


    /// This is the type we are trying to build,
    /// it represents the ammount of tetris pieces left in a special form of tetris. 
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

    // This derive macro creates the type-level equivalent of InitializedFields in the 
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

    /// We manually reexport the field accessors submodule 
    /// as if_field to avoid name colisions
    pub use self::type_level_InitializedFields::{
        fields as if_field,
    };

    /**
    This is the ConstInitializedFields we start with.
    
    `InitializedFields_Uninit` is the uninitialized version of ConstInitializedFields,
    which we must initialize to use.

    We use the special field accessor `All` to 
    initialize all the fields with the passed value (which in this case is `UninitField`).
    */
    pub type AllUninitialized=SetField<
        InitializedFields_Uninit,
        if_field::All,
        UninitField
    >;

    /**
    This is the ConstInitializedFields required to build the TetrisPieces.

    This is an alternate way to initialize all the fields,
    allowing us to pass any value for fields
    while ensuring that all fields are initialized.
    */
    pub type AllInitialized=Construct<
        InitializedFields_Uninit,
        tlist!(
            $( (if_field::$field , InitField) ,)*
        )
    >;


    /**
    We declare a datatype which uses a ConstValue-parameter `C` (type-level-value=ConstValue).
    
    Note that the type we use in the rest of the example is 
    `TetrisBuilder` not `__TetrisBuilder`.

    In the future MutConstValue might be superceded (leaving it for pre Rust 1.30 users) 
    by a proc-macro attribute so as to not require a dummy type declaration.
    */
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
    // so I just use TypeIdentity to alias AllUninitialized into I.
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
        
        /// This declares a Type-level function which is allowed to mutate the 
        /// ConstValue-parameter `C` of `TetrisBuilder<C>`.
        mutator_fn!{
            type This[C]=(TetrisBuilder<C>)

            // The AllowedSelf type here determines whether the function is allowed to 
            // mutate C for a value/reference/mutable-reference of TetrisBuilder<C>.
            // For some functions on some types it may be valid to 
            // use some combination of the 3.
            type AllowedSelf=(allowed_self_constructors::ByVal)

            /**
            This is the function,note that we must declare generic parameters 
            for the function inside `[..]`instead of `<..>`,
            this is mostly for implementation simplicity.
            */
            fn InitializeField[I,Field](I,Field)
            // The `[..]` is here to make this easier to parse.
            where [ I:SetField_<Field,InitField,Output=Out>, ]
            { 
                // `let` here declares a type variable,which can be initialized anywhere. 
                let Out;
                // This is the return value of the function,like in regular Rust.
                Out 
            }
        }

        impl<C> TetrisBuilder< C >{
            $(
                // Here we initialize the field and set the 
                // same field on the `C` type parameter (a `ConstInitializedFields<..>`) 
                // as initialized.
                //
                // The `__OutSelf` here is how we emulate "output" types,
                // this is simpler than many alternatives.
                pub fn $field<__OutSelf>(mut self,value:usize)->__OutSelf
                where 
                    Self:MCPBounds<InitializeField,if_field::$field,NextSelf=__OutSelf>
                {
                    self.$field=Some(value);
                    // The `::T` is an associated constant defined in core_extensions::SelfOps,
                    // which allows us to emulate passing types as regular parameters.
                    self.mutparam(InitializeField::NEW,if_field::$field::T)
                }

            )*
        }
    }


    // If this impl block were on TetrisBuilder<AllInitialized>
    // it would just say that the build method does not exist
    impl<C> TetrisBuilder< C >{
        fn build(self)->TetrisPieces
        // TypeIdentity is used here to assert that C and AllInitialized are the same type.
        where C:TypeIdentity<Type= AllInitialized >
        {
            TetrisPieces{
                $( $field:self.$field.unwrap(), )*
            }
        }

    }
}}


declare_setter!{
    l_pieces,
    i_pieces,
    z_pieces,
    s_pieces,
    o_pieces 
}


```

### Reading error messages

If we comment out `.o_pieces(50)` and try to build the code we'll get 
a somewhat difficult to decode error message,
use the ```[^\(\)\[\]{}>`<,= ]+::``` regex  to remove the junk in the error message,
it should produce something like this:
```
error[E0271]: type mismatch resolving `<ConstInitializedFields<InitField, InitField, InitField, InitField, UninitField<o_pieces>> as SetField_<o_pieces, InitField>>::Output == ConstInitializedFields<InitField, InitField, InitField, InitField, UninitField>`
  --> type_level_examples\src\playground_01.rs:11:10
   |
11 |         .build();
   |          ^^^^^ expected struct `InitField`, found struct `UninitField`
   |
   = note: expected type `ConstInitializedFields<_, _, _, _, InitField>`
              found type `ConstInitializedFields<_, _, _, _, UninitField>`
   = note: required because of the requirements on the impl of `TypeFn_<(ConstInitializedFields<InitField, InitField, InitField, InitField, UninitField<o_pieces>>, (o_pieces, InitField))>` for `SetFieldValuePair`
   <some notes elided to shorten this example error message>
```

If we read the first note,we'll see that the it says that the last field is uninitialized,
which must mean that we forgot to initialize o_pieces(the last field we declared),
so add a call to the o_pieces method before build and it should compile fine.

The error is caused by this constraint `C:TypeIdentity<Type= AllInitialized >`,
which asserts that C and AllInitialized must be the same type.

# License

type_level is licensed under either of

    Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
    MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in type_level by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
