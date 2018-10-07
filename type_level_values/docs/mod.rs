/*! 

Where all the documentation about how to use the library resides.

*/

#[macro_use]
pub mod doc_code_snippets_macro;

#[path = "attribute_typelevel.md"]
pub mod attribute_typelevel;

#[path = "attribute_const_constructor.md"]
pub mod attribute_const_constructor;

#[path = "attribute_shared.md"]
pub mod attribute_shared;

pub mod reference_privacy;

#[path = "appendix_error_messages.md"]
pub mod appendix_error_messages;

#[path = "appendix_patterns.md"]
pub mod appendix_patterns;

/**

The guide for how to use this library,starting with the introduction.

*/
pub mod guide{

    #[path = "introduction.md"]
    pub mod introduction;

    include!{"./guide_01.rs"}

    include!{"./guide_02.rs"}

    include!{"./guide_03.rs"}

    include!{"./guide_04.rs"}

    include!{"./guide_05.rs"}

    include!{"./guide_06.rs"}

    include!{"./guide_07.rs"}

    include!{"./guide_08.rs"}

    include!{"./guide_09.rs"}

    include!{"./guide_10.rs"}

    include!{"./guide_11.rs"}

    include!{"./guide_12.rs"}
}

