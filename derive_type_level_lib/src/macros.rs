#[macro_export]
macro_rules! annotations_and_bounds {
    (inner; $decls:expr, $impl_ind:expr,let($annotations:ident, $bounds:ident) $(,)*) => {
        let $annotations;
        let $bounds;
        {
            let decls: &StructDeclarations = &$decls;
            let outer = &decls.attribute_settings.derived[$impl_ind];
            let inner = &decls.attribute_settings.derived[$impl_ind];
            $annotations = decls
                .declarations
                .iter()
                .map(move |_| outer.chain_impl_annotations(inner));

            $bounds = decls
                .declarations
                .iter()
                .map(move |_| outer.chain_bound_tokens(inner));
        }
    };
    (outer; $decls:expr, $impl_ind:expr,let($annotations:ident, $bounds:ident) $(,)*) => {
        let $annotations;
        let $bounds;
        {
            let decls: &StructDeclarations = &$decls;
            let outer = &decls.attribute_settings.derived[$impl_ind];
            $annotations = outer.impl_annotations();
            $bounds = outer.bound_tokens();
        }
    };
}

#[macro_export]
macro_rules! to_stream {
    ( $stream:ident ; $($expr:expr),* $(,)* ) => {{
        // use quote::TokenStreamExt;

        $( $expr.to_tokens($stream); )*
    }}
}