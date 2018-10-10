pub mod disabled_enabled_iter;

use self::disabled_enabled_iter::DisabledEnabled;

#[allow(unused_imports)]
use core_extensions::{Void,SelfOps};


use itertools::{ConsTuples,Product};


use std::io::Write as ioWrite;
use std::{iter,slice,env, fs, io, path};

bitflags! {
    pub struct EnabledImpl: u32 {
        const CONST_EQ =1<<0;
        const CONST_ORD=1<<1;
    }
}

impl Default for EnabledImpl{
    fn default()->Self{
        EnabledImpl::empty()
    }
}


#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum StructOrEnum{
    Struct,
    Enum,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum ConstOrRunt{
    Const,
    Runt,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Privacy{
    Public,
    PrivateField,
}


fn get_typename(
    s_or_e:StructOrEnum,
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
        StructOrEnum::Struct=>"Struct",
        StructOrEnum::Enum=>"Enum",
    });
    if impls.contains(EnabledImpl::CONST_EQ ) {
        name.push_str("Eq");
    }
    if impls.contains(EnabledImpl::CONST_ORD) {
        name.push_str("Ord");
    }
    match privacy {
        Privacy::PrivateField=>name.push_str("Priv"),
        Privacy::Public=>{},
    }

    name
}


#[allow(non_upper_case_globals)]
static StructOrEnum_VARIANTS:[StructOrEnum;2]=[StructOrEnum::Struct,StructOrEnum::Enum];

type ImplProduct=ConsTuples<
    Product<
        Product<
            iter::Cloned<slice::Iter<'static, StructOrEnum>>, 
            DisabledEnabled<EnabledImpl>
        >,
        DisabledEnabled<EnabledImpl>
    >, 
    ((StructOrEnum, EnabledImpl), EnabledImpl)
>;


pub struct OrImpls<I>{
    iter:I,
}

impl<I> OrImpls<I>{
    pub fn new(iter:I)->Self{
        Self{iter}
    }
}

pub type ItemType=(StructOrEnum,Privacy,EnabledImpl);

impl<I> Iterator for OrImpls<I>
where I:Iterator<Item=(StructOrEnum,EnabledImpl,EnabledImpl)>,
{
    type Item=ItemType;

    fn next(&mut self)->Option<Self::Item>{
        self.iter.next().map(|(soe,x0,x1)|{
            (soe,Privacy::Public,(x0|x1))
        })
    }
}

