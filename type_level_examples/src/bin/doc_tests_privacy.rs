#[allow(unused_imports)]
#[macro_use]
extern crate type_level_values;
#[macro_use]
extern crate derive_type_level;

#[allow(unused_imports)]
use type_level_values::prelude::*;


pub mod submod {

    #[derive(TypeLevel)]
    #[typelevel(
        // print_derive,
        // skip_derive,
    )]
    #[allow(dead_code)]
    pub struct Rect {
        x: u32,
        pub(crate) y: u32,
        pub w: u32,
        pub(self) h: u32,
        pub(super) a: u32,
    }
}

///////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////////////

mod submod3 {

    mod submod2 {
        pub struct GenericType0<T>(T);
    }

    use self::submod2::GenericType0;

    pub type What = GenericType0<u32>;

    pub struct What2 {
        pub field2: GenericType0<u32>,
    }
}

pub use self::submod3::What;
pub use self::submod3::What2;

fn main() {}
