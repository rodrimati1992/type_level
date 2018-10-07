use prelude::*;

/// An example type which uses a Const-parameter and allows extension methods.
#[derive(ConstConstructor)]
#[cconstructor(
    Type = "ConstUserExtMeth",
    ConstParam = "C",
    extension_methods = "true",
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


#[derive(ConstConstructor)]
#[cconstructor(
    Type = "TestingUnsized",
    ConstParam = "C",
)]
pub struct TestingUnsizedInner<T:?Sized,C> {
    pub const_: ConstWrapper<C>,
    pub value:T,
}


#[derive(ConstConstructor)]
#[cconstructor(
    // print_derive,
    Type = "TestingUnsizedOuter",
    ConstParam = "C",
)]
pub struct TestingUnsizedOuter_<T:?Sized,C> {
    pub const_: TestingUnsized<T,C>,
}

