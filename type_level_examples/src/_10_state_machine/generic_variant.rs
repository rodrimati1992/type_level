use type_level_values::core_extensions::{CallInto, TryFrom,Void};
use type_level_values::collection_ops::Len_;
use type_level_values::prelude::*;

use super::ranged_usize::{RangedUsize, RangedTrait};
use std::ops::Sub;

/**
Macro associated with GenericVariants which allows:

- Declaring the type of a GenericVariants more ergonomically.

- Matching on the value of a GenericVariants correctly

  (matching on the Impossible parameterized variants would otherwise be allowed) .

# Declaring a GenericVariant type

Here is how to use `g_variants` as a type macro to declare a `GenericVariants`
with as many variants as types listed in the macro:

- `g_variants![]` ,0 variants : 
    declares a GenericVariant<U0>,which is impossible to construct.

- `g_variants![u8]`,1 variant : 
    declares an GenericVariant<U1, u32>.

- `g_variants![u8,u16]`,2 variants : 
    declares an GenericVariant<U2, u8,u16>.

- `g_variants![u8,u16,u32]`,3 variants  : 
    declares an GenericVariant<U3, u8,u16,u32>.

- `g_variants![u8,u16,u32,u64]`,4 variants : 
    declares an GenericVariant<U4, u8,u16,u32,u64>.

- `g_variants![u8,u16,u32,u64,i8]`,5 variants : 
    declares an GenericVariant<LenGt4<U5>, u8,u16,u32,u64,GenericVariant<U1,i8>>.

- `g_variants![u8,u16,u32,u64,i8,i16]`,6 variants : 
    declares an GenericVariant<LenGt4<U6>, u8,u16,u32,u64,GenericVariant<U2,i8,i16>>.


# Matching on a GenericVariant.

Here is how to use `g_variants` to match on a GenericVariant:

g_variants!{match ( <expression> ) {
    $(
        <pattern> => <expression>, 
    )*
}}

# Example of matching
```

# pub fn main_ (){

type NaturalNumber=g_variant!( u8,u16,u32,u64,usize );

let list=vec![
    NaturalNumber::V0( 1 ),
    NaturalNumber::V1( 0x100 ),
    NaturalNumber::V2( 0x10000 ),
    NaturalNumber::V3( 0x100000000 ),
    NaturalNumber::Rem(GenericVariant::V0( !0 )),
];

for elem in list {
    g_variants!{match( elem ){
        n => {
            let _:u8=n;
            println("{:x} is a u8"   ,n)
        },
        n => println("{:x} is a u16"  ,n),
        n => println("{:x} is a u32"  ,n),
        n => println("{:x} is a u64"  ,n),
        n => {
            let _:usize=n;
            println("{:x} is a usize",n)
        },
    }}
}

# }


```


*/
macro_rules! g_variants {
    ()=>{
        $crate::_10_state_machine::generic_variant::GenericVariants<
            U0,
            Impossible,
            Impossible,
            Impossible,
            Impossible,
        >
    };
    ( $ty0:ty, )=>{
        $crate::_10_state_machine::generic_variant::GenericVariants<
            U1,
            $ty0,
            Impossible,
            Impossible,
            Impossible,
        >
    };
    ( $ty0:ty,$ty1:ty, )=>{
        $crate::_10_state_machine::generic_variant::GenericVariants<
            U2,
            $ty0,
            $ty1,
            Impossible,
            Impossible,
        >
    };
    ( $ty0:ty,$ty1:ty,$ty2:ty, )=>{
        $crate::_10_state_machine::generic_variant::GenericVariants<
            U3,
            $ty0,
            $ty1,
            $ty2,
            Impossible,
        >
    };
    ( $ty0:ty,$ty1:ty,$ty2:ty,$ty3:ty, )=>{
        $crate::_10_state_machine::generic_variant::GenericVariants<
            U4,
            $ty0,
            $ty1,
            $ty2,
            $ty3,
        >
    };
    ( $ty0:ty,$ty1:ty,$ty2:ty,$ty3:ty, $($rest:tt)* )=>{
        $crate::_10_state_machine::generic_variant::GenericVariants<
            LenGt4<<U4 as Add<Len<g_variants!($($rest)*)>>>::Output>,
            $ty0,
            $ty1,
            $ty2,
            $ty3,
            g_variants!($($rest)*)
        >
    };


    /////////////////////////////////////////


    ( match ($val:expr) {$patt0:pat=> $expr0:expr,}  ) => {{
        use $crate::_10_state_machine::generic_variant::{Impossible,GenericVariants};
        let __val:GenericVariants<_,_,Impossible,Impossible,Impossible,Impossible>=$val;
        match __val {
             GenericVariants::V0($patt0)=>$expr0,
             GenericVariants::V1(Impossible{..})
            |GenericVariants::V2(Impossible{..})
            |GenericVariants::V3(Impossible{..})
            |GenericVariants::Rem{rem:Impossible{..},..}
            =>unreachable!(),
        }

    }};
    ( match ($val:expr) {
            $patt0:pat=> $expr0:expr,
            $patt1:pat=> $expr1:expr,
        }
    ) => {{
        use $crate::_10_state_machine::generic_variant::{Impossible,GenericVariants};
        let __val:GenericVariants<_,_,_,Impossible,Impossible,Impossible>=$val;
        match __val {
             GenericVariants::V0($patt0)=>$expr0,
             GenericVariants::V1($patt1)=>$expr1,
             GenericVariants::V2(Impossible{..})
            |GenericVariants::V3(Impossible{..})
            |GenericVariants::Rem{rem:Impossible{..},..}
            =>unreachable!(),
        }
    }};
    ( match ($val:expr) {
            $patt0:pat=> $expr0:expr,
            $patt1:pat=> $expr1:expr,
            $patt2:pat=> $expr2:expr,
        }
    ) => {{
        use $crate::_10_state_machine::generic_variant::{Impossible,GenericVariants};
        let __val:GenericVariants<_,_,_,_,Impossible,Impossible>=$val;
        match __val {
             GenericVariants::V0($patt0)=>$expr0,
             GenericVariants::V1($patt1)=>$expr1,
             GenericVariants::V2($patt2)=>$expr2,
             GenericVariants::V3(Impossible{..})
            |GenericVariants::Rem{rem:Impossible{..},..}
            =>unreachable!(),
        }
    }};
    ( match ($val:expr) {
            $patt0:pat=> $expr0:expr,
            $patt1:pat=> $expr1:expr,
            $patt2:pat=> $expr2:expr,
            $patt3:pat=> $expr3:expr,

        }
    ) => {{
        use $crate::_10_state_machine::generic_variant::{Impossible,GenericVariants};
        let __val:GenericVariants<_,_,_,_,_,Impossible>=$val;
        match __val {
            GenericVariants::V0($patt0)=>$expr0,
            GenericVariants::V1($patt1)=>$expr1,
            GenericVariants::V2($patt2)=>$expr2,
            GenericVariants::V3($patt3)=>$expr3,
            GenericVariants::Rem{rem:Impossible{..},..}=>unreachable!(),
        }
    }};
    ( match ($val:expr) {
            $patt0:pat=> $expr0:expr,
            $patt1:pat=> $expr1:expr,
            $patt2:pat=> $expr2:expr,
            $patt3:pat=> $expr3:expr,
            $($rem:tt)+

        }
    ) => {{
        use $crate::_10_state_machine::generic_variant::{Impossible,GenericVariants};
        let __val:GenericVariants<LenGt4<_>,_,_,_,_,_>=$val;
        match __val {
            GenericVariants::V0($patt0)=>$expr0,
            GenericVariants::V1($patt1)=>$expr1,
            GenericVariants::V2($patt2)=>$expr2,
            GenericVariants::V3($patt3)=>$expr3,
            GenericVariants::Rem{rem,..}=>{
                match_g_variant!{ match (rem) { $($rem)* } }
            },
        }
    }};
}

