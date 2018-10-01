use crate_::std_types::cmp_ordering::{Equal_, Greater_, Less_, OrderingTrait, OrderingType};
use prelude::*;

use core_extensions::type_level_bool::{Boolean, BooleanType, False, True};
use core_extensions::Void;

use typenum::consts::{U0, U1};

impl ConstType for BooleanType {}

impl ConstTypeOf_ for True {
    type Type = BooleanType;
}
impl ConstTypeOf_ for False {
    type Type = BooleanType;
}

impl ConstEq_<False> for False {
    type Output = True;
}
impl ConstEq_<True> for False {
    type Output = False;
}
impl ConstEq_<False> for True {
    type Output = False;
}
impl ConstEq_<True> for True {
    type Output = True;
}

impl ConstOrd_<False> for False {
    type Output = Equal_;
}
impl ConstOrd_<True> for False {
    type Output = Less_;
}
impl ConstOrd_<False> for True {
    type Output = Greater_;
}
impl ConstOrd_<True> for True {
    type Output = Equal_;
}

impl IntoRuntime<bool> for True {
    fn to_runtime() -> bool {
        true
    }
}
impl IntoRuntime<bool> for False {
    fn to_runtime() -> bool {
        false
    }
}

#[cfg(rust_1_20)]
impl IntoConstant<bool> for True {
    const VALUE: bool = true;
}

#[cfg(rust_1_20)]
impl IntoConstant<bool> for False {
    const VALUE: bool = false;
}

impl IntoConstType_ for bool {
    type ToConst = BooleanType;
}

//////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////

pub mod boolean_variants {
    use super::*;
    use enum_stuff::{Discriminant, DiscriminantFor};
    use typenum::consts::{U0, U1};

    pub type FalseVariant = Discriminant<False, BooleanType, U0>;
    pub type TrueVariant = Discriminant<True, BooleanType, U1>;

    impl GetDiscriminant for True {
        type Discriminant = TrueVariant;
        type Variant = True;
    }

    impl GetDiscriminant for False {
        type Discriminant = FalseVariant;
        type Variant = False;
    }

}

///////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, Default)]
pub struct VoidType;

impl ConstType for VoidType {}

impl ConstTypeOf_ for Void {
    type Type = VoidType;
}

///////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////
