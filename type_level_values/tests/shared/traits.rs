use syn::Ident;
use core_extensions::prelude::*;

use shared::either::Either;

pub trait Trivial<X:?Sized>{
    type Trivial:?Sized;
}


impl<X:?Sized,Y:?Sized> Trivial<Y> for X{
    type Trivial=X;
}


pub trait OptIdent:TypeIdentity<Type=Option<Ident>>{
    fn or_index(&self,index:usize)->Either<&Ident,usize>{
        self.into_type_ref().as_ref().map_or(Either::Right(index),Either::Left)
    }
}

impl OptIdent for Option<Ident>{}