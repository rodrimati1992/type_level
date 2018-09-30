/*! 


These are things that all the derive macros in the derive_type_level crate share.


# Attributes


All meta-lists (`word( ... )`) allow multiple (possible repeated) attributes.


# Metadata attributes

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



# Attributes on a Type/Variant:

- skip_derive (optional attribute) : 
    causes the derive macro to not generate any code,useful when debugging 
    a code generation error.

- print_derive (optional attribute) : 
    causes the derive macro to panic with the code it would have generated,
    useful when debugging a code generation error.





*/
