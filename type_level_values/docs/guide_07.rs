doc_code_snippets! {
    mod "guide_07",
    type_ident=Guide07,
    template=r##"

This chapter demonstrates a Mutator Function used to mutate the 
ConstValue-parameter of a type wrapped in an Arc.


To demonstrate mutation of a ConstValue-parameter
we'll use a type which wraps an RwLock,
restricting when it allows mutable access of its contents.


//@use_codeblock:access_enum,ignore

This declares an enum describing whether one has read or mutable access 
to the contents of the RwLock.

//@use_codeblock:rw_locker_struct,ignore

This declares a wrapper around an RwLock which also takes an Access ConstValue-parameter.

//@use_codeblock:rw_locker_new,ignore

This declares the constructor,which always returns the RwLocker with `RwAccess`
because the caller can restrict access to `ReadAccess` with 
`.mutparam(RestrictAccess,().ty_())`.


//@use_codeblock:read_method,ignore

This wraps the RwLock::read method,callable with any ConstValue-parameter.

The reason it is a generic parameter instead of `RwLocker<_,ReadAccess >`
is because  it would disallow calling the method with `RwLocker<_,RwAccess >`.

//@use_codeblock:write_method,ignore

This wraps the RwLock::write method,accessible only if the ConstValue-parameter is RwAccess.

//@use_codeblock:restrict_access,ignore

This defines a Mutator Function which restricts the RwLocker to have read access 
(instead of mutable).

The `pub fn Name=FunctionType` syntax is used to delegate to another TypeFn_.

//@use_codeblock:replace_with,ignore

This is a function which accesses the RwLock's contents mutably,
setting them to the default value of the type.


//@use_codeblock:read_value,ignore

This is a function only has immutable access,
because it uses a generic parameter with no bounds,
and simply prints the contents of the RwLock.

//@use_codeblock:main_0,ignore

This is the start of the main function.

Here we initialize locker in an `Arc` and show that both functions are callable
because RwLocker is created with `RwAccess`.

//@use_codeblock:main_1,ignore

Here we clone the Arc,creating another handle to the value,
changing its ConstValue-parameter from `RwAccess` to `ReadAccess`,
note that `locker` was not affected by `restricted_locker` changing its ConstValue-parameter.


<br><br><br><br><br><br><br><br><br><br>
<hr>
<br><br><br><br><br><br><br><br><br><br>


# The entire thing

//@use_codeblock:all,rust

"##,

    code=r##"


//@codeblock-start:all


#[macro_use]
extern crate derive_type_level;
#[macro_use]
extern crate type_level_values;


use type_level_values::prelude::*;
use type_level_values::field_traits::{SetField,SetField_};
use type_level_values::fn_adaptors::{Const};


use std::sync::Arc;
use std::sync::{RwLock,RwLockReadGuard,RwLockWriteGuard,LockResult,TryLockResult};
use std::fmt;


//@codeblock-start:access_enum

#[derive(TypeLevel)]
#[typelevel(
    derive(ConstEq,ConstOrd),
    reexport(Variants),
)]
pub enum Access{
    ReadAccess,
    RwAccess,
}

//@codeblock-end:access_enum



//@codeblock-start:rw_locker_struct

#[derive(MutConstValue)]
#[mcv(
    derive(Debug),
    Type = "RwLocker",ConstValue = "C",
)]
pub struct __RwLocker<T,C>{
    lock:RwLock<T>,
    access:ConstWrapper<C>,
}

//@codeblock-end:rw_locker_struct



//@codeblock-start:rw_locker_new

impl<T> RwLocker<T,RwAccess>{
    pub fn new(value:T)->Self{
        Self{
            lock:RwLock::new(value),
            access:ConstWrapper::NEW,
        }
    }
}

//@codeblock-end:rw_locker_new



//@codeblock-start:read_method

impl<T,C> RwLocker<T,C>{
    pub fn read(&self) -> LockResult<RwLockReadGuard<T>>{
        self.lock.read()
    }
}

//@codeblock-end:read_method



//@codeblock-start:write_method

impl<T> RwLocker<T,RwAccess>{
    pub fn write(&self) -> LockResult<RwLockWriteGuard<T>>{
        self.lock.write()
    }
}

//@codeblock-end  :write_method



//@codeblock-start:restrict_access


mutator_fn!{
    type This[T,A]=(RwLocker<T,A>)
    type AllowedSelf=(allowed_self_constructors::All)

    pub fn RestrictAccess=Const<ReadAccess>;
}

//@codeblock-end:restrict_access



//@codeblock-start:replace_with

fn replace_with_default<T>(locker:&RwLocker<T,RwAccess>)
where 
    T:Default
{
    *locker.write().unwrap()=T::default()
}

//@codeblock-end  :replace_with



//@codeblock-start:read_value

fn read_value<T,C>(locker:&RwLocker<T,C>)
where 
    T:fmt::Debug
{
    println!("{:?}", locker.read().unwrap() );
}

//@codeblock-end  :read_value




fn main(){
    
    //@codeblock-start:main_0

    //  locker : Arc< RwLocker< String, RwAccess > > 
    let locker=Arc::new(RwLocker::new("hello".to_string()));

    {
        read_value(&locker);
        replace_with_default(&locker);
    }

    //@codeblock-end:main_0



    //@codeblock-start:main_1
    
    {
        //  restricted_locker : Arc< RwLocker< String, ReadAccess > > 
        let restricted_locker=RwLocker::mutparam_arc(
            locker.clone(),
            RestrictAccess::NEW,
            ().ty_(),
        );
        
        read_value( &restricted_locker );

        // can't call this function,because restricted_locker only has read access.
        // replace_with_default( &restricted_locker );

        // can call it with locker because it still has RwAccess.
        replace_with_default( &locker );
    }

    //@codeblock-end:main_1

}



"##,
}
