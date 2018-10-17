use std::fmt::{self,Display,Debug};

#[derive(Copy,Clone,PartialEq,Eq,Ord,PartialOrd,Hash)]
pub enum Either<L,R>{
    Left(L),
    Right(R),
}

macro_rules! either {
    ($value:expr, $pattern:pat => $result:expr) => (
        match $value {
            Either::Left($pattern) => $result,
            Either::Right($pattern) => $result,
        }
    )
}

impl<L,R> Debug for Either<L,R>
where 
    L:Debug,
    R:Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        either!(*self, ref inner => inner.fmt(f))
    }
}
impl<L,R> Display for Either<L,R>
where 
    L:Display,
    R:Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        either!(*self, ref inner => inner.fmt(f))
    }
}