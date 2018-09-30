

#[derive(Clone,Copy,Debug)]
#[derive(TypeLevel)]
#[typelevel(skip_derive)]
#[typelevel(
    reexport="pub",
    derive(ConstEq,ConstOrd),
    items(
        IntoConstType(NoImpls),
        GetDiscriminant(),
    ),
    rename="DirectionConst",
    rename_trait="DirectionInterface",
    rename_consttype="DirectionConstType",
)]
pub enum Direction{
    #[typelevel(rename="LeftVariant")]
    Left,
    Right,
    Other{
        #[typelevel(rename="centerx")]
        value0:u32,
        value1:u32,
    }
}

    # [ allow ( non_snake_case ) ]
# [ allow ( non_camel_case_types ) ]
# [ allow ( dead_code ) ]
pub mod type_level_Direction
{
use super :: * ;
use type_level_values :: reexports :: * ; mod private
{
pub trait Sealed
{
}
pub trait __PrivTrait
{
}
pub struct __HasPrivFields ;
impl __PrivTrait
for __HasPrivFields
{
}
}
use self :: private :: Sealed ;
# [ derive ( Copy , Clone ) ]
pub struct DirectionConstType ;
impl ConstType
for DirectionConstType
{
}
pub trait DirectionInterface : DerivedTraits < Type = DirectionConstType >
{
} mod __fields
{
# [ derive ( Clone , Copy ) ]
pub struct centerx ;
impl super :: Field_
for centerx
{ type Inside = super :: DirectionConstType ;
}
# [ derive ( Clone , Copy ) ]
pub struct value1 ;
impl super :: Field_
for value1
{ type Inside = super :: DirectionConstType ;
}
# [ derive ( Clone , Copy ) ]
pub struct All ;
impl super :: Field_
for All
{ type Inside = super :: DirectionConstType ;
}
}
pub mod fields
{
pub
use super :: __fields ::
{ centerx , value1 ,
} ;
pub
use super :: __fields ::
{ All ,
} ;
}
impl < > DirectionInterface
for LeftVariant < >
where Self : LeftVariantTrait ,
{
}
pub type LeftVariant_Uninitialized = LeftVariant < > ;
pub struct LeftVariant < > ;
impl < > DirectionInterface
for Right < >
where Self : RightTrait ,
{
}
pub type Right_Uninitialized = Right < > ;
pub struct Right < > ;
impl < value0 , value1 , > DirectionInterface
for Other < value0 , value1 , >
where Self : OtherTrait ,
{
}
pub type Other_Uninitialized = Other < Void , Void , > ;
pub struct Other < value0 , value1 , >
{
pub centerx : PhantomWrapper < value0 > ,
pub value1 : PhantomWrapper < value1 > ,
}
impl < Field , FieldVal > std_ :: ops :: Index < Field >
for LeftVariant < >
where Self : GetField_ < Field , Output = FieldVal > ,
{ type Output = PhantomWrapper < FieldVal > ;
# [ inline ( always ) ]
fn index ( & self , _ : Field ) -> & Self :: Output
{ PhantomWrapper :: markertype_ref ( )
}
}
impl < NewValue > SetField_ < self :: fields :: All , NewValue >
for LeftVariant < >
{ type Output = LeftVariant < > ;
}
impl < Field , FieldVal > std_ :: ops :: Index < Field >
for Right < >
where Self : GetField_ < Field , Output = FieldVal > ,
{ type Output = PhantomWrapper < FieldVal > ;
# [ inline ( always ) ]
fn index ( & self , _ : Field ) -> & Self :: Output
{ PhantomWrapper :: markertype_ref ( )
}
}
impl < NewValue > SetField_ < self :: fields :: All , NewValue >
for Right < >
{ type Output = Right < > ;
}
impl < value0 , value1 , NewValue > SetField_ < self :: fields :: centerx , NewValue >
for Other < value0 , value1 , >
{ type Output = Other < NewValue , value1 , > ;
}
impl < value0 , value1 , > GetField_ < self :: fields :: centerx >
for Other < value0 , value1 , >
{ type Output = value0 ;
}
impl < value0 , value1 , > GetFieldRuntime_ < self :: fields :: centerx , Direction < > >
for Other < value0 , value1 , >
where
{ type Runtime = u32 ;
}
impl < value0 , value1 , NewValue > SetField_ < self :: fields :: value1 , NewValue >
for Other < value0 , value1 , >
{ type Output = Other < value0 , NewValue , > ;
}
impl < value0 , value1 , > GetField_ < self :: fields :: value1 >
for Other < value0 , value1 , >
{ type Output = value1 ;
}
impl < value0 , value1 , > GetFieldRuntime_ < self :: fields :: value1 , Direction < > >
for Other < value0 , value1 , >
where
{ type Runtime = u32 ;
}
impl < value0 , value1 , Field , FieldVal > std_ :: ops :: Index < Field >
for Other < value0 , value1 , >
where Self : GetField_ < Field , Output = FieldVal > ,
{ type Output = PhantomWrapper < FieldVal > ;
# [ inline ( always ) ]
fn index ( & self , _ : Field ) -> & Self :: Output
{ PhantomWrapper :: markertype_ref ( )
}
}
impl < value0 , value1 , NewValue > SetField_ < self :: fields :: All , NewValue >
for Other < value0 , value1 , >
{ type Output = Other < NewValue , NewValue , > ;
}
impl < > IntoRuntime < Direction < >>
for LeftVariant < >
where
{
fn to_runtime ( ) -> Direction < >
{ Direction :: Left
{
}
}
}
impl < > IntoConstant < Direction < >>
for LeftVariant < >
where
{ const VALUE : Direction < > = Direction :: Left
{
} ;
}
impl < > ConstTypeOf_
for LeftVariant < >
{ type Type = DirectionConstType ;
}
pub trait LeftVariantTrait : Sealed + DirectionInterface + GetDiscriminant < Discriminant = variants :: LeftVariantVariant > +
{
}
impl < > LeftVariantTrait
for LeftVariant < >
where
{
}
pub type LeftVariantFromTrait < This >= IgnoreFirst < This , LeftVariant < > > ;
impl < > Sealed
for LeftVariant < >
{
}
pub trait LeftVariantWithRuntime < > : LeftVariantTrait
{
}
impl < > LeftVariantWithRuntime < >
for LeftVariant < >
where Self : LeftVariantTrait ,
{
}
impl < > IntoRuntime < Direction < >>
for Right < >
where
{
fn to_runtime ( ) -> Direction < >
{ Direction :: Right
{
}
}
}
impl < > IntoConstant < Direction < >>
for Right < >
where
{ const VALUE : Direction < > = Direction :: Right
{
} ;
}
impl < > ConstTypeOf_
for Right < >
{ type Type = DirectionConstType ;
}
pub trait RightTrait : Sealed + DirectionInterface + GetDiscriminant < Discriminant = variants :: RightVariant > +
{
}
impl < > RightTrait
for Right < >
where
{
}
pub type RightFromTrait < This >= IgnoreFirst < This , Right < > > ;
impl < > Sealed
for Right < >
{
}
pub trait RightWithRuntime < > : RightTrait
{
}
impl < > RightWithRuntime < >
for Right < >
where Self : RightTrait ,
{
}
impl < value0 , value1 , > IntoRuntime < Direction < >>
for Other < value0 , value1 , >
where value0 : IntoRuntime < u32 , value0 > , value1 : IntoRuntime < u32 , value1 > ,
{
fn to_runtime ( ) -> Direction < >
{ Direction :: Other
{ value0 : < value0 as IntoRuntime < u32 , value0 > > :: to_runtime ( ) , value1 : < value1 as IntoRuntime < u32 , value1 > > :: to_runtime ( ) ,
}
}
}
impl < value0 , value1 , > IntoConstant < Direction < >>
for Other < value0 , value1 , >
where value0 : IntoConstant < u32 , value0 > , value1 : IntoConstant < u32 , value1 > ,
{ const VALUE : Direction < > = Direction :: Other
{ value0 : value0 :: VALUE , value1 : value1 :: VALUE ,
} ;
}
impl < value0 , value1 , > ConstTypeOf_
for Other < value0 , value1 , >
{ type Type = DirectionConstType ;
}
pub trait OtherTrait : Sealed + DirectionInterface + GetDiscriminant < Discriminant = variants :: OtherVariant > + GetField_ < fields :: centerx > + GetField_ < fields :: value1 > +
{ type centerx : ; type value1 : ;
}
impl < value0 , value1 , > OtherTrait
for Other < value0 , value1 , >
where
{ type centerx = value0 ; type value1 = value1 ;
}
pub type OtherFromTrait < This >= IgnoreFirst < This , Other < < This as OtherTrait > :: centerx , < This as OtherTrait > :: value1 , > > ;
impl < value0 , value1 , > Sealed
for Other < value0 , value1 , >
{
}
pub trait OtherWithRuntime < > : OtherTrait
{ type rt_centerx : ; type rt_value1 : ;
}
impl < value0 , value1 , > OtherWithRuntime < >
for Other < value0 , value1 , >
where Self : OtherTrait ,
{ type rt_centerx = value0 ; type rt_value1 = value1 ;
}
use self :: variants :: * ;
pub mod variants
{
use super :: * ;
use super :: typenum_reexports :: * ;
pub type LeftVariantVariant = Discriminant < super :: variant_names :: LeftVariant , DirectionConstType , U0 > ;
impl < > GetDiscriminant
for LeftVariant < >
where
{ type Discriminant = LeftVariantVariant ; type Variant = super :: variant_names :: LeftVariant ;
}
pub type RightVariant = Discriminant < super :: variant_names :: Right , DirectionConstType , U1 > ;
impl < > GetDiscriminant
for Right < >
where
{ type Discriminant = RightVariant ; type Variant = super :: variant_names :: Right ;
}
pub type OtherVariant = Discriminant < super :: variant_names :: Other , DirectionConstType , U2 > ;
impl < value0 , value1 > GetDiscriminant
for Other < value0 , value1 , >
where
{ type Discriminant = OtherVariant ; type Variant = super :: variant_names :: Other ;
}
}
# [ doc ( hidden ) ]
pub mod variant_names
{
pub struct LeftVariant ;
pub struct Right ;
pub struct Other ;
}
impl < > Copy
for LeftVariant < >
{
}
impl < > Clone
for LeftVariant < >
{
# [ inline ( always ) ]
fn clone ( & self ) -> Self
{ * self
}
} unsafe
impl < > MarkerType
for LeftVariant < >
{
}
impl < > AsTList_
for LeftVariant < >
where
{ type Output = TNil ;
}
impl < __Other , DiscrL , DiscrR > ConstEq_ < __Other >
for LeftVariant < >
where Self : GetDiscriminant < Discriminant = DiscrL > , __Other : GetDiscriminant < Discriminant = DiscrR > , DiscrL : ConstEq_ < DiscrR > ,
{ type Output = < DiscrL as ConstEq_ < DiscrR >> :: Output ;
}
impl < __Other , DiscrL , DiscrR > ConstOrd_ < __Other >
for LeftVariant < >
where Self : GetDiscriminant < Discriminant = DiscrL > , __Other : GetDiscriminant < Discriminant = DiscrR > , DiscrL : ConstOrd_ < DiscrR > ,
{ type Output = < DiscrL as ConstOrd_ < DiscrR >> :: Output ;
}
impl < > Copy
for Right < >
{
}
impl < > Clone
for Right < >
{
# [ inline ( always ) ]
fn clone ( & self ) -> Self
{ * self
}
} unsafe
impl < > MarkerType
for Right < >
{
}
impl < > AsTList_
for Right < >
where
{ type Output = TNil ;
}
impl < __Other , DiscrL , DiscrR > ConstEq_ < __Other >
for Right < >
where Self : GetDiscriminant < Discriminant = DiscrL > , __Other : GetDiscriminant < Discriminant = DiscrR > , DiscrL : ConstEq_ < DiscrR > ,
{ type Output = < DiscrL as ConstEq_ < DiscrR >> :: Output ;
}
impl < __Other , DiscrL , DiscrR > ConstOrd_ < __Other >
for Right < >
where Self : GetDiscriminant < Discriminant = DiscrL > , __Other : GetDiscriminant < Discriminant = DiscrR > , DiscrL : ConstOrd_ < DiscrR > ,
{ type Output = < DiscrL as ConstOrd_ < DiscrR >> :: Output ;
}
impl < value0 , value1 , > Copy
for Other < value0 , value1 , >
{
}
impl < value0 , value1 , > Clone
for Other < value0 , value1 , >
{
# [ inline ( always ) ]
fn clone ( & self ) -> Self
{ * self
}
} unsafe
impl < value0 , value1 , > MarkerType
for Other < value0 , value1 , >
{
}
impl < value0 , value1 , > AsTList_
for Other < value0 , value1 , >
where
{ type Output = TList < value0 , TList < value1 , TNil > > ;
}
impl < value0 , value1 , __Other > ConstEq_ < __Other >
for Other < value0 , value1 , >
where Self : VariantAsTList_ , __Other : VariantAsTList_ , VariantAsTList < Self > : ConstEq_ < VariantAsTList < __Other >> ,
{ type Output = __CEq < VariantAsTList < Self > , VariantAsTList < __Other >> ;
}
impl < value0 , value1 , __Other > ConstOrd_ < __Other >
for Other < value0 , value1 , >
where Self : VariantAsTList_ , __Other : VariantAsTList_ , VariantAsTList < Self > : ConstOrd_ < VariantAsTList < __Other >> ,
{ type Output = __COrd < VariantAsTList < Self > , VariantAsTList < __Other >> ;
}
}
pub
use self :: type_level_Direction ::
{ DirectionInterface , DirectionConstType , LeftVariantFromTrait , LeftVariantTrait , LeftVariantWithRuntime , RightFromTrait , RightTrait , RightWithRuntime , OtherFromTrait , OtherTrait , OtherWithRuntime , LeftVariant , LeftVariant_Uninitialized , Right , Right_Uninitialized , Other , Other_Uninitialized , variants , fields ,
} ;

        pub struct DirectionConv;

        mod type_level_DirectionConv{
            use type_level_values::prelude::*;
            use super::*;

            impl IntoConstType_<Direction> for DirectionConv{
                type ToConst=DirectionConstType;
            }
        }


        #[derive(Clone,Copy,Debug)]
        #[derive(TypeLevel)]
        pub struct People{
            #[typelevel(delegate(IntoConstType="DirectionConv"))]
            george:Direction,
            robert:Direction,

            are_all_alive:bool,
        }


    