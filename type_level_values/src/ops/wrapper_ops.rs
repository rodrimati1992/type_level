type_fn!{define-trait
    fn_type=UnwrapOp
    /// Returns the wrapped value if Self is the ok/some variant,otherwise fails to compile.
    trait=Unwrap_ []
    type=Unwrap
}
