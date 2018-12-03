use proc_macro2::Span;
use proc_macro2::TokenStream;

use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

macro_rules! declare_common_tokens {
    (
        with_new[ $( $field_new:ident = $token_new:ident , )* ]
        with_default[ $( $field_default:ident = $token_default:ident , )* ]
        token_streams[ $( $field_ts:ident = $ts_str:expr , )* ]
        types[ $( $field_ty:ident = $ty_str:expr , )* ]
        idents[ $( $field_ident:ident = $ident_str:expr , )* ]
    ) => {
        use syn::token::{
            $($token_new,)*
            $($token_default,)*
        };

        #[derive(Debug)]
        pub struct CommonTokens{
            $( pub $field_new : $token_new , )*
            $( pub $field_default : $token_default , )*
            $( pub $field_ts : TokenStream , )*
            $( pub $field_ty : ::syn::Type , )*
            $( pub $field_ident : ::syn::Ident , )*
        }

        impl CommonTokens{
            pub fn new()->Self{
                let span=Span::call_site();
                Self{
                    $( $field_new : $token_new::new(span) , )*
                    $( $field_default : Default::default() , )*
                    $( $field_ts : ::syn::parse_str($ts_str).unwrap() , )*
                    $( $field_ty : ::syn::parse_str($ty_str).unwrap() , )*
                    $( $field_ident : ::syn::parse_str($ident_str).unwrap() , )*
                }
            }
        }
    }
}

impl Default for CommonTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl Eq for CommonTokens {}
impl PartialEq for CommonTokens {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl PartialOrd for CommonTokens {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl Ord for CommonTokens {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

declare_common_tokens!{
    with_new[
    ]

    with_default[
        lt=Lt,
        gt=Gt,
        comma=Comma,
        colon=Colon,
        colon2=Colon2,
        add=Add,
        semicolon=Semi,
        pub_=Pub,
        super_=Super,
        crate_=Crate,
        in_=In,
        use_=Use,
        low_self=Self_,
        cap_self=CapSelf,
        brace=Brace,
        bracket=Bracket,
        paren=Paren,
    ]

    token_streams[
        doc_hidden=r#"#[cfg_attr(not(feature="priv_docs"),doc(hidden))]"#,
        allow_unused_imports="#[allow(unused_imports)]",
        priv_trait="__PrivTrait",
    ]

    types[
        priv_struct="__IsPriv",
    ]

    idents[
        fields_mod="fields",
        variants_mod="variants",
        dund_fields_mod="__fields",
        priv_mod="__private_mod",
    ]
}
