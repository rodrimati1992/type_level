use super::*;

use crate_::extern_types::typenum::{SignedInteger};

#[allow(unused_imports)]
use crate_::ops::{
    Sub1,Add1,
    ConstInto,ConstIntoMt,
};
use crate_::std_ops::{
    SubTA,AddTA,ShlTA,ShrTA,NegTA,
    ShlOp,NegOp,
};

use std::mem;

#[test]
fn size_of_(){

    macro_rules! for_size {
        ( $shift:ty , $equiv_type:ty ) => {{
            type Shifted=ShlTA<U1,$shift>;
            type Unshifted=ShrTA<Shifted,U8>;
            assert_eq!(
                size_of::<$equiv_type>(), 
                size_of::<RangedIntL<$equiv_type,U0, Add1<Unshifted>>>()
            );
            assert_eq!(
                size_of::<$equiv_type>(), 
                size_of::<RangedIntL<$equiv_type,U0, Sub1<Shifted>>>()
            );
            assert_eq!(
                size_of::<$equiv_type>(), 
                size_of::<RangedIntL<$equiv_type,U0, Shifted>>()
            );
            assert_eq!(
                size_of::<$equiv_type>(), 
                size_of::<RangedIntL<$equiv_type,U1000, Shifted>>()
            );
            assert_eq!(
                size_of::<$equiv_type>(), 
                size_of::<RangedIntL<$equiv_type,U1000000, Shifted>>()
            );
        }}
    }

    for_size!{ U8,u8 }
    for_size!{ U16,u16 }
    for_size!{ U32,u32 }
    for_size!{ U64,u64 }
    #[cfg(feature="i128")] { for_size!{ U128,u128 } }
}

