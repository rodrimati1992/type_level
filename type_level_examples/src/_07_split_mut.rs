//! This example demonstrates an implementation of mutable reference splitting,
//! where 2 mutable references are returned,with disjoint access to the fields .
//!
//! The main limitation of this approach is
//! that no derived traits are accessible on the split
//! references due to it being unsafe to implement,
//! that it requires using some private uses of unsafe,
//! and that it uses a PinnedMut (instead of a &mut Rectangle)
//! to access mutable methods on the Rectangle.
//!
//!
//!

use type_level_values::field_traits::*;
use type_level_values::new_types::{TList, TNil};
use type_level_values::ops::{VariantAsTList_};
use type_level_values::collection_ops::{FoldL_,};
use type_level_values::prelude::*;
use type_level_values::fn_adaptors::{ApplyRhs,Const};
// use type_level_values::reexports::type_level_bool::False;

use std::cmp::{self, PartialOrd};
use std::mem;

pub fn main_ () {
    let mut rect = Rectangle::new().mutated(|r| {
        r.set_x(50);
        r.set_y(60);
        r.set_w(70);
        r.set_h(80);
    });
    let mut rect_b = Rectangle::new().mutated(|r| {
        r.set_x(10);
        r.set_y(20);
        r.set_w(30);
        r.set_h(40);
    });

    {
        let _: &mut Rectangle<RectangleAcessibleDefault, _> = &mut rect;
        let (rect_inner, rem) = rect.split_mut(tlist_val![ra::x, ra::y, ra::w, ra::h]);
        let _: PinnedMut<Rectangle<RectangleAcessibleDefault, _>> = rect_inner;
        let _: PinnedMut<Rectangle<RectangleAcessibleNone, _>> = rem;
    }
    {
        let (_rect_inner, _rem_a) = rect.split_mut(tlist_val![ra::x, ra::y]);
        let (_rect_b_inner, _rem_b) = rect_b.split_mut(tlist_val![ra::x, ra::y]);
    }
    println!("{:?}", rect);
    println!("{:?}", rect_b);

    {
        let (x, mut rem) = rect.split_mut(tlist_val![ra::x]);

        let (y, mut rem) = rem.split_mut(tlist_val![ra::y]);

        let _: SetField<RectangleAcessibleNone, ra::x, Accessible> = x.accessible_fields_ty();

        let _: SetField<RectangleAcessibleNone, ra::y, Accessible> = y.accessible_fields_ty();

        let _: SetFieldsTo<RectangleAcessibleDefault, tlist![ra::x, ra::y], Inaccessible> =
            rem.accessible_fields_ty();

        // this wont compile
        // rem.split_mut(tlist_val![ra::x]);
        // rem.split_mut(tlist_val![ra::y]);

        let (_w, mut rem) = rem.split_mut(tlist_val![ra::w]);

        assert_eq!(
            rem.accessible_fields(),
            RectangleAcessible {
                x: Accessibility::Inaccessible,
                y: Accessibility::Inaccessible,
                w: Accessibility::Inaccessible,
                h: Accessibility::Accessible,
            }
        );

        let (_h, rem) = rem.split_mut(tlist_val![ra::h]);

        rem.accessible_fields_ty().assert_ty(
            ConstRectangleAcessible {
                x: Inaccessible.into(),
                y: Inaccessible::CW,
                w: Inaccessible.into_(ConstWrapper::T),
                h: Inaccessible::CW,
            }.ty_(),
        );
    }

    {
        let (mut x_y, mut w_h) = rect.split_mut(tlist_val![ra::x, ra::y]);

        x_y.set_position(2, 3);

        assert_eq!(x_y.get_position(), (2, 3));

        // this won't compile
        // w_h.set_position(2,3);

        // can't accidentally borrow a variable that is in use by another reference.
        // let (x_w,rem)=w_h.split_mut(tlist_val![ ra::w,ra::x ]);

        {
            let (mut x, mut y) = x_y.split_mut(tlist_val![ra::x]);

            // does not compile
            // x.set_position(2,3);
            // y.set_position(2,3);

            assert_eq!(x.x(), 2);
            assert_eq!(y.y(), 3);

            mem::swap(x.x_mut(), y.y_mut());

            assert_eq!(x.x(), 3);
            assert_eq!(y.y(), 2);

            mem::swap(x.x_mut(), y.y_mut());
        }

        // does not compile
        // x_y.set_w(2);
        // x_y.set_h(3);
        // println!("area:{}",x_y.get_area());

        // does not compile
        // w_h.set_x(2);
        // w_h.set_y(3);
        w_h.set_w(4);
        w_h.set_h(5);
        assert_eq!(w_h.get_area(), 20);
        w_h.set_dimensions(x_y.x() * x_y.y(), 10);
        assert_eq!(w_h.get_area(), 60);
    }

    assert_eq!(rect.x(), 2);
    assert_eq!(rect.y(), 3);
    assert_eq!(rect.w(), 6);
    assert_eq!(rect.h(), 10);
}

