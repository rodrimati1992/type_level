use super::*;
use super::type_level_shared::*;



#[derive(TypeLevel)]
#[typelevel(derive_str)]
#[typelevel(items(
    GetDiscriminant(NoImpls),
    IntoConstType(NoImpls),
    IntoRuntime(NoImpls),
    AsTList(NoImpls),
))]
pub struct Tupled(
    u32,
    (),
);

#[derive(TypeLevel)]
#[typelevel(derive_str)]
#[typelevel(items(
    GetDiscriminant(NoImpls),
    IntoConstType(NoImpls),
    IntoRuntime(NoImpls),
    AsTList(NoImpls),
))]
pub struct Braced{
    a:u32,
    b:(),
}

#[derive(TypeLevel)]
#[typelevel(derive_str)]
#[typelevel(items(
    GetDiscriminant(NoImpls),
    IntoConstType(NoImpls),
    IntoRuntime(NoImpls),
    AsTList(NoImpls),
))]
pub struct Unit;


fn full_test(constval:&str,runtime_type:&str,derive:&str){
    use self::type_level_Tupled::*;
   
    use self::Privacy::*;

    let ctokens=CommonTokens::new();

    let type_constructor=runtime_type.split("<").next().unwrap_or("").trim();

    let tlmod_ident=parse_ident(&format!("type_level_{}",type_constructor));
    let tl_mods=type_level_modules(&ctokens,tlmod_ident);

    let variants=DataType::new(type_constructor,tl_mods,Variants::no_checking())
        .add_check(
            UnparsedItemCheck::trait_impl("ConstEq_<__Other>",constval)
                .set_nonexistant()
        )
        .add_check(
            UnparsedItemCheck::trait_impl("ConstOrd_<__Other>",constval)
                .set_nonexistant()
        )
        .add_check(
            UnparsedItemCheck::trait_impl("GetDiscriminant",constval)
                .set_nonexistant()
        )
        .add_check(
            UnparsedItemCheck::trait_impl("IntoConstType_",runtime_type)
                .set_nonexistant()
        )
        .add_check(
            UnparsedItemCheck::trait_impl(format!("IntoRuntime< {} >",runtime_type),constval)
                .set_nonexistant()
        )
        .add_check(
            UnparsedItemCheck::trait_impl("AsTList_",constval)
                .set_nonexistant()
        )
    ;


    test_items(
        variants,
        &ctokens,
        derive,
    );

}




#[test]
#[allow(non_snake_case)]
fn tests_Tupled(){
    full_test(
        "ConstTupled<field_0,field_1,__IsPriv,>",
        "Tupled<>",
        Tupled::TYPELEVEL_DERIVE,
    );
}

#[test]
#[allow(non_snake_case)]
fn tests_Braced(){
    full_test(
        "ConstBraced<field_0,field_1,__IsPriv,>",
        "Braced<>",
        Braced::TYPELEVEL_DERIVE,
    );
}

#[test]
#[allow(non_snake_case)]
fn tests_Unit(){
    full_test(
        "ConstUnit<>",
        "Unit<>",
        Unit::TYPELEVEL_DERIVE,
    );
}




#[derive(TypeLevel)]
// #[typelevel(print_derive)]
// #[typelevel(skip_derive)]
#[typelevel(derive_str)]
#[typelevel(items(
    GetDiscriminant(NoImpls),
    IntoConstType(NoImpls),
    IntoRuntime(NoImpls),
    AsTList(NoImpls),
))]
pub enum AnEnum<TypeParam>{
    UnitVar,
    TupledVar((),()),
    Braced{
        x:TypeParam,
        y:TypeParam,
    }
}



#[test]
#[allow(non_snake_case)]
fn tests_AnEnum(){
    let runtime_type="AnEnum<TypeParam,>";
    let enum_=AnEnum::<()>::TYPELEVEL_DERIVE;
    full_test("UnitVar<>",runtime_type,enum_);
    full_test("TupledVar<field_0,field_1,>",runtime_type,enum_);
    full_test("Braced<x,y,>",runtime_type,enum_);
}

