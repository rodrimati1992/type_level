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



type_fn!{define_trait
    /// Unwraps a 0/1 element container into the contained value.
    ///
    /// On values like `None_` this returns `()`
    ///
    /// On values like `Some_<V>`/`Ok_<V>`/`Err_<V>` this returns `V`
    trait=IntoInner_ []
    type=IntoInner
    fn_type=IntoInnerOp
}