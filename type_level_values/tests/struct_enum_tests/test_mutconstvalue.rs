use shared::traits::Trivial;

use type_level_values::core_extensions::Void;

use type_level_values::prelude::*;
use type_level_values::ops::AssertEq;
use type_level_values::user_traits::const_traits::*;


#[derive(MutConstValue)]
#[mcv(
    derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd),
    Type(name = "MutWrapper"),
    ConstValue = "M",
    DeriveStr,
    Items(
        ConstLayoutIndependent(
            bound="Self:Trivial<U10>",
            attr(cfg(any(feature="identity_feature",feature="U20"))),
            doc="U30",
        ),
        GetConstParam_(
            bound="Self:Trivial<U13>",
            attr(cfg(any(feature="identity_feature",feature="U23"))),
            doc="U33",
        ),
    ),

)]
pub struct MutWrapperInner<T:Clone=(), M:Clone=ConstWrapper<False>> 
where   
    T:Trivial<U0>,
    T:Trivial<U1>,
{
    value: T,
    mutability: ConstWrapper<M>,
}


mutator_fn!{
    type This[T:Clone,M]=(MutWrapper<T,M>)
    type AllowedSelf=(allowed_self_constructors::All)

    pub fn ChangeValueS[I,I2]( I ,I2){ I2 }
}


fn same_cc<L,R>()
where L:SameConstConstructor<R>
{}



fn same_layout<L,R>()
where L:ConstLayoutIndependent<R>
{}


macro_rules! check_associated_types {
    ( $val0:ty=$val0_full:ty,$val1:ty=$val1_full:ty,$valcc:ty ) => {{
        let _:AssertEq<$val0,$val0_full>;
        let _:AssertEq<$val1,$val1_full>;

        let _:AssertEq<GetConstParam<$val0>,U0>;
        let _:AssertEq<GetConstParam<$val1>,U1>;
        
        let _:AssertEq<GetConstConstructor<$val0>,$valcc>;
        let _:AssertEq<GetConstConstructor<$val1>,$valcc>;
        
        let _:AssertEq<ApplyConstParam<$valcc,U0>,$val0>;
        let _:AssertEq<ApplyConstParam<$valcc,U1>,$val1>;

        let _:AssertEq<SetConstParam<$val0,U1>,$val1>;
        let _:AssertEq<SetConstParam<$val1,U0>,$val0>;


        let _=same_layout::<$val0,$val1>();
        let _=same_cc::<$val0,$val1>();
    }}
}


macro_rules! mut_param_ {
    ($value:ident,$constmethod:ident,$constval:ty,$expected_ty:ty) => ({
        let tmp:$expected=$value.clone().mutparam($constmethod::NEW,<$constval as SelfOps>::T);
        tmp
    })
}


#[test]
fn check_associated_types_Struct0(){
    type Val0=MutWrapper<Vec<()>,U0>;
    type Val0Full=MutWrapper_Ty<Vec<()>,ConstWrapper<U0>>;
    type Val1=MutWrapper<Vec<()>,U1>;
    type Val1Full=MutWrapper_Ty<Vec<()>,ConstWrapper<U1>>;
    type ValCC=MutWrapper_CC<Vec<()>>;

    check_associated_types!(
        Val0=Val0Full,
        Val1=Val1Full,
        ValCC
    );


    let value=vec![0,1,2,3];

    let mw=||->MutWrapper<Vec<u32>,True>{
        MutWrapper_Ty{
            value:value.clone(),
            mutability:True::CW2,
        }
    };



    // Checking that no funny bussiness is going on after changing the ConstValue.
    assert_eq!(
        mw().mutparam(ChangeValueS::NEW,False::T).piped(|x:MutWrapper<Vec<u32>,False>| x ).value,
        value
    );

}





#[derive(MutConstValue)]
#[mcv(
    derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd),
    Type(name = "AnEnum"),
    ConstValue = "M",
)]
pub enum AnEnumInner<M:Clone= ConstWrapper<False> >{
    VarA,
    VarB(u32,u32),
    VarC{
        value: Vec<()>,
        mutability: ConstWrapper<M>,
    }
}

use self::AnEnum_Ty::*;


mutator_fn!{
    type This[M]=(AnEnum<M>)
    type AllowedSelf=(allowed_self_constructors::All)

    pub fn ChangeValueE[I,I2]( I ,I2){ I2 }
}

use type_level_values::user_traits::MutConstParam;


#[test]
fn check_associated_types_Enum0(){

    type Val0=AnEnum<U0>;
    type Val0Full=AnEnum_Ty<ConstWrapper<U0>>;
    type Val1=AnEnum<U1>;
    type Val1Full=AnEnum_Ty<ConstWrapper<U1>>;
    type ValCC=AnEnum_CC;

    check_associated_types!(
        Val0=Val0Full,
        Val1=Val1Full,
        ValCC
    );

    {
        let new_var0:AnEnum<U17>=VarA::<ConstWrapper<False>>.mutparam(ChangeValueE::NEW,U17::T);
        // Checking that no funny bussiness is going on after changing the ConstValue.
        assert_eq!(new_var0,VarA);
    }

    {
        let new_var1:AnEnum<Vec<()>>=VarB::<ConstWrapper<False>>(0,10)
            .mutparam(ChangeValueE::NEW,Vec::<()>::T);
        assert_eq!(new_var1,VarB(0,10));
    }

    fn check_var3(value: Vec<()>){
        let mut var2a=VarC{
            value:value.clone(),
            mutability:U33::CW2
        };

        {
            let var2b:&mut AnEnum<U13>=var2a.mutparam_mut(ChangeValueE::NEW,U13::T);
            assert_eq!(
                *var2b,
                VarC{ value:value.clone(), mutability:ConstWrapper::NEW }
            );
        }

        let var2c:AnEnum<U13>=var2a.clone().mutparam(ChangeValueE::NEW,U13::T);
        assert_eq!(
            var2c,
            VarC{ value:value.clone(), mutability:ConstWrapper::NEW }
        );
    
        let var2d:AnEnum<Void>=var2c.clone().mutparam(ChangeValueE::NEW,Void::T);
        assert_eq!(
            var2c,
            VarC{ value:value.clone(), mutability:ConstWrapper::NEW }
        );
    }

    check_var3(vec![();107]);
    check_var3(vec![();73]);

}
