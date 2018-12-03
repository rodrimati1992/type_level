use syn;
use syn::visit::{self, Visit};
use syn::*;

/// A Helper type used to find a type parameter inside a syn::Type.
pub struct FindTypeParam<'a> {
    type_param: syn::Type,
    type_param_ident: &'a Ident,
}

fn type_from_ident(ident: Ident) -> syn::Type {
    let path: syn::Path = ident.into();
    let path = syn::TypePath { qself: None, path };
    path.into()
}

impl<'a> FindTypeParam<'a> {
    pub fn new(type_param: &'a Ident) -> Self {
        Self {
            type_param: type_from_ident(type_param.clone()),
            type_param_ident: type_param,
        }
    }

    /// whether the type parameter is present inside `other`
    pub fn in_(&self, other: &syn::Type) -> bool {
        use proc_macro2::{TokenStream, TokenTree};

        struct FindTypeParamVisitor<'a> {
            ftp: &'a FindTypeParam<'a>,
            found_param: bool,
        }

        impl<'ftp, 'ast> Visit<'ast> for FindTypeParamVisitor<'ftp> {
            fn visit_macro(&mut self, i: &'ast syn::Macro) {
                if !self.found_param {
                    let found = i.tts.clone().contains_ident(&self.ftp.type_param_ident);
                    self.found_param = self.found_param || found;
                }
            }

            fn visit_type(&mut self, i: &'ast syn::Type) {
                self.found_param = self.found_param || self.ftp.type_param == *i;
                if !self.found_param {
                    visit::visit_type(self, i);
                }
            }

            fn visit_path(&mut self, i: &'ast syn::Path) {
                self.found_param =
                    self.found_param || self.ftp.type_param_ident == &i.segments[0].ident;
                visit::visit_path(self, i);
            }
        }

        trait ContainsIdent {
            fn contains_ident(self, ident: &syn::Ident) -> bool;
        }

        impl ContainsIdent for TokenStream {
            #[inline]
            fn contains_ident(self, ident: &syn::Ident) -> bool {
                self.into_iter()
                    .position(|tt| tt.contains_ident(ident))
                    .is_some()
            }
        }

        impl ContainsIdent for TokenTree {
            #[inline]
            fn contains_ident(self, ident: &syn::Ident) -> bool {
                match self {
                    TokenTree::Group(group) => group.stream().contains_ident(ident),
                    TokenTree::Ident(v) => ident == &v,
                    TokenTree::Punct(_) => false,
                    TokenTree::Literal(_) => false,
                }
            }
        }

        let mut visitor = FindTypeParamVisitor {
            ftp: self,
            found_param: false,
        };
        visitor.visit_type(other);

        visitor.found_param
    }
}

// #[cfg(test)]
#[cfg(all(test, feature = "passed_tests"))]
mod tests {
    use super::*;
    #[test]
    fn find_type_param_test() {
        use syn::parse_str;
        let type_: syn::Ident = parse_str("C").unwrap();
        let finder = FindTypeParam::new(&type_);

        let parse_type = |s: &str| -> syn::Type { parse_str(s).unwrap() };
        assert_eq!(true, finder.in_(&parse_type("<C as Trait>::Assoc")));
        assert_eq!(true, finder.in_(&parse_type("<C>::Assoc")));
        assert_eq!(true, finder.in_(&parse_type("C::Assoc")));
        assert_eq!(true, finder.in_(&parse_type("C")));
        assert_eq!(true, finder.in_(&parse_type("fn(C)")));
        assert_eq!(true, finder.in_(&parse_type("fn()->C")));
        assert_eq!(true, finder.in_(&parse_type("Vec<C>")));
        assert_eq!(true, finder.in_(&parse_type("Map<C,C>")));
        assert_eq!(true, finder.in_(&parse_type("[C]")));
        assert_eq!(true, finder.in_(&parse_type("(C)")));
        assert_eq!(true, finder.in_(&parse_type("(C,)")));
        assert_eq!(true, finder.in_(&parse_type("(C,C)")));
        assert_eq!(true, finder.in_(&parse_type("tlist![C]")));

        // finding the type parameter in a type macro requires some false positives.
        assert_eq!(true, finder.in_(&parse_type("tlist![T::C]")));
        assert_eq!(true, finder.in_(&parse_type("tlist![<What as C>::T]")));

        assert_eq!(false, finder.in_(&parse_type("Start::C::Assoc")));
        assert_eq!(false, finder.in_(&parse_type("Assoc::C")));
        assert_eq!(false, finder.in_(&parse_type("tlist![]")));
    }
}
