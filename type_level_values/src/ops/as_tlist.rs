use crate_::discriminant::GetDiscriminant;
use new_types::type_list::{TList, TListType, TypeLevelListTrait};
use prelude::*;

/// Converts a ConstValue to a tlist,mostly used for deriving traits.
pub trait AsTList_ {
    type Output;
}

type_fn!{use_trait
    trait=AsTList_ []
    type=AsTList
    fn_type=AsTListOp
}

/// Converts an ConstValue to a tlist with the discriminant as the first element,
/// mostly used for deriving traits.
pub trait VariantAsTList_ {
    type Output: TypeLevelListTrait;
}

impl<This, out> VariantAsTList_ for This
where
    This: GetDiscriminant + AsTList_,
    TList<<This as GetDiscriminant>::Discriminant, <This as AsTList_>::Output>:
        TypeIdentity<Type = out>,
    out: ConstTypeOf_<Type = TListType> + TypeLevelListTrait,
{
    type Output = out;
}

type_fn!{use_trait
    trait=VariantAsTList_ []
    type=VariantAsTList
    fn_type=VariantAsTListOp
}

#[cfg(all(test, feature = "passed_tests"))]
mod test {
    use super::*;

    #[test]
    fn test_as_tlist() {
        let _: AssEqTy<AsTList<None_>, tlist![]>;

        let _: AssEqTy<AsTList<Some_<()>>, tlist![()]>;
        let _: AssEqTy<AsTList<Some_<U32>>, tlist![U32]>;

        let _: AssEqTy<AsTList<Ok_<()>>, tlist![()]>;
        let _: AssEqTy<AsTList<Ok_<U32>>, tlist![U32]>;

        let _: AssEqTy<AsTList<Err_<()>>, tlist![()]>;
        let _: AssEqTy<AsTList<Err_<U32>>, tlist![U32]>;

        let _: AssEqTy<AsTList<ConstRange<U10, U20>>, tlist![U10, U20]>;

        #[cfg(rust_1_26)]
        let _: AssEqTy<AsTList<ConstRangeInclusive<U10, U20>>, tlist![U10, U20]>;

        let _: AssEqTy<AsTList<()>, tlist![]>;
        let _: AssEqTy<AsTList<(U10,)>, tlist![U10]>;
        let _: AssEqTy<AsTList<(U10, U20)>, tlist![U10, U20]>;
        let _: AssEqTy<AsTList<(U10, U20, U30)>, tlist![U10, U20, U30]>;

        let _: AssEqTy<AsTList<tlist![]>, tlist![]>;
        let _: AssEqTy<AsTList<tlist![U10,]>, tlist![U10]>;
        let _: AssEqTy<AsTList<tlist![U10, U20]>, tlist![U10, U20]>;
        let _: AssEqTy<AsTList<tlist![U10, U20, U30]>, tlist![U10, U20, U30]>;
    }

    #[test]
    fn test_variant_as_tlist() {
        use new_types::type_list::{TList_Discr, TNil_Discr};
        use std_types::option::{None_Discr, Some_Discr};
        use std_types::range::Range_Discr;
        #[cfg(rust_1_26)]
        use std_types::range_inclusive::RangeInclusive_Discr;
        use std_types::result::{Err_Discr, Ok_Discr};
        use std_types::tuples::Tuple_Discr;

        type Test<L, R> = AssEqTy<VariantAsTList<L>, R>;

        let _: Test<None_, tlist![None_Discr]>;

        let _: Test<Some_<()>, tlist![Some_Discr, ()]>;
        let _: Test<Some_<U32>, tlist![Some_Discr, U32]>;

        let _: Test<Ok_<()>, tlist![Ok_Discr, ()]>;
        let _: Test<Ok_<U32>, tlist![Ok_Discr, U32]>;

        let _: Test<Err_<()>, tlist![Err_Discr, ()]>;
        let _: Test<Err_<U32>, tlist![Err_Discr, U32]>;

        let _: Test<ConstRange<U10, U20>, tlist![Range_Discr, U10, U20]>;

        #[cfg(rust_1_26)]
        let _: Test<
            ConstRangeInclusive<U10, U20>,
            tlist![RangeInclusive_Discr, U10, U20],
        >;

        let _: Test<(), tlist![Tuple_Discr]>;
        let _: Test<(U10,), tlist![Tuple_Discr, U10]>;
        let _: Test<(U10, U20), tlist![Tuple_Discr, U10, U20]>;
        let _: Test<(U10, U20, U30), tlist![Tuple_Discr, U10, U20, U30]>;

        let _: Test<tlist![], tlist![TNil_Discr]>;
        let _: Test<tlist![U10,], tlist![TList_Discr, U10]>;
        let _: Test<tlist![U10, U20], tlist![TList_Discr, U10, U20]>;
        let _: Test<tlist![U10, U20, U30], tlist![TList_Discr, U10, U20, U30]>;
    }
}
