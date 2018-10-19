use prelude::*;

use crate_::ops::*;
use crate_::fn_types::{DivOp,SubOp};
use crate_::fn_adaptors::{
    ReturnRhs,
    Const,
};


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


/// Safe division function which returns None_ when the divisor is 0.
/// 
/// if R==0 ,returns None_, otherwise returns Some_<L / R>.
pub type SafeDivOp=
    If<(ReturnRhs,IsZeroOp),
        Const<None_>,
        (DivOp,_new_some),
    >;

/// Safe unsigned subtraction function which returns None_ when Lhs < Rhs.
/// 
/// if L>=R ,returns Some_<L - R>, otherwise returns None_.
pub type SafeSubOp=
    If<ConstGEOp,
        (SubOp,_new_some),
        Const<None_>
    >;


type_fn!{
    fn _new_some[v](v){ Some_<v> }
}


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
        type Test<L,R,Val>=
            AssertEq<TypeFn<SafeDivOp,(L,R)>,Val>;

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
        type AssertSub<L,R,Val>=
            AssertEq<TypeFn<SafeSubOp,(L,R)>,Val>;

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
}