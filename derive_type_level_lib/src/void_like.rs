//! Contains types and functions for impossible situations.



use std::{
    fmt,
    cmp,
};

/// Type for impossible situations.
///
/// Created because of a bug in Rustc where a variant storing Void is somehow constructed.
#[derive(Debug, Copy, Clone, Hash)]
pub struct VoidLike(());


impl VoidLike{
    /// Converts a `VoidLike` to any type.
    ///
    /// Note that because `VoidLike` is impossible to construct,
    /// this method is unreachable.
    pub fn to<T>(self)->T{
        panic!("Cannot instantiate a VoidLike-like")
    }
}

// Conflicts with the built-in `impl Into<T> for T{...}`
// impl<T> Into<T> for VoidLike{
//     fn into(_:Self)->T{
//         self.to()
//     }
// }


#[cfg(std)]
impl ::std::error::Error for VoidLike {
    fn description(&self) -> &str {
        self.to()
    }
}

impl fmt::Display for VoidLike {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        self.to()
    }
}

impl Eq for VoidLike {}

impl<T:?Sized> PartialEq<T> for VoidLike {
    fn eq(&self,_:&T)->bool{
        self.to()
    }
}
impl Ord for VoidLike {
    fn cmp(&self, _: &Self) -> cmp::Ordering {
        self.to()
    }
}
impl<T:?Sized> PartialOrd<T> for VoidLike {
    fn partial_cmp(&self, _: &T) -> Option<cmp::Ordering> {
        self.to()
    }
}


#[cfg(feature = "serde")]
pub use self::serde_impl::DeserializeVoidLikeError;
#[cfg(feature = "serde")]
mod serde_impl{
    use super::*;
    use serde::{Serialize,Deserialize,Serializer,Deserializer};
    use serde::de::Error;

    /// Represents a deserialization error,when trying to deserialize a struct or enum variant 
    /// containing a `VoidLike` field.
    ///
    /// Returned by serde::Deserialize::deserialize every time it's called.
    #[derive(Debug,Copy,Clone)]
    pub struct DeserializeVoidLikeError;

    impl fmt::Display for DeserializeVoidLikeError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("Cant deserialize a struct or \
                        enum variant containing a core_extensions::VoidLike.")
        }
    }

    /// This impl is only enabled if the "serde" feature is enabled.
    ///
    /// This always Returns an `Err(D::Error::custom(DeserializeVoidLikeError))`.
    impl<'de> Deserialize<'de> for VoidLike {
        fn deserialize<D>(_: D) -> Result<Self, D::Error>
            where D: Deserializer<'de>
        {
            Err(D::Error::custom(DeserializeVoidLikeError))
        }
    }

    /// This impl is only enabled if the "serde" feature is enabled.
    ///
    impl Serialize for VoidLike{
        fn serialize<S>(&self,_: S) -> Result<S::Ok, S::Error>
            where S: Serializer
        {
            self.to()
        }
    }
}