#[test]
fn values(){

    macro_rules! test_values {
        (
            repr=$integer:ty,
            start=$start:ty,
            end=$end:ty,
            inside_range=[ $($inside:expr),* $(,)* ],
            outside_range=[ $($outside:expr),* $(,)* ],
        ) => ({
            type UsedRange = ConstRange<$start, $end>;
            type UsedRIR=RangedIntR<$integer,UsedRange>;

            let range:UsedRange = ConstRange {
                start: <$start>::CW,
                end: <$end>::CW,
            };
            let ranged_int = |n:$integer| {
                RangedIntR::with_range(n, range).unwrap().value()
            };
            let new=RangedIntR::<$integer,UsedRange>::new;

            assert_eq!(
                mem::size_of::<$integer>(),
                mem::size_of::<RangedIntR<$integer,UsedRange>>(),
            );

            for number in vec![$($inside,)*] {
                assert_eq!(ranged_int(number),number);
            }
            for number in vec![$($outside,)*]{
                assert_eq!(
                    new(number),
                    Err(IntOutsideRange {
                        value:number ,
                        start: <UsedRIR as RangedTrait>::start(),
                        end: <UsedRIR as RangedTrait>::end(),
                        len: <UsedRIR as RangedTrait>::len(),
                    })
                );
            }
        })
    }

    test_values!{
        repr=u8,
        start=U0,
        end=U10,
        inside_range=[0,1,2,5,7,8,9],
        outside_range=[10,15,255],
    }
    test_values!{
        repr=i8,
        start=N10,
        end=P10,
        inside_range=[-10,-9,-1,0,1,2,5,7,8,9],
        outside_range=[-12,-11,10,15,127],
    }

    test_values!{
        repr=u8,
        start=U0,
        end=U100,
        inside_range=[0,1,2,5,9,50,97,98,99],
        outside_range=[100,150,255],
    }

    test_values!{
        repr=u8,
        start=U10,
        end=U100,
        inside_range=[10,11,12,50,97,98,99],
        outside_range=[0,5,9,100,150,255],
    }

    test_values!{
        repr=i8,
        start=P10,
        end=P100,
        inside_range=[10,11,12,50,97,98,99],
        outside_range=[0,5,9,126,127,-5,-9,-126,-127,-128],
    }

    test_values!{
        repr=i8,
        start=N128,
        end=P128,
        inside_range=[-128,-127,-126,0,126,127],
        outside_range=[],
    }

    test_values!{
        repr=u16,
        start=U100,
        end=U356,
        inside_range=[100,101,110,111,112,150,197,198,199],
        outside_range=[0,5,9,98,99,356,357,358],
    }


    test_values!{
        repr=i16,
        start=N128,
        end=P129,
        inside_range=[-128,-127,-126,0,126,127,128],
        outside_range=[0x8000u16 as _,-130,-129,129,130,0x7fff],
    }

    test_values!{
        repr=u16,
        start=U256,
        end=U256,
        inside_range=[],
        outside_range=[0,5,9,98,99,256,356,357,358],
    }

    test_values!{
        repr=i16,
        start=P128,
        end=P128,
        inside_range=[],
        outside_range=[
            0,5,9,98,99,127,128,129,356,357,358,-5,-9,-98,-99,-127,-128,-129,-356,-357,-358
        ],
    }

    {
        type Start=ShlTA<U1,U16>;
        type End  =AddTA<Start,ShlTA<U1,U16>>;

        test_values!{
            repr=u32,
            start=Start,
            end=End,
            inside_range=[0x1_0000,0x1_0001,0x1_0002,0x1_fffd,0x1_fffe,0x1_ffff],
            outside_range=[0,5,9,98,99,356,357,358,0xfffe,0xffff,0x2_0000,0x2_0001,0x2_0002],
        }
    }
    {
        type Start=ShlTA<U1,U32>;
        type End  =AddTA<Start,ShlTA<U1,U16>>;

        let off=0x1_0000_0000;

        test_values!{
            repr=u64,
            start=Start,
            end=End,
            inside_range=[off+0,off+1,off+2,off+0xfffd,off+0xfffe,off+0xffff],
            outside_range=[
                0,1,2,
                off-3,
                off-2,
                off-1,
                off+0x1_0000,
                off+0x1_0001,
                off+0x1_0002
            ],
        }
    }
    {
        type Start=ShlTA<U1,U32>;
        type End  =AddTA<Start,Sub1<ShlTA<U1,U32>>>;

        let off=0x1_0000_0000;

        test_values!{
            repr=u64,
            start=Start,
            end=End,
            inside_range=[off+0,off+1,off+2,off+0xffff_fffd,off+0xffff_fffe],
            outside_range=[
                0,1,2,
                off-3,
                off-2,
                off-1,
                off+0xffff_ffff,
            ],
        }
    }
    {
        type Start=U0;
        type End  =ShlTA<U1,U8>;

        let last=0xff;

        test_values!{
            repr=u8,
            start=Start,
            end=End,
            inside_range=[
                0,1,2,
                last-2,
                last-1,
                last,
            ],
            outside_range=[],
        }
    }
    {
        type Start=U0;
        type End  =ShlTA<U1,U16>;

        let last=0xffff;

        test_values!{
            repr=u16,
            start=Start,
            end=End,
            inside_range=[
                0,1,2,
                last-2,
                last-1,
                last,
            ],
            outside_range=[],
        }
    }
    {
        type Start=U0;
        type End  =ShlTA<U1,U32>;

        let last=0xffff_ffff;

        test_values!{
            repr=u32,
            start=Start,
            end=End,
            inside_range=[
                0,1,2,
                last-2,
                last-1,
                last,
            ],
            outside_range=[],
        }
    }
    {
        type Start=U10;
        type End  =SubTA<ShlTA<U1,U64>,U10>;

        let lastu64=0xffff_ffff_ffff_ffff;

        test_values!{
            repr=u64,
            start=Start,
            end=End,
            inside_range=[
                10,11,12,
                lastu64-12,
                lastu64-11,
                lastu64-10,
            ],
            outside_range=[
                0,1,2,3,4,5,6,7,8,9,
                lastu64-9,
                lastu64-8,
                lastu64-3,
                lastu64-2,
                lastu64-1,
                lastu64,
            ],
        }
    }
}


macro_rules! test_range_my_types {
    (
        ($start:ty,$end:ty),
        is_empty=$is_empty:expr,
        repr=$integer:ty,
        start=$range_start:expr,
        end=$range_end:expr,
    ) => ({
        type UsedRange=ConstRange<$start,$end>;
        assert_eq!(
            <RangedIntR<$integer,UsedRange> as RangedTrait>::is_empty(),
            $is_empty
        );
        assert_eq!(
            <RangedIntR<$integer,UsedRange> as RangedTrait>::start(),
            $range_start
        );
        assert_eq!(
            <RangedIntR<$integer,UsedRange> as RangedTrait>::end(),
            $range_end
        );
    })
}

