use super::*;
use super::type_level_shared::*;



#[derive(TypeLevel)]
#[typelevel(
    // print_derive,
    derive_str,
    reexport(Traits,Variants)
)]
#[allow(dead_code)]
pub struct TupleStruct(
    #[typelevel(pub_trait_getter)]
    pub u32,
    pub u32,
    #[typelevel(bound="Trivial<Self::field_0>+Trivial<Self::priv_field_3>")]
    pub(super) u32,
    #[typelevel(bound_runt="Trivial<Self::field_5>+Trivial<Self::priv_eeee>")]
    pub(crate) u32,
    #[typelevel(rename="eeee")]
    u32,
    #[typelevel(pub_trait_getter)]
    u32,
);


#[derive(TypeLevel)]
#[typelevel(
    // print_derive,
    derive_str,
    reexport(Traits,Variants)
)]
#[allow(dead_code)]
pub struct BracedStruct<T=u64>{
    #[typelevel(pub_trait_getter)]
    pub a:u32,
    pub b:u32,
    #[typelevel(bound="Trivial<Self::a>+Trivial<Self::priv_d>")]
    pub(super) c:u32,
    #[typelevel(bound_runt="Trivial<Self::priv_eeee>+Trivial<Self::f>+Into<T>")]
    pub(crate) d:T,
    #[typelevel(rename="eeee")]
    e:u32,
    #[typelevel(pub_trait_getter)]
    f:u32,
}


#[derive(TypeLevel)]
#[typelevel(
    derive_str,
    reexport(Traits,Variants)
)]
#[allow(dead_code)]
enum AnEnum{
    VarA(u32),
    VarB{
        a:u32,
        #[typelevel(bound="Trivial<U10>")]
        #[typelevel(bound_runt="Trivial<Self::a>")]
        b:u32,
    },
    VarC(
        #[typelevel(bound="Trivial<Self::uh>")]
        u32,
        #[typelevel(rename="uh")]
        u32
    ),
}




macro_rules! test_variant {
    (   
        const_value=$cv_ident:ident,
        value=$value:ident,
        $(is_private=$is_private:ty,)*
        dt_trait       =$dt_t :ident,
        dt_with_runtime=$dt_wr:ident,
        fields=[
            $(( 
                $f_value:ty ,
                $f_t:ident ,
                $f_wr:ident ,
                $f_acc:ident
            )),* 
            $(,)* 
        ]
    ) => (
        type $value=$cv_ident< $( $f_value ,)* $($is_private)* >;
        $(
            let _:AssertEq< <$value as $dt_t >::$f_t  , $f_value >;
            let _:AssertEq< <$value as $dt_wr>::$f_wr , $f_value >;
            let _:AssertEq< GetField<$value,fields::$f_acc> , $f_value >;
        )*
    )
}


#[test]
#[allow(non_snake_case)]
fn tests_TupleStruct(){
    use self::type_level_TupleStruct::*;

    test_variant!{
        const_value=ConstTupleStruct,
        value=ValA,
        is_private=__IsPriv,
        dt_trait=TupleStructTrait,
        dt_with_runtime=TupleStructWithRuntime,
        fields=[
            (U0 ,field_0     ,rt_field_0     ,U0     ),
            (U2 ,field_1     ,rt_field_1     ,U1     ),
            (U4 ,priv_field_2,rt_priv_field_2,field_2),
            (U6 ,priv_field_3,rt_priv_field_3,field_3),
            (U8 ,priv_eeee   ,rt_priv_eeee   ,eeee   ),
            (U10,field_5     ,rt_field_5     ,field_5),
        ]
    }

    let _:AssertEq<
        tlist![ fields::U0,fields::U1],
        TupleStruct_PubFields,
    >;

    let _:AssertEq<
        tlist![ 
            fields::U0,
            fields::U1,
            fields::field_2,
            fields::field_3,
            fields::eeee,
            fields::field_5 
        ],
        TupleStruct_AllFields,
    >;

    use self::Privacy::*;

    let tl_mods=type_level_modules(&CommonTokens::new(),parse_ident("type_level_TupleStruct"));
    let struct_fields=DataType::new(tl_mods,Variants::typelevel())
        .add_tl_variant(TLVariant{
            const_value:"ConstTupleStruct",
            dt_trait:"TupleStructTrait",
            wr_trait:"TupleStructWithRuntime",
            kind:VariantKind::Tupled,
            fields:Some(vec![
                Field::positional(SHARED_FIELD_ATTR,Inherited,"0","pub"),
                Field::positional(SHARED_FIELD_ATTR,Inherited,"1","pub"),
                Field::positional(SHARED_FIELD_ATTR,Private  ,"2",PUB_DSUPER).mutated(|f| {
                    f.bound=Some("Trivial<Self::field_0>+Trivial<Self::priv_field_3>") ;
                }),
                Field::positional(SHARED_FIELD_ATTR,Private  ,"3","pub(crate)").mutated(|f| {
                    f.bound_runt=
                        Some("Trivial<Self::field_5>+Trivial<Self::priv_eeee>") ;
                }),
                Field::ren_acc   (SHARED_FIELD_ATTR,Private,"4","eeee","pub(in super)"),
                Field::positional(SHARED_FIELD_ATTR,Private,"5","pub(in super)").mutated(|f|{
                    f.pub_assoc_ty=true;
                }),
            ])
        })
        .add_reexports(TLModIndex::DunderFieldMod,[
            "pub use super :: integer_reexports :: {U0,U1,};",
        ].iter().cloned())
        .add_reexports(TLModIndex::FieldsMod,[
            "pub use super::__fields::U0;",
            "pub use super::__fields::U1;",
            "pub(crate)use super::__fields::eeee ;",
            "pub(crate)use super::__fields::field_2;",
            "pub(crate)use super::__fields::field_3;",
            "pub(crate)use super::__fields::field_5;",
            "pub(crate)use super::__fields::All;",
        ].iter().cloned());


    test_items(
        struct_fields,
        &CommonTokens::new(),
        TupleStruct::TYPELEVEL_DERIVE,
    );
}

