doc_code_snippets! {
    mod "guide_03",
    type_ident=Guide03,


    template=r##"




Here's an example of using a type-level-Range as a ConstValue-parameter to a struct.


Trait/Type glossary:

- ConstValue: A trait for type-level values,also used here to refer to implementors of it.

- ConstRange: the ConstValue equivalent of ::std::ops::Range.

- RangeTrait: 
    a trait that allows using a ConstRange in generic contexts.
    Automatically generated by the TypeLevel derive macro.


- ConstWrapper:
    Zero-sized wrapper type for ConstValues that unconditionally implements 
    Eq/PartialEq/Ord/PartialOrd/Copy/Clone/Debug/etc ,
    delegating IntoRuntime/GetField/SetField/etc to the wrapped ConstValue.


- IntoRuntime : trait for converting a ConstValue to a runtime-value.




//@use_codeblock:struct_decl,ignore

The MutConstValue derive macro generates the type alias `RangedUsize` ,
which passes `R` wrapped inside a ConstWrapper.


//@use_codeblock:rangeusize-impl,ignore

Here the ConstValue range parameter limits which RangedUsize is constructible.


Note the requirement of the IntoRuntime trait in the methods that need to convert the 
ConstRange to a runtime value.
The preferred approach with multiple ConstValue is to put them in a struct 
instead of passing them separately.

//@use_codeblock:assert_zst,ignore

This checks that this is trully zero memory overhead,
because the range only exists on the type level.


//@use_codeblock:main,ignore

Here are a many examples of ranged integers,with different ranges and different integer values.


<br><br><br><br><br><br><br><br><br><br>
<hr>
<br><br><br><br><br><br><br><br><br><br>


# The entire thing

//@use_codeblock:all,rust





"##,

    code=r##"


//@codeblock-start:all

#[macro_use]
extern crate derive_type_level;
#[macro_use]
extern crate type_level_values;


use type_level_values::prelude::*;



//@codeblock-start:struct_decl


use std::ops::Range;


#[derive(MutConstValue)]
#[mcv(
    doc="
        Type that wraps and limits the range of a usize,
        using a ConstRange to determine the range it is limited to.
    ",
    derive(Debug,Copy,Clone,PartialEq,PartialOrd,Eq,Ord),
    Type="RangedUsize",ConstValue="R",
)]
pub struct __RangedUsize<R>{
    range:ConstWrapper<R>,
    n:usize,
}

//@codeblock-end:struct_decl

//@codeblock-start:rangeusize-impl

impl<R> RangedUsize<R>{
    fn new(n:usize)->Option<Self>
    where 
        R:IntoRuntime<Range<usize>>,
    {
        let range_=R::to_runtime();
        if range_.start <= n && n < range_.end {
            Some(Self{ n, range:ConstWrapper::NEW })
        }else{
            None
        }
    }

    fn with_range(n:usize,_range:R)->Option<Self>
    where 
        R:IntoRuntime<Range<usize>>,
    {
        Self::new(n)
    }

    fn value(self)->usize{
        self.n
    }

    fn range(self)->Range<usize>
    where R:IntoRuntime<Range<usize>>
    {
        R::to_runtime()
    }
}

//@codeblock-end:rangeusize-impl


fn main(){

    //@codeblock-start:assert_zst

    assert_eq!(
        ::std::mem::size_of::<usize>(),
        ::std::mem::size_of::<RangedUsize<ConstRange<U0,U1000>>>()
    );

    //@codeblock-end:assert_zst

    //@codeblock-start:main
    
    {
        type UsedRange=ConstRange<U0,U10>;
        let range:UsedRange=ConstRange{
            start:U0::CW,
            end:U10::CW,
        };
        let ranged_int=|n| RangedUsize::with_range( n , range ).unwrap().value() ;
        
        assert_eq!( ranged_int( 0 ) , 0 );
        assert_eq!( ranged_int( 5 ) , 5 );
        assert_eq!( ranged_int( 9 ) , 9 );
        assert_eq!( RangedUsize::new( 10 ) , None::<RangedUsize<UsedRange>> );
    }
    
    {
        type UsedRange=ConstRange<U0,U100>;
        let range:UsedRange=ConstRange{
            start:U0::CW,
            end:U100::CW,
        };
        let ranged_int=|n| RangedUsize::with_range( n , range ).unwrap().value() ;
                
        assert_eq!( ranged_int( 0 ) , 0 );
        assert_eq!( ranged_int( 5 ) , 5 );
        assert_eq!( ranged_int( 9 ) , 9 );
        assert_eq!( ranged_int( 50 ) , 50 );
        assert_eq!( ranged_int( 99 ) , 99 );
        assert_eq!( RangedUsize::new( 100 ) , None::<RangedUsize<UsedRange>> );
    }

    {
        type UsedRange=ConstRange<U10,U100>;
        let range=UsedRange::MTVAL;
        let ranged_int=|n| RangedUsize::with_range( n , range ).unwrap().value() ;
        
        assert_eq!( RangedUsize::new( 0 ) , None::<RangedUsize<UsedRange>> );
        assert_eq!( RangedUsize::new( 5 ) , None::<RangedUsize<UsedRange>> );
        assert_eq!( RangedUsize::new( 9 ) , None::<RangedUsize<UsedRange>> );
        assert_eq!( ranged_int( 10 ) , 10 );
        assert_eq!( ranged_int( 50 ) , 50 );
        assert_eq!( ranged_int( 99 ) , 99 );
        assert_eq!( RangedUsize::new( 100 ) , None::<RangedUsize<UsedRange>> );
    }

    //@codeblock-end:main


}


"##,

}