fn type_impls_permutations()->iter::Chain<OrImpls<ImplProduct>,iter::Once<ItemType>>{
    OrImpls::new(iproduct!(
        StructOrEnum_VARIANTS.iter().cloned(),
        DisabledEnabled::new(EnabledImpl::CONST_EQ),
        DisabledEnabled::new(EnabledImpl::CONST_ORD)
    )).chain(iter::once(
        ( StructOrEnum::Struct ,Privacy::PrivateField ,EnabledImpl::CONST_EQ )
    ))
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
    for (soe,privacy,impls) in type_impls_permutations() {
        let name=get_typename(soe,privacy,ConstOrRunt::Const,impls);
        let deriving_type=get_typename(soe,privacy,ConstOrRunt::Runt,impls);
        let consttype=format!("{}Type",deriving_type);
        let pre_priv=match privacy {
            Privacy::PrivateField=>"priv_",
            Privacy::Public=>"",
        };
        
        writeln!(w,"#[allow(non_snake_case)]")?;
        writeln!(w,"#[test]")?;
        writeln!(w,"fn test_{name}(){{",name=name)?;
        writeln!(w,"use self::type_level_{}::*;",deriving_type)?;
        
        match soe {
            StructOrEnum::Enum=>{
                writeln!(w,"type Open0=Open<U0>;")?;
                writeln!(w,"type Open1=Open<U10>;")?;
                writeln!(w,"type Open2=Open<U100>;")?;
            }
            StructOrEnum::Struct=>{
                writeln!(w,"type Val0={name}<U0,U0>;",name=name)?;

                writeln!(w,"type Val1={name}<U0,U10>;",name=name)?;
                
                writeln!(w,"type Val2={name}<U10,U0>;",name=name)?;
                
                writeln!(w,"type Val3={name}<U10,U10>;",name=name)?;
            }
        }

        
        match (impls.contains(EnabledImpl::CONST_EQ ),soe) {
            (false,_)=>{},
            (true ,StructOrEnum::Struct)=>{
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
            (true ,StructOrEnum::Enum  )=>{
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
    let _:TestEq<Open0,Open1,False>;
    let _:TestEq<Open0,Open2,False>;
    let _:TestEq<Open1,Open1,True>;
    let _:TestEq<Open1,Open2,False>;
    let _:TestEq<Open2,Open2,True>;

")?;
            },
        }

        match (impls.contains(EnabledImpl::CONST_ORD ),soe) {
            (false,_)=>{},
            (true,StructOrEnum::Struct)=>{
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
            (true,StructOrEnum::Enum)=>{
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
    let _:TestOrd<Open0,Open1,Less_>;
    let _:TestOrd<Open0,Open2,Less_>;
    let _:TestOrd<Open1,Open1,Equal_>;
    let _:TestOrd<Open1,Open2,Less_>;
    let _:TestOrd<Open2,Open2,Equal_>;

")?;

            },
        }
        match soe {
            StructOrEnum::Struct=>{
                writeln!(w,"

let _:AssertEq<
    GetDiscrOf<Val1>,
    Discriminant<variants::{deriving}_Variant,{consttype},U0>
>;

let _:AssertEq<GetVariantOf<Val1>,variants::{deriving}_Variant>;

let _:TestSetField<Val1,fields::x,U7,{name}<U7,U10>>;
let _:TestSetField<Val1,fields::y,U7,{name}<U0,U7>>;

let _:TestGetF<Val1,fields::x,U0 >;
let _:TestGetF<Val1,fields::y,U10>;

let _:TestGetFR<Val1,fields::x,{deriving}<u32>,bool>;
let _:TestGetFR<Val1,fields::y,{deriving}<u32>,Option<u32>>;

let _:AssertEq<<{deriving}<u32> as IntoConstType_>::ToConst , {consttype} >;

assert_eq_into!(
    {name}<False,Some_<U0>>,
    {deriving}{{ x:false,y:Some(0u32) }}
);
assert_eq_into!(
    {name}<True,Some_<U0>>,
    {deriving}{{ x:true,y:Some(0u32) }}
);
assert_eq_into!(
    {name}<True,None_>,
    {deriving}{{ x:true,y:None::<()> }}
);

let _:AssertEq<ConstTypeOf<Val0>,{consttype} >;
let _:AssertEq<ConstTypeOf<Val1>,{consttype} >;
let _:AssertEq<ConstTypeOf<Val2>,{consttype} >;
let _:AssertEq<ConstTypeOf<Val3>,{consttype} >;


let _:AssertEq<<Val1 as {deriving}Trait>::x , U0>;
let _:AssertEq<<Val1 as {deriving}Trait>::{ppriv}y , U10>;

let _:AssertEq<<Val1 as {deriving}WithRuntime<u32>>::rt_x , U0>;
let _:AssertEq<<Val1 as {deriving}WithRuntime<u32>>::rt_{ppriv}y , U10>;

let _:AssertEq<AsTList<Val0>,tlist![U0,U0]>;
let _:AssertEq<AsTList<Val1>,tlist![U0,U10]>;
let _:AssertEq<AsTList<Val2>,tlist![U10,U0]>;
let _:AssertEq<AsTList<Val3>,tlist![U10,U10]>;

let _:AssertEq<
    <{deriving}_Uninit as InitializationValues>::Uninitialized,
    {name}< UninitField<fields::x> , UninitField<fields::y> >
>;

let _:AssertEq<
    <{deriving}_Uninit as InitializationValues>::Initialized,
    {name}< IsInitField<fields::x> , IsInitField<fields::y> >
>;

",   
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

let _:TestSetField<Open0,fields::remaining,U7,Open<U7>>;
let _:TestSetField<Open0,fields::remaining,U3,construct!(Open_Uninit=>fields::remaining=U3)>;

let _:TestGetF<Open2,fields::remaining,U100 >;

let _:TestGetFR<Open0,fields::remaining,{deriving},u32>;

let _:AssertEq<<{deriving} as IntoConstType_>::ToConst , {consttype} >;

assert_eq_into!(
    HalfOpen,
    {deriving}::HalfOpen
);
assert_eq_into!(
    Open0,
    {deriving}::Open{{remaining:0}}
);
assert_eq_into!(
    Open1,
    {deriving}::Open{{remaining:10}}
);
assert_eq_into!(
    Open2,
    {deriving}::Open{{remaining:100}}
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


let _:AssertEq<<Open0 as OpenTrait>::remaining , U0>;
let _:AssertEq<<Open1 as OpenTrait>::remaining , U10>;
let _:AssertEq<<Open2 as OpenTrait>::remaining , U100>;

fn assert_closed()
where  
    HalfOpen:HalfOpenTrait+HalfOpenWithRuntime,
    Closed:ClosedTrait+ClosedWithRuntime,
{{}}

assert_closed();

let _:AssertEq<<Open0 as OpenWithRuntime>::rt_remaining , U0>;
let _:AssertEq<<Open1 as OpenWithRuntime>::rt_remaining , U10>;
let _:AssertEq<<Open2 as OpenWithRuntime>::rt_remaining , U100>;

let _:AssertEq<AsTList<HalfOpen>,tlist![]>;
let _:AssertEq<AsTList<Open0>,tlist![U0]>;
let _:AssertEq<AsTList<Open1>,tlist![U10]>;
let _:AssertEq<AsTList<Open2>,tlist![U100]>;
let _:AssertEq<AsTList<Closed>,tlist![]>;


let _:AssertEq<<HalfOpen_Uninit as InitializationValues>::Initialized,HalfOpen>;
let _:AssertEq<<HalfOpen_Uninit as InitializationValues>::Uninitialized,HalfOpen>;

let _:AssertEq<
    <Open_Uninit as InitializationValues>::Uninitialized,
    Open< UninitField<fields::remaining> >
>;

let _:AssertEq<
    <Open_Uninit as InitializationValues>::Initialized,
    Open< IsInitField<fields::remaining> >
>;

let _:AssertEq<<Closed_Uninit as InitializationValues>::Initialized,Closed>;
let _:AssertEq<<Closed_Uninit as InitializationValues>::Uninitialized,Closed>;


",
                    //name=name,
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
    for (soe,privacy,impls) in type_impls_permutations() {
        let deriving_type=get_typename(soe,privacy,ConstOrRunt::Runt,impls);
        
        write!(w,"
#[derive(TypeLevel)]
//#[typelevel(print_derive)]
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
                writeln!(w,"#[derive(Debug,PartialEq,Eq)]")?;
                writeln!(w,"pub struct {deriving}<T> {{",deriving=deriving_type)?;
                writeln!(w,"    pub x:bool,")?;
                writeln!(w,"    {} y:Option<T>,",priv_)?;
                writeln!(w,"}}\n")?;

                if privacy==Privacy::PrivateField {
                    let type_alias=
                        get_typename(soe,Privacy::PrivateField,ConstOrRunt::Const,impls);
                        
                    writeln!(w,"\
                        pub type {alias}<X,Y>=construct!(\n\
                            self::type_level_{deriving}::{deriving}_Uninit=>
                            self::type_level_{deriving}::fields::x =X ,\n\
                            self::type_level_{deriving}::fields::y =Y ,\n\
                        );\n
                    ",
                        deriving=deriving_type,
                        alias=type_alias,
                    )?;
                }
            }
            StructOrEnum::Enum=>{
                writeln!(w,"#[derive(Debug,PartialEq,Eq)]")?;
                writeln!(w,"pub enum {deriving} {{",deriving=deriving_type)?;
                writeln!(w,"    HalfOpen,")?;
                writeln!(w,"    Open{{remaining:u32}},")?;
                writeln!(w,"    Closed,")?;
                writeln!(w,"}}\n")?;
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