#[test]
#[allow(non_snake_case)]
fn tests_BracedStruct(){
    use self::type_level_BracedStruct::*;

    type Value=ConstBracedStruct<(),(),(),u8,(),(),__IsPriv>;
    let _:AssertEq<
        <Value as BracedStructWithRuntime>::rt_priv_d,
        u8,
    >;
    let value:u32= 
        <<Value as BracedStructWithRuntime>::rt_priv_d 
                as Into<u32>
        >::into(10u8);

    assert_eq!(value,10u32);


    test_variant!{
        const_value=ConstBracedStruct,
        value=ValA,
        is_private=__IsPriv,
        dt_trait=BracedStructTrait,
        dt_with_runtime=BracedStructWithRuntime,
        fields=[
            (U0 ,a        ,rt_a        ,a   ),
            (U2 ,b        ,rt_b        ,b   ),
            (U4 ,priv_c   ,rt_priv_c   ,c   ),
            (u8 ,priv_d   ,rt_priv_d   ,d   ),
            (U8 ,priv_eeee,rt_priv_eeee,eeee),
            (U10,f        ,rt_f        ,f   ),
        ]
    }

    let _:AssertEq<
        tlist![ fields::a,fields::b],
        BracedStruct_PubFields,
    >;

    let _:AssertEq<
        tlist![ 
            fields::a,
            fields::b,
            fields::c,
            fields::d,
            fields::eeee,
            fields::f 
        ],
        BracedStruct_AllFields,
    >;


    use self::Privacy::*;

    let ctokens=CommonTokens::new();

    let tl_mods=type_level_modules(&ctokens,parse_ident("type_level_BracedStruct"));
    let struct_fields=DataType::new(tl_mods,Variants::typelevel())
        .add_tl_variant(TLVariant{
            const_value:"ConstBracedStruct",
            dt_trait:"BracedStructTrait",
            wr_trait:"BracedStructWithRuntime",
            kind:VariantKind::Braced,
            fields:Some(vec![
                Field::named(SHARED_FIELD_ATTR,Inherited,"a","pub"),
                Field::named(SHARED_FIELD_ATTR,Inherited,"b","pub"),
                Field::named(SHARED_FIELD_ATTR,Private  ,"c",PUB_DSUPER).mutated(|f| {
                    f.bound=Some("Trivial<Self::a>+Trivial<Self::priv_d>") ;
                }),
                Field::named(SHARED_FIELD_ATTR,Private  ,"d","pub(crate)").mutated(|f| {
                    f.bound_runt=Some("Trivial<Self::priv_eeee>+Trivial<Self::f>+Into<T>") ;
                }),
                Field::named(SHARED_FIELD_ATTR,Private,"eeee","pub(in super)"),
                Field::named(SHARED_FIELD_ATTR,Private  ,"f","pub(in super)").mutated(|f|{
                    f.pub_assoc_ty=true;
                }),
            ])
        })
        .add_reexports(TLModIndex::DunderFieldMod,[
            " pub use super :: integer_reexports ::{ } ; "
        ].iter().cloned())
        .add_reexports(TLModIndex::FieldsMod,[
            "pub use super :: __fields ::a;",
            "pub use super :: __fields ::b;",
            "pub(crate) use super :: __fields::c;",
            "pub(crate) use super :: __fields::d;",
            "pub(crate) use super :: __fields::eeee;",
            "pub(crate) use super :: __fields::f;",
            "pub(crate) use super :: __fields::All;",
        ].iter().cloned());


    test_items(
        struct_fields,
        &ctokens,
        // Have to use the <TypeName>:: syntax to get TypeName with defaulted type parameters
        <BracedStruct>::TYPELEVEL_DERIVE, 
    );
}

