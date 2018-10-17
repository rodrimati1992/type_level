use super::*;

use type_level_values::ops::AssertEq;
use type_level_values::field_traits::GetField;
use type_level_values::prelude::*;

use shared::traits::{Trivial};
use check_variants::{
    VariantKind,
    Privacy,
    Variants,
    Variant,
    Field,
    // AccessorKind,
    SHARED_FIELD_ATTR,
    PUB_DSUPER,
    test_reexport,
};




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
pub struct BracedStruct{
    #[typelevel(pub_trait_getter)]
    pub a:u32,
    pub b:u32,
    #[typelevel(bound="Trivial<Self::a>+Trivial<Self::priv_d>")]
    pub(super) c:u32,
    #[typelevel(bound_runt="Trivial<Self::priv_eeee>+Trivial<Self::f>")]
    pub(crate) d:u32,
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



    use self::Privacy::*;

    let struct_fields=Variants{
        name:"TupleStruct",
        variants:vec![
            Variant{
                const_value:"ConstTupleStruct",
                dt_trait:"TupleStructTrait",
                wr_trait:"TupleStructWithRuntime",
                kind:VariantKind::Tupled,
                fields:vec![
                    Field::positional(SHARED_FIELD_ATTR,Inherited,"0","pub"),
                    Field::positional(SHARED_FIELD_ATTR,Inherited,"1","pub"),
                    Field::positional(SHARED_FIELD_ATTR,Private  ,"2",PUB_DSUPER).mutated(|f| {
                        f.bound=Some("Trivial<Self::field_0>+Trivial<Self::priv_field_3>") ;
                    }),
                    Field::positional(SHARED_FIELD_ATTR,Private  ,"3","pub(crate)").mutated(|f| {
                        f.bound_runt=Some("Trivial<Self::field_5>+Trivial<Self::priv_eeee>") ;
                    }),
                    Field::ren_acc   (SHARED_FIELD_ATTR,Private,"4","eeee","pub(in super)"),
                    Field::positional(SHARED_FIELD_ATTR,Private,"5","pub(in super)").mutated(|f|{
                        f.pub_assoc_ty=true;
                    }),
                ]
            }
        ],
    };

    test_reexport(
        &struct_fields,
        &CommonTokens::new(),
        &[
            "pub use super :: typenum_reexports :: { U0 , U1 , } ;",
        ],
        &[
            "pub use super::__fields::{U0,U1,} ;",
            "pub(crate)use super::__fields::{eeee,field_2,field_3,field_5,All,} ;",
        ],
        TupleStruct::TYPELEVEL_DERIVE,
    );
}

#[test]
#[allow(non_snake_case)]
fn tests_BracedStruct(){
    use self::type_level_BracedStruct::*;

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
            (U6 ,priv_d   ,rt_priv_d   ,d   ),
            (U8 ,priv_eeee,rt_priv_eeee,eeee),
            (U10,f        ,rt_f        ,f   ),
        ]
    }


    use self::Privacy::*;

    let struct_fields=Variants{
        name:"BracedStruct",
        variants:vec![
            Variant{
                const_value:"ConstBracedStruct",
                dt_trait:"BracedStructTrait",
                wr_trait:"BracedStructWithRuntime",
                kind:VariantKind::Braced,
                fields:vec![
                    Field::named(SHARED_FIELD_ATTR,Inherited,"a","pub"),
                    Field::named(SHARED_FIELD_ATTR,Inherited,"b","pub"),
                    Field::named(SHARED_FIELD_ATTR,Private  ,"c",PUB_DSUPER).mutated(|f| {
                        f.bound=Some("Trivial<Self::a>+Trivial<Self::priv_d>") ;
                    }),
                    Field::named(SHARED_FIELD_ATTR,Private  ,"d","pub(crate)").mutated(|f| {
                        f.bound_runt=Some("Trivial<Self::priv_eeee>+Trivial<Self::f>") ;
                    }),
                    Field::named(SHARED_FIELD_ATTR,Private,"eeee","pub(in super)"),
                    Field::named(SHARED_FIELD_ATTR,Private  ,"f","pub(in super)").mutated(|f|{
                        f.pub_assoc_ty=true;
                    }),
                ]
            }
        ],
    };


    test_reexport(
        &struct_fields,
        &CommonTokens::new(),
        &[
            "pub use super :: typenum_reexports ::{ } ;"
        ],
        &[
            "pub use super :: __fields :: { a , b , } ;",
            "pub(crate) use super :: __fields::{ c , d , eeee , f , All ,} ;",
        ],
        BracedStruct::TYPELEVEL_DERIVE,
    );
}

#[test]
#[allow(non_snake_case)]
fn tests_AnEnum(){
    use self::type_level_AnEnum::*;

    test_variant!{
        const_value=VarA,
        value=ValA,
        dt_trait=VarATrait,
        dt_with_runtime=VarAWithRuntime,
        fields=[
            (U0,field_0,rt_field_0,U0),
        ]
    }
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
    
    use self::Privacy::*;

    let priv_="pub(in super)";

    let enum_fields=Variants{
        name:"AnEnum",
        variants:vec![
            Variant{
                const_value:"VarA",
                dt_trait:"VarATrait",
                wr_trait:"VarAWithRuntime",
                kind:VariantKind::Tupled,
                fields:vec![
                    Field::positional(SHARED_FIELD_ATTR,Inherited,"0",priv_),
                ]
            },
            Variant{
                const_value:"VarB",
                dt_trait:"VarBTrait",
                wr_trait:"VarBWithRuntime",
                kind:VariantKind::Braced,
                fields:vec![
                    Field::named(SHARED_FIELD_ATTR,Inherited,"a",priv_),
                    Field::named(SHARED_FIELD_ATTR,Inherited,"b",priv_).mutated(|f| {
                        f.bound=Some("Trivial<U10>") ;
                        f.bound_runt=Some("Trivial<Self::a>") ;
                    }),
                ]
            },
            Variant{
                const_value:"VarC",
                dt_trait:"VarCTrait",
                wr_trait:"VarCWithRuntime",
                kind:VariantKind::Tupled,
                fields:vec![
                    Field::positional(SHARED_FIELD_ATTR,Inherited,"0",priv_).mutated(|f| {
                        f.bound=Some("Trivial<Self::uh>") ;
                    }),
                    Field::ren_acc(SHARED_FIELD_ATTR,Inherited,"1","uh",priv_),
                ]
            },
        ],
    };

    test_reexport(
        &enum_fields,
        &CommonTokens::new(),
        &[
            "pub ( in super :: super ) use super :: typenum_reexports :: { U0 , } ;"
        ],
        &[
            "pub ( in super :: super ) use super :: __fields :: { U0 , a , b , uh , } ;",
            "pub ( in super :: super ) use super :: __fields :: { All , } ;",
        ],
        AnEnum::TYPELEVEL_DERIVE,
    );
}
