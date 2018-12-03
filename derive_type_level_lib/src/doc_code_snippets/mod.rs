//! Derive macro used to geenerate documentation which includes snippets of code
//! taken from a larger segment of code.
//!
//!
//! # Markers in the code
//!
//! Takes the code between the start and end markers,all blocks of different names are independent.
//!
//! Syntax:
//!
//! `//@codeblock-(start | end):<name_of_block>,  `
//!
//! - `start` is the start of the block.
//!
//! - `end` is the end of the block.
//!
//! ex:`//@codeblock-start:all`
//! ex:`//@codeblock-end  :struct declaraction`
//!
//!
//! # syntax for markers in the documentation
//!
//! `//@use_codeblock:<name_of_block>, ignore | rust `
//!
//! - `ignore` means that this code won't be tested
//!
//! - `rust` means that this code will be tested as rust code
//!

use std::collections::{BTreeMap, BTreeSet};
use std::iter;

use syn::{self, DeriveInput, Lit, Meta, MetaNameValue, NestedMeta};

use proc_macro2::TokenStream;

use core_extensions::SelfOps;
use core_extensions::SliceExt;
use core_extensions::StringExt;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) enum CodeLineType<'a> {
    Marker {
        name: &'a str,
        position: MarkerPosition,
    },
    Line(&'a str),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) enum MarkerPosition {
    Start,
    End,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct CodeBlock<'a> {
    name: &'a str,
    start: usize,
    end: usize,
}

fn classify_lines<'a>(text: &'a str) -> Vec<CodeLineType<'a>> {
    text.lines()
        .enumerate()
        .map(|(line_number, line)| {
            const START_OF_MARKER: &'static str = "//@codeblock-";
            let untrimmed = line;
            let line = line.trim_left();
            if line.starts_with(START_OF_MARKER) {
                let mut words = line[START_OF_MARKER.len()..].split(':').map(|s| s.trim());
                let position = words.next().unwrap_or_else(|| {
                    panic!(
                        "\n\nno marker position provided on line {}\n\n",
                        line_number
                    )
                });
                let position = match position {
                    "start" => MarkerPosition::Start,
                    "end" => MarkerPosition::End,
                    _ => panic!(
                        "\n\nInvalid marker position '{}',\
                         must be either 'start' or 'end' on line {} \n\n",
                        position, line_number
                    ),
                };
                let name = words.next().unwrap_or_else(|| {
                    panic!("\n\nno marker name provided on line {}\n\n", line_number)
                });
                CodeLineType::Marker { name, position }
            } else {
                CodeLineType::Line(untrimmed)
            }
        }).chain(iter::once(CodeLineType::Line(&text[text.len()..])))
        .collect()
}

fn code_blocks<'a>(lines: &[CodeLineType<'a>]) -> BTreeMap<&'a str, CodeBlock<'a>> {
    let mut code_blocks = BTreeMap::<&'a str, CodeBlock<'a>>::new();
    let mut code_block_names = BTreeSet::<&'a str>::new();

    for (line_number, line_type) in lines.iter().enumerate() {
        match *line_type {
            CodeLineType::Marker {
                name,
                position: MarkerPosition::Start,
            } => {
                let prev_code_block = code_blocks.insert(
                    name,
                    CodeBlock {
                        name,
                        start: line_number,
                        end: lines.len() - 1,
                    },
                );
                if let Some(prev_code_block) = prev_code_block {
                    panic!(
                        "\n\ncan't reuse code block named:'{}' on line {}\n\n",
                        prev_code_block.name, line_number
                    );
                }
                code_block_names.insert(name);
            }
            CodeLineType::Marker {
                name,
                position: MarkerPosition::End,
            } => {
                let code_block = code_blocks.get_mut(name).unwrap_or_else(|| {
                    panic!(
                        "\n\nInvalid code block name:{}\nmust be one of:{}\n\n",
                        name,
                        string_alternatives(code_block_names.iter().cloned())
                    )
                });
                code_block.end = line_number;
            }
            CodeLineType::Line { .. } => {}
        }
    }
    code_blocks
}

