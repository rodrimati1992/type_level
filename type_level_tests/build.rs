extern crate core_extensions;
extern crate derive_type_level_lib;

#[allow(unused_imports)]
use derive_type_level_lib::typelevel as derive_typelevel;
use derive_type_level_lib::const_constructor as derive_cconstr;

#[allow(unused_imports)]
use core_extensions::{measure_time, SelfOps};

fn main() {
    let struct_decl = r##"

// #[derive(Clone,Copy,Debug)]
// #[derive(TypeLevel)]
// //{insert_here}
// #[typelevel(
//     reexport="pub",
//     derive(ConstEq,ConstOrd),
//     items(
//         IntoConstType(NoImpls),
//         GetDiscriminant(),
//     ),
//     rename="DirectionConst",
//     rename_trait="DirectionInterface",
//     rename_consttype="DirectionConstType",
// )]
// pub enum Direction{
//     #[typelevel(rename="LeftVariant")]
//     Left,
//     Right,
//     Other{
//         #[typelevel(rename="centerx")]
//         value0:u32,
//         value1:u32,
//     }
// }


#[derive(ConstConstructor)]
#[cconstructor(Type(use_="11"),ConstParam="C")]
pub struct Wrapper<C>{
    _const:PhantomWrapper<C>,
}


    "##;

    let (dur, derived) =
        measure_time::measure(|| derive_cconstr::derive_from_str(&struct_decl));

    println!("cargo:warning=taken {} to run derive.", dur);

}
