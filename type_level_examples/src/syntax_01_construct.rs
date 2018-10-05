//! This example shows off the different ways a type-level value can be constructed
//! and converted to a runtime value.
//!


use type_level_values::field_traits::*;
use type_level_values::prelude::*;
// use type_level_values::std_types::option::fields as option_f;
use type_level_values::std_types::*;


use std::fmt;
use std::mem;

trait Extensions: Sized {
    #[inline]
    fn assert_zst(self) -> Self {
        assert_eq!(mem::size_of_val(&self), 0);
        self
    }

    #[inline]
    fn assert_size_println(self, size: usize) -> Self
    where
        Self: fmt::Debug,
    {
        assert_eq!(mem::size_of_val(&self), size);
        println!("{:?}", self);
        self
    }
}

impl<T> Extensions for T {}

fn constructing_std_types() {
    // accessing the constant through the module
    let _: Less_ = cmp_ordering::Less_.assert_zst();
    let _: Less_ = Less_.assert_zst();

    let _: Some_<U1> = Some_(U1::CW.assert_zst());
    let _ = Some_(U1::CW.assert_zst());
    let _ = None_.assert_zst();

    let _ = Ok_(False::CW.assert_zst());
    // this is equivalent to the previous line for instantiated values.
    let _: Ok_<False> = Ok_(False.into()).assert_zst();

    let _ = Err_(True::CW.assert_zst());
    // this is equivalent to the previous line for instantiated values.
    let _: Err_<True> = Err_(True.into()).assert_zst();
}

fn constructing_types_from_type_level_values() {
    let list: tlist![u32, u32, True] = tlist_val![u32, u32, True];
    assert_eq!(mem::size_of_val(&list), 0);
}

////////////////////////////////////////////

#[derive(Clone, Copy, Debug, TypeLevel, PartialEq)]
#[typelevel(reexport(Variants), derive(ConstEq, ConstOrd))]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, TypeLevel, PartialEq)]
#[typelevel(reexport(Struct, Traits), derive(ConstEq, ConstOrd))]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

use self::type_level_Point::fields as point_f;

#[derive(Clone, Copy, Debug, TypeLevel, PartialEq)]
#[typelevel(reexport(Struct, Traits), derive(ConstEq, ConstOrd))]
pub struct Entity {
    pub position: Option<Point>,
    pub facing: Direction,
    pub is_alive: bool,
}

use self::type_level_Entity::fields as entity_f;

////////////////////////////////////////////

fn constructing_new_types() {
    let point0: ConstPoint<U0, U3> = ConstPoint {
        x: U0::CW,
        y: U3::CW,
    }.assert_zst();

    assert_eq!(
        point0.into_runtime_ty(Point::T).assert_size_println(8),
        Point { x: 0, y: 3 }
    );

    let _entity0: ConstEntity<
        Some_<construct!(PointType=> point_f::x = U1, point_f::y = U2,)>,
        Up,
        True,
    > = ConstEntity::MTVAL.assert_zst();

    let _entity1 = ConstEntity {
        position: Some_(
            ConstPoint {
                x: U0::CW,
                y: U0::CW,
            }.to_cw(),
        ).to_cw(),
        facing: Up.to_cw(),
        is_alive: True.to_cw(),
    }.assert_zst();

    let _entity2 = <construct!(EntityType=>
        entity_f::position = None_,
        entity_f::facing = Up,
        entity_f::is_alive = False,
    )>::MTVAL
        .assert_zst();
}

pub fn main_ () {
    constructing_std_types();
    constructing_types_from_type_level_values();
    constructing_new_types();
}

/////////////////////////////////////////////////////////////////////

pub mod privacies {
    #[derive(TypeLevel)]
    #[typelevel(
        // print_derive,
        // skip_derive,
    )]
    #[allow(dead_code)]
    pub struct Privacies {
        pub a: u32,
        pub(crate) b: u32,
        pub(in syntax_01_construct::privacies) c: u32,
        pub(super) d: u32,
        pub(self) e: u32,
        f: u32,
    }
}
