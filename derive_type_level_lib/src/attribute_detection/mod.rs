use super::*;

#[allow(unused_imports)]
use core_extensions::IteratorExt;


#[macro_use]
pub(crate) mod indexable_struct;

pub(crate) mod item_metadata;
pub(crate) mod my_meta;

/// Attribute-related stuff for the `TypeLevel` derive macro
pub(crate) mod typelevel;

/// Attribute-related stuff for the `ConstConstructor` derive macro
pub(crate) mod const_constructor;

pub(crate) use self::my_meta::{MyMeta, MyNested};

////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////

pub(crate) mod shared {
    use super::indexable_struct::{GetEnumIndices, InvalidMultiIndex};
    use super::*;

    use ArenasRef;

    use syn::punctuated::Punctuated;
    use syn::token::Comma;
    use syn::NestedMeta;
    use syn::TypeParamBound;

    pub(crate) fn foreach_nestedmeta_index<'alloc,I, WI, WE>(
        list: &'alloc Punctuated<NestedMeta, Comma>,
        arenas:ArenasRef<'alloc>,
        mut with_index: WI,
        mut with_error: WE,
    ) where
        WI: FnMut(&mut MyNested<'alloc>,I),
        WE: FnMut(&mut MyNested<'alloc>,InvalidMultiIndex<&str>),
        I: GetEnumIndices,
    {
        for attr in list {
            let mut attr: MyMeta = attr.into_with(arenas);

            let value=&mut attr.value;

            let impl_indices = I::many_from_str(&attr.word.str).unwrap_or_else(|x| {
                with_error(value,x);
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
        arenas: ArenasRef<'a>
    ) -> &'a syn::Ident {
        match new_ident {
            &MyNested::Value(ref val) => {
                let x = syn::parse_str(val);
                let x = x.unwrap_or_else(|_| panic!("must be parsable as an Ident:'{}'", val));
                arenas.idents.alloc(x)
            }
            v => panic!("cannot be parsed as an identifier:{:#?}", v),
        }
    }



    pub(crate) fn type_from_nested<'a>(
        new_ident: &MyNested<'a>,
        arenas: ArenasRef<'a>
    ) -> &'a syn::Type {
        match new_ident {
            &MyNested::Value(ref val) => {
                let x = syn::parse_str(val);
                let x = x.unwrap_or_else(|_| panic!("must be parsable as a type:'{}'", val));
                arenas.types.alloc(x)
            }
            v => panic!("cannot be parsed as a type:{:#?}", v),
        }
    }

    pub(crate) fn bounds_from_str<C>(str_:&str,extend_into:&mut C)
    where 
        C:Extend<TypeParamBound>
    {
        str_.split('+').map(|s|{
            syn::parse_str(s.trim())
                .unwrap_or_else(|e|{
                    panic!(
                        r#"expected bounds (eg:"Debug","Default+Clone")\n\
                           instead found:'{}'\n\
                           error:{:#?}\n"#, 
                        s,
                        e
                    );
                })
        }).extending(extend_into)
    }

}
