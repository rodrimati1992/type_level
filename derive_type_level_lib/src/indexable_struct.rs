use std::fmt::Debug;
use std::ops::IndexMut;

use syn::Ident;


#[doc(hidden)]
pub mod reexp{
    pub use arrayvec::ArrayString;
    pub use std::str::FromStr;
}

#[derive(Debug)]
pub struct InvalidIndex;

#[derive(Debug)]
pub struct InvalidMultiIndex<I>(pub I);

/// Alias for the traits implemented by all indexable structs.
pub trait IndexableStruct:
    StructIndexType + Default + IndexMut<<Self as StructIndexType>::Index>
{
}

impl<This> IndexableStruct for This 
where This: StructIndexType + Default + IndexMut<Self::Index> 
{}

/// Implementation detail for indexable structs.
pub trait StructIndexType {
    type Index: GetEnumIndices;
}

/// Trait to get enum-indices for an indexable struct.
pub trait GetEnumIndices: Debug + Copy + PartialEq + Eq + 'static {
    /// Gets the indices for `ident`,
    /// returns InvalidMultiIndex on an invalid index.
    fn many_from_ident(ident: &Ident) -> Result<&'static [Self], InvalidMultiIndex<&Ident>>;

    /// Gets the indices for `string`,
    /// returns InvalidMultiIndex on an invalid index.
    fn many_from_str(string: &str) -> Result<&'static [Self], InvalidMultiIndex<&str>>;

    /// A message listing all the indices.
    fn indices_message()->String;

    const INDICES:&'static [Self];
}

#[doc(hidden)]
pub type IndicesMap<IndexEnum> = ::std::collections::BTreeMap<&'static str, &'static [IndexEnum]>;

#[macro_export]
macro_rules! declare_indexable_struct {
    (
        $(#[$index_attrs:meta])*
        enum index=$enum_index:ident

        $(#[$indexable_struct_attrs:meta])*
        struct indexable=$indexable_struct:ident

        variants=[
            $( ($field:ident , $index:ident $(,)* ) ),*
            $(,)*
        ]

        multi_indices=[
            $( ($mul_ind_name:expr , [ $($index_alias:ident),* $(,)* ] $(,)* ) ),*
            $(,)*
        ]
    ) => {


        ////////////////////////////////////////////////////////////////////


        $(#[$index_attrs])*
        #[derive(Debug,Copy,Clone,PartialEq,Eq,Ord,PartialOrd,Hash)]
        pub enum $enum_index{
            $( $index, )*
        }


        impl $crate::indexable_struct::reexp::FromStr for $enum_index{
            type Err=$crate::indexable_struct::InvalidIndex;
            fn from_str(s:&str)->Result<Self,Self::Err>{
                match s {
                    $( stringify!( $index )=>Ok($enum_index::$index), )*
                    _=>Err($crate::indexable_struct::InvalidIndex),
                }
            }
        }

        //////////////////////////////////////////////////////////////////////


        impl $enum_index{
            const VARIANTS:&'static[&'static str]=&[ $( stringify!( $index ) ),* ];
            const IND_ALIASES:&'static [(&'static str,&'static [Self])]=&[
                $( ($mul_ind_name , &[ $( $enum_index::$index_alias ,)* ]) ,)*
            ];

            #[inline]
            fn indices_map()
            ->&'static $crate::indexable_struct::IndicesMap<Self>
            {
                use core_extensions::utils::as_slice;

                lazy_static! {
                    static ref MANY_IMPL_MAP: 
                        $crate::indexable_struct::IndicesMap<$enum_index> 
                    ={
                        [
                            ("all",$enum_index::INDICES),
                        ].iter().cloned()
                        .chain($enum_index::IND_ALIASES.iter().cloned())
                        .chain(
                            $enum_index::VARIANTS.iter().cloned()
                                .zip( $enum_index::INDICES.iter().map(as_slice) )
                        )
                        .collect()
                    };
                }
                &*MANY_IMPL_MAP
            }
        }

        impl $crate::indexable_struct::GetEnumIndices for $enum_index {
            /// Gets the indices for `ident`,
            /// returns InvalidMultiIndex<on an invalid index.
            fn many_from_ident(
                ident: &::syn::Ident
            )-> Result<
                &'static [Self], 
                $crate::indexable_struct::InvalidMultiIndex<&::syn::Ident>
            >{
                #[allow(unused_imports)]
                use std::fmt::Write;
                
                use $crate::indexable_struct::reexp::ArrayString;

                let mut str_: ArrayString<[_; 128]> = ArrayString::new();
                let _ = write!(str_, "{}", ident);

                Self::indices_map()
                    .get(&*str_)
                    .cloned()
                    .ok_or($crate::indexable_struct::InvalidMultiIndex(ident))
            }

            /// Gets the indices for `string`,
            /// returns InvalidMultiIndex<on an invalid index.
            fn many_from_str(
                string: &str
            )-> Result<
                &'static [Self], 
                $crate::indexable_struct::InvalidMultiIndex<&str>
            > {
                Self::indices_map()
                    .get(string)
                    .cloned()
                    .ok_or($crate::indexable_struct::InvalidMultiIndex(string))
            }

            fn indices_message()->String{
                let mut buffer=String::new();
                for key in Self::indices_map().keys() {
                    buffer.push_str(key);
                    buffer.push(' ');
                    buffer.push('/');
                }
                buffer.pop();
                buffer
            }

            const INDICES:&'static [Self]=&[
                $($enum_index::$index,)*
            ];
        }


        //////////////////////////////////////////////////////////////////////


        $(#[$indexable_struct_attrs])*
        /// A generic struct which can be indexed with an associated enum.
        #[derive(Debug,Clone,PartialEq,Eq)]
        pub struct $indexable_struct<T>{
            $(pub $field:T ,)*
        }



        impl<T> $indexable_struct<T>
        where Self:Default,
        {
            #[inline]
            #[allow(dead_code)]
            pub fn new()->Self{
                Default::default()
            }

            #[allow(dead_code)]
            pub fn map<F,U>(self,mut f:F)->$indexable_struct<U>
            where F:FnMut($enum_index,T)->U
            {
                $indexable_struct{
                    $($field:f($enum_index::$index,self.$field),)*
                }
            }
            
            #[allow(dead_code)]
            pub fn to_vec(self)->Vec<($enum_index,T)>{
                vec![
                    $( ( $enum_index::$index , self.$field ) ,)*
                ]
            }
        }


        impl<T> $crate::indexable_struct::StructIndexType 
        for $indexable_struct<T>
        {
            type Index=$enum_index;
        }


        impl<T> ::std::ops::Index<$enum_index> for $indexable_struct<T>{
            type Output=T;
            fn index(&self, index: $enum_index) -> &Self::Output {
                match index {
                    $( $enum_index::$index=>&self.$field, )*
                }
            }
        }

        impl<T> ::std::ops::IndexMut<$enum_index> for $indexable_struct<T>{
            fn index_mut(&mut self, index: $enum_index) -> &mut Self::Output {
                match index {
                    $( $enum_index::$index=>&mut self.$field, )*
                }
            }
        }
    }
}
