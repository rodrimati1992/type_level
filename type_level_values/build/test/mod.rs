pub mod disabled_enabled_iter;

// use self::disabled_enabled_iter::DisabledEnabled;

#[allow(unused_imports)]
use core_extensions::{Void,SelfOps};

use std::io::Write as ioWrite;
use std::{iter,slice,env, fs, io, path};


#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub enum VariantKind{
    Braced,
    Tupled,
    Unit,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Privacy{
    Public,
    PrivateField,
}


#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub enum StructOrEnum{
    Struct,
    Enum,
}



bitflags! {
    pub struct EnabledImpl: u32 {
        const EMPTY=0;
        const CONST_EQ =0b0000_0001;
        const CONST_ORD=0b0000_0010;
        const CONST_EQ_ORD=0b0000_0011;
    }
}

impl Default for EnabledImpl{
    fn default()->Self{
        EnabledImpl::empty()
    }
}


#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum ConstOrRunt{
    Const,
    Runt,
}


fn get_typename(
    s_or_e:StructOrEnum,
    var_kind:VariantKind,
    privacy:Privacy,
    c_or_r:ConstOrRunt,
    impls:EnabledImpl,
)->String {
    let mut name=String::new();

    if let (Privacy::PrivateField,ConstOrRunt::Const)=(privacy,c_or_r) {
        name.push_str("New");
    }

    if c_or_r==ConstOrRunt::Const && s_or_e==StructOrEnum::Struct {
        name.push_str("Const");
    }

    name.push_str(match s_or_e {
        StructOrEnum::Struct=>"Struct_",
        StructOrEnum::Enum=>"Enum_",
    });
    name.push_str(match var_kind {
        VariantKind::Unit  =>"Unit_",
        VariantKind::Braced=>"Braced_",
        VariantKind::Tupled=>"Tupled_",
    });
    if impls.contains(EnabledImpl::CONST_EQ ) {
        name.push_str("Eq_");
    }
    if impls.contains(EnabledImpl::CONST_ORD) {
        name.push_str("Ord_");
    }
    match privacy {
        Privacy::PrivateField=>name.push_str("Priv"),
        Privacy::Public=>{},
    }

    name
}


pub type ItemType=(StructOrEnum,VariantKind,Privacy,EnabledImpl);

pub static TEST_CASES:&'static [ItemType]={
    use self::StructOrEnum as SOE;
    use self::EnabledImpl as EI;
    use self::VariantKind as VK;
    &[
        (SOE::Struct,VK::Braced,Privacy::Public      ,EI::EMPTY),
        (SOE::Struct,VK::Braced,Privacy::Public      ,EI::CONST_EQ),
        (SOE::Struct,VK::Braced,Privacy::Public      ,EI::CONST_ORD),
        (SOE::Struct,VK::Braced,Privacy::Public      ,EI::CONST_EQ_ORD),
        (SOE::Enum  ,VK::Braced,Privacy::Public      ,EI::EMPTY),
        (SOE::Enum  ,VK::Braced,Privacy::Public      ,EI::CONST_EQ),
        (SOE::Enum  ,VK::Braced,Privacy::Public      ,EI::CONST_ORD),
        (SOE::Enum  ,VK::Braced,Privacy::Public      ,EI::CONST_EQ_ORD),
        (SOE::Struct,VK::Braced,Privacy::PrivateField,EI::CONST_EQ),
        (SOE::Struct,VK::Tupled,Privacy::PrivateField,EI::CONST_EQ_ORD),
        (SOE::Struct,VK::Tupled,Privacy::Public      ,EI::CONST_EQ_ORD),
        (SOE::Enum  ,VK::Tupled,Privacy::Public      ,EI::CONST_EQ_ORD),
        (SOE::Struct,VK::Unit  ,Privacy::Public      ,EI::CONST_EQ_ORD),
        (SOE::Enum  ,VK::Unit  ,Privacy::Public      ,EI::CONST_EQ_ORD),
    ]
};

fn type_impls_permutations()->iter::Cloned<slice::Iter<'static,ItemType>>{
    TEST_CASES.iter().cloned()
}