// Wrapper struct used to have a different impl for GenericVariants  with more than 4 variants.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct LenGt4<Len>(pub Len);

/// An uninstantiable wrapper type around Void used for match patterns.
pub struct Impossible(pub Void);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
/// A Generic Enum type.
pub enum GenericVariants<
    Len = U0,
    V0 = Impossible,
    V1 = Impossible,
    V2 = Impossible,
    V3 = Impossible,
    Rem = Impossible,
> {
    V0(V0),
    V1(V1),
    V2(V2),
    V3(V3),
    Rem { len: ConstWrapper<Len>, rem: Rem },
}

////////////////////////////////////////////////////////////////////

impl<V0, V1, V2, V3, Len, Rem> Len_ for GenericVariants<LenGt4<Len>, V0, V1, V2, V3, Rem> {
    type Output = Len;
}

////////////////////////////////////////////////////////////////////

/// Instantiates the defaulted version of the same variant in the Other GenericVariants,
/// even if it has a different type parameter for the variant.
pub trait DefaultSameVariant<Other>: VariantsTrait {
    fn default_same_variant(&self) -> Other;
}

impl<Other> DefaultSameVariant<Other> for Impossible {
    fn default_same_variant(&self) -> Other {
        unreachable!("GenericVariants of a different length")
    }
}

