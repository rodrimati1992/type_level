use prelude::*;

/// An example type which uses a Const-parameter and allows extension methods.
#[derive(MutConstValue)]
#[mcv(
    derive(Debug,Copy,Clone,Default),
    Type = "ConstUserExtMeth",
    Param = "C",
    ExtensionMethods = "true",
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

////////////////////////////////////////////////////////


#[derive(MutConstValue)]
#[mcv(
    derive(Debug,Copy,Clone,Default),    
    Type = "TestingUnsized",Param = "C",
)]
pub struct TestingUnsizedInner<T:?Sized,C> {
    pub const_: ConstWrapper<C>,
    pub value:T,
}


#[derive(MutConstValue)]
#[mcv(
    derive(Debug,Copy,Clone,Default),
    Type = "TestingUnsizedOuter",Param = "C",
)]
pub struct TestingUnsizedOuter_<T:?Sized,C> {
    pub const_: TestingUnsized<T,C>,
}

////////////////////////////////////////////////////////


#[derive(Debug,Copy,Clone,Default)]
pub struct NoConstLayoutIndependent<T>(pub T);


#[derive(MutConstValue)]
#[mcv(
    derive(Debug,Copy,Clone,Default),
    Type= "StoredInside", Param = "I"
)]
pub struct StoredInsideInner<T,I> {
    pub value:T,
    marker: NoConstLayoutIndependent<I>,
}

impl<T,I> StoredInside<T,I>{
    pub fn new(value:T,_marker:I)->Self{
        Self{
            value,
            marker:NoConstLayoutIndependent(ConstWrapper::NEW)
        }
    }
}

const_method!{
    type ConstConstructor[T]=( StoredInsideCC<T> )
    type AllowedConversions=( allowed_conversions::All )
    pub fn ChangeParam[I,I2](I,I2){ I2 }
}


