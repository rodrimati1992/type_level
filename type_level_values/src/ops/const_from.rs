use extern_types::typenum::{BitType, SignedInteger, UnsignedInteger};
use prelude::*;

use core_extensions::type_level_bool::{Boolean, BooleanType, False, True};

use typenum::bit::{B0, B1};
use typenum::consts::{P1, U0, U1, Z0};

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
    S: ConstFrom_<O> + ConstType,
{
    type Output = S::Output;
}

type_fn!{alias ConstFromOp[IntoConstType,From_]=ConstFrom_}
type_fn!{alias ConstIntoOp[From_,IntoConstType]=ConstInto_}

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

#[cfg(all(test,feature="passed_tests"))]
mod tests {
    use super::*;

    #[test]
    fn identity_conversion() {

        macro_rules! check_identity_conv {
            ( $($type_:ty),*  $(,)* ) => (
                $(
                    let _:AssEqTy<$type_, ConstFrom<ConstTypeOf<$type_>,$type_> >;

                )*
            )
        }

        check_identity_conv!{
            True,False,
            U0,U1,U2,U3,U4,U5,
            Z0,
            N1,N2,N3,N4,N5,
            P1,P2,P3,P4,P5,
            None_,Some_<False>,Some_<True>,
            Ok_<False>,Ok_<True>,
            Err_<False>,Err_<True>,
            ConstRange<U0,U0>,
            ConstRange<U1,U4>,
            ConstRange<U6,U10000>,
            ConstRangeInclusive<U0,U0>,
            ConstRangeInclusive<U1,U4>,
            ConstRangeInclusive<U6,U10000>,
        }
    }
}
