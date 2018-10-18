use prelude::*;

use std::fmt;
use std::cmp::{PartialEq,Eq,Ord,PartialOrd};


#[derive(TypeLevel)]
#[typelevel(
    derive(Debug,PartialEq,Eq,Ord,PartialOrd),
    reexport(Variants)
)]
struct Tupled(
    u32,
    u32,
);

#[derive(TypeLevel)]
#[typelevel(
    derive(Debug,PartialEq,Eq,Ord,PartialOrd),
    reexport(Variants)
)]
struct Braced{
    x:u32,
    y:u32,
}

#[derive(TypeLevel)]
#[typelevel(
    derive(Debug,PartialEq,Eq,Ord,PartialOrd),
    reexport(Variants)
)]
struct UnitStruct;


#[derive(TypeLevel)]
#[typelevel(
    derive(Debug,PartialEq,Eq,Ord,PartialOrd),
    reexport(Variants)
)]
enum AnEnum{
    VarA,
    VarB(u32,u32),
    VarC{
        a:u32,
        b:u32,
        c:u32,
    },
}



fn check<V>()
where V:fmt::Debug+PartialEq+Eq+Ord+PartialOrd,
{}

#[test]
fn test_delegated_derives(){
    check::< ConstTupled<U0,U10> >();
    check::< ConstBraced<U0,U10> >();
    check::< ConstUnitStruct >();
    check::< VarA >();
    check::< VarB<(),()> >();
    check::< VarC<(),(),()> >();
}