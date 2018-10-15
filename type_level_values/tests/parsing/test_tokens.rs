use derive_type_level_lib::common_tokens::CommonTokens;

use syn;
use syn::Ident;

use std::cmp::Ordering;
use std::ops::Deref;
use std::mem;
use std::hash::{Hash,Hasher};

#[derive(Debug)]
pub struct TestTokens{
    pub c_tokens:CommonTokens ,
    pub type_level_mod:Ident, 
}



impl Deref for TestTokens{
    type Target=CommonTokens;
    fn deref(&self)->&Self::Target{
        &self.c_tokens
    }
}
