use proc_macro::{TokenStream, TokenTree};

extern crate proc_macro;

#[proc_macro]
pub fn make_answer(item: TokenStream) -> TokenStream {}
