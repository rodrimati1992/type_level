
use type_level_values::prelude::*;
use type_level_values::field_traits::{SetField,SetField_};

pub fn main_(){    
    let pieces=TetrisBuilder::new()
        .l_pieces(10)
        .i_pieces(20)
        .z_pieces(30)
        .s_pieces(40)
        .o_pieces(50)
        .build();

    assert_eq!(
        pieces,
        TetrisPieces{
            l_pieces:10,
            i_pieces:20,
            z_pieces:30,
            s_pieces:40,
            o_pieces:50,
        }
    )
}

/////////////////////////////////////////////////////////////////

#[derive(Clone, Debug,PartialEq)]
pub struct TetrisPieces{
    l_pieces:usize,
    i_pieces:usize,
    z_pieces:usize,
    s_pieces:usize,
    o_pieces:usize,
}


/////////////////////////////////////////////////////////////////

#[derive(TypeLevel)]
#[typelevel(reexport(Variants))]
pub enum FieldInitialization{
    InitField,
    UninitField,
}

///////////////////////////////////////////////////////////////////


#[derive(TypeLevel)]
#[typelevel(
    derive(ConstEq,ConstOrd),
    reexport(Struct,Traits),
)]
pub struct InitializedFields{
    pub l_pieces:FieldInitialization,
    pub i_pieces:FieldInitialization,
    pub z_pieces:FieldInitialization,
    pub s_pieces:FieldInitialization,
    pub o_pieces:FieldInitialization,
}

pub use self::type_level_InitializedFields::fields;

pub type AllUninitialized=SetField<
    InitializedFields_Uninit,
    fields::All,
    UninitField
>;

pub type AllInitialized=SetField<
    InitializedFields_Uninit,
    fields::All,
    InitField
>;

//////////////////////////////////////////////////////////////////


#[derive(Clone, Debug, ConstConstructor)]
#[cconstructor(Type = "TetrisBuilder",ConstParam = "C")]
pub struct TetrisBuilderInner<C>{
    l_pieces:Option<usize>,
    i_pieces:Option<usize>,
    z_pieces:Option<usize>,
    s_pieces:Option<usize>,
    o_pieces:Option<usize>,
    initialization:PhantomWrapper<C>,
}

impl TetrisBuilder< AllUninitialized >{
    fn new()->Self {
        TetrisBuilder::default()
    }
}

impl Default for TetrisBuilder< AllUninitialized >{
    fn default()->Self{
        TetrisBuilder{
            l_pieces:None,
            i_pieces:None,
            z_pieces:None,
            s_pieces:None,
            o_pieces:None,
            initialization:PhantomWrapper::NEW,
        }
    }
}

mod builder_internal{
    
    use super::*;
    
    const_method!{
        type ConstConstructor[]=( TetrisBuilderCC )
        type AllowedConversions=( allowed_conversions::ByVal )

        fn InitializeField[I,Field](I,Field)
        where [ I:SetField_<Field,InitField,Output=Out>, ]
        { let Out;Out }
    }

    macro_rules! declare_setter {
        ( $field:ident ) => (
            pub fn $field<__OutSelf>(mut self,value:usize)->__OutSelf
            where 
                Self:MCPBounds<InitializeField,fields::$field,NextSelf=__OutSelf>
            {
                self.$field=Some(value);
                self.mutparam(InitializeField::new(),Default::default())
            }

        )
    }

    impl<C> TetrisBuilder< C >{

        declare_setter!{ l_pieces }
        declare_setter!{ i_pieces }
        declare_setter!{ z_pieces }
        declare_setter!{ s_pieces }
        declare_setter!{ o_pieces }

    }
}


impl<C> TetrisBuilder< C >{

    fn build(self)->TetrisPieces
    where C:TypeIdentity<Type=AllInitialized>
    {
        TetrisPieces{
            l_pieces:self.l_pieces.unwrap(),
            i_pieces:self.i_pieces.unwrap(),
            z_pieces:self.z_pieces.unwrap(),
            s_pieces:self.s_pieces.unwrap(),
            o_pieces:self.o_pieces.unwrap(),
        }
    }

}

