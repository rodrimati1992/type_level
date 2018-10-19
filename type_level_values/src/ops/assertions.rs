use prelude::*;

type_fn!{define_trait
    /// Asserts that Self is the same type as R.
    trait=AssertEq_ [R]
    /// Asserts that Self is the same type as R.
    type=AssertEq
    /// Asserts that Self is the same type as R.
    fn_type=AssertEqOp
}


type_fn!{
    captures(Rhs)
    /// Asserts that This is the same type as Rhs.
    pub fn AssertEqMt[This](This)
    where[ This:AssertEq_<Rhs> ]
    { Rhs }
}


impl<L,R> AssertEq_<R> for L
where
    L:TypeIdentity<Type=R>
{
    type Output=R;
}



type_fn!{define_trait
    /// Asserts that Func implements `TypeFn_<Val>`,returning Val unmodified.
    trait=AssertFunc_ [Func]
    /// Asserts that Func implements `TypeFn_<Val>`,returning Val unmodified.
    type=AssertFunc
    /// Asserts that Func implements `TypeFn_<Val>`,returning Val unmodified.
    fn_type=AssertFuncOp
}

impl<This,Func> AssertFunc_<Func> for This
where 
    Func:TypeFn_<This>,
{
    type Output=This;
}


type_fn!{
    captures(Func)
    /// Asserts that Func implements `TypeFn_<Val>`,returning Val unmodified.
    pub fn AssertFuncMt[Val](Val)
    where[ Func:TypeFn_<Val> ]
    { Val }
}


 
type_fn!{define_trait
    /// Asserts that `Func:TypeFn_<Self>` evaluates to Ret,returning Self unmodified.
    trait=AssertFnRet_ [Func,Ret]
    /// Asserts that `Func:TypeFn_<Val>` evaluates to Ret,returning Val unmodified.
    type=AssertFnRet
    /// Asserts that `Func:TypeFn_<Val>` evaluates to Ret,returning Val unmodified.
    fn_type=AssertFnRetOp
}


impl<This,Func,Ret> AssertFnRet_<Func,Ret> for This
where
    Func:TypeFn_<This,Output=Ret>
{
    type Output=This;
}

type_fn!{
    captures(Func,Ret)
    /// Asserts that Func implements `TypeFn_<Val>`,returning Val unmodified.
    pub fn AssertFnRetMt[Val](Val)
    where[ Func:TypeFn_<Val,Output=Ret> ]
    { Val }
}

