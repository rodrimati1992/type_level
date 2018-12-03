use prelude::*;

use crate_::fn_adaptors::{Const, GetLhs, GetRhs, IdentityFn};
use crate_::ops::*;
use crate_::std_ops::{DivOp, SubOp};

use std_::ops::{Add, Sub};

/// Integer constants for a ConstType,it is also be implemented for all values of a ConstType.
pub trait IntegerConsts {
    type Zero;
    type One;

    // this is an OptionType,since there may not be a minimum value.
    type Min;
    // this is an OptionType,since there may not be a maximum value.
    type Max;
}

impl<T, Type> IntegerConsts for T
where
    ConstTypeOfOp: TypeFn_<T, Output = Type>,
    Type: IntegerConsts,
{
    type Zero = Type::Zero;
    type One = Type::One;
    type Min = Type::Min;
    type Max = Type::Max;
}

type_fn!{define_trait
    /// Subtracts Rhs from Lhs ,stopping at te minimum value..
    ///
    /// Equivalent to `|lhs,rhs| lhs.saturating_sub(rhs) `
    trait=SatSub_ [R]
    /// Subtracts Rhs from Lhs ,stopping at te minimum value..
    ///
    /// Equivalent to `|lhs,rhs| lhs.saturating_sub(rhs) `
    type=SatSub
    /// Subtracts Rhs from Lhs ,stopping at te minimum value..
    ///
    /// Equivalent to `|lhs,rhs| lhs.saturating_sub(rhs) `
    fn_type=SatSubOp
    /// Subtracts Rhs from Lhs ,stopping at te minimum value..
    ///
    /// Equivalent to `|lhs,rhs| lhs.saturating_sub(rhs) `
    method_like=SatSubMt
}

type_fn!{define_trait
    /// Safe division function which returns None_ when
    /// the divisor is 0 (or the division would overflow) .
    ///
    /// Equivalent to `|lhs,rhs| lhs.checked_div(rhs) `
    trait=SafeDiv_ [R]
    /// Safe division function which returns None_ when
    /// the divisor is 0 (or the division would overflow) .
    ///
    /// Equivalent to `|lhs,rhs| lhs.checked_div(rhs) `
    type=SafeDiv
    /// Safe division function which returns None_ when
    /// the divisor is 0 (or the division would overflow) .
    ///
    /// Equivalent to `|lhs,rhs| lhs.checked_div(rhs) `
    fn_type=SafeDivOp
    /// Safe division function which returns None_ when
    /// the divisor is 0 (or the division would overflow) .
    ///
    /// Equivalent to `|lhs,rhs| lhs.checked_div(rhs) `
    method_like=SafeDivMt
}

type_fn!{define_trait
    /// Safe unsigned subtraction function which returns None_ when subtracting would overflow.
    ///
    /// Equivalent to `|lhs,rhs| lhs.checked_sub(rhs) `
    trait=SafeSub_ [R]
    /// Safe unsigned subtraction function which returns None_ when subtracting would overflow.
    ///
    /// Equivalent to `|lhs,rhs| lhs.checked_sub(rhs) `
    type=SafeSub
    /// Safe unsigned subtraction function which returns None_ when subtracting would overflow.
    ///
    /// Equivalent to `|lhs,rhs| lhs.checked_sub(rhs) `
    fn_type=SafeSubOp
    /// Safe unsigned subtraction function which returns None_ when subtracting would overflow.
    ///
    /// Equivalent to `|lhs,rhs| lhs.checked_sub(rhs) `
    method_like=SafeSubMt
}

type_fn!{define_trait
    /// Subtracts 1 from Self,stopping at te minimum value.
    ///
    /// Equivalent to `|lhs| lhs.saturating_sub(1) `
    trait=SatSub1_ []
    /// Subtracts 1 from Self,stopping at te minimum value.
    ///
    /// Equivalent to `|lhs| lhs.saturating_sub(1) `
    type=SatSub1
    /// Subtracts 1 from Self,stopping at te minimum value.
    ///
    /// Equivalent to `|lhs| lhs.saturating_sub(1) `
    fn_type=SatSub1Op
}

type_fn!{define_trait
    /// Returns whether N is 0.
    trait=IsZero_ []
    /// Returns whether N is 0.
    type=IsZero
    /// Returns whether N is 0.
    fn_type=IsZeroOp
}

type_fn!{define_trait
    /// Returns the absolute value (with the same ConstType).
    trait=AbsVal_ []
    /// Returns the absolute value (with the same ConstType).
    type=AbsVal
    /// Returns the absolute value (with the same ConstType).
    fn_type=AbsValOp
}

