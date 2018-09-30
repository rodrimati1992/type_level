use self::type_level_Dim2d::fields;
use super::*;

use crate_::field_traits::{GetFieldFn, GetFieldOp, MapFieldOp};
use crate_::ops::fn_adaptors::*;
use crate_::ops::fn_types::*;

use core_extensions::type_level_bool::False;
use typenum::consts::{
    U0, U1, U10, U11, U12, U13, U14, U15, U16, U2, U3, U4, U5, U6, U7, U8, U9, Z0,
};

#[derive(Debug, Clone, TypeLevel)]
#[typelevel(
    // skip_derive,
    // print_derive,
    reexport(Struct),
)]
pub struct Dim2d {
    pub width: u32,
    pub height: u32,
}

type Dim2dR<T> = AsRuntime<T, Dim2d>;

type Wrapper0 = Dim2dR<construct!(Dim2dType=> fields::width = U3, fields::height = U5,)>;

#[test]
fn get_field() {
    let v0 = Wrapper0::NEW;

    assert_eq!(v0.field_runt(fields::width), 3);
    assert_eq!(v0.field_runt(fields::height), 5);

    let _: U3 = v0.field(fields::width);
    let _: U5 = v0.field(fields::height);
}

#[test]
fn set_field() {
    let v0 = Wrapper0::NEW;

    let v0 = v0
        .set_field_val(fields::width, U0::MTVAL)
        .set_field_val(fields::height, U1::MTVAL);

    assert_eq!(v0[fields::width].get_runt(), 0);
    assert_eq!(v0[fields::height].get_runt(), 1);

    let _: U0 = *v0.width;
    let _: U1 = *v0.height;
}

#[test]
fn map_field() {
    let v0 = Wrapper0::NEW;

    let v0 = v0
        .map_field(fields::width, <ApplyRhs<AddOp, U3>>::PW)
        .map_field_fn(fields::height, |v| v * U2::MTVAL);

    assert_eq!(v0.field_runt(fields::width), 6);
    assert_eq!(v0.field_runt(fields::height), 10);

    let _: U6 = *v0.width;
    let _: U10 = *v0.height;
}

#[test]
fn map_all_to() {
    let v0 = Wrapper0::NEW;

    {
        let v0 = v0.map_to(fields::width, <GetFieldOp<fields::height>>::PW);
        assert_eq!(v0.width.get_as(u32::T), 5);
        assert_eq!(v0.height.get_as(u32::T), 5);
        let _: U5 = *v0.width;
        let _: U5 = *v0.height;
    }
    {
        let v0: ConstWrapper<_, _> = v0.map_to_fn(fields::height, |v| v.width.get());
        assert_eq!(v0.width.get_as(u32::T), 3);
        assert_eq!(v0.height.get_as(u32::T), 3);
        let _: U3 = *v0.width;
        let _: U3 = *v0.height;
    }
}

#[test]
fn map_all() {
    let v0 = Wrapper0::NEW;

    {
        let v0 = v0.map(ApplyNonSelf::<MapFieldOp, (fields::height, ApplyRhs<AddOp, U3>)>::PW);
        assert_eq!(v0.width.get_as(u32::T), 3);
        assert_eq!(v0.height.get_as(u32::T), 8);
        let _: U3 = *v0.width;
        let _: U8 = *v0.height;
    }
    {
        let v0 = v0.map_fn(|v| {
            v.to_pw()
                .set_field_val(fields::height, *v.height + U3::MTVAL)
        });
        assert_eq!(v0.width.get_as(u32::T), 3);
        assert_eq!(v0.height.get_as(u32::T), 8);
        let _: U3 = *v0.width;
        let _: U8 = *v0.height;
    }
}
