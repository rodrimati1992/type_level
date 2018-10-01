/*!
Contains all the built-in ExtensionConstMethods .

*/

use prelude::*;

const_method!{
    type ConstConstructor[]=( T )
    type AllowedConversions=( allowed_conversions::All )

    extension_method=internal_blanket_impl;

    /// Identity ConstMethod.
    pub fn IdentityConstMethod[I](I,()){ I }
}

///////////////////////////////////////////////////////////////////////////////////////////

const_method!{
    type ConstConstructor[]=( T )
    type AllowedConversions=( allowed_conversions::All )

    extension_method=internal_blanket_impl;
    safety=unsafe;

    /// Unsafe ConstMethod which allows mutating the Const-parameter of any type.
    ///
    /// # Safety
    ///
    /// This ConstMethod is unsafe because it can mutate the Const-parameter of any type ,
    /// which means that extra care has to be taken to maintain the invariants of the
    /// Const-parameter being mutated.
    ///
    pub fn Ext_MapConstParam[I,F](I,F)
    where [ F:TypeFn_<I> ]
    { F::Output }
}

///////////////////////////////////////////////////////////////////////////////////////////

const_method!{
    type ConstConstructor[]=( T )
    type AllowedConversions=( allowed_conversions::All )

    extension_method=internal_blanket_impl;
    safety=unsafe;

    /// Unsafe ConstMethod which allows setting the Const-parameter of any type.
    ///
    /// # Safety
    ///
    /// This ConstMethod is unsafe because it can mutate the Const-parameter of any type ,
    /// which means that extra care has to be taken to maintain the invariants of the
    /// Const-parameter being mutated.
    ///
    pub fn Ext_SetConstParam[I,I2](I,I2){ I2 }
}
