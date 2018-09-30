extern crate core_extensions;
extern crate derive_type_level_lib;
extern crate regex;

use std::fs::File;
use std::io::Write;

use derive_type_level_lib::typelevel as derive_typelevel;

use regex::Regex;

use core_extensions::{measure_time, SelfOps};

fn main() {
    let struct_decl = r##"

#[derive(Clone,Copy,Debug)]
#[derive(TypeLevel)]
//{insert_here}
#[typelevel(
    reexport="pub",
    derive(ConstEq,ConstOrd),
    items(
        IntoConstType(NoImpls),
        GetDiscriminant(),
    ),
    rename="DirectionConst",
    rename_trait="DirectionInterface",
    rename_consttype="DirectionConstType",
)]
pub enum Direction{
    #[typelevel(rename="LeftVariant")]
    Left,
    Right,
    Other{
        #[typelevel(rename="centerx")]
        value0:u32,
        value1:u32,
    }
}

    "##;

    let (dur, derived) =
        measure_time::measure(|| derive_typelevel::derive_from_str(&struct_decl));

    println!("cargo:warning=taken {} to run derive.", dur);

    let derived_s = derived.to_string();

    let prepend_with_newline = Regex::new(r#" (impl|#|pub|fn|use|for|where|\{|\})"#)
        .unwrap()
        .replace_all(&derived_s, "\n$1");

    let replaced_struct_decl = struct_decl.replace("//{insert_here}", "#[typelevel(skip_derive)]");

    let mut file = File::create("./src/derived_state.rs").unwrap();
    file.write_all(replaced_struct_decl.as_bytes()).unwrap();
    file.write_all(prepend_with_newline.as_bytes()).unwrap();
    file.write_all(
        r##"

        pub struct DirectionConv;

        mod type_level_DirectionConv{
            use type_level_values::prelude::*;
            use super::*;

            impl IntoConstType_<Direction> for DirectionConv{
                type ToConst=DirectionConstType;
            }
        }


        #[derive(Clone,Copy,Debug)]
        #[derive(TypeLevel)]
        pub struct People{
            #[typelevel(delegate(IntoConstType="DirectionConv"))]
            george:Direction,
            robert:Direction,

            are_all_alive:bool,
        }


    "##
            .as_bytes(),
    ).unwrap();
}
