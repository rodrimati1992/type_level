use super::*;

#[allow(unused_imports)]
use core_extensions::IteratorExt;

pub(crate) mod item_metadata;
pub(crate) mod my_meta;

/// Attribute-related stuff for the `TypeLevel` derive macro
pub(crate) mod typelevel;

/// Attribute-related stuff for the `MutConstValue` derive macro
pub(crate) mod mutconstvalue;

pub(crate) use self::my_meta::{MyMeta, MyNested};

////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////

pub(crate) mod shared {
    use super::*;
    use indexable_struct::{GetEnumIndices, InvalidMultiIndex};

    use core_extensions::SelfOps;
    use ArenasRef;

    use syn::punctuated::Punctuated;
    use syn::token::Comma;
    use syn::{NestedMeta, TypeParamBound};

    pub(crate) use parse_syn::{
        parse_ident, parse_syn_path, parse_type, parse_visibility, parse_where_pred,
    };

    pub(crate) fn foreach_nestedmeta_index<'alloc, I, WI, WE>(
        list: &'alloc Punctuated<NestedMeta, Comma>,
        arenas: ArenasRef<'alloc>,
        mut with_index: WI,
        mut with_error: WE,
    ) where
        WI: FnMut(&mut MyNested<'alloc>, I),
        WE: FnMut(&mut MyNested<'alloc>, InvalidMultiIndex<&str>),
        I: GetEnumIndices,
    {
        for attr in list {
            let mut attr: MyMeta = attr.into_with(arenas);

            let value = &mut attr.value;

            let impl_indices = I::many_from_str(&attr.word.str).unwrap_or_else(|x| {
                with_error(value, x);
                &[]
            });

            for impl_index in impl_indices {
                with_index(value, *impl_index);
            }
        }
    }

    #[derive(Debug)]
    pub(crate) struct NotUpdated;

    pub(crate) trait UpdateWithMeta<'alloc> {
        fn update_with_meta(
            &mut self,
            meta: &MyMeta<'alloc>,
            arenas: ArenasRef<'alloc>,
        ) -> Result<(), NotUpdated>;
    }

    impl<'alloc> UpdateWithMeta<'alloc> for () {
        #[inline]
        fn update_with_meta(
            &mut self,
            _meta: &MyMeta<'alloc>,
            _arenas: ArenasRef<'alloc>,
        ) -> Result<(), NotUpdated> {
            Err(NotUpdated)
        }
    }

    pub(crate) fn ident_from_nested<'a>(
        new_ident: &MyNested<'a>,
        arenas: ArenasRef<'a>,
    ) -> &'a syn::Ident {
        match new_ident {
            &MyNested::Value(ref val) => arenas.idents.alloc(parse_ident(val)),
            v => panic!("cannot be parsed as an identifier:{:#?}", v),
        }
    }

    pub(crate) fn typaram_from_nested<'a>(
        new_ident: &MyNested<'a>,
        arenas: ArenasRef<'a>,
    ) -> (&'a syn::Ident, Option<&'a syn::Type>) {
        match new_ident {
            &MyNested::Value(ref val) => {
                let mut iter = val.splitn(2, '=');
                let param_ = iter
                    .next()
                    .unwrap_or("")
                    .piped(|x| arenas.idents.alloc(parse_ident(x)));
                let type_ = iter.next().map(|x| &*arenas.types.alloc(parse_type(x)));
                (param_, type_)
            }
            v => panic!("cannot be parsed as an identifier :{:#?}", v),
        }
    }

    pub(crate) fn type_from_nested<'a>(
        new_ident: &MyNested<'a>,
        arenas: ArenasRef<'a>,
    ) -> &'a syn::Type {
        match new_ident {
            &MyNested::Value(ref val) => arenas.types.alloc(parse_type(val)),
            v => panic!("cannot be parsed as a type:{:#?}", v),
        }
    }

    pub(crate) fn bounds_from_str<C>(str_: &str, extend_into: &mut C)
    where
        C: Extend<TypeParamBound>,
    {
        str_.split('+')
            .map(|s| {
                syn::parse_str(s.trim()).unwrap_or_else(|e| {
                    panic!(
                        r#"expected bounds (eg:"Debug","Default+Clone")\n\
                           instead found:'{}'\n\
                           error:{:#?}\n"#,
                        s, e
                    );
                })
            }).extending(extend_into)
    }

}
