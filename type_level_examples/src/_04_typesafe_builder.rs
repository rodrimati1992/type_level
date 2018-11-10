//! This example demonstrates a zero overhead builder using a ConstValue-parameter
//! to track initialization of the fields.
//!
//!
//!


pub fn main_ () {
    let animal = AnimalBuilder::new()
        .children(2)
        .years_lived(10)
        .family("marsupials".into())
        .build();
    println!("{:?}", animal);
}


//////////////////////////////////////////////////////////////////////////////////////////////

// use std::fmt;
// use std::fmt::Debug;
use std::mem::{self, ManuallyDrop};
use std::ptr;

use type_level_values::initialization::*;
use type_level_values::field_traits::*;
use type_level_values::prelude::*;

//////////////////////////////////////////////////////////////////////////////////////////////


#[derive(TypeLevel)]
#[typelevel(derive(Debug))]
pub struct AnimalInitialization {
    years_lived: FieldInit,
    children: FieldInit,
    family: FieldInit,
}

use self::type_level_AnimalInitialization::{
    fields as ai_field, AnimalInitializationTrait,
    AnimalInitialization_Uninit,
};


pub type AnimalUninitialized = 
    <AnimalInitialization_Uninit as InitializationValues>::Uninitialized;
    
pub type AnimalInitialized = 
    <AnimalInitialization_Uninit as InitializationValues>::Initialized;


//////////////////////////////////////////////////////////////////////////////////////////////

type AnimalBuilder<I> = AnimalBuilder_Ty<ConstWrapper<I>>;

#[derive(MutConstValue)]
#[mcv(
    Type(use_ = "AnimalBuilder"), 
    ConstValue = "I"
)]
pub struct AnimalBuilderInner<I>
where
    I: IntoRuntime<AnimalInitialization>,
{
    years_lived: ManuallyDrop<u32>,
    children: ManuallyDrop<u32>,
    family: ManuallyDrop<String>,
    _marker: ConstWrapper<I>,
}

impl<I> AnimalBuilder<I> 
where 
    AnimalUninitialized:TypeIdentity<Type=I>,
    I: IntoRuntime<AnimalInitialization>,
{
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
                    FieldInit::IsInitField{..}=>ManuallyDrop::drop(&mut self.$field),
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
            Self:MCPBounds<InitializeField,ai_field::$field,NextConst=__NC,NextSelf=__NT>,
            __NC:IntoRuntime<AnimalInitialization>,
        {
            unsafe{
                ptr::write(&mut self.$field,ManuallyDrop::new($field));
                self.mutparam(InitializeField::NEW,Default::default())
            }
        }
    }
}

impl<I> AnimalBuilder<I>
where
    I: IntoRuntime<AnimalInitialization>,
{
    init_method!{years_lived:u32}
    init_method!{children:u32   }
    init_method!{family:String  }
}

impl<I> AnimalBuilder<I>
where
    I: IntoRuntime<AnimalInitialization>,
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


mutator_fn!{
    type This[I]=(AnimalBuilder<I>)
    where[ I: IntoRuntime<AnimalInitialization>, ]

    type AllowedSelf=(allowed_self_constructors::ByVal)

    /// Equivalent to `|| map_field(builder,field,initialize_field_helper)`
    pub fn InitializeField[builder,field](builder,field)
    where[ MapFieldOp:TypeFn_<(builder,field,InitializeFieldHelper),Output=Out> ]
    {let Out;Out}
}

type_fn!{
    pub fn InitializeFieldHelper[V](UninitField<V>){ IsInitField<V> }
}


//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Animal {
    pub years_lived: u32,
    pub children: u32,
    pub family: String,
}

//////////////////////////////////////////////////////////////////////////////////////////////

