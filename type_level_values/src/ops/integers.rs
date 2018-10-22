use prelude::*;

use crate_::ops::*;
use crate_::fn_types::{DivOp,SubOp};
use crate_::fn_adaptors::{
    IdentityFn,
    GetRhs,
    GetLhs,
    Const,
};

use std_::ops::{Add,Sub};


/// Integer constants for a ConstType,it is also be implemented for all values of a ConstType.
pub trait IntegerConsts{
    type Zero;
    type One;
}


impl<T,Type> IntegerConsts for T
where 
    ConstTypeOfOp:TypeFn_<T,Output=Type>,
    Type:IntegerConsts,
{
    type Zero=Type::Zero;
    type One =Type::One;
}


type_fn!{define_trait
    /// Returns the base 2 logarithm of this number
    trait=Log2_ []
    type=Log2
    fn_type=Log2Op
}



/// Returns whether N is 0.
pub type IsOne<N>=
    TypeFn<IsOneOp,N>;

/// Returns whether N is 1.
pub type IsZero<N>=
    TypeFn<IsZeroOp,N>;

type_fn!{
    /// Returns whether Val is 0.
    pub fn IsZeroOp[N](N)
    where[
        N:IntegerConsts<Zero=Zero>,
        N:ConstEq_<Zero,Output=Out>,
    ]{
        let Zero;let Out;
        Out
    }
}

type_fn!{
    /// Returns whether Val is 1.
    pub fn IsOneOp[N](N)
    where[
        N:IntegerConsts<One=One>,
        N:ConstEq_<One,Output=Out>,
    ]{
        let One;let Out;
        Out
    }
}




/// Gets the value for 0 as defined by the type of N.
pub type Get0<N>=TypeFn<Get0Op,N>;

type_fn!{
    /// Gets the value for 0 as defined by the type of N.
    pub fn Get0Op[N](N)
    where[ N:IntegerConsts<Zero=Out> ]
    { let Out;Out }
}



/// Gets the value for 1 as defined by the type of N.
pub type Get1<N>=TypeFn<Get1Op,N>;

type_fn!{
    /// Gets the value for 1 as defined by the type of N.
    pub fn Get1Op[N](N)
    where[ N:IntegerConsts<One=Out> ]
    { let Out;Out }
}



/// Safe division function which returns None_ when the divisor is 0.
/// 
/// if R==0 ,returns None_, otherwise returns Some_<L / R>.
pub type SafeDiv<L,R>=
    TypeFn<SafeDivOp,(L,R)>;

/// Safe division function which returns None_ when the divisor is 0.
/// 
/// if R==0 ,returns None_, otherwise returns Some_<L / R>.
pub type SafeDivOp=
    If<(GetRhs,IsZeroOp),
        Const<None_>,
        (DivOp,NewSome),
    >;



/// Safe unsigned subtraction function which returns None_ when Lhs < Rhs.
/// 
/// if L>=R ,returns Some_<L - R>, otherwise returns None_.
pub type SafeSub<L,R>=
    TypeFn<SafeSubOp,(L,R)>;


/// Safe unsigned subtraction function which returns None_ when Lhs < Rhs.
/// 
/// if L>=R ,returns Some_<L - R>, otherwise returns None_.
pub type SafeSubOp=
    If<ConstGEOp,
        (SubOp,NewSome),
        Const<None_>
    >;


/// Adds 1 to N.
pub type Add1<N>=
    TypeFn<Add1Op,N>;

type_fn!{
    /// Adds 1 to N.
    pub fn Add1Op[N](N)
    where[
        N:IntegerConsts<One=One>,
        N:Add<One,Output=Out>,
    ]{
        let One;let Out;
        Out
    }
}    



/// Substracts 1 from N.
pub type Sub1<N>=
    TypeFn<Sub1Op,N>;

type_fn!{
    /// Substracts 1 from N.
    pub fn Sub1Op[N](N)
    where[
        N:IntegerConsts<One=One>,
        N:Sub<One,Output=Out>,
    ]{
        let One;let Out;
        Out
    }
}    



