use super::*;
use super::type_level_shared::*;




#[derive(TypeLevel)]
// #[typelevel(print_derive)]
// #[typelevel(skip_derive)]
#[typelevel(derive_str)]
#[typelevel(items(
    ConstEq(
        bound="Self:Trivial<U10>",
        attr(cfg(any(feature="identity_feature",feature="U20"))),
        doc="U30",
    ),
    ConstOrd(
        bound="Self:Trivial<U11>",
        attr(cfg(any(feature="identity_feature",feature="U21"))),
        doc="U31",
    ),
    GetDiscriminant(
        bound="Self:Trivial<U12>",
        attr(cfg(any(feature="identity_feature",feature="U22"))),
        doc="U32",
    ),
    IntoConstType(
        bound="Self:Trivial<U13>",
        attr(cfg(any(feature="identity_feature",feature="U23"))),
        doc="U33",
    ),
    IntoRuntime(
        bound="Self:Trivial<U14>",
        attr(cfg(any(feature="identity_feature",feature="U24"))),
        doc="U34",
    ),
    AsTList(
        bound="Self:Trivial<U15>",
        attr(cfg(any(feature="identity_feature",feature="U25"))),
        doc="U35",
    ),
))]
#[typelevel(
    rename="TupledRen",
    rename_trait="TupledBBQ",
    rename_wr_trait="TupledWTF",
)]
pub struct Tupled<T>(
    u32,
    T,
);



