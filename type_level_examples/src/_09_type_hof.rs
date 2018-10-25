//! This example shows off a type-level higher order function
//! to define a constructor function where the type of the first parameter depends
//! on the value of the second parameter (which is a ConstValue).
//!

#![deny(overflowing_literals)]

// use type_level_values::ops::{ TypeFn, TypeFn_};
use type_level_values::prelude::*;

use type_level_values::core_extensions::Void;


pub fn main_ () {
    main_0();
    main_1();
    main_2();
}

fn main_0() {
    let _ = CondType0::new("what", True);
    let _ = CondType0::new(100, True);
    let _ = CondType0::new((), True);

    // These lines don't compile.
    // let _=CondType0::new("what",False);
    // let _= CondType0::new(100, False);

    let _ = CondType0::new((), False);
}

fn main_1() {
    let _ = CondType1::new("what", True);
    let _ = CondType1::new(100, True);
    let _ = CondType1::new((), True);

    // all of these are the same type
    let _: Vec<Void> = CondType1::<VecFn, _>::new(default(), False).value;
    let _ = CondType1::<VecFn, _>::new(default(), False).value;
    let _ = CondType1::<VecFn, _>::new(Vec::new(), False).value;

    // all of these are the same type
    let _: Option<Void> = CondType1::<OptionFn, _>::new(default(), False).value;
    let _ = CondType1::<OptionFn, _>::new(default(), False).value;
    let _ = CondType1::<OptionFn, _>::new(None, False).value;
}

fn main_2() {
    let _ = CondType2::new("what", True);
    let _ = CondType2::new(100, True);
    let _ = CondType2::new((), True);

    let _: CondType2<u8, _> = CondType2::new(0, U0::MTVAL);
    let _: CondType2<u8, _> = CondType2::new(1, U0::MTVAL);
    let _ = CondType2::new(255, U0::MTVAL);
    let _ = CondType2::new(default(), U0::MTVAL);
    // let _                          = CondType2::new(256, U0::MTVAL);

    let _: CondType2<u16, _> = CondType2::new(0, U1::MTVAL);
    let _: CondType2<u16, _> = CondType2::new(1, U1::MTVAL);
    let _ = CondType2::new(255, U1::MTVAL);
    let _ = CondType2::new(256, U1::MTVAL);
    let _ = CondType2::new(0xffff, U1::MTVAL);
    let _ = CondType2::new(default(), U1::MTVAL);

    let _: CondType2<f32, _> = CondType2::new(0.2, U2::MTVAL);
    let _: CondType2<f32, _> = CondType2::new(1.5, U2::MTVAL);
    let _ = CondType2::new(255.1, U2::MTVAL);
    let _ = CondType2::new(default(), U2::MTVAL);

    let _: CondType2<(&str, &str), _> = CondType2::new(("", ""), U3::MTVAL);
    let _: CondType2<(u8, u8), _> = CondType2::new((99, 100), U3::MTVAL);
    let _: CondType2<((), ()), _> = CondType2::new(((), ()), U3::MTVAL);

    assert_eq!(CondType2::new(default(), U3::MTVAL).value, ("", ""));
    assert_eq!(CondType2::new(default(), U3::MTVAL).value, (0, 0));
    assert_eq!(CondType2::new(default(), U3::MTVAL).value, (false, false));
}

//////////////////////////////////////////////////////////////////////////////////

fn default<T>() -> T
where
    T: Default,
{
    T::default()
}

//////////////////////////////////////////////////////////////////////////////////

#[derive(TypeLevel)]
#[allow(dead_code)]
pub struct CFParams {
    pub constant: (),
    pub func: (),
}

pub use self::type_level_CFParams::{CFParamsTrait, ConstCFParams};

#[derive(Debug, Copy, Clone, ConstConstructor)]
#[cconstructor(
    // print_derive,
    Type = "CondType", 
    ConstParam = "C"
)]
pub struct CondTypeInner<T, C> {
    _marker: ConstWrapper<C>,
    value: T,
}

impl<T, C, _Out> CondType<T, C>
where
    C: CFParamsTrait,
    C::func: TypeFn_<(T, C::constant), Output = _Out>,
{
    fn new(value: _Out, _condition: C::constant) -> CondType<_Out, C> {
        CondType {
            value,
            _marker: ConstWrapper::NEW,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////

type_fn!{
    pub fn CondField0[T](T ,True ){ T }
           CondField0   ((),False){ () }
}

pub type CondType0<T, C> = CondType<T, ConstCFParams<C, CondField0>>;

//////////////////////////////////////////////////////////////////////////////////

type_fn!{
    pub fn CondField1[T](T ,True ){ T }

    CondField1[F](F,False)
    where [ F:TypeFn_<Void> ]
    { F::Output }
}

type_fn!{ pub fn VecFn[T](T){ Vec<T> } }
type_fn!{ pub fn OptionFn[T](T){ Option<T> } }

pub type CondType1<T, C> = CondType<T, ConstCFParams<C, CondField1>>;

//////////////////////////////////////////////////////////////////////////////////

type_fn!{
    pub fn CondField2[T](T ,True ){ T }
           CondField2   ((),U0){ u8 }
           CondField2   ((),U1){ u16 }
           CondField2   ((),U2){ f32 }
           CondField2[T](T,U3){ (T,T) }
}

pub type CondType2<T, C> = CondType<T, ConstCFParams<C, CondField2>>;

//////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////
