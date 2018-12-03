/*!
Where the MutConstParam trait resides.
*/

use super::*;

use field_traits::GetFieldMt;

use user_traits::const_traits::{
    ConstLayoutIndependent, GetAllowedSelfOp, GetConstParam_, SetConstParam, SetConstParam_,
};

use user_traits::self_constructors_type::type_level_AllowedConstructors::fields as fields_ac;

use std_::mem::{size_of, size_of_val};
#[cfg(feature = "std")]
use std_::rc::Rc;
#[cfg(feature = "std")]
use std_::sync::Arc;

/**
Extension trait to mutate the ConstValue-parameter of Self.

The parameters of the methods in this trait follow this pattern:

- self/this: the receiver of the method,the type whose ConstValue-parameter will change.

- _mutator_func : the Mutator Function which mutates the ConstValue parameter of self.

- Msg : the second parameter to _mutator_func .


# Example

```

# #[macro_use]
# extern crate derive_type_level;
# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;
# use type_level_values::fn_adaptors::*;

fn main(){
    let mut admin=User::new(AdminLevel);
    {
        let regular:&mut User<UserLevel>=
            admin.mutparam_mut(DowngradePrivilege::NEW,().ty_());
        regular.run_command("cat -r ./");
        
        // The next line wont compile because the User does not have the privileges.
        // regular.restart_servers();

        // We can upgrade from a regular user to an admin by passing 
        // an UpgradeKey to upgrade_mut.
        let key=UpgradeKey::new(127).unwrap();
        regular
            .upgrade_mut(key)
            .restart_servers();
    }
    admin.run_command("close_everything");
    admin.restart_servers();
}


mod user{
    use super::*;

    #[derive(TypeLevel)]
    #[typelevel(reexport(Variants,Traits))]
    pub enum Privilege{
        AdminLevel,
        UserLevel,
    }

    #[derive(MutConstValue)]
    #[mcv(
        Type = "User",ConstValue = "P"
    )]
    pub struct __User<P> {
        _privilege: ConstWrapper<P>,
    }

    impl<P> User<P> {
        pub fn new(privilege:P) -> Self {
            Self {
                _privilege: ConstWrapper::NEW,
            }
        }
        pub fn run_command(&mut self,command:&str){
            println!("running command:{:?}",command);
        }

        pub fn upgrade_mut(&mut self,_upgrade_key:UpgradeKey)->&mut User<AdminLevel>{
            self.mutparam_mut(UpgradePrivilege::NEW,().ty_())
        }

        pub fn upgrade(self,_upgrade_key:UpgradeKey)->User<AdminLevel>{
            self.mutparam(UpgradePrivilege::NEW,().ty_())
        }
    }

    impl<P> User<P>
    where P:AdminLevelTrait
    {
        pub fn restart_servers(&mut self){
            println!("shutting down servers");
            println!("starting up servers");
        }
    }

    mutator_fn!{
        type This[P]=(User<P>)
        type AllowedSelf=(allowed_self_constructors::All)

        pub fn DowngradePrivilege=Const<UserLevel>;
    }

    mutator_fn!{
        type This[P]=(User<P>)
        type AllowedSelf=(allowed_self_constructors::All)

        fn UpgradePrivilege=Const<AdminLevel>;
    }

    //////////////////////////////////////////////////////////////////////
    
    
    pub struct UpgradeKey(());

    impl UpgradeKey{
        pub fn new(key:u64)->Option<Self>{
            if key==127 {
                Some(UpgradeKey(()))
            }else{
                None
            }
        }
    }


}
pub use self::user::*;



```







*/
pub trait MutConstParam {
    /// Mutates the ConstValue-parameter of Self.
    #[inline(always)]
    fn mutparam<Op, Msg>(self, _mutator_typefn: Op, _msg: VariantPhantom<Msg>) -> Self::NextSelf
    where
        Self: Sized + MutConstParamConstraints<Op, Msg, fields_ac::by_val>,
        Self::NextSelf: Sized,
    {
        unsafe { transmute_ignore_size(self) }
    }