fn full_test<F>(constval:&str,runtime_type:&str,derive:&str,additional_checks:F)
where F:FnOnce(DataType<TLModIndex>)->DataType<TLModIndex>
{
    use self::type_level_Tupled::*;
   
    use self::Privacy::*;

    let ctokens=CommonTokens::new();

    let type_constructor=runtime_type.split("<").next().unwrap_or("").trim();

    let tlmod_ident=parse_ident(&format!("type_level_{}",type_constructor));
    let tl_mods=type_level_modules(&ctokens,tlmod_ident);

    let variants=DataType::new(tl_mods,Variants::no_checking())
        .add_check(
            UnparsedItemCheck::trait_impl("ConstEq_<__Other>",constval)
                .add_where_pred("Self:Trivial<U10>")
                .add_attribute(r#"#[cfg(any(feature="identity_feature",feature="U20"))]"#)
                .add_attribute(r#"#[doc="U30"]"#)
        )
        .add_check(
            UnparsedItemCheck::trait_impl("ConstOrd_<__Other>",constval)
                .add_where_pred("Self:Trivial<U11>")
                .add_attribute(r#"#[cfg(any(feature="identity_feature",feature="U21"))]"#)
                .add_attribute(r#"#[doc="U31"]"#)
        )
        .add_check(
            UnparsedItemCheck::trait_impl("GetDiscriminant",constval)
                .add_where_pred("Self:Trivial<U12>")
                .add_attribute(r#"#[cfg(any(feature="identity_feature",feature="U22"))]"#)
                .add_attribute(r#"#[doc="U32"]"#)
        )
        .add_check(
            UnparsedItemCheck::trait_impl("IntoConstType_",runtime_type)
                .add_where_pred("Self:Trivial<U13>")
                .add_attribute(r#"#[cfg(any(feature="identity_feature",feature="U23"))]"#)
                .add_attribute(r#"#[doc="U33"]"#)
        )
        .add_check(
            UnparsedItemCheck::trait_impl(format!("IntoRuntime< {} >",runtime_type),constval)
                .add_where_pred("Self:Trivial<U14>")
                .add_attribute(r#"#[cfg(any(feature="identity_feature",feature="U24"))]"#)
                .add_attribute(r#"#[doc="U34"]"#)
        )
        .add_check(
            UnparsedItemCheck::trait_impl("AsTList_",constval)
                .add_where_pred("Self:Trivial<U15>")
                .add_attribute(r#"#[cfg(any(feature="identity_feature",feature="U25"))]"#)
                .add_attribute(r#"#[doc="U35"]"#)
        )
        .add_check(
            UnparsedItemCheck::trait_impl("Debug",constval)
                .set_nonexistant()
        )
        .piped(additional_checks)
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
        TUPLED_CONSTVAL,
        "Tupled<T,>",
        Tupled::<()>::TYPELEVEL_DERIVE,
        |datatype|{
            datatype
            .add_check(
                UnparsedItemCheck::trait_impl("TupledBBQ",TUPLED_CONSTVAL)
            )
            .add_check(
                UnparsedItemCheck::trait_impl("TupledWTF<T,>",TUPLED_CONSTVAL)
            )
        }
    );
}








#[derive(TypeLevel)]
#[typelevel(derive_str)]
#[typelevel(items(
    ConstEq(
        bound="Self:Trivial<U10>",
        attr(cfg(any(feature="identity_feature",feature="U20"))),
        doc="U30",
    ),
    ConstOrd(
        bound="Self:Trivial<U11>",
        attr(cfg(any(feature="identity_feature",feature="U21"))),
        doc="U31",
    ),
    GetDiscriminant(
        bound="Self:Trivial<U12>",
        attr(cfg(any(feature="identity_feature",feature="U22"))),
        doc="U32",
    ),
    IntoConstType(
        bound="Self:Trivial<U13>",
        attr(cfg(any(feature="identity_feature",feature="U23"))),
        doc="U33",
    ),
    IntoRuntime(
        bound="Self:Trivial<U14>",
        attr(cfg(any(feature="identity_feature",feature="U24"))),
        doc="U34",
    ),
    AsTList(
        bound="Self:Trivial<U15>",
        attr(cfg(any(feature="identity_feature",feature="U25"))),
        doc="U35",
    ),
))]
#[typelevel(rename_consttype="AnEnumTypo")]
pub enum AnEnum<TypeParam>{
    #[typelevel(rename_consttype="ThisIsANoOp")]
    #[typelevel(rename_constvalue="UnitVarWhat")]
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
    full_test("UnitVarWhat<>",runtime_type,enum_,|datatype|{
        datatype
        .add_check(
            UnparsedItemCheck::trait_impl("ConstType","AnEnumTypo")
        )
    });
    full_test("TupledVar<field_0,field_1,>",runtime_type,enum_,|datatype|datatype);
    full_test("Braced<x,y,>",runtime_type,enum_,|datatype|datatype);
}


static TUPLED_CONSTVAL:&str="ConstTupled<field_0,field_1,__IsPriv,>";

#[allow(non_snake_case)]
mod should_panic_tests_Tupled{
    use super::*;
    use self::type_level_Tupled::*;   
    use self::Privacy::*;


    fn with_single_impl(impl_block:UnparsedItemCheck){

        let ctokens=CommonTokens::new();

        let tl_mods=type_level_modules(&ctokens,parse_ident("type_level_Tupled"));


        let variants=DataType::new(tl_mods,Variants::no_checking())
            .add_check(impl_block);

        test_items(
            variants,
            &ctokens,
            Tupled::<()>::TYPELEVEL_DERIVE,
        );
    }

    #[test]
    #[should_panic]
    fn tests_Tupled_where_preds(){
        with_single_impl(
            UnparsedItemCheck::trait_impl("ConstEq_<__Other>",TUPLED_CONSTVAL)
                .add_where_pred("Self:Trivial<NotAType>")
        )
    }
    
    #[test]
    #[should_panic]
    fn tests_Tupled_attr(){
        with_single_impl(
            UnparsedItemCheck::trait_impl("ConstEq_<__Other>",TUPLED_CONSTVAL)
                .add_attribute(r#"#[cfg(feature="not a feature")]"#)
        )
    }


    #[test]
    #[should_panic]
    fn tests_Tupled_doc(){
        with_single_impl(
            UnparsedItemCheck::trait_impl("ConstEq_<__Other>",TUPLED_CONSTVAL)
                .add_attribute(r#"#[doc="not a feature"]"#)
        )
    }

}