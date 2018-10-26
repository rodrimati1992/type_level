use super::*;

use user_traits::const_traits::{
    AllowOp, ConstLayoutIndependent, GetConstConstructor_, SetConstParam, SetConstParam_,
};

use user_traits::const_methods::{ComputeConstParam_, OpAttrs};

use user_traits::allowed_conversions_type::type_level_AllowedConversions::fields as fields_ac;
use user_traits::allowed_conversions_type::AllowedConversionsTrait;

use std_::mem::{size_of, size_of_val};
#[cfg(feature = "std")]
use std_::rc::Rc;
#[cfg(feature = "std")]
use std_::sync::Arc;


/**
Extension trait to
mutate the Const-parameter of Self (these methods are side-effect free).

The parameters of the methods in this trait follow this pattern:

- self/this: the receiver of the method,the type whose Const-parameter will change.

- ConstMethod : the ConstMethod being called .

- Msg : the type of the parameter to the ConstMethod .


# Example

```

# #[macro_use]
# extern crate derive_type_level;
# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;

fn main(){
    let mut admin=User::new(AdminLevel);
    {
        let regular:&mut User<UserLevel>=
            admin.mutparam_mut(DowngradePrivilege,Default::default());
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
        Type = "User",Param = "P"
    )]
    pub struct UserInner<P> {
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
            self.mutparam_mut(UpgradePrivilege::new(),Default::default())
        }

        pub fn upgrade(self,_upgrade_key:UpgradeKey)->User<AdminLevel>{
            self.mutparam(UpgradePrivilege::new(),Default::default())
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

    const_method!{
        type ConstConstructor[]=( UserCC )
        type AllowedConversions=( allowed_conversions::All )

        pub fn DowngradePrivilege[P](P,()){ UserLevel }
    }

    const_method!{
        type ConstConstructor[]=( UserCC )
        type AllowedConversions=( allowed_conversions::All )

        fn UpgradePrivilege[P](P,()){ AdminLevel }
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
pub trait MutConstParam: GetConstConstructor_ {
    /// Mutates the Const-parameter of Self.
    #[inline(always)]
    fn mutparam<Op, Msg>(self, _const_method: Op, _msg: VariantPhantom<Msg>) -> Self::NextSelf
    where
        Self: Sized + MutConstParamConstraints<Op, Msg, fields_ac::by_val>,
        Self::NextSelf: Sized,
    {
        unsafe { transmute_ignore_size(self) }
    }

    /// Mutates the Const-parameter of Box<Self>.
    #[inline(always)]
    #[cfg(feature = "std")]
    fn mutparam_box<Op, Msg>(
        self: Box<Self>,
        _const_method: Op,
        _msg: VariantPhantom<Msg>,
    ) -> Box<Self::NextSelf>
    where
        Self: MutConstParamConstraints<Op, Msg, fields_ac::by_val>,
    {
        unsafe { transmute_ignore_size(self) }
    }

    /// Mutates the Const-parameter of &'a Self.
    #[inline(always)]
    fn mutparam_ref<'a, Op, Msg>(
        &'a self,
        _const_method: Op,
        _msg: VariantPhantom<Msg>,
    ) -> &'a Self::NextSelf
    where
        Self: MutConstParamConstraints<Op, Msg, fields_ac::by_ref>,
        Self::NextSelf: 'a,
    {
        unsafe { transmute_ignore_size(self) }
    }

    /// Mutates the Const-parameter of Rc<Self>.
    #[inline(always)]
    #[cfg(feature = "std")]
    fn mutparam_rc<Op, Msg>(
        this: Rc<Self>,
        _const_method: Op,
        _msg: VariantPhantom<Msg>,
    ) -> Rc<Self::NextSelf>
    where
        Self: MutConstParamConstraints<Op, Msg, fields_ac::by_ref>,
    {
        unsafe { transmute_ignore_size(this) }
    }

    /// Mutates the Const-parameter of Arc\<Self>.
    #[inline(always)]
    #[cfg(feature = "std")]
    fn mutparam_arc<Op, Msg>(
        this: Arc<Self>,
        _const_method: Op,
        _msg: VariantPhantom<Msg>,
    ) -> Arc<Self::NextSelf>
    where
        Self: MutConstParamConstraints<Op, Msg, fields_ac::by_ref>,
    {
        unsafe { transmute_ignore_size(this) }
    }

    /// Mutates the Const-parameter of &'a mut Self.
    #[inline(always)]
    fn mutparam_mut<'a, Op, Msg>(
        &'a mut self,
        _const_method: Op,
        _msg: VariantPhantom<Msg>,
    ) -> &'a mut Self::NextSelf
    where
        Self: MutConstParamConstraints<Op, Msg, fields_ac::by_mut>,
        Self::NextSelf: 'a,
    {
        unsafe { transmute_ignore_size(self) }
    }
}

impl<This> MutConstParam for This where This: GetConstConstructor_ {}

/// Trait used to alias the constraints for every MutConstParam method,
/// which are on the blanket impl of this trait.
pub trait MutConstParamConstraints<Op, Msg, Pointerness>: MCPBounds<Op, Msg> {}

impl<This: ?Sized, Op, Msg, Pointerness> MutConstParamConstraints<Op, Msg, Pointerness> for This
where
    This: MCPBounds<Op, Msg>,
    Op: OpAttrs,
    Op::Conversions: AllowedConversionsTrait,
    Op::Conversions: GetField_<Pointerness, Output = True>,
{}

/// The constraints for calling any MutConstParam methods.
pub trait MCPBounds<Op, Msg> {
    /// The mutated Const-parameter.
    type NextConst;

    /// The type of `Self` after changing its Const-parameter
    type NextSelf: ?Sized;
}

impl<This: ?Sized, NextConst, Op, Msg> MCPBounds<Op, Msg> for This
where
    This: GetConstConstructor_,
    Op: ComputeConstParam_<This::Const, Msg, Output = NextConst>,
    This::Constructor: AllowOp<Op>,
    This: SetConstParam_<NextConst>,
    This: ConstLayoutIndependent<SetConstParam<This, NextConst>>,
{
    type NextConst = NextConst;
    type NextSelf = SetConstParam<This, NextConst>;
}
