/*! 

The ConstConstructor derive macro provides these features:

- Allows using the ConstType parameter without requiring it to implement any 
    of the other derived traits.

- Allows defining Const-methods for the deriving type,using the const_method macro.

- Allows sequencing operations that mutate the Const-parameter.

For more details on Const-methods look [here](../const_methods/index.html)

For more details on the user_traits module look [here](../user_traits/index.html)


# Generated Items

This macro generates 2 items by default (this can be overriden):

- \<ConstConstructor>:
    A type representing the deriving type without the ConstValue-parameter.
    Eg:the ConstConstructor RectangleCC<T> for the type Rectangle<T,Const>.

- \<TypeAlias>:
    A type alias which passes the ConstType parameter wrapped in a ConstWrapper,
    so that the ConstType itself isn't required to implement any traits.<br>
    Eg:`type Rectangle<T,Const>=RectangleInner<T,ConstWrapper<Const>>` .

# Generated impls

- ConstConstructor for <ConstConstructor>:
    Marker trait for types which when provided a ConstType parameter which return
    another type parameterized by the ConstType parameter.

- user_traits::AllowedOps for <ConstConstructor>:
    Describes whether extension `ConstMethod`s are allowed for the deriving type.

- user_traits::ApplyConstParam_ for <ConstConstructor>:
    Applies a ConstType parameter,returning the deriving type 
    parameterized by that ConstType parameter.


- user_traits::GetConstConstructor_ for <DerivingType>:
    Trait which returns the ConstConstructor for the deriving type.

- user_traits::GetConstParam_ for <DerivingType>:
    Trait which returns the ConstType parameter for the deriving type.

- user_traits::ConstLayoutIndependent for <DerivingType>:
    Unsafe trait which guarantees that the type has the same memory layout regardless of the 
    ConstType parameter.


# Things shared with every other macros in the derive_type_level crate

[ The things shared by the other derive macros: ](../attribute_shared/index.html)

[- The metadata attributes:](../attribute_shared.index#metadata-attributes)
    `bound`/`attr`/`doc`.

[- Attributes on a Type:](../attribute_shared.index#attributes-on-a-typevariant)
    `skip_derive`/`print_derive`/`items`.

# Attributes

- Type (required attribute) :
    Determines the name and other optional properties of \<TypeAlias>.
    
    The minimal form is `Type="ident"`,where the string must be a valid identifier.<br>
    The expanded form is `Type(name="ident" $(, <metadata_attribute> )* )`,
    where the string must be a valid identifier.
    
    Or of the form `Type(use_="ident"  $(, <metadata_attribute> )* )"`,
    where the string must be the identifier of a pre-existing \<TypeAlias>.<br>

- ConstParam (required attribute) :
    The identifier of the Const-parameter of this type.<br>
    Of the form `ConstParam="ident"`,
    where the string must be the identifier of one of the type type parameters.
    <br>
    Or of the form `ConstParam="ident = DefaultType"`,
    where `ident` must be the identifier of one of the type type parameters,
    and `DefaultType` must be the default value for that type parameter (in the type alias).

- ConstConstructor (optional attribute) :
    Determines the name and other optional properties of <ConstConstructor>.
    
    The minimal form is `ConstConstructor="ident"`,where the string must be a valid identifier.<br>
    The expanded form is `ConstConstructor(name="ident" $(, <metadata_attribute> )* )`,
    where the string must be a valid identifier.
    
    Or of the form `ConstConstructor(use_="ident"  $(, <metadata_attribute> )* )"`,
    where the string must be the identifier of a pre-existing <ConstConstructor>.<br>


- extension_methods (optional attribute) :
    Determines whether extension ConstMethods are allowed to mutate the Const-parameter <br>
    of the derived type.<br>
    Of the form `extension_methods="false"|"true"`.<br>
    Default value is `false`.

- items  (optional attribute) : 
    Allows specifying the metadata attributes for the generated impls.<br>
    [The impls being the ones mentioned here.](#generated-impls)


# Examples

```

# #[macro_use]
# extern crate derive_type_level;
# extern crate type_level_values;
# fn main(){}

# use type_level_values::prelude::*;
# use type_level_values::core_extensions::*;

#[derive(ConstConstructor)]
#[cconstructor(Type = "ChannelEnd", ConstParam = "S")]
pub struct ChannelEndInner<Chan, S: WrapperTrait> {
    channel: Chan,
    state: ConstWrapper<S>,
}

```

```


# #[macro_use]
# extern crate derive_type_level;
# extern crate type_level_values;
# fn main(){}

# use type_level_values::prelude::*;
# use type_level_values::core_extensions::*;

#[derive(Copy, Clone, Default, Debug, PartialEq, ConstConstructor)]
#[cconstructor(
    Type(
        name = "Rectangle",
        doc = "A rectangle where certain fields are inaccessible based on a const parameter.",
        doc = "Many impls are also implemented on [RectangleInner].",
    ),
    ConstParam = "I",
)]
#[repr(C)]
pub struct RectangleInner<I, P> {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    accessible_fields: VariantPhantom<(I, P)>,
}

```

```


# #[macro_use]
# extern crate derive_type_level;
# extern crate type_level_values;
# fn main(){}

# use type_level_values::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd,ConstConstructor)]
#[cconstructor(
    Type(
        name = "MutabilityWrapper",
    ),
    ConstParam = "M",
    ConstConstructor(
        name="MutabilityWrapperConstConstructor",
        doc="\
            This is the ConstConstructor of `MutabilityWrapper`
        ",
        attr(derive(Debug,PartialEq)),
    )
)]
pub struct MutabilityWrapperInner<T, M> {
    value: T,
    mutability: PhantomData<M>,
}

```

*/
