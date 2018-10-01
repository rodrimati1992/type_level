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
    const_: PhantomWrapper<C>,
}

impl<C> ConstUserExtMeth<C> {
    pub fn new() -> Self {
        Self {
            const_: PhantomWrapper::NEW,
        }
    }
}

////////////////////////////////////////////////////////
