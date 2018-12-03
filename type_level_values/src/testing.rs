use prelude::*;

pub(crate) struct AssEqTy<T: ?Sized, U: ?Sized>(
    VariantPhantom<(VariantPhantom<T>, VariantPhantom<U>)>,
)
where
    T: TypeIdentity<Type = U>;
