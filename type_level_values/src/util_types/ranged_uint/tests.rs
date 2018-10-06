use super::*;
#[allow(unused_imports)]
use typenum::operator_aliases::{Sub1,Add1,Diff as Sub_,Sum,Shleft,Shright};

#[test]
fn size_of_(){

    macro_rules! for_size {
        ( $shift:ty , $equiv_type:ty ) => {{
            type Shifted=Shleft<U1,$shift>;
            type Unshifted=Shright<Shifted,U8>;
            assert_eq!(size_of::<$equiv_type>(), size_of::<RangedUIntL<U0, Add1<Unshifted>>>());
            assert_eq!(size_of::<$equiv_type>(), size_of::<RangedUIntL<U0, Sub1<Shifted>>>());
            assert_eq!(size_of::<$equiv_type>(), size_of::<RangedUIntL<U0, Shifted>>());
            assert_eq!(size_of::<$equiv_type>(), size_of::<RangedUIntL<U1000, Shifted>>());
            assert_eq!(size_of::<$equiv_type>(), size_of::<RangedUIntL<U1000000, Shifted>>());
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
            start=$start:ty,
            end=$end:ty,
            inside_range=[ $($inside:expr),* $(,)* ],
            outside_range=[ $($outside:expr),* $(,)* ],
        ) => ({
            type UsedRange = ConstRange<$start, $end>;
            let range:UsedRange = ConstRange {
                start: <$start>::CW,
                end: <$end>::CW,
            };
            let ranged_int = |n| RangedUIntR::with_range(n, range).unwrap().value();
            let new=RangedUIntR::<UsedRange>::new;

            for number in vec![$($inside,)*] {
                assert_eq!(ranged_int(number),number);
            }
            for number in vec![$($outside,)*]{
                assert_eq!(
                    new(number),
                    Err(UIntOutsideRange {
                        value:number ,
                        start: UsedRange::start(),
                        end: UsedRange::end(),
                        end_inclusive: UsedRange::end_inclusive(),
                    })
                );
            }
        })
    }

    test_values!{
        start=U0,
        end=U10,
        inside_range=[0,1,2,5,7,8,9],
        outside_range=[10,15,255],
    }

    test_values!{
        start=U0,
        end=U100,
        inside_range=[0,1,2,5,9,50,97,98,99],
        outside_range=[100,150,2550],
    }

    test_values!{
        start=U10,
        end=U100,
        inside_range=[10,11,12,50,97,98,99],
        outside_range=[0,5,9,100,150,255],
    }

    test_values!{
        start=U100,
        end=U356,
        inside_range=[100,101,110,111,112,150,197,198,199],
        outside_range=[0,5,9,98,99,356,357,358],
    }

    {
        type Start=Shleft<U1,U16>;
        type End  =Sum<Start,Shleft<U1,U16>>;

        test_values!{
            start=Start,
            end=End,
            inside_range=[0x1_0000,0x1_0001,0x1_0002,0x1_fffd,0x1_fffe,0x1_ffff],
            outside_range=[0,5,9,98,99,356,357,358,0xfffe,0xffff,0x2_0000,0x2_0001,0x2_0002],
        }
    }
}