mod pin_api {
    use super::*;

    use std::ops::Deref;

    /// Similar to ::std::ops::DerefMut
    /// with the additional requirement that the returned &mut Self::PinnedTarget
    /// must not be used to overwrite Self::PinnedTarget it points to,
    /// using mem::replace/mem::swap/etc.
    pub trait DerefMutPin {
        type PinnedTarget;
        unsafe fn deref_mut_pin(&mut self) -> &mut Self::PinnedTarget;
    }

    impl<I, P> DerefMutPin for Rectangle<I, P> {
        type PinnedTarget = Self;
        unsafe fn deref_mut_pin(&mut self) -> &mut Self {
            self
        }
    }

    impl<'a, I> DerefMutPin for PinnedMut<'a, I> {
        type PinnedTarget = I;

        unsafe fn deref_mut_pin(&mut self) -> &mut I {
            &mut self.mut_ref
        }
    }

    #[derive(Debug, PartialEq, Eq, Ord, Hash)]
    pub struct PinnedMut<'a, I: 'a> {
        mut_ref: &'a mut I,
    }

    impl<'a, I: 'a> PartialOrd for PinnedMut<'a, I>
    where
        I: PartialOrd,
    {
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            self.mut_ref.partial_cmp(&other.mut_ref)
        }
    }

    impl<'a, I> PinnedMut<'a, I> {
        pub fn new(mut_ref: &'a mut I) -> Self {
            PinnedMut { mut_ref }
        }
    }

    impl<'a, I> Deref for PinnedMut<'a, I> {
        type Target = I;

        fn deref(&self) -> &I {
            &*self.mut_ref
        }
    }

}

pub use self::pin_api::{DerefMutPin, PinnedMut};

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct IsValue;

// Intentionally does not implement any traits.
// Implement traits after handling the fact that a split reference
// should not access all the fields in the derived code.
pub struct IsMutRef;

pub mod rectangle {
    use super::*;
    #[derive(MutConstValue)]
    #[mcv(
        doc="
            A rectangle where certain fields are inaccessible based on a const parameter.
            Many impls are also implemented on [Rectangle].
        ",
        derive(Copy, Clone, Default, Debug, PartialEq),
        repr(C),
        Type(
            name = "Rectangle",
            doc = "A rectangle where certain fields are inaccessible based on a const parameter.",
            doc = "Many impls are also implemented on [RectangleInner].",
        ),
        ConstValue = "I",
    )]
    pub struct RectangleInner<I, P> {
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        accessible_fields: VariantPhantom<(I, P)>,
    }

    impl Rectangle<RectangleAcessibleDefault, IsValue> {
        pub fn new() -> Self {
            Rectangle::default()
        }
    }

    macro_rules! getters_setters {
        ( $(($field:ident : $field_ty:ty ,$setter:ident,$mut_acc:ident))* ) => {
            impl<I,P> Rectangle<I,P>{
                $(
                    pub fn $field(&self) -> $field_ty
                    where I: RectangleAcessibleTrait<$field=Accessible>,
                    {
                        self.$field
                    }
                )*
            }

            /// Defining methods for Rectangle\<I> and PinnedMut\<'a,Rectangle\<I>>
            pub trait RectangleMutAccessors<I,P>:DerefMutPin<PinnedTarget=Rectangle<I,P>>{
                $(
                    fn $setter(&mut self, $field: $field_ty)
                    where I: RectangleAcessibleTrait<$field=Accessible>,
                    {
                        // this is safe because i'm only replacing a field I am allowed to access
                        unsafe{
                            self.deref_mut_pin().$field = $field;
                        }
                    }
                    fn $mut_acc(&mut self)->&mut $field_ty
                    where I: RectangleAcessibleTrait<$field=Accessible>,
                    {
                        // this is safe because i'm only getting a reference I am allowed to get,
                        // and the lifetime is constrained by the method signature.
                        unsafe{
                            &mut (*(self.deref_mut_pin() as *mut Rectangle<I,P>)).$field
                        }
                    }

                )*
                #[inline(always)]
                /// Returns 2 mutable references into this rectangle which have disjoint access
                /// to the fields.
                ///
                /// The first mutable reference has access to
                /// the fields mentioned in the `Fields` type-level-list,
                /// while the second reference has access to the remaining fields
                /// that were accessible.
                ///
                /// It is a compiletime error to attempt to split on fields innaccessible
                /// to `&mut self`.
                ///
                ///
                fn split_mut<'s,Fields, _Out0, _Out1>(
                    &'s mut self,
                    _: Fields,
                ) -> (
                    PinnedMut<'s,Rectangle<_Out0,IsMutRef>>,
                    PinnedMut<'s,Rectangle<_Out1,IsMutRef>>
                )
                where
                    Fields: FoldL_<
                        RectangleAcessibleNone,
                        SplitMutSetField<Accessible>,
                        Output = _Out0
                    >,
                    Fields: FoldL_<I, SplitMutSetField<Inaccessible>, Output = _Out1>,
                {
                    unsafe {
                        let this=self.deref_mut_pin() as *mut Rectangle<I,P>;
                        (
                            PinnedMut::new(&mut *(this as *mut Rectangle<_Out0,IsMutRef>)),
                            PinnedMut::new(&mut *(this as *mut Rectangle<_Out1,IsMutRef>)),
                        )
                    }
                }
            }
            impl<This,I,P> RectangleMutAccessors<I,P> for This
            where This:DerefMutPin<PinnedTarget=Rectangle<I,P>>
            {}
        }
    }

    getters_setters!{
        (x:u32,set_x,x_mut)
        (y:u32,set_y,y_mut)
        (w:u32,set_w,w_mut)
        (h:u32,set_h,h_mut)
    }

    impl<I> Rectangle<I, IsValue> {
        pub fn reset(mut self) -> Rectangle<RectangleAcessibleDefault, IsValue> {
            self.x = 0;
            self.y = 0;
            self.w = 0;
            self.h = 0;
            self.mutparam(Reset::NEW, ().ty_())
        }
    }
    impl<I, P> Rectangle<I, P>
    where
        I: RectangleAcessibleTrait,
    {
        pub fn accessible_fields(&self) -> RectangleAcessible
        where
            I: IntoRuntime<RectangleAcessible>,
        {
            I::to_runtime()
        }

        pub fn accessible_fields_ty(&self) -> I {
            I::MTVAL
        }
    }

    type_fn!{
        captures(To)
        fn SplitMutSetField[Rect,Field](Rect,Field)
        where[ Rect:Piped_<MapFieldMt<Field, ApplyRhs<CheckUnequal,To>>,Output=Out> ]
        {
            let Out;
            Out
        }
    }

    type_fn!{
        fn CheckUnequal(Accessible  ,Inaccessible){ Inaccessible }
           CheckUnequal(Inaccessible,Accessible){ Accessible }
    }

    mutator_fn!{
        type This[I, P]=(Rectangle<I, P>)
        type AllowedSelf=(allowed_self_constructors::ByVal)
        
        fn Reset=Const<RectangleAcessibleDefault>;
    }

}

