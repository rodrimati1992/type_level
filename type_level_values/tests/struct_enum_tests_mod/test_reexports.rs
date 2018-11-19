use super::*;

use core_extensions::BoolExt;
use core_extensions::IteratorExt;

use derive_type_level_lib::parse_syn::{
    parse_syn_use,
    parse_ident,
};



mod reexp_s0{
    #[allow(dead_code)]
    #[derive(TypeLevel)]
    #[typelevel(derive_str,reexport())]
    pub struct Reexport{
        x:u32,
        y:u32,
    }
}
mod reexp_e0{
    #[allow(dead_code)]
    #[derive(TypeLevel)]
    #[typelevel(derive_str,reexport())]
    pub enum Reexport{
        X,
        Y,
    }
}

mod reexp_s1{
    #[allow(dead_code)]
    #[derive(TypeLevel)]
    #[typelevel(derive_str,reexport(Traits))]
    pub struct Reexport{
        x:u32,
        y:u32,
    }
}

mod reexp_e1{
    #[allow(dead_code)]
    #[derive(TypeLevel)]
    #[typelevel(derive_str,reexport(Traits))]
    pub enum Reexport{
        X,
        Y,
    }
}

mod reexp_s2{
    #[allow(dead_code)]
    #[derive(TypeLevel)]
    #[typelevel(derive_str,reexport(Struct))]
    pub struct Reexport{
        x:u32,
        y:u32,
    }
}

mod reexp_e2{
    #[allow(dead_code)]
    #[derive(TypeLevel)]
    #[typelevel(derive_str,reexport(Variants))]
    pub enum Reexport{
        X,
        Y,
    }
}


mod reexp_s3{
    #[allow(dead_code)]
    #[derive(TypeLevel)]
    #[typelevel(derive_str,reexport(Discriminants))]
    pub struct Reexport{
        x:u32,
        y:u32,
    }

}

mod reexp_e3{
    #[allow(dead_code)]
    #[derive(TypeLevel)]
    #[typelevel(derive_str,reexport(Discriminants))]
    pub enum Reexport{
        X,
        Y,
    }
}

mod reexp_s4{
    #[allow(dead_code)]
    #[derive(TypeLevel)]
    #[typelevel(derive_str,reexport(Fields))]
    pub struct Reexport{
        x:u32,
        y:u32,
    }
}

mod reexp_e4{
    #[allow(dead_code)]
    #[derive(TypeLevel)]
    #[typelevel(derive_str,reexport(Fields))]
    pub enum Reexport{
        X,
        Y,
    }
}

mod reexp_s5{
    #[allow(dead_code)]
    #[derive(TypeLevel)]
    #[typelevel(derive_str,reexport(Traits,Variants,Discriminants,Fields))]
    pub struct Reexport{
        x:u32,
        y:u32,
    }
}

mod reexp_e5{
    #[allow(dead_code)]
    #[derive(TypeLevel)]
    #[typelevel(derive_str,reexport(Traits,Variants,Discriminants,Fields))]
    pub enum Reexport{
        X,
        Y,
    }
}


#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct Reexported{
    pub traits:bool,
    pub variants:bool,
    pub discriminants:bool,
    pub fields:bool,
}

impl Reexported{
    // pub fn new(traits:bool,variants:bool,discriminants:bool,fields:bool)->Self{
    //     Self{traits,variants,discriminants,fields}
    // }

    pub fn set_all(value:bool)->Self{
        Self{
            traits:value,
            variants:value,
            discriminants:value,
            fields:value,
        }
    }

}