impl<Len, V0, V1, V2, V3, Rem, VB0, VB1, VB2, VB3, OtherRem>
    DefaultSameVariant<GenericVariants<LenGt4<Len>, VB0, VB1, VB2, VB3, OtherRem>>
    for GenericVariants<LenGt4<Len>, V0, V1, V2, V3, Rem>
where
    Self: VariantsTrait,
    Rem: VariantsTrait,
    OtherRem: VariantsTrait,
    VB0: Default,
    VB1: Default,
    VB2: Default,
    VB3: Default,
    Rem: DefaultSameVariant<OtherRem>,
{
    fn default_same_variant(&self) -> GenericVariants<LenGt4<Len>, VB0, VB1, VB2, VB3, OtherRem> {
        use self::GenericVariants as GV;
        match self {
            &GV::V0(_) => GV::V0(Default::default()),
            &GV::V1(_) => GV::V1(Default::default()),
            &GV::V2(_) => GV::V2(Default::default()),
            &GV::V3(_) => GV::V3(Default::default()),
            &GV::Rem { ref rem, .. } => GV::Rem {
                rem: rem.default_same_variant(),
                len: ConstWrapper::NEW,
            },
        }
    }
}

////////////////////////////////////////////////////////////////////

/// To access the values of GenericVariants in a generic context.
pub trait VariantsTrait {
    type V0;
    type V1;
    type V2;
    type V3;

    type Rem: VariantsTrait;
    type Len;
}

impl VariantsTrait for Impossible {
    type V0 = Impossible;
    type V1 = Impossible;
    type V2 = Impossible;
    type V3 = Impossible;

    type Rem = Impossible;
    type Len = U0;
}

impl<Len, V0, V1, V2, V3, Rem> VariantsTrait for GenericVariants<Len, V0, V1, V2, V3, Rem>
where
    Rem: VariantsTrait,
{
    type V0 = V0;
    type V1 = V1;
    type V2 = V2;
    type V3 = V3;
    type Rem = Rem;
    type Len = Len;
}

////////////////////////////////////////////////////////////////////

impl From<RangedUsize<U0, U0>> for Impossible {
    fn from(_range: RangedUsize<U0, U0>) -> Impossible {
        unreachable!()
    }
}

impl<End> TryFrom<RangedUsize<U0, End>> for Impossible {
    type Error = InvalidVariant;
    fn try_from(_range: RangedUsize<U0, End>) -> Result<Impossible, Self::Error> {
        Err(InvalidVariant { len: 0, index: 0 })
    }
}

impl<Len, LenSub4, V0, V1, V2, V3, Rem> From<RangedUsize<U0, Len>>
    for GenericVariants<LenGt4<Len>, V0, V1, V2, V3, Rem>
where
    V0: Default,
    V1: Default,
    V2: Default,
    V3: Default,
    Len: Sub<U4, Output = LenSub4>,
    LenSub4: IntoRuntime<usize>,
    Rem: From<RangedUsize<U0, LenSub4>>,
    RangedUsize<U0, LenSub4>: RangedTrait<Integer=usize>,
{
    fn from(ranged: RangedUsize<U0, Len>) -> Self {
        match ranged.value() {
            0 => GenericVariants::V0(Default::default()),
            1 => GenericVariants::V1(Default::default()),
            2 => GenericVariants::V2(Default::default()),
            3 => GenericVariants::V3(Default::default()),
            value => {
                let rem_range: RangedUsize<U0, LenSub4> =
                    value.saturating_sub(4).piped(RangedUsize::new).unwrap();

                GenericVariants::Rem {
                    len: ConstWrapper::NEW,
                    rem: Rem::from(rem_range),
                }
            }
        }
    }
}

impl<Len, LenSub4, V0, V1, V2, V3, Rem> TryFrom<usize>
    for GenericVariants<LenGt4<Len>, V0, V1, V2, V3, Rem>
