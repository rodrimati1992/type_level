//! This example demonstrates a struct that uses a const-struct to
//! configure which fields are accessible.
//!
//! Rectangle\<I> is a rectangle with a ConstValue-parameter that determines what fields are accessible.
//!

use type_level_values::field_traits::*;
use type_level_values::fn_adaptors::Const;
use type_level_values::prelude::*;

pub fn main_() {
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

    let mut rect = rect.mutparam(MakeInaccessible::NEW, ra::position::T);

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
        let rect: &mut Rectangle<_> = rect.mutparam_mut(MakeInaccessible::NEW, ra::dimension::T);

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

    // Uses the "ResetVis" Mutator Function internally,
    // to reset all the fields and the ConstValue-parameter
    // to their default values.
    let rect = rect.reset();
    assert_eq!(rect, rect_a);

    // this won't compile because ResetVis has a private constructor
    // let rect=rect.mutparam(ResetVis::NEW,());
    assert_eq!(rect, rect_a);
}

// ////////////////////////////////////////////////////////////////////////////////////////////////

pub mod rectangle {
    use super::*;

    #[derive(MutConstValue)]
    #[mcv(
        // PrintDerive,
        // SkipDerive,
        derive(Clone, Copy, Debug, Default, PartialEq),
        doc="
            A rectangle where certain fields are inaccessible based on a const parameter.
            Many impls are also implemented on [Rectangle].
        ",
        Type(
            name = "Rectangle",
            doc = "A rectangle where certain fields are inaccessible based on a const parameter.",
            doc = "Many impls are also implemented on [RectangleInner].",
        ),
        ConstValue = "I"
    )]
    pub struct __Rectangle<I> {
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        marker: ConstWrapper<I>,
    }

    impl Rectangle<RectangleAcessibleDefault> {
        pub fn new() -> Self {
            Rectangle::<RectangleAcessibleDefault>::default()
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
            Self: MCPBounds<ResetVis, (), NextSelf = Rectangle<RectangleAcessibleDefault>>,
        {
            self.x = 0;
            self.y = 0;
            self.w = 0;
            self.h = 0;
            self.mutparam(ResetVis::NEW, Default::default())
        }
    }

    mutator_fn!{
        type This[I]=(Rectangle<I>)
        type AllowedSelf=(allowed_self_constructors::All)

        pub fn MakeInaccessible[I,Field](I,Field)
        where [ I:SetField_<Field,Inaccessible>, ]
        {I::Output}
    }

    mutator_fn!{
        type This[I]=(Rectangle<I>)
        type AllowedSelf=(allowed_self_constructors::ByVal)

        fn ResetVis=Const<RectangleAcessibleDefault>;
    }

}

pub use self::rectangle::*;

#[derive(Clone, Copy, Debug, TypeLevel)]
#[typelevel(derive(ConstEq, ConstOrd))]
pub struct RectangleAcessible {
    pub position: bool,
    pub dimension: bool,
}

use self::type_level_RectangleAcessible::{
    fields as ra, RectangleAcessibleTrait, RectangleAcessibleTrait as RectAT,
    RectangleAcessible_Uninit,
};

pub type RectangleAcessibleDefault = SetField<RectangleAcessible_Uninit, ra::All, Accessible>;

#[derive(Clone, Copy, Debug, TypeLevel)]
#[typelevel(derive(ConstEq, ConstOrd))]
pub enum Accessibility {
    Accessible,
    Inaccessible,
}

use self::type_level_Accessibility::{Accessible, Inaccessible};
