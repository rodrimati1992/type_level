/*! 



The `TypeLevel` derive attribute,used to create a compile-time equivalent of a type.



# Generated items:

All of the generated items are inside a module called type_level_\<DerivingType>,
where `<DerivingType>` is the name of the type deriving TypeLevel.


Items inside type_level_\<DerivingType>:

- \<ConstValue> :
    A type that represents compile-time values of the deriving type.<br>
    For enums it generates as many as there are variants.
    <br>
    The default name for structs is Const\<DerivingType>.
    The default name for enum variants is <VariantName> ,the same as in the enum declaration.


- \<ConstType>:
    A marker type representing the ConstType equivalent of the deriving type.
    <br>
    The default name is \<DerivingType>Type

- \<Struct/Variant>Trait:
    Trait used to operate on a ConstValue in generic contexts and 
    to get the fields of the struct/variant.
    <br>
    The default name for structs is \<DerivingType>Trait.
    The default name for enum variants is <VariantName>Trait.

- \<Struct/Variant>FromTrait:
    To get the \<ConstValue> type from a type parameter constrained by \<Struct/Variant>Trait .
    <br>
    The default name for structs is \<DerivingType>FromTrait.
    The default name for enum variants is <VariantName>FromTrait.

- fields :
    A module containing field accessors.
    Used in the GetField_ / SetField_ traits to get and set the value of the field.

- variants :
    A module containing the discriminants and 
    variant name(which is a unit struct) of the type.
    <br>
    The default name for the discriminant of structs is \<DerivingType>\_Discr.
    The default name for the discriminant of enum variants is <VariantName>\_Discr.
    <br>
    The default name for the variant name of structs is \<DerivingType>\_Variant.
    The default name for the variant name of enum variants is <VariantName>\_Variant.


# Attributes

All Attributes are written inside #[typelevel(  )] .

[ The things shared by the other derive macros: ](../attribute_shared/index.html)

[- The metadata attributes:](../attribute_shared.index#metadata-attributes)
    `bound`/`attr`/`doc`.

[- Attributes on a Type/Variant:](../attribute_shared.index#attributes-on-a-typevariant)
    `skip_derive`/`print_derive`/`items`.

<br>

## Metadata attributes

Most attributes lists support these attributes:

- bound:
    Bounds added to the generated item.<br>
    Of the form `bound=" Type:Bound "` ,the string has to be a single where predicate.

- attr:
    Attributes that will be added to the generated item.<br>
    Of the form `attrs( ... )`  .
    

- doc : 
    A documentation attribute the will be added to the generated item.<br>
    Of the form `doc="documentation"` , the string can span multiple lines.


## Attributes on a Struct/Enum:

- rename_consttype:
    Renames the ConstType generated for the Type.
    ConstType is marker type used as the "type" of a ConstValue,
    in which ConstValue is the compiletime equivalent of a value.<br>
    Of the form `rename_consttype = "new_name"`,the string must be a valid identifier.

- reexport :
    Reexports the generated items outside of the generated module ,
    reexported to the module of the deriving type.<br>
    Of the form `reexport="pub"`,where the string has to be a valid visibility.
    <br>
    Or Of the form `reexport( $(<reexport_kind>),* )`
    Where <reexport_kind> enables re-exporting a group of items and is one/many of:
    - Traits:<br>
        For structs \<DerivingType>Trait and \<DerivingType>IntoRuntime.<br>
        For enums \<DerivingType>Trait,<Variant>Trait and \<DerivingType>IntoRuntime.
    <br>
    - Variants/Struct:<br>
        For structs Const\<DerivingType> .<br>
        For enums types of the same name as the variants.
    <br>
    - Discriminants:the `variants` module
    <br>
    - Fields:the `fields` module.

- derive_str :
    Creates the associated constant `const TYPELEVEL_DERIVE:&'static str` on the deriving type,
    containing the output of the derive macro.

## Attributes on a Type/Variant:



- rename :
    Renames the ConstValue equivalent of the derived Type/Variant.<br>
    Of the form `rename = "new_name"`,the string must be a valid identifier.

- rename_trait :
    Renames the trait used to access the fields of the ConstValue equivalent 
    for the derived Type/Variant.<br>
    Of the form `rename_trait = "new_name"`,the string must be a valid identifier.

- derive : 
    Derives all Built-in traits,
    delegating all unsupported traits to the #[derive(...)] attribute.<br>
    Of the form `derive(ConstEq,ConstOrd, ... )`.

- items : 
    Allows specifying Metadata for the generated items and how/whether they are implemented.
    The generated items are for the Built-in traits and all the Automatically implementd Traits.
    Of the form `items( NameOfImpls0(..),NameOfImpls1(..), ... )`
    where NameOfImpls can be one of 
    ConstEq/ConstOrd/IntoConstType/IntoRuntime/AsTList/runtime_conv.



- the Metadata attributes.Applied to the ConstValue equivalent of the Type/Variant.


#### items Subattribute

The #[typelevel(items( ... ))] attribute allows 
specifying Metadata for the generated traits and how/whether they are implemented.
The generated traits include the Built-in traits and all the Automatic Traits.

The syntax is `#[typelevel(items(impl_name( ... )))]`,
where impl_name is the name of an trait/alias,eg:IntoRuntime/ConstEq.

Valid attributes inside items(impl_name( ... )):

- NoImpls:
    Disables this implementation.

- DefaultImpls:
    Generates the default implementation.

- Internal:
    Generates an implementation for a different type,instead of the type being derived.
    Has no effect on traits that do not involve the deriving type.<br>
    Of the form `Internal="type_identifier"`,where the string is a valid identifier.
    Or of the form `Internal(Type="type_identifier",Manual)`,
        where the trait must be manually implemented.

    
- the Metadata attributes,applied to the impl itself.




## Attributes on a field:


- pub_trait_accessor:
    Allows accessing the value of a private field through the \<DerivingType>Trait.
    Does not allow using GetField to access the value of the field.


- bound:
    Allows adding a bound to the associated type of the \<DerivingType>Trait
    representing this field.
    <br>
    Of the form `bound="bound"` where bound must be a valid constraint.
    

- bound_runt:
    Allows adding a bound to the associated type of the \<DerivingType>WithRuntime
    representing this field.
    <br>
    Of the form `bound_runt="bound"` where bound must be a valid constraint.


- rename:
    Renames the field in the generated code.
    Currently only possible for Struct/Struct Variants.<br>
    Of the form `rename="name"`,the string must be a valid identifier.

## Built-in traits

This derive macro allows deriving some traits declared in type_level_values ,
using `#[typelevel(derive( ... ))]`.

The derivable traits are:

- ConstEq:Compares 2 ConstValues for equality.

- ConstOrd:Compares 2 ConstValues for ordering.


## Automatic impls

The automatically generated impls are impls which are generated unless the user disables them 
inside `#[typelevel(items(...))]`.

The automatic impls are:

- IntoRuntime:
    Converts a ConstValue into a runtime value.

- IntoConstType:
    Returns the ConstType for the type.

- GetDiscriminant:
    Returns the discriminant of the ConstValue.
    This derive macro generates a distinct variant for every struct type/enum variant.

- AsTList:
    Converts a ConstValue to a `TList` , a type-list type .
    Used for deriving some traits.


## Trait aliases

These are aliases for multiple traits,for use inside #[typelevel(items(...))] :

- runtime_conv:for IntoConstType and IntoRuntime.


# Examples


```


# #[macro_use]
# extern crate derive_type_level;
# extern crate type_level_values;
# fn main(){}


#[derive(Clone, Copy, Debug, TypeLevel)]
#[typelevel(reexport(Variants))]
pub enum Mutability {
    Mutable,
    Immutable,
}


```


```


# #[macro_use]
# extern crate derive_type_level;
# extern crate type_level_values;
# fn main(){}

#[derive(Clone, Copy, Debug, TypeLevel)]
#[typelevel(derive(ConstEq, ConstOrd))]
pub struct RectangleAcessible {
    position: bool,
    dimension: bool,
}


```


```


# #[macro_use]
# extern crate derive_type_level;
# extern crate type_level_values;
# fn main(){}


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, TypeLevel)]
#[typelevel(
    derive(ConstEq, ConstOrd,Debug),
    items(
        ConstEq(
            attr(cfg(feature="impls")),
            doc="Compares a State for equality",
        ),
        ConstOrd(
            attr(cfg(feature="impls")),
        ),
        runtime_conv(NoImpls),
    ),
)]
pub enum State {
    Open { remaining: u64 },
    Closed,
}


```



```


# #[macro_use]
# extern crate derive_type_level;
# extern crate type_level_values;
# fn main(){}


#[derive(TypeLevel)]
#[typelevel(
    reexport,
    items(runtime_conv(NoImpls)),
)]
enum Direction<T> {
    Up,
    Down,
    Left,
    Right,
    Other(T),
}



```


*/
