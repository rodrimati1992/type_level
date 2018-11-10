/*! 

The MutConstValue derive macro provides these features:

- Allows using the ConstValue parameter without requiring it to implement any 
    of the other derived traits.

- Allows mutating the ConstValue parameter with TypeFns (so long as they are allowed).


For more details on the user_traits module look [here](../../user_traits/index.html)


# Generated Items

This macro generates 3 (visible) items:

- \<NewType\> (named \<TypeAlias\>\_Ty) :
    The same as the deriving type,to which all the attributes in 
    `#[mcv(attrs()]` are delegated.
    <br>
    This is a separate type from the deriving type to stay forward compatible with 
    an attribute to specify phantom generic parameters,
    as in generic parameters that are not stored as values.

- \<TypeAlias\> (the name of which is specified by the `#[mcv(Type="...")]` attribute):
    A type alias which passes the ConstValue parameter wrapped in a ConstWrapper\<\_\>,
    (ConstWrapper implements every trait even if its  parameter doesn't).<br>
    Eg:`type Rectangle<T,Const>=RectangleInner<T,ConstWrapper<Const>>` .

# Public generated impls

- user_traits::GetConstParam_ for \<NewType\>:
    Trait which returns the ConstValue parameter for \<NewType\>.

- user_traits::ConstLayoutIndependent for \<NewType\>:
    Unsafe trait which guarantees that the type has the same memory layout regardless of the 
    ConstValue parameter.


# Things shared with every other macros in the derive_type_level crate

[ The things shared by the other derive macros: ](../attribute_shared/index.html)

[- The metadata attributes:](../attribute_shared.index#metadata-attributes)
    `bound`/`attr`/`doc`.

[- Attributes on the deriving type:](../attribute_shared.index#attributes-on-a-typevariant)
    `SkipDerive`/`PrintDerive`/`Items`.

# Attributes

Any attribute which is not PascalCase is automatically delegated to \<NewType\>.

- Type (required attribute) :
    Determines the name and other optional properties of \<TypeAlias>.
    
    The minimal form is `Type="ident"`,where the string must be a valid identifier.<br>
    The expanded form is `Type(name="ident" $(, <metadata_attribute> )* )`,
    where the string must be a valid identifier.
    
    Or of the form `Type(use_="ident"  $(, <metadata_attribute> )* )"`,
    where the string must be the identifier of a pre-existing \<TypeAlias>.<br>

- ConstValue (required attribute) :
    The identifier of the ConstValue-parameter of this type.<br>
    Of the form `ConstValue="ident"`,
    where the string must be the identifier of one of the type type parameters.
    <br>
    Or of the form `ConstValue="ident = DefaultType"`,
    where `ident` must be the identifier of one of the type type parameters,
    and `DefaultType` must be the default value for that type parameter (in the type alias).

- Attrs (optional attribute):
    Allows specifying attributes for the generated \<DerivingType\>\_Ty.
    Use this in case that the attribute starts with an uppercase character,eg:'Capitalized'.

- Items  (optional attribute) : 
    Allows specifying the metadata attributes for the generated impls.
    <br>
    The impls being of these traits:
    <br>- ConstLayoutIndependent for \<NewType\>
    <br>- GetConstParam_ for \<NewType\>


# Examples

```

# #[macro_use]
# extern crate derive_type_level;
# extern crate type_level_values;
# fn main(){}

# use type_level_values::prelude::*;
# use type_level_values::core_extensions::*;

#[derive(MutConstValue)]
#[mcv(
    doc="This doc comment gets applied to ChannelEnd_Ty",
    derive(Debug,Copy,Clone),
    repr(transparent),
    Type = "ChannelEnd", ConstValue = "S",
)]
pub struct __ChannelEnd<Chan, S: WrapperTrait> {
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

#[derive(MutConstValue)]
#[mcv(
    doc = "
        This is a rectangle ,and this doc comment gets applied to Rectangle_Ty
    ",
    derive(Copy, Clone, Default, Debug, PartialEq),
    repr(C),
    Type(
        name = "Rectangle",
        doc = "A rectangle where certain fields are inaccessible based on a const parameter.",
        doc = "Many impls are also implemented on [RectangleInner].",
    ),
    ConstValue = "I",
)]
pub struct __Rectangle<I, P> {
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

#[derive(MutConstValue)]
#[mcv(
    derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd),
    Type(name = "MutabilityWrapper"),
    ConstValue = "M",
)]
pub struct __MutabilityWrapper<T, M> {
    value: T,
    mutability: PhantomData<M>,
}

```

*/
