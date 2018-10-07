//! Custom derives for `output.rs`

extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate output;

use self::proc_macro::TokenStream;
use output::{human, json, Render};
use syn::DeriveInput;

#[proc_macro_derive(RenderOutput)]
pub fn render_output(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let exp = quote! {
        impl Render for #name {
            fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), Error> {
                unimplemented!()
            }

            fn render_json(&self, fmt: &mut json::Formatter) -> Result<(), Error> {
                unimplemented!()
            }
        }
    };

    TokenStream::from(exp)
}
