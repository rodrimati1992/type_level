use prelude::*;

type_fn!{define_trait
    /// Asserts that Self is the same type as R.
    trait=AssertEq_ [R]
    /// Asserts that Self is the same type as R.
    type=AssertEq
    /// Asserts that Self is the same type as R.
    fn_type=AssertEqOp
}


impl<L,R> AssertEq_<R> for L
where
    L:TypeIdentity<Type=R>
{
    type Output=R;
}