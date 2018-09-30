//!
//!
//!

#[macro_use]
extern crate type_level_values;
#[macro_use]
extern crate derive_type_level;

use type_level_values::field_traits::*;
use type_level_values::prelude::*;
use type_level_values::std_types::option::fields as option_f;
use type_level_values::std_types::*;


use std::fmt::Debug;

///////////////////////////////////////////////////////

#[derive(TypeLevel)]
pub struct Debuggable {
    #[typelevel(bound = "Debug + Default")]
    pub value: (),
}

use self::type_level_Debuggable::{ConstDebuggable, DebuggableTrait};

fn print_<T: DebuggableTrait>(_: T) {
    let v = T::value::default();
    println!("T::value::default() == {:?}", v);
}

///////////////////////////////////////////////////////

#[derive(TypeLevel)]
#[typelevel(reexport(Struct, Traits))]
pub struct MyRange<T> {
    #[typelevel(bound_runt = "IntoRuntime<T>")]
    pub start: T,
    #[typelevel(bound_runt = "IntoRuntime<T>")]
    pub end: T,
}

fn print_range<R: MyRangeWithRuntime<usize>>(_: R) {
    println!("start={:?}", R::rt_start::to_runtime());
    println!("end  ={:?}", R::rt_end::to_runtime());
}

///////////////////////////////////////////////////////

fn main() {
    {
        let what = ConstDebuggable { value: ().into() };

        print_(what);
    }

    {
        let range = ConstMyRange {
            start: U0::PW,
            end: U11::PW,
        };
        print_range(range);
    }
}