pub type SatSub1<N>=
    TypeFn<SatSub1Op,N>;

/// Subtracts 1 from an unsigned integer.Stopping at 0.
pub type SatSub1Op=
    If<IsZeroOp,IdentityFn,Sub1Op>;



pub type SatSub<L,R>=
    TypeFn<SatSubOp,(L,R)>;

/// Subtracts Rhs from Lhs returning 0 if Lhs <= Rhs.
///
/// Equivalent to `|lhs,rhs| lhs.saturating_sub(rhs) `
pub type SatSubOp=
    If<ConstLtOp,(GetLhs,Get0Op),SubOp>;


//#[cfg(all(test,feature="passed_tests"))]
#[cfg(test)]
mod tests{
    use super::*;

    macro_rules! TypeFnEq {
        (  $func:ty,$alias:ident ,$value:ty,$equals:ty ) => (
            (
                AssertEq<TypeFn<$func,$value>,$equals>,
                AssertEq<TypeFn<$func,$value>,$alias<$value>>,
            )
        )
    }
 
    #[test]
    fn get_constants(){
        type TestZero<N,Val>=(
            AssertEq<Get0<N>,Val>,
            AssertFnRet<N,Get0Op,Val>,
        );
        type TestOne<N,Val>=(
            AssertEq<Get1<N>,Val>,
            AssertFnRet<N,Get1Op,Val>,
        );

        let _:TestZero<U0,U0>;
        let _:TestZero<U1,U0>;
        let _:TestZero<U2,U0>;

        let _:TestZero<N2,Z0>;
        let _:TestZero<N1,Z0>;
        let _:TestZero<Z0,Z0>;
        let _:TestZero<P1,Z0>;
        let _:TestZero<P2,Z0>;



        let _:TestOne<U0,U1>;
        let _:TestOne<U1,U1>;
        let _:TestOne<U2,U1>;

        let _:TestOne<N2,P1>;
        let _:TestOne<N1,P1>;
        let _:TestOne<Z0,P1>;
        let _:TestOne<P1,P1>;
        let _:TestOne<P2,P1>;



    }


    #[test]
    fn is_one_zero(){
        let _:TypeFnEq!(IsZeroOp,IsZero,U0,True);
        let _:TypeFnEq!(IsZeroOp,IsZero,U1,False);
        let _:TypeFnEq!(IsZeroOp,IsZero,U2,False);

        let _:TypeFnEq!(IsZeroOp,IsZero,N2,False);
        let _:TypeFnEq!(IsZeroOp,IsZero,N1,False);
        let _:TypeFnEq!(IsZeroOp,IsZero,Z0,True);
        let _:TypeFnEq!(IsZeroOp,IsZero,P1,False);
        let _:TypeFnEq!(IsZeroOp,IsZero,P2,False);

        let _:TypeFnEq!(IsOneOp,IsOne,U0,False);
        let _:TypeFnEq!(IsOneOp,IsOne,U1,True);
        let _:TypeFnEq!(IsOneOp,IsOne,U2,False);

        let _:TypeFnEq!(IsOneOp,IsOne,N2,False);
        let _:TypeFnEq!(IsOneOp,IsOne,N1,False);
        let _:TypeFnEq!(IsOneOp,IsOne,Z0,False);
        let _:TypeFnEq!(IsOneOp,IsOne,P1,True);
        let _:TypeFnEq!(IsOneOp,IsOne,P2,False);
    }

