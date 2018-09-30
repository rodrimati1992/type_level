macro_rules! doc_code_snippets {
    (
        mod
        $module_name:expr,type_ident =
        $type_ident:ident,template =
        $template:expr,code =
        $code:expr,
    ) => {
        #[derive(DocCodeSnippets)]
        #[doccode(code=$code)]
        #[doccode(mod_=$module_name)]
        #[doccode(template=$template)]
        #[allow(dead_code)]
        struct $type_ident;
    };
}
