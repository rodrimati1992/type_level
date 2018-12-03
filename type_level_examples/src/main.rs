extern crate type_level_examples;

use std::env::args;

use type_level_examples::*;

// Used because this crate supports Rust down to 1.20.0
// when `?` for Option was stabilized in Rust 1.22.0
macro_rules! try_opt {
    ($expr:expr) => {
        match $expr {
            Some(v) => v,
            None => return None,
        }
    };
}

pub enum ExampleKind {
    Regular,
    Playground,
    Syntax,
}

pub struct Example(ExampleKind, u32);

impl Example {
    fn from_list(list: &[String]) -> Option<Self> {
        let kind;

        let index = match list.get(0).map(|x| &**x) {
            Some("playground") => {
                kind = ExampleKind::Playground;
                1
            }
            Some("syntax") => {
                kind = ExampleKind::Syntax;
                1
            }
            Some(x) if x.parse::<u32>().is_ok() => {
                kind = ExampleKind::Regular;
                0
            }
            _ => return None,
        };
        let number_string = try_opt!(list.get(index));
        let number = try_opt!(number_string.parse::<u32>().ok());

        Some(Example(kind, number))
    }
}

pub fn main() {
    let which_binary = args().skip(1).collect::<Vec<_>>();
    let which_binary = Example::from_list(&which_binary);

    match which_binary {
        Some(Example(ExampleKind::Regular, 1)) => _01_deserialize_uninitialized::main_(),
        Some(Example(ExampleKind::Regular, 2)) => _02_mut_wrapper::main_(),
        Some(Example(ExampleKind::Regular, 3)) => _03_vis_wrapper::main_(),
        Some(Example(ExampleKind::Regular, 4)) => _04_typesafe_builder::main_(),
        Some(Example(ExampleKind::Regular, 5)) => _05_capabilities::main_(),
        Some(Example(ExampleKind::Regular, 6)) => _06_channel::main_(),
        Some(Example(ExampleKind::Regular, 7)) => _07_split_mut::main_(),
        Some(Example(ExampleKind::Regular, 8)) => _08_ranged_int::main_(),
        Some(Example(ExampleKind::Regular, 9)) => _09_type_hof::main_(),
        #[cfg(rust_1_26)]
        Some(Example(ExampleKind::Regular, 10)) => _10_state_machine::main_(),
        Some(Example(ExampleKind::Playground, 1)) => playground_01::main(),
        Some(Example(ExampleKind::Playground, 2)) => playground_02::main_(),
        Some(Example(ExampleKind::Syntax, 1)) => syntax_01_construct::main_(),
        _ => panic!(
            "\n\n\
Arguments must be one of: 
    `1-10`:Examples demonstrating functionality implemented using type-level values.
    `playground 1-2`:A local playground.
    `syntax 1`:The executable part of examples showing some piece of syntax .


Examples:
    0
    1
    2
    10
    playground 1
    playground 2
    syntax 1
\n\n"
        ),
    }
}
