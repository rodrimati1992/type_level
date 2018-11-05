use prelude::*;

use super::{
    ReplaceWithParamFn,
};

use fn_adaptors::GetRhs;

/// An example type which uses a Const-parameter and allows extension methods.
#[derive(MutConstValue)]
#[mcv(
    derive(Debug,Copy,Clone,Default),
    Type = "ConstUserExtMeth",
    ConstValue = "C",
)]
pub struct ConstUserExtMethInner<C> {
    #[allow(dead_code)]
    const_: ConstWrapper<C>,
}

impl<C> ConstUserExtMeth<C> {
    pub fn new() -> Self {
        Self {
            const_: ConstWrapper::NEW,
        }
    }
}

impl<Func,I> AllowMutatorFn<Func> for ConstUserExtMeth<I>{}

////////////////////////////////////////////////////////


#[derive(MutConstValue)]
#[mcv(
    derive(Debug,Copy,Clone,Default),    
    Type = "TestingUnsized",ConstValue = "C",
)]
pub struct TestingUnsizedInner<T:?Sized,C> {
    pub const_: ConstWrapper<C>,
    pub value:T,
}


#[derive(MutConstValue)]
#[mcv(
    derive(Debug,Copy,Clone,Default),
    Type = "TestingUnsizedOuter",ConstValue = "C",
)]
pub struct TestingUnsizedOuter_<T:?Sized,C> {
    pub const_: TestingUnsized<T,C>,
}

////////////////////////////////////////////////////////



/// A Type which does not implement ConstLayoutIndependent
#[derive(Debug,Copy,Clone,Default)]
pub struct NoConstLayoutIndependent<T>(pub T);


#[derive(MutConstValue)]
#[mcv(
    Type= "StoredInside", ConstValue = "I"
)]
pub struct StoredInsideInner<T,I> 
where 
    I:WrapperTrait,
    UnwrapConst<I>:Sized,
{
    pub value:T,
    marker: NoConstLayoutIndependent<UnwrapConst<I>>,
}

impl<T,I> StoredInside<T,I>{
    pub fn new(value:T,marker:I)->Self{
        Self{
            value,
            marker:NoConstLayoutIndependent(marker)
        }
    }
}

impl<T,I> AllowMutatorFn<ReplaceWithParamFn> for StoredInside<T,I>{}