fn expand_template(module_name: &str, code: &str, template: &str) -> String {
    use std::fmt::Write;

    let code_lines = classify_lines(code);
    let blocks = code_blocks(&code_lines);

    let mut buffer = String::with_capacity(template.len() + code.len());

    let mut contiguous_empty_lines = 0;

    let _ = writeln!(
        buffer,
        "#[allow(dead_code)]\n\
         pub mod {} {{",
        module_name
    );
    for (line_number, line) in template.lines().enumerate() {
        const START_OF_MARKER: &'static str = "//@use_codeblock:";
        let untrimmed = line;
        let line = line.trim_left();
        if line.starts_with(START_OF_MARKER) {
            let mut words = line[START_OF_MARKER.len()..].split(',').map(|s| s.trim());
            let name = words.next().unwrap_or_else(|| {
                panic!(
                    "\n\nNo code block name provided on line {}\n\n",
                    line_number
                )
            });

            let code_block = blocks.get(name).unwrap_or_else(|| {
                panic!(
                    "\n\nInvalid code block name:{}\nmust be one of:{}\n\n",
                    name,
                    string_alternatives(blocks.keys().cloned())
                )
            });
            let code_block_type = words.next().unwrap_or_else(|| {
                panic!(
                    "\n\ncode block type provided for marker '{}', \n\
                     must be either 'ignore'/'rust' ,\n\
                     on line {}\n\n",
                    name, line_number
                )
            });

            buffer.push_str("//! ```");
            buffer.push_str(&code_block_type);
            buffer.push('\n');

            let lines = &code_lines[code_block.start..(code_block.end + 1)];

            let minimum_indentation = lines
                .iter()
                .filter_map(|line| match line {
                    &CodeLineType::Line(code_line) if !code_line.trim().is_empty() => {
                        Some(code_line.line_indentation())
                    }
                    _ => None,
                }).min()
                .unwrap_or(0);

            for code_line in lines {
                if let CodeLineType::Line(code_line) = *code_line {
                    let code_line = code_line.slice_lossy(minimum_indentation..code_line.len(), ());
                    if code_line.trim().is_empty() {
                        contiguous_empty_lines += 1;
                    } else {
                        contiguous_empty_lines = 0;
                    }
                    if contiguous_empty_lines <= 2 {
                        buffer.push_str("//! ");
                        buffer.push_str(code_line);
                        buffer.push('\n');
                    }
                }
            }
            buffer.push_str("//! ```\n");
        } else {
            buffer.push_str("//! ");
            buffer.push_str(untrimmed);
            buffer.push('\n');
        }
    }
    buffer.push_str("}\n");
    buffer
}

pub fn derive_from_derive_input(ast: DeriveInput) -> TokenStream {
    let mut code = None::<String>;
    let mut module_name = None::<String>;
    let mut template = None::<String>;

    for attr in ast.attrs {
        let meta_list = match attr.interpret_meta() {
            Some(Meta::List(meta_list)) => if meta_list.ident == "doccode" {
                meta_list
            } else {
                continue;
            },
            _ => continue,
        };

        for nested0 in meta_list.nested {
            match nested0 {
                NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                    ident,
                    lit: Lit::Str(str_),
                    ..
                })) => {
                    let some_ = Some(str_.value());
                    if ident == "code" {
                        code = some_;
                    } else if ident == "mod_" {
                        module_name = some_;
                    } else if ident == "template" {
                        template = some_;
                    }
                }
                v => panic!("unrecognized attribute:{:#?}", v),
            }
        }
    }

    let code = code.expect(r#"did not pass the #[doccode(code="...")] parameter."#);
    let module_name = module_name.expect(r#"did not pass the #[doccode(mod_="...")] parameter."#);
    let template = template.expect(r#"did not pass the #[doccode(template="...")] parameter."#);

    expand_template(&module_name, &code, &template)
        .parse::<::proc_macro2::TokenStream>()
        .unwrap()
        .into_(TokenStream::T)
}

pub fn derive_from_str(input: &str) -> TokenStream {
    let ast: DeriveInput = syn::parse_str(input).unwrap();
    derive_from_derive_input(ast)
}

pub fn derive_from_token_stream(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(input).unwrap();
    derive_from_derive_input(ast)
}

fn string_alternatives<'a, I>(iter: I) -> String
where
    I: Iterator<Item = &'a str>,
{
    let mut peek = iter.peekable();
    let mut buffer = String::new();

    while let Some(elem) = peek.next() {
        buffer.push_str(elem);
        if peek.peek().is_some() {
            buffer.push('/');
        }
    }

    buffer
}