    /// Mutates the ConstValue-parameter of Box<Self>.
    #[inline(always)]
    #[cfg(feature = "std")]
    fn mutparam_box<Op, Msg>(
        self: Box<Self>,
        _mutator_typefn: Op,
        _msg: VariantPhantom<Msg>,
    ) -> Box<Self::NextSelf>
    where
        Self: MutConstParamConstraints<Op, Msg, fields_ac::by_val>,
    {
        unsafe { transmute_ignore_size(self) }
    }

    /// Mutates the ConstValue-parameter of &'a Self.
    #[inline(always)]
    fn mutparam_ref<'a, Op, Msg>(
        &'a self,
        _mutator_typefn: Op,
        _msg: VariantPhantom<Msg>,
    ) -> &'a Self::NextSelf
    where
        Self: MutConstParamConstraints<Op, Msg, fields_ac::by_ref>,
        Self::NextSelf: 'a,
    {
        unsafe { transmute_ignore_size(self) }
    }

    /// Mutates the ConstValue-parameter of Rc<Self>.
    #[inline(always)]
    #[cfg(feature = "std")]
    fn mutparam_rc<Op, Msg>(
        this: Rc<Self>,
        _mutator_typefn: Op,
        _msg: VariantPhantom<Msg>,
    ) -> Rc<Self::NextSelf>
    where
        Self: MutConstParamConstraints<Op, Msg, fields_ac::by_ref>,
    {
        unsafe { transmute_ignore_size(this) }
    }

    /// Mutates the ConstValue-parameter of Arc\<Self>.
    #[inline(always)]
    #[cfg(feature = "std")]
    fn mutparam_arc<Op, Msg>(
        this: Arc<Self>,
        _mutator_typefn: Op,
        _msg: VariantPhantom<Msg>,
    ) -> Arc<Self::NextSelf>
    where
        Self: MutConstParamConstraints<Op, Msg, fields_ac::by_ref>,
    {
        unsafe { transmute_ignore_size(this) }
    }

    /// Mutates the ConstValue-parameter of &'a mut Self.
    #[inline(always)]
    fn mutparam_mut<'a, Op, Msg>(
        &'a mut self,
        _mutator_typefn: Op,
        _msg: VariantPhantom<Msg>,
    ) -> &'a mut Self::NextSelf
    where
        Self: MutConstParamConstraints<Op, Msg, fields_ac::by_mut>,
        Self::NextSelf: 'a,
    {
        unsafe { transmute_ignore_size(self) }
    }
}

impl<This: ?Sized> MutConstParam for This {}

/// Trait used to alias the constraints for every MutConstParam method,
/// which are on the blanket impl of this trait.
pub trait MutConstParamConstraints<Op, Msg, Pointerness>: MCPBounds<Op, Msg> {}

impl<This: ?Sized, Op, Msg, Pointerness> MutConstParamConstraints<Op, Msg, Pointerness> for This
where
    This: MCPBounds<Op, Msg>,
    Op: Piped_<(GetAllowedSelfOp, GetFieldMt<Pointerness>), Output = True>,
{}

/// The constraints for calling any MutConstParam methods.
pub unsafe trait MCPBounds<Op, Msg> {
    /// The mutated ConstValue-parameter.
    type NextConst;

    /// The type of `Self` after changing its ConstValue-parameter
    type NextSelf: ?Sized;
}

unsafe impl<This: ?Sized, NextConst, Func, Msg> MCPBounds<Func, Msg> for This
where
    Func: TypeFn_<(This::Const, Msg), Output = NextConst>,
    This: AllowMutatorFn<Func>
        + GetConstParam_
        + SetConstParam_<NextConst>
        + ConstLayoutIndependent<SetConstParam<This, NextConst>>,
{
    type NextConst = NextConst;
    type NextSelf = SetConstParam<This, NextConst>;
}
