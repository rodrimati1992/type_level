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
    Adding the  `__IsPriv` type parameter to the Const<TypeName> struct.
    <br>
    It is necessary that one provides a way to construct the struct,
    using the `<Variant>_Uninit` alias.


# Field name/Privacy table

These are the tables of visibilities/names given to fields.

'Private' means more private than the type.

'ModPubPriv' means the most public of the private fields.

'Public' means as public as the type.

### Tupled Structs

Tupled structs,ie:`struct What(u32,u32);`

&lt;field_position&gt; means the position of the field,starting from 0.

<style type="text/css">
.tg    {
    border-collapse:collapse;
    border-spacing:0;
}
.tg td{
        font-family:Arial, sans-serif;
        font-size:14px;
        padding:10px 5px;
        border-style:solid;
        border-width:1px;
        overflow:hidden;
        word-break:normal;
        border-color:black;
    }
.tg th{font-family:Arial, sans-serif;
    font-size:14px;
    font-weight:normal;
    padding:10px 5px;
    border-style:solid;
    border-width:1px;
    overflow:hidden;
    word-break:normal;
    border-color:black;
}
.tg .tg-baqh{
    text-align:center;
    vertical-align:top
}
</style>
<table class="tg">
    <tr>
        <th class="tg-baqh"></th>
        <th class="tg-baqh">&lt;DerivingType&gt;Trait</th>
        <th class="tg-baqh">&lt;DerivingType&gt;WithRuntime</th>
        <th class="tg-baqh">mod fields</th>
        <th class="tg-baqh">Const&lt;DerivingType&gt;</th>
    </tr>
    <tr>
        <td class="tg-baqh">
            private field 
        </td>
        <td class="tg-baqh">
            name:priv_field_&lt;field_position&gt;<br>
            visibility: #[doc(hidden)]
        </td>
        <td class="tg-baqh">
            name:rt_priv_field_&lt;field_position&gt;<br>
            visibility: #[doc(hidden)]
        </td>
        <td class="tg-baqh">
            name:field_&lt;field_position&gt;<br>
            visibility:ModPubPriv
        </td>
        <td class="tg-baqh">
            name:field_&lt;field_position&gt;<br>
            visibility:same visibility
        </td>
    </tr>
    <tr>
        <td class="tg-baqh">
            public field
        </td>
        <td class="tg-baqh">
            name:field_&lt;field_position&gt;<br>
            visibility: visibility of &lt;DerivingType&gt;
        </td>
        <td class="tg-baqh">
            name:rt_field_&lt;field_position&gt;<br>
            visibility: visibility of &lt;DerivingType&gt;
        </td>
        <td class="tg-baqh">
            name:U&lt;field_position&gt; (a type-level integer ,ie:U0)<br>
            visibility:pub (because of the U* accessor)
        </td>
        <td class="tg-baqh">
            name:&lt;field_position&gt;<br>
            visibility:same visibility
        </td>
    </tr>
</table>


### Braced Structs

Braced structs ie:`struct A{ field_name:u32 }`.

&lt;fieldname&gt; is the name of the field.

<table class="tg">
    <tr>
        <th class="tg-baqh"></th>
        <th class="tg-baqh">&lt;DerivingType&gt;Trait</th>
        <th class="tg-baqh">&lt;DerivingType&gt;WithRuntime</th>
        <th class="tg-baqh">mod fields</th>
        <th class="tg-baqh">Const&lt;DerivingType&gt;</th>
    </tr>
    <tr>
        <td class="tg-baqh">
            private field
        </td>
        <td class="tg-baqh">
            name:priv_&lt;fieldname&gt;<br>
            visibility: #[doc(hidden)]
        </td>
        <td class="tg-baqh">
            name:rt_priv_&lt;fieldname&gt;<br>
            visibility: #[doc(hidden)]
        </td>
        <td class="tg-baqh">
            name:&lt;fieldname&gt;<br>
            visibility:ModPubPriv
        </td>
        <td class="tg-baqh">
            name:&lt;fieldname&gt;<br>
            visibility:same visibility
        </td>
    </tr>
    <tr>
        <td class="tg-baqh">
           public field
        </td>
        <td class="tg-baqh">
            name:&lt;fieldname&gt;<br>
            visibility: visibility of &lt;DerivingType&gt;
        </td>
        <td class="tg-baqh">
            name:rt_&lt;fieldname&gt;<br>
            visibility: visibility of &lt;DerivingType&gt;
        </td>
        <td class="tg-baqh">
            name:&lt;fieldname&gt;<br>
            visibility: visibility of &lt;DerivingType&gt;
        </td>
        <td class="tg-baqh">
            name:&lt;fieldname&gt;<br>
            visibility:same visibility
        </td>
    </tr>
</table>




*/
