/*! 

Where all the documentation about how to use the library resides.

# Introduction 

[For the introduction to this library go here.](./introduction/index.html)

# Guide

This guide will guide any user towards defining type-level-values and  using them,
starting with simple examples,then getting gradually more complex.

The guide chapters are the `guide_*` submodules.

# Reference

This is the reference for topics not coverred in api documentation:
    
- [TypeLevel derive macro.](./attribute_typelevel/index.html) 

- [ConstConstructor derive macro.](./attribute_const_constructor/index.html)

- [Privacy: Details on how TypeLevel deals with privacy.](./reference_privacy/index.html)



*/

#[macro_use]
pub mod doc_code_snippets_macro;

#[path = "introduction.md"]
pub mod introduction;

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