#[test]
fn test_range_my_types(){
    test_range_my_types!{
        (U0,U10),
        is_empty=false,
        repr=u8,
        start=0,
        end=Some(10),
    }

    test_range_my_types!{
        (U0,U100),
        is_empty=false,
        repr=u8,
        start=0,
        end=Some(100),
    }

    test_range_my_types!{
        (U10,U100),
        is_empty=false,
        repr=u8,
        start=10,
        end=Some(100),
    }

    test_range_my_types!{
        (U100,U356),
        is_empty=false,
        repr=u16,
        start=100,
        end=Some(356),
    }

    {
        type Start=ShlTA<U1,U16>;
        type End  =AddTA<Start,ShlTA<U1,U16>>;

        test_range_my_types!{
            (Start,End),
            is_empty=false,
            repr=u32,
            start=0x1_0000,
            end=Some(0x2_0000),
        }
    }
    {
        type Start=ShlTA<U1,U32>;
        type End  =AddTA<Start,ShlTA<U1,U16>>;
        test_range_my_types!{
            (Start,End),
            is_empty=false,
            repr=u64,
            start=0x1_0000_0000,
            end=Some(0x1_0001_0000),
        }
    }
    {
        type Start=ShlTA<U1,U32>;
        type End  =AddTA<Start,Sub1<ShlTA<U1,U32>>>;
        test_range_my_types!{
            (Start,End),
            is_empty=false,
            repr=u64,
            start=0x1_0000_0000,
            end=Some(0x2_0000_0000-1),
        }
    }
    {
        type Start=U0;
        type End  =ShlTA<U1,U8>;
        test_range_my_types!{
            (Start,End),
            is_empty=false,
            repr=u8,
            start=0,
            end=None,
        }
    }
    {
        type Start=U0;
        type End  =ShlTA<U1,U16>;
        test_range_my_types!{
            (Start,End),
            is_empty=false,
            repr=u16,
            start=0,
            end=None,
        }
    }
    {
        type Start=U0;
        type End  =ShlTA<U1,U32>;
        test_range_my_types!{
            (Start,End),
            is_empty=false,
            repr=u32,
            start=0,
            end=None,
        }
    }
    {
        type Start=U10;
        type End  =SubTA<ShlTA<U1,U64>,U10>;
        test_range_my_types!{
            (Start,End),
            is_empty=false,
            repr=u64,
            start=10,
            end=Some(0xffff_ffff_ffff_ffff-9),
        }
    }
    {
        type Start=U0;
        type End  =ShlTA<U1,U64>;
        test_range_my_types!{
            (Start,End),
            is_empty=false,
            repr=u64,
            start=0,
            end=None,
        }
    }


    type MinSignedRange<Shift>=TypeFn<
        (ShlOp,ConstIntoMt<SignedInteger>,NegOp),
        (U1,Shift)
    >;

    {
        type Start=MinSignedRange<U7>;
        type End  =MinSignedRange<U7>;
        
        let start=-128;
        
        test_range_my_types!{
            (Start,End),
            is_empty=true,
            repr=i8,
            start=start,
            end=Some(start),
        }
    }
    {
        type Start=MinSignedRange<U15>;
        type End  =MinSignedRange<U15>;

        let start=0x8000u16 as i16;

        test_range_my_types!{
            (Start,End),
            is_empty=true,
            repr=i16,
            start=start,
            end=Some(start),
        }
    }
    {
        type Start=MinSignedRange<U31>;
        type End  =MinSignedRange<U31>;

        let start=0x8000_0000u32 as i32;

        test_range_my_types!{
            (Start,End),
            is_empty=true,
            repr=i32,
            start=start,
            end=Some(start),
        }
    }
    {
        type Start=MinSignedRange<U63>;
        type End  =MinSignedRange<U63>;

        let start=0x8000_0000_0000_0000u64 as i64;

        test_range_my_types!{
            (Start,End),
            is_empty=true,
            repr=i64,
            start=start,
            end=Some(start),
        }
    }



    {
        type Start=NegTA<End>;
        type End  =ConstInto<ShlTA<U1,U7>,SignedInteger>;
        test_range_my_types!{
            (Start,End),
            is_empty=false,
            repr=i8,
            start=-128,
            end=None,
        }
    }
    {
        type Start=NegTA<End>;
        type End  =ConstInto<ShlTA<U1,U15>,SignedInteger>;
        test_range_my_types!{
            (Start,End),
            is_empty=false,
            repr=i16,
            start=0x8000u16 as i16,
            end=None,
        }
    }
    {
        type Start=NegTA<End>;
        type End  =ConstInto<ShlTA<U1,U31>,SignedInteger>;
        test_range_my_types!{
            (Start,End),
            is_empty=false,
            repr=i32,
            start=0x8000_0000u32 as i32,
            end=None,
        }
    }
    {
        type Start=NegTA<End>;
        type End  =ConstInto<ShlTA<U1,U63>,SignedInteger>;
        test_range_my_types!{
            (Start,End),
            is_empty=false,
            repr=i64,
            start=0x8000_0000_0000_0000u64 as i64,
            end=None,
        }
    }
    

    

    {
        type Start=U0;
        type End  =U0;
        test_range_my_types!{
            (Start,End),
            is_empty=true,
            repr=u8,
            start=0,
            end=Some(0),
        }
    }
    {
        type Start=U10;
        type End  =U10;
        test_range_my_types!{
            (Start,End),
            is_empty=true,
            repr=u8,
            start=10,
            end=Some(10),
        }
    }
    {
        type Start=U256;
        type End  =U256;
        test_range_my_types!{
            (Start,End),
            is_empty=true,
            repr=u16,
            start=256,
            end=Some(256),
        }
    }

}