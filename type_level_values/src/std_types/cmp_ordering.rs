/*!
Contains the type-level equivalent of a std::cmp::Ordering.
*/

use prelude::*;

use ops::const_from::ConstFrom_;
use ops::{ConstEq, ConstOrd,AssertEq};
use collection_ops::{Reverse_,Reverse};

use typenum::consts::{U0, U1, U2};

use std_::cmp::Ordering as StdOrdering;
use std_::ops::{BitAnd, BitOr};

use core_extensions::type_level_bool::{False, True};

#[derive(TypeLevel)]
#[typelevel(
    derive(),
    // skip_derive,
    // print_derive,
    reexport = "pub",
    items(runtime_conv(Internal = "StdOrdering")),
)]
#[allow(dead_code)]
#[doc(hidden)]
pub enum Ordering {
    #[typelevel(rename = "Less_", rename_trait = "LessTrait")]
    Less,
    #[typelevel(rename = "Equal_", rename_trait = "EqualTrait")]
    Equal,
    #[typelevel(rename = "Greater_", rename_trait = "GreaterTrait")]
    Greater,
}

//////////////////////////////////////////////////////////////////////////////////


impl Reverse_ for Less_{
    type Output=Greater_;
}

impl Reverse_ for Equal_{
    type Output=Equal_;
}

impl Reverse_ for Greater_{
    type Output=Less_;
}


//////////////////////////////////////////////////////////////////////////////////

impl<Rhs> BitAnd<Rhs> for Less_ {
    type Output = Self;
    fn bitand(self, _: Rhs) -> Self::Output {
        self
    }
}

impl<Rhs> BitAnd<Rhs> for Equal_ {
    type Output = Rhs;
    fn bitand(self, v: Rhs) -> Self::Output {
        v
    }
}

impl<Rhs> BitAnd<Rhs> for Greater_ {
    type Output = Self;
    fn bitand(self, _: Rhs) -> Self::Output {
        self
    }
}

//////////////////////////////////////////////////////////////////////////////////

impl<Rhs> BitOr<Rhs> for Less_ {
    type Output = Rhs;
    fn bitor(self, v: Rhs) -> Self::Output {
        v
    }
}

impl<Rhs> BitOr<Rhs> for Equal_ {
    type Output = Self;
    fn bitor(self, _: Rhs) -> Self::Output {
        self
    }
}

impl<Rhs> BitOr<Rhs> for Greater_
where
    Rhs: OrderingTrait,
{
    type Output = Rhs;
    fn bitor(self, v: Rhs) -> Self::Output {
        v
    }
}

//////////////////////////////////////////////////////////////////////////////////

impl ConstEq_<Less_> for Less_ {
    type Output = True;
}
impl ConstEq_<Equal_> for Less_ {
    type Output = False;
}
impl ConstEq_<Greater_> for Less_ {
    type Output = False;
}

impl ConstEq_<Less_> for Equal_ {
    type Output = False;
}
impl ConstEq_<Equal_> for Equal_ {
    type Output = True;
}
impl ConstEq_<Greater_> for Equal_ {
    type Output = False;
}

impl ConstEq_<Less_> for Greater_ {
    type Output = False;
}
impl ConstEq_<Equal_> for Greater_ {
    type Output = False;
}
impl ConstEq_<Greater_> for Greater_ {
    type Output = True;
}

//////////////////////////////////////////////////////////////////////////////////

impl ConstOrd_<Less_> for Less_ {
    type Output = Equal_;
}
impl ConstOrd_<Equal_> for Less_ {
    type Output = Less_;
}
impl ConstOrd_<Greater_> for Less_ {
    type Output = Less_;
}

impl ConstOrd_<Less_> for Equal_ {
    type Output = Greater_;
}
impl ConstOrd_<Equal_> for Equal_ {
    type Output = Equal_;
}
impl ConstOrd_<Greater_> for Equal_ {
    type Output = Less_;
}

impl ConstOrd_<Less_> for Greater_ {
    type Output = Greater_;
}
impl ConstOrd_<Equal_> for Greater_ {
    type Output = Greater_;
}
impl ConstOrd_<Greater_> for Greater_ {
    type Output = Equal_;
}