    #[test]
    fn safe_div(){
        type Test<L,R,Val>=(
            AssertEq<TypeFn<SafeDivOp,(L,R)>,Val>,
            AssertEq<SafeDiv<L,R>,Val>,
        );

        let _:Test<U0,U0,None_>;
        let _:Test<U1,U0,None_>;
        let _:Test<U2,U0,None_>;
        
        let _:Test<U0,U1,Some_<U0>>;
        let _:Test<U1,U1,Some_<U1>>;
        let _:Test<U2,U1,Some_<U2>>;
        
        

        let _:Test<P2,N2,Some_<N1>>;
        let _:Test<P1,N2,Some_<Z0>>;
        let _:Test<Z0,N2,Some_<Z0>>;
        
        let _:Test<P2,N1,Some_<N2>>;
        let _:Test<P1,N1,Some_<N1>>;
        let _:Test<Z0,N1,Some_<Z0>>;
        
        let _:Test<Z0,Z0,None_>;
        let _:Test<P1,Z0,None_>;
        let _:Test<P2,Z0,None_>;
        
        let _:Test<Z0,P1,Some_<Z0>>;
        let _:Test<P1,P1,Some_<P1>>;
        let _:Test<P2,P1,Some_<P2>>;
        
        let _:Test<Z0,P2,Some_<Z0>>;
        let _:Test<P1,P2,Some_<Z0>>;
        let _:Test<P2,P2,Some_<P1>>;
    }

    #[test]
    fn safe_sub(){
        type AssertSub<L,R,Val>=(
            AssertFnRet<(L,R),SafeSubOp,Val>,
            AssertEq<SafeSub<L,R>,Val>,
        );

        type Test<L,R,Val>=(
            AssertSub<L,R,Some_<Val>>,
            AssertSub<R,L,None_>,
        );

        let _:AssertSub<U0,U0,Some_<U0>>;
        let _:AssertSub<U1,U1,Some_<U0>>;
        let _:AssertSub<U2,U2,Some_<U0>>;
        let _:AssertSub<U3,U3,Some_<U0>>;

        let _:Test<U1,U0,U1>;
        let _:Test<U2,U0,U2>;
        let _:Test<U2,U1,U1>;
        let _:Test<U3,U0,U3>;
        let _:Test<U3,U1,U2>;
        let _:Test<U3,U2,U1>;
    }

    #[test]
    fn add_sub_1(){
        type TestAdd1<N,Val>=(
            AssertEq<Add1<N>,Val>,
            AssertFnRet<N,Add1Op,Val>,
        );
        type TestSub1<N,Val>=(
            AssertEq<Sub1<N>,Val>,
            AssertFnRet<N,Sub1Op,Val>,
        );

        let _:TestAdd1<U0,U1>;
        let _:TestAdd1<U1,U2>;
        let _:TestAdd1<U2,U3>;

        let _:TestSub1<U1,U0>;
        let _:TestSub1<U2,U1>;
        let _:TestSub1<U3,U2>;
    }

    #[test]
    fn saturating_sub(){
        type Test0<N,Val>=(
            AssertFnRet<N,SatSub1Op,Val>,
            AssertEq<SatSub1<N>,Val>,
        );

        let _:Test0<U0,U0>;
        let _:Test0<U1,U0>;
        let _:Test0<U2,U1>;
        let _:Test0<U3,U2>;
        let _:Test0<U4,U3>;


        type Test1<L,R,Val>=(
            AssertFnRet<(L,R),SatSubOp,Val>,
            AssertFnRet<(R,L),SatSubOp,Get0<L>>,
            AssertEq<SatSub<L,R>,Val>,
            AssertEq<SatSub<R,L>,Get0<L>>,
        );

        let _:Test1<U0,U0,U0>;
        let _:Test1<U1,U1,U0>;
        let _:Test1<U2,U2,U0>;
        let _:Test1<U3,U3,U0>;
        let _:Test1<U4,U4,U0>;

        let _:Test1<U1,U0,U1>;
        let _:Test1<U2,U1,U1>;
        let _:Test1<U3,U2,U1>;
        let _:Test1<U4,U3,U1>;
        let _:Test1<U5,U4,U1>;
        
        let _:Test1<U2,U0,U2>;
        let _:Test1<U3,U1,U2>;
        let _:Test1<U4,U2,U2>;
        let _:Test1<U5,U3,U2>;
        let _:Test1<U6,U4,U2>;



    }
}