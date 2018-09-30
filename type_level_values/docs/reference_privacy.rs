/*!

This page describes how the TypeLevel macro deals with privacy.

For an example of a type with private fields 
looks at [this chapter of the guide](../guide_10/index.html)

# Enums

TypeLevel deals with privacy on enums the same way structs in which all the fields are 
the same privacy as the struct itself do.

Enums variants implicitly have the same privacy as the enum type itself.


# Structs

TypeLevel deals with pricacy on structs by :
    
- Creating field accessors in `type_level<TypeName>::fields` with 2 levels of privacy:
    
    - The privacy of the struct type itself.

    - The privacy of the most public private field,
        even if the field was originally more private than that.


- Hiding the associated types reoresenting fields more private than the struct ,
    in the <TypeName>Trait and <TypeName>IntoRuntime traits.
    <br>
    Fields are hidden with #[doc(hidden)],
    and by prefixing `priv_` (in <TypeName>Trait) / `rt_priv_` (in <TypeName>Trait) 
    to the associated type representing the field.

- When there is at least one field more private that the struct:
    Adding the  `__PrivFields` type parameter to the Const<TypeName> struct.
    <br>
    It is necessary that one provides a way to construct the struct,
    using the `<Variant>_Uninit` alias.






*/
