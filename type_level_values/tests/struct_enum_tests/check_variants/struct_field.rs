use super::*;

pub(crate) struct Field<'a>{
    pub(crate) attributes:Cow<'a,str>,
    pub(crate) privacy:Privacy,
    pub(crate) name:Cow<'a,str>,
    pub(crate) trait_base:Cow<'a,str>,
    pub(crate) accessor:Cow<'a,str>,
    pub(crate) accessor_kind:AccessorKind,
    pub(crate) bound:Option<&'a str>,
    pub(crate) bound_runt:Option<&'a str>,
    pub(crate) pub_assoc_ty:bool,
    pub(crate) visibility:&'a str,
}

impl<'a> Field<'a>{
    pub(crate) fn named(
        attributes:&'a str,
        privacy:Privacy,
        name:&'a str,
        visibility:&'a str,
    )->Self{
        Self{
            attributes:attributes.into(),
            privacy,
            name:name.into(),
            trait_base:name.into(),
            accessor  :name.into(),
            accessor_kind:AccessorKind::Struct,
            bound:None,
            bound_runt:None,
            pub_assoc_ty:false,
            visibility,
        }
    }
    pub(crate) fn positional(
        attributes:&'a str,
        privacy:Privacy,
        name:&'a str,
        visibility:&'a str,
    )->Self{
        let (acc,ak)=match privacy {
            Privacy::Inherited=>(format!("U{}"     ,name),AccessorKind::Integer),
            Privacy::Private  =>(format!("field_{}",name),AccessorKind::Struct),
        };
        Self{
            attributes:attributes.into(),
            privacy,
            name:name.into(),
            trait_base:format!("field_{}",name).into(),
            accessor:acc.into(),
            accessor_kind:ak,
            bound:None,
            bound_runt:None,
            pub_assoc_ty:false,
            visibility,
        }
    }
    pub(crate) fn ren_acc<S>(
        attributes:&'a str,
        privacy:Privacy,
        name:&'a str,
        rename:S,
        visibility:&'a str,
    )->Self
    where S:Into<Cow<'a,str>>,
    {
        let rename=rename.into();
        Field{
            attributes:attributes.into(),
            privacy,
            name:name.into(),
            trait_base:rename.clone(),
            accessor:rename.clone(),
            accessor_kind:AccessorKind::Struct,
            bound:None,
            bound_runt:None,
            pub_assoc_ty:false,
            visibility,
        }
    }

    pub(crate) fn assoc_ty_privacy(&self)->Privacy{
        if self.pub_assoc_ty {
            Privacy::Inherited
        }else{
            self.privacy
        }
    }
}

