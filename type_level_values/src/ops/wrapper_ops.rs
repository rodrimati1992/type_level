use crate_::fn_adaptors::Const;

type_fn!{define_trait
    /// Returns the wrapped value if Self is an ok/some value,otherwise fails to compile.
    trait=Unwrap_ []
    type=Unwrap
    fn_type=UnwrapOp
}

type_fn!{define_trait
    /// Returns the wrapped value if Self is an ok/some value,otherwise returns Default_.
    trait=UnwrapOr_ [Default_]
    type=UnwrapOr
    fn_type=UnwrapOrOp
    method_like=UnwrapOrMt
}
type_fn!{define_trait
    /// Returns the wrapped value if Self is an ok/some value,
    /// otherwise returns the result of calling DefaultFunc.
    ///
    /// The parameter of DefaultFunc is `()` for OptionType,
    /// `Error` for ResultType.
    trait=UnwrapOrElse_ [DefaultFunc]
    type=UnwrapOrElse
    fn_type=UnwrapOrElseOp
    method_like=UnwrapOrElseMt
}


impl<Def,This,Out> UnwrapOr_<Def> for This
where This:UnwrapOrElse_<Const<Def>,Output=Out>,
{
    type Output=Out;
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