fn impls_test<W:ioWrite>(mut w:W)->io::Result<()> {
    writeln!(w,"{}","
type TestEq<L,R,Val>=( 
    AssertEq<ConstEq<L,R>,Val> ,
    AssertEq<ConstEq<R,L>,Val> ,
);

type TestOrd<L,R,Val>=( 
    AssertEq<ConstOrd<L,R>,Val> ,
    AssertEq<ConstOrd<R,L>,Reverse<Val>> ,
);

type TestSetField<This,Field,Val,NewThis>=
    TestSetFieldHelper<
        SetField<This,Field,Val>,
        Field,
        Val,
        NewThis,
    >;
    
type TestSetFieldHelper<NewThis,Field,Val,Expected>=(
    AssertEq<NewThis,Expected>,
    AssertEq<GetField<NewThis,Field>,Val>,
);

type TestGetFR<This,Field,Runt,Val>=
    AssertEq<GetFieldRuntime<This,Field,Runt>,Val>;
    
type TestGetF<This,Field,Val>=(
    AssertEq<GetField<This,Field>,Val>,
    AssertEq<<This as Index<Field>>::Output,ConstWrapper<Val>>,
);
    

")?;
    for (soe,var_kind,privacy,impls) in type_impls_permutations() {
        let name=get_typename(soe,var_kind,privacy,ConstOrRunt::Const,impls);
        let deriving_type=get_typename(soe,var_kind,privacy,ConstOrRunt::Runt,impls);
        let consttype=format!("{}Type",deriving_type);
        let pre_priv=match privacy {
            Privacy::PrivateField=>"priv_",
            Privacy::Public=>"",
        };
        
        let co =if var_kind==VariantKind::Unit {"//"}else{""};
        let col=if var_kind==VariantKind::Unit {"/*"}else{""};
        let cor=if var_kind==VariantKind::Unit {"*/"}else{""};

        writeln!(w,"
            #[allow(non_snake_case)]
            #[test]
            fn test_{name}(){{
            use self::type_level_{deriving}::*;
        ",name=name,deriving=deriving_type)?;
    
        match soe {
            StructOrEnum::Enum=>{
                writeln!(w,"
                    type Open0=Open {col} <U0> {cor};
                    type Open1=Open {col} <U10> {cor};
                    type Open2=Open {col} <U100> {cor};
                ", col=col,cor=cor )?;
            }
            StructOrEnum::Struct=>{
                writeln!(w,"
                    type Val0={name} {col} <U0,U0> {cor};
                    type Val1={name} {col} <U0,U10> {cor};
                    type Val2={name} {col} <U10,U0> {cor};
                    type Val3={name} {col} <U10,U10> {cor};
                ", col=col,cor=cor , name=name)?;
            }
        }

        
        match (impls.contains(EnabledImpl::CONST_EQ ),var_kind,soe) {
            (false,_,_)=>{},
            (true,VariantKind::Unit,StructOrEnum::Struct)=>{
                writeln!(w,"let _:TestEq<{name},{name},True>;",name=name)?;
            }
            (true ,_,StructOrEnum::Struct)=>{
                writeln!(w,"

    let _:TestEq<Val0,Val0,True>;
    let _:TestEq<Val0,Val1,False>;
    let _:TestEq<Val0,Val2,False>;
    let _:TestEq<Val0,Val3,False>;
                    
    let _:TestEq<Val1,Val1,True>;
    let _:TestEq<Val1,Val2,False>;
    let _:TestEq<Val1,Val3,False>;

    let _:TestEq<Val2,Val2,True>;
    let _:TestEq<Val2,Val3,False>;
                    
    let _:TestEq<Val3,Val3,True>;

")?;
            },
            (true ,_,StructOrEnum::Enum  )=>{
                writeln!(w,"

    let _:TestEq<HalfOpen,HalfOpen,True>;
    let _:TestEq<HalfOpen,Open0,False>;
    let _:TestEq<HalfOpen,Open1,False>;
    let _:TestEq<HalfOpen,Open2,False>;
    let _:TestEq<HalfOpen,Closed,False>;

    let _:TestEq<Open0,Closed,False>;                
    let _:TestEq<Open1,Closed,False>;
    let _:TestEq<Open2,Closed,False>;
                    
    let _:TestEq<Open0,Open0,True>;
    let _:TestEq<Open1,Open1,True>;
    let _:TestEq<Open2,Open2,True>;

    {co}let _:TestEq<Open0,Open1,False>;
    {co}let _:TestEq<Open0,Open2,False>;
    {co}let _:TestEq<Open1,Open2,False>;
",
    co=co
)?;
            },
        }

        match (impls.contains(EnabledImpl::CONST_ORD ),var_kind,soe) {
            (false,_,_)=>{},
            (true,VariantKind::Unit,StructOrEnum::Struct)=>{
                writeln!(w,"let _:TestOrd<{name},{name},Equal_>;",name=name)?;
            }
            (true,_,StructOrEnum::Struct)=>{
                writeln!(w,"
    let _:TestOrd<Val0,Val0,Equal_>;
    let _:TestOrd<Val0,Val1,Less_>;
    let _:TestOrd<Val0,Val2,Less_>;
    let _:TestOrd<Val0,Val3,Less_>;

    let _:TestOrd<Val1,Val1,Equal_>;
    let _:TestOrd<Val1,Val2,Less_>;
    let _:TestOrd<Val1,Val3,Less_>;

    let _:TestOrd<Val2,Val2,Equal_>;
    let _:TestOrd<Val2,Val3,Less_>;

    let _:TestOrd<Val3,Val3,Equal_>;

")?;
            },
            (true,_,StructOrEnum::Enum)=>{
                writeln!(w,"
    
    let _:TestOrd<HalfOpen,HalfOpen,Equal_>;
    let _:TestOrd<HalfOpen,Open0,Less_>;
    let _:TestOrd<HalfOpen,Open1,Less_>;
    let _:TestOrd<HalfOpen,Open2,Less_>;
    let _:TestOrd<HalfOpen,Closed,Less_>;

    let _:TestOrd<Open0,Closed,Less_>;
    let _:TestOrd<Open1,Closed,Less_>;
    let _:TestOrd<Open2,Closed,Less_>;

    let _:TestOrd<Open0,Open0,Equal_>;
    let _:TestOrd<Open1,Open1,Equal_>;
    let _:TestOrd<Open2,Open2,Equal_>;
    {co}let _:TestOrd<Open0,Open1,Less_>;
    {co}let _:TestOrd<Open0,Open2,Less_>;
    {co}let _:TestOrd<Open1,Open2,Less_>;

",
    co=co
)?;

            },
        }
        match soe {
            StructOrEnum::Struct=>{
                let (const_fields,runt_fields,assoc_fields)=match var_kind {
                     VariantKind::Unit
                    |VariantKind::Tupled=>(("U0","U1"),("0","1"),("field_0","field_1")),
                    VariantKind::Braced=>(("x","y"),("x","y"),("x","y")),
                };
                writeln!(w,"

                    let _:AssertEq<
                        GetDiscrOf<Val1>,
                        Discriminant<variants::{deriving}_Variant,{consttype},U0>
                    >;

                    {co}let _:AssertEq<GetVariantOf<Val1>,variants::{deriving}_Variant>;

                    {co}let _:TestSetField<Val1,fields::{x},U7,{name}<U7,U10>>;
                    {co}let _:TestSetField<Val1,fields::{y},U7,{name}<U0,U7>>;
                    {co}let _:AssertEq<SetField<Val1,fields::All,U7>,{name}<U7,U7>>;
                    {co}
                    {co}let _:TestGetF<Val1,fields::{x},U0 >;
                    {co}let _:TestGetF<Val1,fields::{y},U10>;
                    {co}
                    {co}let _:TestGetFR<Val1,fields::{x},{deriving}<u32>,bool>;
                    {co}let _:TestGetFR<Val1,fields::{y},{deriving}<u32>,Option<u32>>;

                    let _:AssertEq<
                        <{deriving} {col}<u32>{cor} as IntoConstType_>::ToConst , 
                        {consttype} 
                    >;

                    assert_eq_into!(
                        {name} {col} <False,Some_<U0>> {cor}, 
                        {deriving} {col} {{ {rx}:false,{ry}:Some(0u32) }} {cor}
                    );
                    assert_eq_into!(
                        {name} {col} <True,Some_<U0>> {cor},
                        {deriving} {col} {{ {rx}:true,{ry}:Some(0u32) }} {cor}
                    );
                    assert_eq_into!(
                        {name} {col} <True,None_> {cor},
                        {deriving} {col} {{ {rx}:true,{ry}:None::<()> }} {cor}
                    );

                    let _:AssertEq<ConstTypeOf<Val0>,{consttype} >;
                    let _:AssertEq<ConstTypeOf<Val1>,{consttype} >;
                    let _:AssertEq<ConstTypeOf<Val2>,{consttype} >;
                    let _:AssertEq<ConstTypeOf<Val3>,{consttype} >;


                    {co}let _:AssertEq<<Val1 as {deriving}Trait>::{at_x} , U0>;
                    {co}let _:AssertEq<<Val1 as {deriving}Trait>::{ppriv}{at_y} , U10>;
                    {co}
                    {co}let _:AssertEq<<Val1 as {deriving}WithRuntime<u32>>::rt_{at_x} , U0>;
                    {co}let _:AssertEq<
                    {co}    <Val1 as {deriving}WithRuntime<u32>>::rt_{ppriv}{at_y} , 
                    {co}    U10
                    {co}>;

                    let _:AssertEq<AsTList<Val0>,tlist![{col} U0,U0 {cor}]>;
                    let _:AssertEq<AsTList<Val1>,tlist![{col} U0,U10 {cor}]>;
                    let _:AssertEq<AsTList<Val2>,tlist![{col} U10,U0 {cor}]>;
                    let _:AssertEq<AsTList<Val3>,tlist![{col} U10,U10 {cor}]>;

                    let _:AssertEq<
                        <{deriving}_Uninit as InitializationValues>::Uninitialized,
                        {name} {col} <
                            UninitField<fields::{x}> , 
                            UninitField<fields::{y}> 
                        > {cor}
                    >;

                    let _:AssertEq<
                        <{deriving}_Uninit as InitializationValues>::Initialized,
                        {name} {col} < IsInitField<fields::{x}> , IsInitField<fields::{y}> > {cor}
                    >;

                    ",   
                    col=col,cor=cor,co=co,
                    x=const_fields.0,
                    y=const_fields.1,
                    at_x=assoc_fields.0,
                    at_y=assoc_fields.1,
                    rx=runt_fields.0,
                    ry=runt_fields.1,
                    name=name,
                    deriving=deriving_type,
                    consttype=consttype,
                    ppriv=pre_priv,
                )?;


            }
            StructOrEnum::Enum=>{
                writeln!(w,"

                    let _:AssertEq<
                        GetDiscrOf<HalfOpen>,
                        Discriminant<variants::HalfOpen_Variant,{consttype},U0>
                    >;
                    let _:AssertEq<
                        GetDiscrOf<Open0>,
                        Discriminant<variants::Open_Variant,{consttype},U1>
                    >;
                    let _:AssertEq<
                        GetDiscrOf<Open1>,
                        Discriminant<variants::Open_Variant,{consttype},U1>
                    >;
                    let _:AssertEq<
                        GetDiscrOf<Closed>,
                        Discriminant<variants::Closed_Variant,{consttype},U2>
                    >;

                    let _:AssertEq<GetVariantOf<HalfOpen>,variants::HalfOpen_Variant>;
                    let _:AssertEq<GetVariantOf<Open0>,variants::Open_Variant>;
                    let _:AssertEq<GetVariantOf<Open1>,variants::Open_Variant>;
                    let _:AssertEq<GetVariantOf<Closed>,variants::Closed_Variant>;

                    {co}let _:TestSetField<Open0,fields::remaining,U7,Open<U7>>;
                    {co}let _:AssertEq<SetField<Open0,fields::All,U7>,Open<U7>>;
                    {co}let _:TestSetField<
                    {co}    Open0,
                    {co}    fields::remaining,
                    {co}    U3,
                    {co}    construct!(Open_Uninit=>fields::remaining=U3)
                    {co}>;
                    {co}
                    {co}let _:TestGetF<Open2,fields::remaining,U100 >;
                    {co}
                    {co}let _:TestGetFR<Open0,fields::remaining,{deriving},u32>;

                    let _:AssertEq<<{deriving} as IntoConstType_>::ToConst , {consttype} >;

                    assert_eq_into!(
                        HalfOpen,
                        {deriving}::HalfOpen
                    );
                    assert_eq_into!(
                        Open0,
                        {deriving}::Open {co} {{remaining:0}}
                    );
                    assert_eq_into!(
                        Open1,
                        {deriving}::Open {co} {{remaining:10}}
                    );
                    assert_eq_into!(
                        Open2,
                        {deriving}::Open {co} {{remaining:100}}
                    );
                    assert_eq_into!(
                        Closed,
                        {deriving}::Closed
                    );

                    let _:AssertEq<ConstTypeOf<HalfOpen>,{consttype} >;
                    let _:AssertEq<ConstTypeOf<Open0>,{consttype} >;
                    let _:AssertEq<ConstTypeOf<Open1>,{consttype} >;
                    let _:AssertEq<ConstTypeOf<Open2>,{consttype} >;
                    let _:AssertEq<ConstTypeOf<Closed>,{consttype} >;


                    {co}let _:AssertEq<<Open0 as OpenTrait>::remaining , U0>;
                    {co}let _:AssertEq<<Open1 as OpenTrait>::remaining , U10>;
                    {co}let _:AssertEq<<Open2 as OpenTrait>::remaining , U100>;

                    fn assertions()
                    where  
                        Open0:OpenTrait+OpenWithRuntime,
                        Open1:OpenTrait+OpenWithRuntime,
                        Open2:OpenTrait+OpenWithRuntime,
                        HalfOpen:HalfOpenTrait+HalfOpenWithRuntime,
                        Closed:ClosedTrait+ClosedWithRuntime,
                    {{}}

                    assertions();

                    {co}let _:AssertEq<<Open0 as OpenWithRuntime>::rt_remaining , U0>;
                    {co}let _:AssertEq<<Open1 as OpenWithRuntime>::rt_remaining , U10>;
                    {co}let _:AssertEq<<Open2 as OpenWithRuntime>::rt_remaining , U100>;

                    let _:AssertEq<AsTList<HalfOpen>,tlist![]>;
                    let _:AssertEq<AsTList<Open0>,tlist![{col}U0  {cor}]>;
                    let _:AssertEq<AsTList<Open1>,tlist![{col}U10 {cor}]>;
                    let _:AssertEq<AsTList<Open2>,tlist![{col}U100{cor}]>;
                    let _:AssertEq<AsTList<Closed>,tlist![]>;


                    let _:AssertEq<
                        <HalfOpen_Uninit as InitializationValues>::Initialized,
                        HalfOpen
                    >;
                    let _:AssertEq<
                        <HalfOpen_Uninit as InitializationValues>::Uninitialized,
                        HalfOpen
                    >;

                    let _:AssertEq<
                        <Open_Uninit as InitializationValues>::Uninitialized,
                        Open {col} <UninitField<fields::remaining>> {cor}
                    >;

                    let _:AssertEq<
                        <Open_Uninit as InitializationValues>::Initialized,
                        Open {col} <IsInitField<fields::remaining>> {cor}
                    >;

                    let _:AssertEq<
                        <Closed_Uninit as InitializationValues>::Initialized,
                        Closed
                    >;
                    let _:AssertEq<
                        <Closed_Uninit as InitializationValues>::Uninitialized,
                        Closed
                    >;


                    ",
                    co=co ,col=col,cor=cor,
                    deriving=deriving_type,
                    consttype=consttype,
                )?;

            }
        };
        write!(w,"}}\n\n")?;
    }
    Ok(())
}



fn type_decls<W:ioWrite>(mut w:W)->io::Result<()> {
    for (soe,var_kind,privacy,impls) in type_impls_permutations() {
        let deriving_type=get_typename(soe,var_kind,privacy,ConstOrRunt::Runt,impls);
        
        #[allow(unused_variables)]
        let co =if var_kind==VariantKind::Unit {"//"}else{""};
        let col=if var_kind==VariantKind::Unit {"/*"}else{""};
        let cor=if var_kind==VariantKind::Unit {"*/"}else{""};

        write!(w,"
#[derive(TypeLevel)]
//#[typelevel(print_derive)]
#[typelevel(derive_str)]
#[typelevel(derive(
")?;
        if impls.contains(EnabledImpl::CONST_EQ ) {
            write!(w,"ConstEq,")?; 
        }
        if impls.contains(EnabledImpl::CONST_ORD) {
            write!(w,"ConstOrd,")?;
        }
        writeln!(w,"))]")?;
        match soe {
            StructOrEnum::Struct=>{
                let priv_=match privacy {
                    Privacy::Public=>"pub",
                    Privacy::PrivateField=>"",
                };
                match var_kind {
                    VariantKind::Braced=>
                        writeln!(w,"
                            #[derive(Debug,PartialEq,Eq)]
                            pub struct {deriving}<T> {{
                                pub x:bool,
                                {priv_} y:Option<T>,
                            }}
                            type Alias{deriving}={deriving}<()>;
                            ",
                            priv_=priv_,
                            deriving=deriving_type
                        )?,
                    VariantKind::Tupled=>
                        writeln!(w,"
                            #[derive(Debug,PartialEq,Eq)]
                            pub struct {deriving}<T> (
                                pub bool,{priv_} Option<T>
                            );
                            type Alias{deriving}={deriving}<()>;
                            ",
                            priv_=priv_,
                            deriving=deriving_type
                        )?,
                    VariantKind::Unit=>
                        writeln!(w,"
                            #[derive(Debug,PartialEq,Eq)]
                            pub struct {deriving};
                            type Alias{deriving}={deriving};
                            ",
                            deriving=deriving_type
                        )?,
                }

                if privacy==Privacy::PrivateField {
                    let type_alias=
                        get_typename(soe,var_kind,Privacy::PrivateField,ConstOrRunt::Const,impls);
                    let (x,y)=match var_kind {
                        VariantKind::Braced=>("x","y"),
                         VariantKind::Unit
                        |VariantKind::Tupled=>("U0","U1"),
                    };
                    write!(w,"\
                        pub type {alias}{col}<X,Y>{cor}=construct!(\n\
                            self::type_level_{deriving}::{deriving}_Uninit {col} =>
                            self::type_level_{deriving}::fields::{x} =X ,\n\
                            self::type_level_{deriving}::fields::{y} =Y ,\n {cor}
                        );\n
                        ",
                        col=col,cor=cor,
                        alias=type_alias,
                        x=x,y=y,
                        deriving=deriving_type,
                    )?;
                    writeln!(w,"")?;
                }
            }
            StructOrEnum::Enum=>{
                writeln!(w,"
                    #[derive(Debug,PartialEq,Eq)]
                    pub enum {deriving} {{
                        HalfOpen,
                        Open {col} {{remaining:u32}} {cor},
                        Closed,
                    }}\n
                    type Alias{deriving}={deriving};
                ",
                col=col,cor=cor,
                deriving=deriving_type
                )?;
            }
        };
    }
    Ok(())
}

fn imports<W:ioWrite>(mut w:W)->io::Result<()> {
    write!(w,"

#[macro_use]
extern crate derive_type_level;
#[macro_use]
extern crate type_level_values;
#[macro_use]
extern crate derive_type_level_lib;
extern crate syn;
#[macro_use]
extern crate core_extensions;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate lazy_static;


#[allow(dead_code)]
mod tests{{

use std::ops::Index;

use type_level_values::prelude::*;
use type_level_values::ops::*;
use type_level_values::collection_ops::{{Reverse}};
use type_level_values::std_types::cmp_ordering::{{Less_,Equal_,Greater_}};
use type_level_values::discriminant::{{GetDiscrOf,GetVariantOf,Discriminant}};
use type_level_values::field_traits::{{GetField,SetField,GetFieldRuntime}};
use type_level_values::initialization::{{
    InitializationValues,
    IsInitField,
    UninitField,
}};


macro_rules! assert_eq_into{{
    ( $tylevel:ty , $runt:expr ) => ({{
        let runtime_val=$runt;
        assert_eq!( 
            <$tylevel as IntoRuntime<_>>::to_runtime() , 
            runtime_val
        );
        assert_eq!( 
            <$tylevel as IntoConstant<_>>::VALUE , 
            runtime_val
        );
    }})
}}

    ")?;
    Ok(())
}


pub fn build_tests()->io::Result<()>{
    let out_dir = env::var("OUT_DIR").unwrap();
    let test_path = path::Path::new(&out_dir).join("struct_enum_tests.rs");
    let test_file = fs::File::create(&test_path)?;
    let mut test_file = io::BufWriter::new(test_file);

    imports(&mut test_file)?;
    type_decls(&mut test_file)?;
    impls_test(&mut test_file)?;
    
    write!(test_file,"\n}}")?;

    Ok(())
}