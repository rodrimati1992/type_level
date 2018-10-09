use syn::Visibility;
use syn::Path;
use quote::ToTokens;
use proc_macro2::TokenStream;

use common_tokens::CommonTokens;

use core_extensions::prelude::*;

use std::cmp::{PartialOrd,Ordering};


#[derive(Copy,Clone,Debug)]
pub(crate) struct RelativeVis<'a>{
    common_tokens:&'a CommonTokens,
    visibility_kind:VisibilityKind<'a>,
    nesting:u8,
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub(crate) enum VisibilityKind<'a>{
    Private,
    /// 'super' is 1,'super::super is 2,etc.
    Super{
        nth_supermod:usize,
    },
    Absolute(&'a Path),
    Crate,
    Public,
}

#[derive(Copy,Clone,Debug,Eq,PartialEq,PartialOrd)]
pub struct MyVisibility<'a>{
    common_tokens:&'a CommonTokens,
    pub(crate)kind:VisibilityKind<'a>,
}



impl<'a> MyVisibility<'a>{
    pub(crate) fn new(vis:&'a Visibility,common_tokens:&'a CommonTokens)->Self{
        let kind=match vis {
            &Visibility::Public{..}=>{
                VisibilityKind::Public
            }
            &Visibility::Crate{..}=>{
                VisibilityKind::Crate
            }
            &Visibility::Inherited{..}=>{
                VisibilityKind::Private
            }
            &Visibility::Restricted(ref restricted)=>{
                let path=&restricted.path;
                let is_global=restricted.path.leading_colon.is_some();
                let path_seg_0=path.segments.first().unwrap();
                let path_seg_0=path_seg_0.value();
                let is_crate=path_seg_0.ident=="crate";
                if is_global || is_crate {
                    if is_crate && path.segments.len()==1 {
                        VisibilityKind::Crate
                    }else{
                        VisibilityKind::Absolute(path)
                    }
                }else if path_seg_0.ident=="self" {
                    assert!( 
                        path.segments.len()==1,
                        "paths in pub(...) that start with 'self' \
                         must not be followed by anything:\n{:?}.\n",
                        path
                    );

                    VisibilityKind::Private
                }else if path_seg_0.ident=="super" {
                    assert!( 
                        path.segments.iter().all(|segment| segment.ident=="super" ) ,
                        "paths in pub(...) that start with 'super' \
                         must only be followed by 'super':\n{:?}.\n",
                        path
                    );

                    VisibilityKind::Super{
                        nth_supermod:path.segments.len(),
                    }
                }else{
                    VisibilityKind::Absolute(path)
                }
            }
        };

        Self{
            common_tokens,
            kind,
        }
    }


    /// Returns a type which outputs the visibility for items in \[sub-\]modules.
    ///
    /// nesting==0 means the module deriving this trait
    ///
    /// nesting==1 means the module bellow that.
    pub(crate) fn submodule_level(self,nesting:u8)->RelativeVis<'a>{
        RelativeVis{
            common_tokens:self.common_tokens,
            visibility_kind:self.kind,
            nesting,
        }
    }
}

impl<'a> ToTokens for RelativeVis<'a>{
    fn to_tokens(&self,tokens:&mut TokenStream){
        let c_tokens=self.common_tokens;

        if self.visibility_kind==VisibilityKind::Private && self.nesting==0 {
            return;
        }

        match self.visibility_kind {
            VisibilityKind::Private | VisibilityKind::Super{..} =>{
                let supermod=match self.visibility_kind {
                    VisibilityKind::Private=>0,
                    VisibilityKind::Super{nth_supermod}=>nth_supermod,
                    _=>unreachable!(),
                };
                
                let nesting=supermod + self.nesting as usize;

                c_tokens.pub_.to_tokens(tokens);
                c_tokens.paren.surround(tokens,|tokens|{
                    c_tokens.in_.to_tokens(tokens);
                    
                    let mut iter=(0..nesting).peekable();
                    while iter.next().is_some() {
                        c_tokens.super_.to_tokens(tokens);
                        if iter.peek().is_some() {
                            c_tokens.colon2.to_tokens(tokens);
                        }
                    }
                });
            }
            VisibilityKind::Absolute(path)=>{
                c_tokens.pub_.to_tokens(tokens);
                c_tokens.paren.surround(tokens,|tokens|{
                    c_tokens.in_.to_tokens(tokens);
                    path.to_tokens(tokens);
                });
            }
            VisibilityKind::Crate=>{
                c_tokens.pub_.to_tokens(tokens);
                c_tokens.paren.surround(tokens,|tokens|{
                    c_tokens.crate_.to_tokens(tokens);
                });
            }
            VisibilityKind::Public=>{
                c_tokens.pub_.to_tokens(tokens);
            }
        }
    }
}





#[derive(Copy,Clone,Debug)]
pub(crate) struct DocHiddenAttr<'a>{
    common_tokens:&'a CommonTokens,
    is_public:bool,
}


#[derive(Copy,Clone)]
pub(crate) struct IsPublic(pub bool);


impl<'a> DocHiddenAttr<'a>{
    pub fn new(is_public:IsPublic,common_tokens:&'a CommonTokens)->Self{
        Self{
            is_public:is_public.0,
            common_tokens,
        }
    }
}

impl<'a> ToTokens for DocHiddenAttr<'a>{
    fn to_tokens(&self,tokens:&mut TokenStream){
        if !self.is_public {
            self.common_tokens.doc_hidden.to_tokens(tokens);
        }
    }
}





