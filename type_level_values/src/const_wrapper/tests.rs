use self::type_level_Dim2d::fields;
use super::*;

use crate_::field_traits::{GetFieldMt, MapFieldMt};
use crate_::fn_adaptors::*;
use crate_::std_ops::*;

use core_extensions::type_level_bool::False;

#[derive(Debug, Clone, TypeLevel, PartialEq)]
#[typelevel(
    // skip_derive,
    // print_derive,
    reexport(Struct),
)]
struct Dim2d {
    width: u32,
    height: u32,
}

type Wrapper0 = Construct<Dim2dType, ((fields::width, U3), (fields::height, U5))>;

#[test]
fn whole_ops() {
    let v: ConstWrapper<()> = ().to_cw();
    let _: () = v.identity_(());

    let v: ConstWrapper<&'static str> = v.set::<&'static str>();
    let _: &'static str = v.identity_("hello");

    let v: ConstWrapper<True> = v.set_val(True);
    let _: True = v.identity_(True);
}

#[test]
fn get_field() {
    let v0 = Wrapper0::CW;

    assert_eq!(v0[fields::width].get_as(u32::T), 3);
    assert_eq!(v0[fields::height].get_as(u32::T), 5);

    let _: U3 = v0.field(fields::width);
    let _: U5 = v0.field(fields::height);
}

#[test]
fn set_field() {
    {
        let v0 = Wrapper0::CW
            .set_field_val(fields::width, U0::MTVAL)
            .set_field_val(fields::height, U1::MTVAL);

        let _: ConstDim2d<U0, U1> = *v0;

        assert_eq!(
            v0.get_runt(),
            Dim2d {
                width: 0,
                height: 1
            }
        );
    }

    {
        let v0 = Wrapper0::CW
            .set_field::<fields::width, U10>()
            .set_field::<fields::height, U20>();

        let _: ConstDim2d<U10, U20> = *v0;

        assert_eq!(
            v0.get_runt(),
            Dim2d {
                width: 10,
                height: 20
            }
        );
    }
}

#[test]
fn map_field() {
    let v0 = Wrapper0::CW;

    let v0 = v0
        .map_field(fields::width, <AddMt<U3>>::CW)
        .map_field_fn(fields::height, |v| v * U2::MTVAL);

    assert_eq!(v0[fields::width].get_as(u32::T), 6);
    assert_eq!(v0[fields::height].get_as(u32::T), 10);

    let _: U6 = *v0.width;
    let _: U10 = *v0.height;
}

#[test]
fn map_all_to() {
    let v0 = Wrapper0::CW;

    {
        let v0 = v0.map_to(fields::width, <GetFieldMt<fields::height>>::CW);
        assert_eq!(v0.width.get_as(u32::T), 5);
        assert_eq!(v0.height.get_as(u32::T), 5);
        let _: U5 = *v0.width;
        let _: U5 = *v0.height;
    }
    {
        let v0: ConstWrapper<_> = v0.map_to_fn(fields::height, |v| v.width.get());
        assert_eq!(v0.width.get_as(u32::T), 3);
        assert_eq!(v0.height.get_as(u32::T), 3);
        let _: U3 = *v0.width;
        let _: U3 = *v0.height;
    }
}

#[test]
fn map_all() {
    let v0 = Wrapper0::CW;

    {
        let v0 = v0.map(MapFieldMt::<fields::height, AddMt<U3>>::CW);
        assert_eq!(v0.width.get_as(u32::T), 3);
        assert_eq!(v0.height.get_as(u32::T), 8);
        let _: U3 = *v0.width;
        let _: U8 = *v0.height;
    }
    {
        let v0 = v0.map_fn(|v| {
            v.to_cw()
                .set_field_val(fields::height, *v.height + U3::MTVAL)
        });
        assert_eq!(v0.width.get_as(u32::T), 3);
        assert_eq!(v0.height.get_as(u32::T), 8);
        let _: U3 = *v0.width;
        let _: U8 = *v0.height;
    }
}
