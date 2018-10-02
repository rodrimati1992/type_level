//! This example demonstrates a struct that uses a const-struct to
//! configure which fields are accessible.
//!
//! Rectangle<I> is a rectangle with a Const-parameter that determines what fields are accessible.
//!

#[macro_use]
extern crate type_level_values;
#[macro_use]
extern crate derive_type_level;

use type_level_values::field_traits::*;
use type_level_values::prelude::*;


pub mod rectangle {
    use super::*;

    /// A rectangle where certain fields are inaccessible based on a const parameter.
    /// Many impls are also implemented on [Rectangle].
    #[derive(Clone, Copy, Debug, Default, PartialEq, ConstConstructor)]
    #[cconstructor(
        // print_derive,
        // skip_derive,
        Type(
            name = "Rectangle",
            doc = "A rectangle where certain fields are inaccessible based on a const parameter.",
            doc = "Many impls are also implemented on [RectangleInner].",
        ),
        ConstParam = "I"
    )]
    pub struct RectangleInner<I> {
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        marker: I,
    }

    impl Rectangle<RectangleAcessibleDefault> {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl<I> Rectangle<I>
    where
        I: RectangleAcessibleTrait,
    {
        pub fn x(&self) -> u32
        where
            I: RectAT<position = Accessible>,
        {
            self.x
        }
        pub fn y(&self) -> u32
        where
            I: RectAT<position = Accessible>,
        {
            self.y
        }
        pub fn w(&self) -> u32
        where
            I: RectAT<dimension = Accessible>,
        {
            self.w
        }
        pub fn h(&self) -> u32
        where
            I: RectAT<dimension = Accessible>,
        {
            self.h
        }
        pub fn set_x(&mut self, x: u32)
        where
            I: RectAT<position = Accessible>,
        {
            self.x = x;
        }
        pub fn set_y(&mut self, y: u32)
        where
            I: RectAT<position = Accessible>,
        {
            self.y = y;
        }
        pub fn set_w(&mut self, w: u32)
        where
            I: RectAT<dimension = Accessible>,
        {
            self.w = w;
        }
        pub fn set_h(&mut self, h: u32)
        where
            I: RectAT<dimension = Accessible>,
        {
            self.h = h;
        }

        pub fn reset(mut self) -> Rectangle<RectangleAcessibleDefault>
        where
            Self: MCPBounds<Reset, (), NextSelf = Rectangle<RectangleAcessibleDefault>>,
        {
            self.x = 0;
            self.y = 0;
            self.w = 0;
            self.h = 0;
            self.mutparam(Reset::new(), Default::default())
        }
    }

    const_method!{
        type ConstConstructor[]=( RectangleCC )
        type AllowedConversions=( allowed_conversions::All )

        pub fn MakeInaccessible[I,Field](I,Field)
        where [ I:SetField_<Field,Inaccessible>, ]
        {I::Output}
    }

    const_method!{
        type ConstConstructor[]=( RectangleCC )
        type AllowedConversions=( allowed_conversions::ByVal )

        fn Reset[I](I,()){ RectangleAcessibleDefault }
    }

}

pub use rectangle::*;

#[derive(Clone, Copy, Debug, TypeLevel)]
#[typelevel(derive(ConstEq, ConstOrd))]
pub struct RectangleAcessible {
    pub position: bool,
    pub dimension: bool,
}

use self::type_level_RectangleAcessible::{
    fields as ra, RectangleAcessibleTrait,
    RectangleAcessibleTrait as RectAT, RectangleAcessible_Uninit,
};

pub type RectangleAcessibleDefault = SetField<RectangleAcessible_Uninit, ra::All, Accessible>;

#[derive(Clone, Copy, Debug, TypeLevel)]
#[typelevel(derive(ConstEq, ConstOrd))]
pub enum Accessibility {
    Accessible,
    Inaccessible,
}

use self::type_level_Accessibility::{Accessible, Inaccessible};