////////////////////////////////////////////////////////////////////////////////


#[derive(Copy,Clone,Debug,Ord,PartialOrd,Eq,PartialEq)]
enum VKDiscr{
    Private,
    Super,
    Absolute,
    Crate,
    Public,
}

impl<'a> VisibilityKind<'a>{
    fn to_discriminant(&self)->VKDiscr{
        match *self {
            VisibilityKind::Private {..}=>VKDiscr::Private,
            VisibilityKind::Super   {..}=>VKDiscr::Super,
            VisibilityKind::Absolute{..}=>VKDiscr::Absolute,
            VisibilityKind::Crate   {..}=>VKDiscr::Crate,
            VisibilityKind::Public  {..}=>VKDiscr::Public,
        }
    }
}

impl<'a> PartialOrd for VisibilityKind<'a>{
    fn partial_cmp(&self,other:&Self)->Option<Ordering>{
        use self::VisibilityKind as VK;
        
        match self.to_discriminant().cmp(&other.to_discriminant()) {
            expr@Ordering::Less|expr@Ordering::Greater=>return Some(expr),
            _=>{}
        }
        
        match (self,other) {
            (&VK::Super{nth_supermod:nth0},&VK::Super{nth_supermod:nth1})=>
                nth0.partial_cmp(&nth1),
            (&VK::Absolute(path0),&VK::Absolute(path1))=>{
                if path0.segments.iter().zip(&path1.segments).all(|(l,r)|l.ident==r.ident) {
                    path0.segments.len().cmp(&path1.segments.len())
                        .reverse()
                        .piped(Some)
                }else{
                    None
                }
            }
            _=>Some(Ordering::Equal),
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_ordering(){
        let common_tokens=CommonTokens::new();

        macro_rules! new_visibility {
            (   
                $ident:ident=$string:expr
            ) => (
                let $ident:Visibility=syn::parse_str($string).expect($string);
                let $ident=MyVisibility::new(&$ident,&common_tokens).kind;
            )
        }

        new_visibility!{vis_self="pub(self)"}
        new_visibility!{vis_self_b=""}

        new_visibility!{vis_super="pub(super)"}
        new_visibility!{vis_super_1="pub(in super::super)"}
        new_visibility!{vis_super_2="pub(in super::super::super)"}
        
        new_visibility!{vis_mod1_mod2="pub(in  ::mod1::mod2)"}
        new_visibility!{vis_mod1_mod2_mod4="pub(in  ::mod1::mod2::mod4)"}
        new_visibility!{vis_mod1_mod3="pub(in ::mod1::mod3)"}
        new_visibility!{vis_mod1="pub(in ::mod1)"}
        
        new_visibility!{vis_crate="crate"}
        new_visibility!{vis_crate_1="pub(crate)"}
        
        new_visibility!{vis_pub="pub"}



        assert_eq!(vis_self,vis_self_b);
        assert_eq!(vis_crate,vis_crate_1);

        assert_eq!(vis_self.partial_cmp(&vis_super  ) , Some(Ordering::Less) );
        assert_eq!(vis_self.partial_cmp(&vis_super_1) , Some(Ordering::Less) );
        assert_eq!(vis_self.partial_cmp(&vis_super_2) , Some(Ordering::Less) );
        assert_eq!(vis_self.partial_cmp(&vis_mod1     ) , Some(Ordering::Less) );
        assert_eq!(vis_self.partial_cmp(&vis_mod1_mod2) , Some(Ordering::Less) );
        assert_eq!(vis_self.partial_cmp(&vis_crate    ) , Some(Ordering::Less) );
        assert_eq!(vis_self.partial_cmp(&vis_pub    ) , Some(Ordering::Less) );

        assert_eq!(vis_super  .partial_cmp(&vis_super_1), Some(Ordering::Less) );
        assert_eq!(vis_super  .partial_cmp(&vis_super_2), Some(Ordering::Less) );
        assert_eq!(vis_super_1.partial_cmp(&vis_super_2), Some(Ordering::Less) );
        assert_eq!(vis_super_2.partial_cmp(&vis_mod1     ) , Some(Ordering::Less) );
        assert_eq!(vis_super_2.partial_cmp(&vis_mod1_mod2) , Some(Ordering::Less) );
        assert_eq!(vis_super_2.partial_cmp(&vis_crate    ) , Some(Ordering::Less) );
        assert_eq!(vis_super_2.partial_cmp(&vis_pub    ) , Some(Ordering::Less) );
            
        assert_eq!(vis_mod1_mod2_mod4.partial_cmp(&vis_mod1_mod2) , Some(Ordering::Less) );
        assert_eq!(vis_mod1_mod2.partial_cmp(&vis_mod1_mod2_mod4) , Some(Ordering::Greater) );

        assert_eq!(vis_mod1_mod2.partial_cmp(&vis_mod1) , Some(Ordering::Less) );
        assert_eq!(vis_mod1_mod3.partial_cmp(&vis_mod1) , Some(Ordering::Less) );
        assert_eq!(vis_mod1_mod3.partial_cmp(&vis_mod1_mod2) , None );
            
        assert_eq!(vis_mod1.partial_cmp(&vis_crate) , Some(Ordering::Less) );
        assert_eq!(vis_mod1.partial_cmp(&vis_pub) , Some(Ordering::Less) );

        assert_eq!(vis_crate.partial_cmp(&vis_pub) , Some(Ordering::Less) );
    } 
}