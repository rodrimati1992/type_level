use extern_types::typenum::{BitType, SignedInteger, UnsignedInteger};
use prelude::*;

use core_extensions::type_level_bool::{Boolean, BooleanType, False, True};

use typenum::bit::{B0, B1};
use typenum::consts::{P1, U0, U1, Z0};

/// Conversion trait for compile-time values.
pub trait ConstFrom_<Other>: ConstType {
    type Output;
}

impl<T> ConstFrom_<T> for T::Type
where
    T: ConstValue,
{
    type Output = T;
}

pub trait ConstInto_<Other> {
    type Output;
}

impl<S, O> ConstInto_<S> for O
where
    S: ConstFrom_<O>+ConstType,
{
    type Output = S::Output;
}

pub type ConstFrom<This, Other> = <This as ConstFrom_<Other>>::Output;

pub type ConstInto<This, Other> = <This as ConstInto_<Other>>::Output;

mod boolean_impls {
    use super::*;

    impl ConstFrom_<False> for UnsignedInteger {
        type Output = U0;
    }
    impl ConstFrom_<True> for UnsignedInteger {
        type Output = U1;
    }

    impl ConstFrom_<False> for SignedInteger {
        type Output = Z0;
    }
    impl ConstFrom_<True> for SignedInteger {
        type Output = P1;
    }

    impl ConstFrom_<False> for BitType {
        type Output = B0;
    }
    impl ConstFrom_<True> for BitType {
        type Output = B1;
    }

    impl ConstFrom_<B0> for BooleanType {
        type Output = False;
    }
    impl ConstFrom_<B1> for BooleanType {
        type Output = True;
    }

    impl ConstFrom_<U0> for BooleanType {
        type Output = False;
    }
    impl ConstFrom_<U1> for BooleanType {
        type Output = True;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AssertIdentityConv<T>(T)
    where
        T: ConstValue,
        T::Type: ConstFrom_<T, Output = T>;

    #[test]
    fn identity_conversion() {
        let _: AssertIdentityConv<True>;
        let _: AssertIdentityConv<False>;

        let _: AssertIdentityConv<U0>;
        let _: AssertIdentityConv<U1>;

        let _: AssertIdentityConv<Z0>;
        let _: AssertIdentityConv<P1>;
    }
}
