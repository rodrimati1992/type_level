macro_rules! annotations_and_bounds {
    (inner; $decls:expr, $impl_ind:expr,let($annotations:ident, $bounds:ident) $(,)*) => {
        let $annotations;
        let $bounds;
        {
            let decls: &StructDeclarations = &$decls;
            let outer = &decls.attribute_settings.derived[$impl_ind];
            $annotations = decls
                .declarations
                .iter()
                .map(move |_| outer.impl_annotations());

            $bounds = decls
                .declarations
                .iter()
                .map(move |_| outer.bound_tokens());
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

macro_rules! to_stream {
    ( $stream:ident ; $($expr:expr),* $(,)* ) => {{
        // use quote::TokenStreamExt;

        $( $expr.to_tokens($stream); )*
    }}
}