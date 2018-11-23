use type_level_values::prelude::*;
use type_level_values::ops::Panic;

struct Hello_world;

pub fn main() {
    let _:TypeFn<Panic<Hello_world>,()>;
}