/// Returns whether N is 1.
pub type IsOne<N> = TypeFn<IsOneOp, N>;

type_fn!{
    /// Returns whether N is 1.
    pub fn IsOneOp[N](N)
    where[
        N:IntegerConsts<One=One>,
        N:ConstEq_<One,Output=Out>,
    ]{
        let One;let Out;
        Out
    }
}

/// Returns whether N is the minimum value of the type.
pub type IsMin<N> = TypeFn<IsMinOp, N>;

/// Returns whether N is the maximum value of the type.
pub type IsMax<N> = TypeFn<IsMaxOp, N>;

type_fn!{
    /// Returns whether N is the mimimum value of the type.
    pub fn IsMinOp[N](N)
    where[
        N:IntegerConsts<Min=Min>,
        Some_<N>:ConstEq_<Min,Output=Out>,
    ]{
        let Min;let Out;
        Out
    }
}

type_fn!{
    /// Returns whether N is the maximum value of the type.
    pub fn IsMaxOp[N](N)
    where[
        N:IntegerConsts<Max=Max>,
        Some_<N>:ConstEq_<Max,Output=Out>,
    ]{
        let Max;let Out;
        Out
    }
}

/// Gets the value for 0 as defined by N.
pub type Get0<N> = TypeFn<Get0Op, N>;

type_fn!{
    /// Gets the value for 0 as defined by N.
    pub fn Get0Op[N](N)
    where[ N:IntegerConsts<Zero=Out> ]
    { let Out;Out }
}

/// Gets the value for 1 as defined by N.
pub type Get1<N> = TypeFn<Get1Op, N>;

type_fn!{
    /// Gets the value for 1 as defined by N.
    pub fn Get1Op[N](N)
    where[ N:IntegerConsts<One=Out> ]
    { let Out;Out }
}

/// Gets the minimum value as defined by N.
pub type GetMin<N> = TypeFn<GetMinOp, N>;

type_fn!{
    /// Gets the minimum value as defined by N.
    pub fn GetMinOp[N](N)
    where[ N:IntegerConsts<Min=Out> ]
    { let Out;Out }
}

/// Gets the maximum value as defined by N.
pub type GetMax<N> = TypeFn<GetMaxOp, N>;

type_fn!{
    /// Gets the maximum value as defined by N.
    pub fn GetMaxOp[N](N)
    where[ N:IntegerConsts<Max=Out> ]
    { let Out;Out }
}

/// Adds 1 to N.
pub type Add1<N> = TypeFn<Add1Op, N>;

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
pub type Sub1<N> = TypeFn<Sub1Op, N>;

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

#[cfg(all(test, feature = "passed_tests"))]
// #[cfg(test)]
mod tests {
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
    fn get_constants() {
        type TestZero<N, Val> = (AssertEq<Get0<N>, Val>, AssertPipedRet<N, Get0Op, Val>);
        type TestOne<N, Val> = (AssertEq<Get1<N>, Val>, AssertPipedRet<N, Get1Op, Val>);

        let _: TestZero<U0, U0>;
        let _: TestZero<U1, U0>;
        let _: TestZero<U2, U0>;

        let _: TestZero<N2, Z0>;
        let _: TestZero<N1, Z0>;
        let _: TestZero<Z0, Z0>;
        let _: TestZero<P1, Z0>;
        let _: TestZero<P2, Z0>;

        let _: TestOne<U0, U1>;
        let _: TestOne<U1, U1>;
        let _: TestOne<U2, U1>;

        let _: TestOne<N2, P1>;
        let _: TestOne<N1, P1>;
        let _: TestOne<Z0, P1>;
        let _: TestOne<P1, P1>;
        let _: TestOne<P2, P1>;
    }

