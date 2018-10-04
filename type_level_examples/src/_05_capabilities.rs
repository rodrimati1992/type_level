//! This example demonstrates a zero overhead builder using a Const-parameter
//! to track initialization of the fields.
//!
//!
//!

use type_level_values::field_traits::*;
use type_level_values::prelude::*;

use std::ffi::OsStr;
use std::io;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, TypeLevel)]
pub enum Capability {
    EnabledCap,
    DisabledCap,
}

pub use self::type_level_Capability::{DisabledCap, EnabledCap};

/// Possible capabilities that a type may support.
#[derive(TypeLevel)]
#[allow(dead_code)]
pub struct Capabilities {
    pub filesystem: Capability,
    pub execute_command: Capability,
    pub networking: Capability,

    pub destroy_computer: Capability,
    pub send_passwords: Capability,
    pub drop_tables: Capability,
}

use self::type_level_Capabilities::{
    fields as cap_fields, CapabilitiesTrait, Capabilities_Uninit,
};

pub type AllCapabilities = SetField<Capabilities_Uninit, cap_fields::All, EnabledCap>;
pub type NoCapabilities = SetField<Capabilities_Uninit, cap_fields::All, DisabledCap>;

////////////////////////////////////////////////////////////////////////////////////////////

pub trait FieldAccessor<Field> {
    type FieldType;

    fn access_ref(&self, _: Field) -> &Self::FieldType;
    fn access_mut(&mut self, _: Field) -> &mut Self::FieldType;
}

////////////////////////////////////////////////////////////////////////////////////////////

#[derive(ConstConstructor)]
#[cconstructor(
    extension_methods = "False",
    Type(name = "SideEffectful", doc = "oh hi"),
    ConstParam = "Caps"
)]
pub struct SideEffectfulInner<FS, EC, Caps>
where
    Caps: WrapperTrait,
{
    filesystem: FS,
    execute_command: EC,
    _capabilities: ConstWrapperFromTrait<Caps>,
}

macro_rules! capability_accessor {
    ( $(mut_accessor=$mut_accessor:ident,field=$field:ident,type=$field_type:ty;)* ) => {
        $(
            impl<FS,EC,Caps> FieldAccessor<cap_fields::$field> for SideEffectful<FS,EC,Caps>
            where
                FS:FileSystemOps,
                EC:ExecuteCommand,
                Caps:CapabilitiesTrait<$field=EnabledCap>,
            {
                type FieldType=$field_type;

                #[inline(always)]
                fn access_ref(&self,_:cap_fields::$field)->&Self::FieldType{
                    &self.$field
                }

                #[inline(always)]
                fn access_mut(&mut self,_:cap_fields::$field)->&mut Self::FieldType{
                    &mut self.$field
                }
            }
        )*
    }
}

impl<FS, EC, Caps> SideEffectful<FS, EC, Caps>
where
    FS: FileSystemOps,
    EC: ExecuteCommand,
    Caps: CapabilitiesTrait,
{
    pub fn new(filesystem: FS, execute_command: EC, _capabilities: PhantomWrapper<Caps>) -> Self {
        Self {
            filesystem,
            execute_command,
            _capabilities,
        }
    }
}

capability_accessor!{
    mut_accessor=filesystem_mut      , field=filesystem      , type=FS;
    mut_accessor=execute_command_mut , field=execute_command , type=EC;
}

///////////////////////////////////////////////////////////////////////////////////////////

const_method!{
    type ConstConstructor[FS,EC]=( SideEffectfulCC<FS,EC> )
    type AllowedConversions=( allowed_conversions::All )

    pub fn DisableCapability[Caps,Field](Caps,Field)
    where [ Caps:SetField_<Field,DisabledCap> ]
    {Caps::Output}
}

const_method!{
    type ConstConstructor[FS,EC]=( SideEffectfulCC<FS,EC> )
    type AllowedConversions=( allowed_conversions::All )

    // extension_method=True;

    pub fn DisableAll[Caps](Caps,())
    { DisableAll }
}

////////////////////////////////////////////////////////////////////////////////////////////

pub trait FileSystemOps {
    fn create_file(&self, path: &Path) -> io::Result<()>;
    fn delete_file(&self, path: &Path) -> io::Result<()>;
}

pub struct FakeFileSystemOps;

impl FileSystemOps for FakeFileSystemOps {
    fn create_file(&self, path: &Path) -> io::Result<()> {
        println!("created file:{}", path.to_string_lossy());
        Ok(())
    }
    fn delete_file(&self, path: &Path) -> io::Result<()> {
        println!("deleted file:{}", path.to_string_lossy());
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////

pub trait ExecuteCommand {
    fn execute_command<OS>(&self, command: &OsStr, args: &[OS]) -> io::Result<()>
    where
        OS: AsRef<OsStr>;
}

pub struct FakeExecuteCommand;

impl ExecuteCommand for FakeExecuteCommand {
    fn execute_command<OS>(&self, command: &OsStr, _args: &[OS]) -> io::Result<()>
    where
        OS: AsRef<OsStr>,
    {
        println!("executed command {}", command.to_string_lossy());
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////

fn requires_fs_ops<FS>(fs_ops: &FS)
where
    FS: FieldAccessor<cap_fields::filesystem>,
    FS::FieldType: FileSystemOps,
{
    let fs = fs_ops.access_ref(cap_fields::filesystem);
    fs.create_file("./hey-arnold.gif".as_ref()).unwrap();
}

fn requires_execute_command<FS>(fs_ops: &FS)
where
    FS: FieldAccessor<cap_fields::execute_command>,
    FS::FieldType: ExecuteCommand,
{
    let fs = fs_ops.access_ref(cap_fields::execute_command);
    fs.execute_command("rg".as_ref(), &["-h"]).unwrap();
}

////////////////////////////////////////////////////////////////////////////////////////////

pub fn main_ () {
    let side_effects =
        SideEffectful::new(FakeFileSystemOps, FakeExecuteCommand, AllCapabilities::PW);

    let fs = side_effects.access_ref(cap_fields::filesystem);
    fs.create_file("./insults.txt".as_ref()).unwrap();
    fs.delete_file("C:/windows/system32".as_ref()).unwrap();

    let exec_cmd = side_effects.access_ref(cap_fields::execute_command);
    exec_cmd.execute_command("grep".as_ref(), &["-h"]).unwrap();

    requires_fs_ops(&side_effects);
    // This won't compile because the "filesystem" capability is disabled for the reference.
    // requires_fs_ops(
    //     side_effects.mutparam_ref(DisableCapability,cap_fields::filesystem.wrap_msg()));

    requires_execute_command(&side_effects);
    // This won't compile because the "execute_command" capability is disabled for the reference.
    // requires_execute_command(
    //      side_effects.mutparam_ref(DisableCapability,cap_fields::execute_command.wrap_msg()));

    side_effects.mutparam_ref(DisableAll, Default::default());
}
