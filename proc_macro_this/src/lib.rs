use proc_macro::{Ident, TokenStream, TokenTree};

extern crate proc_macro;

#[proc_macro]
pub fn p_macro(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    while let Some(token) = tokens.next() {
        match token {
            TokenTree::Group(g) => {
                println!("Group: {:?}", g);
            }
            TokenTree::Ident(i) => {
                println!("Ident: {}", i);
                if i.to_string() == "x" {}
            }
            TokenTree::Punct(p) => {
                println!("Punct: {}", p)
            }
            TokenTree::Literal(l) => {
                println!("Literal: {}", l)
            }
        }
    }
    TokenStream::new()
}