    #[test]
    fn is_one_zero() {
        let _: TypeFnEq!(IsZeroOp, IsZero, U0, True);
        let _: TypeFnEq!(IsZeroOp, IsZero, U1, False);
        let _: TypeFnEq!(IsZeroOp, IsZero, U2, False);

        let _: TypeFnEq!(IsZeroOp, IsZero, N2, False);
        let _: TypeFnEq!(IsZeroOp, IsZero, N1, False);
        let _: TypeFnEq!(IsZeroOp, IsZero, Z0, True);
        let _: TypeFnEq!(IsZeroOp, IsZero, P1, False);
        let _: TypeFnEq!(IsZeroOp, IsZero, P2, False);

        let _: TypeFnEq!(IsOneOp, IsOne, U0, False);
        let _: TypeFnEq!(IsOneOp, IsOne, U1, True);
        let _: TypeFnEq!(IsOneOp, IsOne, U2, False);

        let _: TypeFnEq!(IsOneOp, IsOne, N2, False);
        let _: TypeFnEq!(IsOneOp, IsOne, N1, False);
        let _: TypeFnEq!(IsOneOp, IsOne, Z0, False);
        let _: TypeFnEq!(IsOneOp, IsOne, P1, True);
        let _: TypeFnEq!(IsOneOp, IsOne, P2, False);
    }

    #[test]
    fn safe_div() {
        type Test<L, R, Val> = (
            AssertEq<TypeFn<SafeDivOp, (L, R)>, Val>,
            AssertEq<SafeDiv<L, R>, Val>,
        );

        let _: Test<U0, U0, None_>;
        let _: Test<U1, U0, None_>;
        let _: Test<U2, U0, None_>;

        let _: Test<U0, U1, Some_<U0>>;
        let _: Test<U1, U1, Some_<U1>>;
        let _: Test<U2, U1, Some_<U2>>;

        let _: Test<P2, N2, Some_<N1>>;
        let _: Test<P1, N2, Some_<Z0>>;
        let _: Test<Z0, N2, Some_<Z0>>;

        let _: Test<P2, N1, Some_<N2>>;
        let _: Test<P1, N1, Some_<N1>>;
        let _: Test<Z0, N1, Some_<Z0>>;

        let _: Test<Z0, Z0, None_>;
        let _: Test<P1, Z0, None_>;
        let _: Test<P2, Z0, None_>;

        let _: Test<Z0, P1, Some_<Z0>>;
        let _: Test<P1, P1, Some_<P1>>;
        let _: Test<P2, P1, Some_<P2>>;

        let _: Test<Z0, P2, Some_<Z0>>;
        let _: Test<P1, P2, Some_<Z0>>;
        let _: Test<P2, P2, Some_<P1>>;
    }

    #[test]
    fn safe_sub() {
        type AssertSub<L, R, Val> = (
            AssertPipedRet<(L, R), SafeSubOp, Val>,
            AssertEq<SafeSub<L, R>, Val>,
        );

        type TestUns<L, R, Val> = (AssertSub<L, R, Some_<Val>>, AssertSub<R, L, None_>);
        type TestSig<L, R, Val> = (AssertSub<L, R, Some_<Val>>,);

        let _: AssertSub<U0, U0, Some_<U0>>;
        let _: AssertSub<U1, U1, Some_<U0>>;
        let _: AssertSub<U2, U2, Some_<U0>>;
        let _: AssertSub<U3, U3, Some_<U0>>;

        let _: TestUns<U1, U0, U1>;
        let _: TestUns<U2, U0, U2>;
        let _: TestUns<U2, U1, U1>;
        let _: TestUns<U3, U0, U3>;
        let _: TestUns<U3, U1, U2>;
        let _: TestUns<U3, U2, U1>;

        let _: AssertSub<N3, N3, Some_<Z0>>;
        let _: AssertSub<N2, N2, Some_<Z0>>;
        let _: AssertSub<N1, N1, Some_<Z0>>;
        let _: AssertSub<Z0, Z0, Some_<Z0>>;
        let _: AssertSub<P1, P1, Some_<Z0>>;
        let _: AssertSub<P2, P2, Some_<Z0>>;
        let _: AssertSub<P3, P3, Some_<Z0>>;

        let _: TestSig<N1, N3, P2>;
        let _: TestSig<N1, N2, P1>;
        let _: TestSig<N1, Z0, N1>;
        let _: TestSig<N1, P1, N2>;
        let _: TestSig<N1, P2, N3>;

        let _: TestSig<Z0, N3, P3>;
        let _: TestSig<Z0, N2, P2>;
        let _: TestSig<Z0, N1, P1>;
        let _: TestSig<Z0, P1, N1>;
        let _: TestSig<Z0, P2, N2>;
        let _: TestSig<Z0, P3, N3>;

        let _: TestSig<P1, N3, P4>;
        let _: TestSig<P1, N2, P3>;
        let _: TestSig<P1, N1, P2>;
        let _: TestSig<P1, Z0, P1>;
        let _: TestSig<P1, P2, N1>;
    }

