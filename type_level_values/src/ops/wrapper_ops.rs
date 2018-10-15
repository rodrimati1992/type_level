type_fn!{define_trait
    /// Returns the wrapped value if Self is the ok/some variant,otherwise fails to compile.
    trait=Unwrap_ []
    type=Unwrap
    fn_type=UnwrapOp
}

type_fn!{define_trait
    /// Returns the wrapped value if Self is the ok/some variant,otherwise returns Default_.
    trait=UnwrapOr_ [Default_]
    type=UnwrapOr
    fn_type=UnwrapOrOp
}