//////////////////////////////////////////////////////////////////////////////////

mod typenum_conv {
    use super::*;

    use extern_types::typenum::TNOrderingType;
    use typenum::{Equal as TNEqual, Greater as TNGreater, Less as TNLess};

    impl ConstFrom_<TNLess> for OrderingType {
        type Output = Less_;
    }
    impl ConstFrom_<TNEqual> for OrderingType {
        type Output = Equal_;
    }
    impl ConstFrom_<TNGreater> for OrderingType {
        type Output = Greater_;
    }

    impl ConstFrom_<Less_> for TNOrderingType {
        type Output = TNLess;
    }
    impl ConstFrom_<Equal_> for TNOrderingType {
        type Output = TNEqual;
    }
    impl ConstFrom_<Greater_> for TNOrderingType {
        type Output = TNGreater;
    }

}

//#[cfg(test)]
#[cfg(all(test,feature="passed_tests"))]
mod tests {
    use super::*;

    #[test]
    fn cmp_ordering_into_runtime() {
        assert_eq!(Less_.into_runtime(), StdOrdering::Less);
        assert_eq!(Equal_.into_runtime(), StdOrdering::Equal);
        assert_eq!(Greater_.into_runtime(), StdOrdering::Greater);
    }

    #[test]
    fn cmp_ordering_reverse() {
        let _: AssertEq<Reverse<Less_>,Greater_>;
        let _: AssertEq<Reverse<Equal_>,Equal_>;
        let _: AssertEq<Reverse<Greater_>,Less_>;
    }
    #[test]
    fn cmp_ordering_comparison() {
        let _: True = <ConstEq<Less_, Less_>>::MTVAL;
        let _: False = <ConstEq<Less_, Equal_>>::MTVAL;
        let _: False = <ConstEq<Less_, Greater_>>::MTVAL;

        let _: False = <ConstEq<Equal_, Less_>>::MTVAL;
        let _: True = <ConstEq<Equal_, Equal_>>::MTVAL;
        let _: False = <ConstEq<Equal_, Greater_>>::MTVAL;

        let _: False = <ConstEq<Greater_, Less_>>::MTVAL;
        let _: False = <ConstEq<Greater_, Equal_>>::MTVAL;
        let _: True = <ConstEq<Greater_, Greater_>>::MTVAL;

        /////////////////////////////////////////////////

        let _: Equal_ = <ConstOrd<Less_, Less_>>::MTVAL;
        let _: Less_ = <ConstOrd<Less_, Equal_>>::MTVAL;
        let _: Less_ = <ConstOrd<Less_, Greater_>>::MTVAL;

        let _: Greater_ = <ConstOrd<Equal_, Less_>>::MTVAL;
        let _: Equal_ = <ConstOrd<Equal_, Equal_>>::MTVAL;
        let _: Less_ = <ConstOrd<Equal_, Greater_>>::MTVAL;

        let _: Greater_ = <ConstOrd<Greater_, Less_>>::MTVAL;
        let _: Greater_ = <ConstOrd<Greater_, Equal_>>::MTVAL;
        let _: Equal_ = <ConstOrd<Greater_, Greater_>>::MTVAL;
    }

    #[test]
    fn cmp_ordering_operators() {
        let _: Less_ = Less_ & Less_;
        let _: Less_ = Less_ & Equal_;
        let _: Less_ = Less_ & Greater_;

        let _: Less_ = Equal_ & Less_;
        let _: Equal_ = Equal_ & Equal_;
        let _: Greater_ = Equal_ & Greater_;

        let _: Greater_ = Greater_ & Less_;
        let _: Greater_ = Greater_ & Equal_;
        let _: Greater_ = Greater_ & Greater_;

        let _: Less_ = Less_ | Less_;
        let _: Equal_ = Less_ | Equal_;
        let _: Greater_ = Less_ | Greater_;

        let _: Equal_ = Equal_ | Less_;
        let _: Equal_ = Equal_ | Equal_;
        let _: Equal_ = Equal_ | Greater_;

        let _: Less_ = Greater_ | Less_;
        let _: Equal_ = Greater_ | Equal_;
        let _: Greater_ = Greater_ | Greater_;
    }

}
