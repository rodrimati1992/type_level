use crate_::extern_types::typenum::{BitType, SignedInteger, UnsignedInteger};
use crate_::prelude::*;
use crate_::ops::AssertEq;

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

type_fn!{use_trait 
    trait=ConstFrom_ [From_]
    type=ConstFrom
    fn_type=ConstFromOp
}

type_fn!{use_trait 
    trait=ConstInto_ [IntoConstType]
    type=ConstInto
    fn_type=ConstIntoOp
    method_like=ConstIntoMt
}

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

// #[cfg(test)]
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


    #[test]
    fn typenum_integer_convs(){
        type Test<L,Type,Expected>=
            AssertEq<
                ConstInto<L,Type>,
                Expected
            >;

        let _:Test< U0,SignedInteger,Z0 >;
        let _:Test< U1,SignedInteger,P1 >;
        let _:Test< U2,SignedInteger,P2 >;
        let _:Test< U3,SignedInteger,P3 >;
        let _:Test< U4,SignedInteger,P4 >;
        let _:Test< U5,SignedInteger,P5 >;

        let _:Test< Z0,UnsignedInteger,U0 >;
        let _:Test< P1,UnsignedInteger,U1 >;
        let _:Test< P2,UnsignedInteger,U2 >;
        let _:Test< P3,UnsignedInteger,U3 >;
        let _:Test< P4,UnsignedInteger,U4 >;
        let _:Test< P5,UnsignedInteger,U5 >;


    }
}