fn text_reexport_inner(
    ctokens:&CommonTokens,
    enum_or_struct:EnumOrStruct,
    reexported:Reexported,
    derive_str:&str,
){
    use std::collections::HashSet;
    use self::EnumOrStruct::{Struct,Enum};

    println!("\n\n{:?} {:?}\n",enum_or_struct,reexported);
    
    let co_if=|cond| if cond { "" }else{ "//" } ; 

    let mut set=HashSet::new();

    let mut reexported_list=Vec::new();

    fn pub_vis(vis:&str)->(&str,&str){
        ("pub",vis)
    }

    fn priv_vis(vis:&str)->(&str,&str){
        ("",vis)
    }

    if reexported!=Reexported::set_all(false) {
        match enum_or_struct {
            Enum=>vec![
                Some("ReexportType"),
                reexported.traits.if_true(|| "ReexportTrait" ),
                reexported.traits.if_true(|| "XTrait" ),
                reexported.traits.if_true(|| "XWithRuntime" ),
                reexported.traits.if_true(|| "YTrait" ),
                reexported.traits.if_true(|| "YWithRuntime" ),
                reexported.fields.if_true(|| "fields" ),
            ],
            Struct=>vec![
                Some("ReexportType"),
                reexported.traits.if_true(|| "ReexportTrait" ),
                reexported.traits.if_true(|| "ReexportWithRuntime" ),
                reexported.fields.if_true(|| "fields" ),
            ],
        }.into_iter()
            .filter_map(|x|x)
            .map(pub_vis)
            .extending(&mut reexported_list);
    }

    if reexported.variants {
        let use_str=match enum_or_struct {
            Enum=>vec![
                pub_vis("X"),
                pub_vis("X_Uninit"),
                pub_vis("Y"),
                pub_vis("Y_Uninit"),
            ],
            Struct=>vec![
                priv_vis("ConstReexport"),
                priv_vis("Reexport_Uninit"),
            ],
        }.into_iter().extending(&mut reexported_list);
    }

    if reexported.discriminants {
        let use_str=match enum_or_struct {
            Enum=>vec![
                pub_vis("variants::X_Variant"),
                pub_vis("variants::X_Discr"),
                pub_vis("variants::Y_Variant"),
                pub_vis("variants::Y_Discr"),
            ],
            Struct=>vec![
                priv_vis("variants::Reexport_Variant"),
                priv_vis("variants::Reexport_Discr"),
            ],
        }.into_iter().extending(&mut reexported_list);
    }

    for (vis,item) in reexported_list {
        let use_str=format!("\
            #[allow(unused_imports)]
            {vis} use self :: type_level_Reexport :: {item};
        ",vis=vis,item=item);
        set.insert(parse_syn_use(&use_str));
    }



    let mut found_reexports=false;

    let tl_mods=type_level_modules(ctokens,parse_ident("type_level_Reexport"));
    let mut visiting=Visiting::new(tl_mods.into());

    visiting.check_derive(derive_str,move|params|{
        if params.mod_index!=TLModIndex::DerivingMod { return } 
        match params.item.clone() {
            VisitItem::Use(ref use_)=>{
                if !set.remove(use_) {
                    let s=format!(
                        "{}\n\nRemaining Items:{}",
                        tokens_to_string(use_),
                        totoken_iter_to_string(&set)
                    );
                    return params.push_err(VIErrorKind::UnexpectedItem,s);
                }
                if set.is_empty() {
                    found_reexports=true;
                }
            }
            VisitItem::EndOfMod if !found_reexports =>{
                return params.push_err(VIErrorKind::ExpectedMoreItems,"expected item reexports")
            }
            _=>{}
        }
    });
    println!("\n\n");
}


#[test]
fn test_reexports(){

    let ref ctokens=CommonTokens::new();

    use self::text_reexport_inner as inner;
    use self::EnumOrStruct::{Struct,Enum};

    let reexported_0=Reexported::set_all(false);
    inner(ctokens,Struct,reexported_0,reexp_s0::Reexport::TYPELEVEL_DERIVE);
    inner(ctokens,Enum  ,reexported_0,reexp_e0::Reexport::TYPELEVEL_DERIVE);

    let reexported_1=Reexported::set_all(false).mutated(|x| x.traits=true );
    inner(ctokens,Struct,reexported_1,reexp_s1::Reexport::TYPELEVEL_DERIVE);
    inner(ctokens,Enum  ,reexported_1,reexp_e1::Reexport::TYPELEVEL_DERIVE);
    
    let reexported_2=Reexported::set_all(false).mutated(|x| x.variants=true );
    inner(ctokens,Struct,reexported_2,reexp_s2::Reexport::TYPELEVEL_DERIVE);
    inner(ctokens,Enum  ,reexported_2,reexp_e2::Reexport::TYPELEVEL_DERIVE);
    
    let reexported_3=Reexported::set_all(false).mutated(|x| x.discriminants=true );;
    inner(ctokens,Struct,reexported_3,reexp_s3::Reexport::TYPELEVEL_DERIVE);
    inner(ctokens,Enum  ,reexported_3,reexp_e3::Reexport::TYPELEVEL_DERIVE);
    
    let reexported_4=Reexported::set_all(false).mutated(|x| x.fields=true );;
    inner(ctokens,Struct,reexported_4,reexp_s4::Reexport::TYPELEVEL_DERIVE);
    inner(ctokens,Enum  ,reexported_4,reexp_e4::Reexport::TYPELEVEL_DERIVE);

    let reexported_5=Reexported::set_all(true);
    inner(ctokens,Struct,reexported_5,reexp_s5::Reexport::TYPELEVEL_DERIVE);
    inner(ctokens,Enum  ,reexported_5,reexp_e5::Reexport::TYPELEVEL_DERIVE);

}