where
    Len: IntoRuntime<usize>,
    V0: Default,
    V1: Default,
    V2: Default,
    V3: Default,
    Len: Sub<U4, Output = LenSub4>,
    LenSub4: IntoRuntime<usize>,
    Rem: From<RangedUsize<U0, LenSub4>>,
    RangedUsize<U0, Len>: RangedTrait<Integer=usize>,
    RangedUsize<U0, LenSub4>: RangedTrait<Integer=usize>,
{
    type Error = InvalidVariant;
    fn try_from(index: usize) -> Result<Self, InvalidVariant> {
        match RangedUsize::<U0, Len>::new(index) {
            Err(e) => Err(InvalidVariant {
                len: e.len.unwrap_or(!0),
                index,
            }),
            Ok(v) => Ok(v.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InvalidVariant {
    pub len: usize,
    pub index: usize,
}

////////////////////////////////////////////////////////////////////

/// Maps the values of the variants of a GenericVariants with the Mapper.
///
/// The Mapper is constrained as a CallInto because it is required to be polymorphic,
/// which closures can't be as of Rust 1.28.
pub trait MapVariants<Mapper>: VariantsTrait {
    type Output: VariantsTrait;

    fn map_variants(self, f: Mapper) -> Self::Output;
}

impl<Mapper> MapVariants<Mapper> for Impossible {
    type Output = Self;

    fn map_variants(self, _: Mapper) -> Self::Output {
        self
    }
}

macro_rules! decl_map_variant {
    (
        $len:ident;
        $($index:pat,$in_:ident,$out:ident;)*
    ) => {

        impl<$($in_,)*> Len_ for GenericVariants<$len,$($in_,)*>{
            type Output=$len;
        }


        impl<$($in_,)*> From<RangedUsize<U0,$len>> for GenericVariants<$len,$($in_,)*>
        where
            $($in_:Default,)*
        {
            fn from(ranged: RangedUsize<U0,$len> )->Self{
                match ranged.value() {
                    $(
                        $index=>GenericVariants::$in_(Default::default()),
                    )*
                    _=>unreachable!(),
                }
            }
        }

        impl<$($in_,)*> TryFrom<usize> for GenericVariants<$len,$($in_,)*>
        where
            $($in_:Default,)*
        {
            type Error=InvalidVariant;
            fn try_from(index:usize)->Result<Self,InvalidVariant>{
                match RangedUsize::<U0,$len>::new(index) {
                    Err(e)=>Err(InvalidVariant{len:e.len.unwrap_or(!0),index} ),
                    Ok(v)=>Ok(v.into()),
                }
            }
        }


	    impl<$($in_,)* $($out,)*>
	        DefaultSameVariant<GenericVariants<$len, $($out,)*>>
	    for GenericVariants<$len, $($in_,)* >
	    where
	        Self:VariantsTrait,
	        $($out:Default,)*
	    {
	        fn default_same_variant(&self)->GenericVariants<$len, $($out,)* >{
		        match *self {
                    $(
                        GenericVariants::$in_(_)=>GenericVariants::$in_(Default::default()),
                    )*
		            _=>unreachable!(),
		        }
	        }
	    }


        impl<Mapper,$($in_,)* $($out,)*> MapVariants<Mapper> for GenericVariants<$len,$($in_,)*>
        where
            $( Mapper:CallInto<$in_,Returns=$out>, )*
        {
            type Output=GenericVariants<$len,$($out,)*>;

            fn map_variants(self,_f:Mapper)->Self::Output{
                match self {
                    $(
                        GenericVariants::$in_(v)=>GenericVariants::$in_(_f.call_into(v)),
                    )*
                    _=>unreachable!()
                }
            }
        }
    }
}

#[allow(unreachable_code)]
#[allow(unused_variables)]
mod unreachable_mod {
    use super::*;

    decl_map_variant!{
        U0;
    }
}

decl_map_variant!{
    U1;
    0,V0,Out0;
}

decl_map_variant!{
    U2;
    0,V0,Out0;
    1,V1,Out1;
}

decl_map_variant!{
    U3;
    0,V0,Out0;
    1,V1,Out1;
    2,V2,Out2;
}

decl_map_variant!{
    U4;
    0,V0,Out0;
    1,V1,Out1;
    2,V2,Out2;
    3,V3,Out3;
}

impl<Mapper, Len, Out0, Out1, Out2, Out3, OutRem, V0, V1, V2, V3, Rem> MapVariants<Mapper>
    for GenericVariants<LenGt4<Len>, V0, V1, V2, V3, Rem>
where
    Mapper: CallInto<V0, Returns = Out0>,
    Mapper: CallInto<V1, Returns = Out1>,
    Mapper: CallInto<V2, Returns = Out2>,
    Mapper: CallInto<V3, Returns = Out3>,
    Rem: MapVariants<Mapper, Output = OutRem>,
    OutRem: VariantsTrait,
{
    type Output = GenericVariants<LenGt4<Len>, Out0, Out1, Out2, Out3, OutRem>;

    fn map_variants(self, f: Mapper) -> Self::Output {
        use self::GenericVariants as GV;
        match self {
            GV::V0(v) => GV::V0(f.call_into(v)),
            GV::V1(v) => GV::V1(f.call_into(v)),
            GV::V2(v) => GV::V2(f.call_into(v)),
            GV::V3(v) => GV::V3(f.call_into(v)),
            GV::Rem { rem, .. } => GV::Rem {
                rem: rem.map_variants(f),
                len: ConstWrapper::NEW,
            },
        }
    }
}
