//! This example demonstrates a zero overhead builder using a Const-parameter
//! to track initialization of the fields.
//!
//!
//!

#[macro_use]
extern crate type_level_values;
#[macro_use]
extern crate derive_type_level;

use std::fmt;
use std::fmt::Debug;
use std::mem::{self, ManuallyDrop};
use std::ptr;

use type_level_values::field_traits::*;
use type_level_values::prelude::*;

//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq, TypeLevel)]
pub enum Initialization {
    Initialized,
    // Uninitialized doesn't implement any regular traits other than Copy/Clone
    // so as to prevent reading uninitialized memory.
    //
    // Only derive Const* impls.
    Uninitialized,
}

use self::type_level_Initialization::{Initialized, Uninitialized};

//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(TypeLevel)]
#[typelevel(derive(Debug))]
pub struct AnimalInitialization {
    years_lived: Initialization,
    children: Initialization,
    family: Initialization,
}

use self::type_level_AnimalInitialization::{
    fields as ai_field, AnimalInitializationTrait, AnimalInitializationTrait as AnimalIT,
    AnimalInitialization_Uninit, ConstAnimalInitialization,
};

pub type AnimalUninitialized = SetField<AnimalInitialization_Uninit, ai_field::All, Uninitialized>;
pub type AnimalInitialized = SetField<AnimalInitialization_Uninit, ai_field::All, Initialized>;

//////////////////////////////////////////////////////////////////////////////////////////////

type AnimalBuilder<I> = AnimalBuilderInner<PhantomWrapper<I>>;

#[derive(ConstConstructor)]
#[cconstructor(Type(use_ = "AnimalBuilder"), ConstParam = "I")]
pub struct AnimalBuilderInner<I>
where
    I: IntoRuntime<AnimalInitialization>,
{
    years_lived: ManuallyDrop<u32>,
    children: ManuallyDrop<u32>,
    family: ManuallyDrop<String>,
    _marker: PhantomWrapper<I>,
}

impl AnimalBuilder<AnimalUninitialized> {
    pub fn new() -> Self {
        unsafe { mem::uninitialized() }
    }
}

impl<I> Drop for AnimalBuilderInner<I>
where
    I: IntoRuntime<AnimalInitialization>,
{
    fn drop(&mut self) {
        let initialization = I::to_runtime();

        macro_rules! drop_field {
            ( $($field:ident),* ) => {
                $(match initialization.$field{
                    Initialization::Initialized=>ManuallyDrop::drop(&mut self.$field),
                    _=>{}
                })*
            }
        }
        unsafe {
            drop_field!{years_lived,children,family}
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! init_method {
    ( $field:ident : $field_ty:ty ) => {
        #[inline(always)]
        pub fn $field<__NC,__NT>(
            mut self,
            $field:$field_ty,
        )->__NT
        where
            Self:MCPBounds<SetF,ai_field::$field,NextConst=__NC,NextSelf=__NT>,
            __NC:IntoRuntime<AnimalInitialization>,
        {
            unsafe{
                ptr::write(&mut self.$field,ManuallyDrop::new($field));
                self.mutparam(SetF::new(),Default::default())
            }
        }
    }
}

impl<I> AnimalBuilder<I>
where
    I: AnimalInitializationTrait + IntoRuntime<AnimalInitialization>,
{
    init_method!{years_lived:u32}
    init_method!{children:u32   }
    init_method!{family:String  }
}
impl<I> AnimalBuilder<I>
where
    I: AnimalInitializationTrait + IntoRuntime<AnimalInitialization>,
{
    pub fn build(self) -> Animal
    where
        I: TypeIdentity<Type = AnimalInitialized>,
    {
        let this = ManuallyDrop::new(self);

        unsafe {
            Animal {
                years_lived: ptr::read(&*this.years_lived),
                children: ptr::read(&*this.children),
                family: ptr::read(&*this.family),
            }
        }
    }
}

const_method!{
    type ConstConstructor[]=( AnimalBuilderCC )
    type AllowedConversions=( allowed_conversions::ByVal )

    pub fn SetF[I,Field](I,Field)
    where [
        I:SetField_<Field,Initialized> ,
    ]
    {I::Output}
}

//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Animal {
    pub years_lived: u32,
    pub children: u32,
    pub family: String,
}

//////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let animal = AnimalBuilder::new()
        .children(2)
        .years_lived(10)
        .family("marsupials".into())
        .build();
    println!("{:?}", animal);
}
