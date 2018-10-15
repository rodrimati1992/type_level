
use syn;
use quote::ToTokens;

use core_extensions::prelude::*;

use std::fmt::{self,Write};

pub(crate) fn display_totokens_list<I>(val:I)->AlwaysDisplay<String>
where 
    I:IntoIterator,
    I::Item:ToTokens,
{   
    let mut buffer=String::new();
    for elem in val{
        write_tokens(elem,&mut buffer);
        buffer.push('\n');
    } 
    AlwaysDisplay( buffer )
}

pub(crate) fn display_totokens<T>(val:&T)->AlwaysDisplay<String>
where T:ToTokens
{    
    AlwaysDisplay( tokens_to_string(val) )
}

pub(crate) fn tokens_to_string<T>(val:T)->String
where T:ToTokens
{
    String::new().mutated(|buff| write_tokens(val,buff) )
}

pub(crate) fn write_tokens<T>(val:T,buffer:&mut String)
where T:ToTokens
{
    write!(buffer,"{}",val.into_token_stream()).drop_()
}


pub(crate) struct AlwaysDisplay<T>(T);

impl<T> fmt::Debug for AlwaysDisplay<T>
where
    T:fmt::Display
{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        fmt::Display::fmt(&self.0,f)
    }
}