/// Defining methods for Rectangle\<I> and PinnedMut\<'a,Rectangle\<I>>
pub trait RectangleMutMethods<I, P>: RectangleMutAccessors<I, P> {
    fn set_position(&mut self, x: u32, y: u32)
    where
        I: RectangleAcessibleTrait<x = Accessible, y = Accessible>,
    {
        self.set_x(x);
        self.set_y(y);
    }
    fn set_dimensions(&mut self, w: u32, h: u32)
    where
        I: RectangleAcessibleTrait<w = Accessible, h = Accessible>,
    {
        self.set_w(w);
        self.set_h(h);
    }
}

impl<This, I, P> RectangleMutMethods<I, P> for This where This: RectangleMutAccessors<I, P> {}

impl<I, P> Rectangle<I, P> {
    fn get_position(&self) -> (u32, u32)
    where
        I: RectangleAcessibleTrait<x = Accessible, y = Accessible>,
    {
        (self.x(), self.y())
    }

    fn get_area(&self) -> u32
    where
        I: RectangleAcessibleTrait<w = Accessible, h = Accessible>,
    {
        self.w() * self.h()
    }
}

pub use self::rectangle::*;

#[derive(Clone, Copy, Debug, TypeLevel, PartialEq)]
#[typelevel(derive(ConstEq, ConstOrd))]
pub struct RectangleAcessible {
    pub x: Accessibility,
    pub y: Accessibility,
    pub w: Accessibility,
    pub h: Accessibility,
}

pub use self::type_level_RectangleAcessible::{
    fields as ra, ConstRectangleAcessible, RectangleAcessibleTrait, RectangleAcessible_Uninit,
};

pub type RectangleAcessibleDefault = SetField<RectangleAcessible_Uninit, ra::All, Accessible>;
pub type RectangleAcessibleNone = SetField<RectangleAcessible_Uninit, ra::All, Inaccessible>;

#[derive(Clone, Copy, Debug, TypeLevel, PartialEq)]
#[typelevel(derive(ConstEq, ConstOrd))]
pub enum Accessibility {
    Accessible,
    Inaccessible,
}

pub use self::type_level_Accessibility::{Accessible, Inaccessible};