fn main() {
    let mut rect = Rectangle::new();
    let rect_a = Rectangle::new().mutated(|r| {
        r.set_x(0);
        r.set_y(0);
        r.set_w(0);
        r.set_h(0);
    });
    assert_eq!(rect, rect_a);

    rect.set_x(100);
    rect.set_y(200);
    rect.set_w(300);
    rect.set_h(400);
    assert_eq!(rect.x(), 100);
    assert_eq!(rect.y(), 200);
    assert_eq!(rect.w(), 300);
    assert_eq!(rect.h(), 400);

    let mut rect = rect.mutparam(MakeInaccessible, ra::position::T);

    // now this wont compile
    // rect.set_x(100);
    // rect.set_y(200);

    // this is still accessible
    rect.set_w(111);
    rect.set_h(222);
    assert_eq!(rect.w(), 111);
    assert_eq!(rect.h(), 222);

    {
        #[allow(unused_variables, dead_code)]
        let rect: &mut Rectangle<_> = rect.mutparam_mut(MakeInaccessible, ra::dimension::T);

        // now none of this will compile
        // rect.set_x(100);
        // rect.set_y(200);
        // rect.set_w(111);
        // rect.set_h(222);
    }

    // This is accessible because ra::dimension was made innacessible only
    // for the mutable reference in the nested block.
    rect.set_w(300);
    rect.set_h(400);
    assert_eq!(rect.w(), 300);
    assert_eq!(rect.h(), 400);

    // Uses the "Reset" ConstMethod internally,to reset all the fields and the Const-parameter
    // to their default values.
    let rect = rect.reset();
    assert_eq!(rect, rect_a);

    // this won't compile because Reset has a private constructor
    // let rect=rect.mutparam(Reset::new(),());
    assert_eq!(rect, rect_a);
}

// ////////////////////////////////////////////////////////////////////////////////////////////////

// #[doc = "A rectangle where certain fields are inaccessible based on a const parameter."]
// #[doc = "Many impls are also implemented on [RectangleInner]."]
// pub type Rectangle<__ConstParam> =
//     RectangleInner<::type_level_values::reexports::PhantomWrapper<__ConstParam>>;
// pub struct RectangleCC {
//     _marker: ::type_level_values::reexports::VariantPhantom<()>,
// }
// #[allow(non_snake_case)]
// #[allow(non_camel_case_types)]
// mod const_constructor_RectangleInner {
//     use super::*;
//     use type_level_values::const_wrapper::{GetConstValue, PhantomWrapper, WrapperTrait};
//     use type_level_values::reexports::*;
//     use type_level_values::user_traits::{
//         ApplyConstParam_, ConstConstructor, ConstLayoutIndependent, GetConstConstructor_,
//         GetConstParam_,
//     };
//     #[doc(hidden)]
//     pub struct ConstDependentField<T>(T);
//     impl user_traits::AllowedOps for RectangleCC where {
//         type ExtensionMethods = type_level_bool::False;
//     }
//     impl ConstConstructor for RectangleCC where {}
//     impl<I> GetConstConstructor_ for RectangleInner<I>
//     where
//         Self: GetConstParam_,
//     {
//         type Constructor = RectangleCC;
//     }
//     unsafe impl<I, __Other: ?Sized> ConstLayoutIndependent<__Other> for RectangleInner<I> where
//         Self: user_traits::SameFieldLayout<ConstDependentField<typenum_reexports::U0>, __Other>
//     {}
//     impl<I, __ConstParam> GetConstParam_ for RectangleInner<I>
//     where
//         I: TypeIdentity<Type = PhantomWrapper<__ConstParam>>,
//     {
//         type Const = __ConstParam;
//     }
//     impl<I, __ConstParam, __Output> ApplyConstParam_<__ConstParam> for RectangleCC
//     where
//         PhantomWrapper<__ConstParam>: TypeIdentity<Type = I>,
//         RectangleInner<I>: TypeIdentity<Type = __Output>,
//         __Output: GetConstConstructor_<Const = __ConstParam, Constructor = Self>,
//     {
//         type Applied = __Output;
//     }
// }