    #[test]
    fn add_sub_1() {
        type TestAdd1<N, Val> = (AssertEq<Add1<N>, Val>, AssertPipedRet<N, Add1Op, Val>);
        type TestSub1<N, Val> = (AssertEq<Sub1<N>, Val>, AssertPipedRet<N, Sub1Op, Val>);

        let _: TestAdd1<U0, U1>;
        let _: TestAdd1<U1, U2>;
        let _: TestAdd1<U2, U3>;

        let _: TestSub1<U1, U0>;
        let _: TestSub1<U2, U1>;
        let _: TestSub1<U3, U2>;

        let _: TestAdd1<N2, N1>;
        let _: TestAdd1<N1, Z0>;
        let _: TestAdd1<Z0, P1>;
        let _: TestAdd1<P1, P2>;
        let _: TestAdd1<P2, P3>;

        let _: TestSub1<N2, N3>;
        let _: TestSub1<N1, N2>;
        let _: TestSub1<Z0, N1>;
        let _: TestSub1<P1, Z0>;
        let _: TestSub1<P2, P1>;
    }

    #[test]
    fn saturating_sub() {
        type Test0<N, Val> = (AssertPipedRet<N, SatSub1Op, Val>, AssertEq<SatSub1<N>, Val>);

        let _: Test0<U0, U0>;
        let _: Test0<U1, U0>;
        let _: Test0<U2, U1>;
        let _: Test0<U3, U2>;
        let _: Test0<U4, U3>;

        let _: Test0<N2, N3>;
        let _: Test0<N1, N2>;
        let _: Test0<Z0, N1>;
        let _: Test0<P1, Z0>;
        let _: Test0<P2, P1>;
        let _: Test0<P3, P2>;
        let _: Test0<P4, P3>;

        type TestUns<L, R, Val> = (
            AssertPipedRet<(L, R), SatSubOp, Val>,
            AssertPipedRet<(R, L), SatSubOp, Get0<L>>,
            AssertEq<SatSub<L, R>, Val>,
            AssertEq<SatSub<R, L>, Get0<L>>,
        );
        type TestSig<L, R, Val> = (
            AssertPipedRet<(L, R), SatSubOp, Val>,
            AssertEq<SatSub<L, R>, Val>,
        );

        let _: TestUns<U0, U0, U0>;
        let _: TestUns<U1, U1, U0>;
        let _: TestUns<U2, U2, U0>;
        let _: TestUns<U3, U3, U0>;
        let _: TestUns<U4, U4, U0>;

        let _: TestUns<U1, U0, U1>;
        let _: TestUns<U2, U1, U1>;
        let _: TestUns<U3, U2, U1>;
        let _: TestUns<U4, U3, U1>;
        let _: TestUns<U5, U4, U1>;

        let _: TestUns<U2, U0, U2>;
        let _: TestUns<U3, U1, U2>;
        let _: TestUns<U4, U2, U2>;
        let _: TestUns<U5, U3, U2>;
        let _: TestUns<U6, U4, U2>;

        let _: TestSig<N1, N3, P2>;
        let _: TestSig<N1, N2, P1>;
        let _: TestSig<N1, N1, Z0>;
        let _: TestSig<N1, Z0, N1>;
        let _: TestSig<N1, P1, N2>;
        let _: TestSig<N1, P2, N3>;

        let _: TestSig<Z0, N3, P3>;
        let _: TestSig<Z0, N2, P2>;
        let _: TestSig<Z0, N1, P1>;
        let _: TestSig<Z0, Z0, Z0>;
        let _: TestSig<Z0, P1, N1>;
        let _: TestSig<Z0, P2, N2>;
        let _: TestSig<Z0, P3, N3>;

        let _: TestSig<P1, N3, P4>;
        let _: TestSig<P1, N2, P3>;
        let _: TestSig<P1, N1, P2>;
        let _: TestSig<P1, Z0, P1>;
        let _: TestSig<P1, P1, Z0>;
        let _: TestSig<P1, P2, N1>;
    }

    #[test]
    fn absolute_val() {
        type Test<val, expected> = AssertEq<AbsVal<val>, expected>;

        let _: Test<N3, P3>;
        let _: Test<N2, P2>;
        let _: Test<N1, P1>;
        let _: Test<Z0, Z0>;
        let _: Test<P1, P1>;
        let _: Test<P2, P2>;
        let _: Test<P3, P3>;

        let _: Test<U0, U0>;
        let _: Test<U1, U1>;
        let _: Test<U2, U2>;
        let _: Test<U3, U3>;
    }
}
