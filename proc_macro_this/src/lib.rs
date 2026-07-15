use proc_macro::TokenStream;

extern crate proc_macro;

#[proc_macro_attribute]
pub fn derive_generics(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    item
}