#[test]
#[allow(non_snake_case)]
fn tests_AnEnum(){
    use self::type_level_AnEnum::*;

    type Assert3Eq<A,B,C>=(
        AssertEq<A,B>,
        AssertEq<B,C>,
    );

    test_variant!{
        const_value=VarA,
        value=ValA,
        dt_trait=VarATrait,
        dt_with_runtime=VarAWithRuntime,
        fields=[
            (U0,field_0,rt_field_0,U0),
        ]
    }

    let _:Assert3Eq<
        VarA_PubFields,
        VarA_AllFields,
        tlist![fields::U0],
    >;

    test_variant!{
        const_value=VarB,
        value=ValB,
        dt_trait=VarBTrait,
        dt_with_runtime=VarBWithRuntime,
        fields=[
            (U10,a,rt_a,a),
            (U20,b,rt_b,b),
        ]
    }

    let _:Assert3Eq<
        VarB_PubFields,
        VarB_AllFields,
        tlist![fields::a,fields::b],
    >;


    test_variant!{
        const_value=VarC,
        value=ValC,
        dt_trait=VarCTrait,
        dt_with_runtime=VarCWithRuntime,
        fields=[
            (U10,field_0,rt_field_0,U0),
            (U20,uh     ,rt_uh     ,uh),
        ]
    }
    
    let _:Assert3Eq<
        VarC_PubFields,
        VarC_AllFields,
        tlist![fields::U0,fields::uh],
    >;


    use self::Privacy::*;

    let priv_="pub(in super)";

    let ctokens=CommonTokens::new();

    let tl_mods=type_level_modules(&ctokens,parse_ident("type_level_AnEnum"));

    let enum_fields=DataType::new(tl_mods,Variants::typelevel())
        .add_tl_variant(TLVariant{
            const_value:"VarA",
            dt_trait:"VarATrait",
            wr_trait:"VarAWithRuntime",
            kind:VariantKind::Tupled,
            fields:Some(vec![
                Field::positional(SHARED_FIELD_ATTR,Inherited,"0",priv_),
            ])
        })
        .add_tl_variant(TLVariant{
            const_value:"VarB",
            dt_trait:"VarBTrait",
            wr_trait:"VarBWithRuntime",
            kind:VariantKind::Braced,
            fields:Some(vec![
                Field::named(SHARED_FIELD_ATTR,Inherited,"a",priv_),
                Field::named(SHARED_FIELD_ATTR,Inherited,"b",priv_).mutated(|f| {
                    f.bound=Some("Trivial<U10>") ;
                    f.bound_runt=Some("Trivial<Self::a>") ;
                }),
            ])
        })
        .add_tl_variant(TLVariant{
            const_value:"VarC",
            dt_trait:"VarCTrait",
            wr_trait:"VarCWithRuntime",
            kind:VariantKind::Tupled,
            fields:Some(vec![
                Field::positional(SHARED_FIELD_ATTR,Inherited,"0",priv_).mutated(|f| {
                    f.bound=Some("Trivial<Self::uh>") ;
                }),
                Field::ren_acc(SHARED_FIELD_ATTR,Inherited,"1","uh",priv_),
            ])
        })
        .add_reexports(TLModIndex::DunderFieldMod,[
            "pub use super :: integer_reexports :: {U0,};"
        ].iter().cloned())
        .add_reexports(TLModIndex::FieldsMod,[
            "pub ( in super :: super ) use super :: __fields :: U0 ;",
            "pub ( in super :: super ) use super :: __fields :: a ;",
            "pub ( in super :: super ) use super :: __fields :: b ;",
            "pub ( in super :: super ) use super :: __fields :: uh ;",
            "pub ( in super :: super ) use super :: __fields :: All ;",
        ].iter().cloned());


    test_items(
        enum_fields,
        &ctokens,
        AnEnum::TYPELEVEL_DERIVE,
    );
}
