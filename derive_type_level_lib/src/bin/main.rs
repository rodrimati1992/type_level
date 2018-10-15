extern crate derive_type_level_lib;



fn main(){
    
    let ret=derive_type_level_lib::typelevel::derive_from_str(r###"
        #[derive(TypeLevel)]
        #[typelevel(
            // print_derive,
            // skip_derive,
            reexport = "pub",
            derive(ConstEq, ConstOrd),
            items(runtime_conv(Internal="StdRange")),
        )]
        #[allow(dead_code)]
        #[doc(hidden)]
        pub struct Range<T> {
            pub start: T,
            pub end: T,
        }
    "###);

    println!("returned:\n\n\n{}\n\n", ret);

    
}
