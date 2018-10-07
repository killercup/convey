//! Custom derives for `output.rs`

#![recursion_limit = "1024"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields};

#[proc_macro_derive(RenderOutput)]
pub fn render_output(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let fields: Vec<_> = match ast.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(..) => s
                .fields
                .iter()
                .map(|x| {
                    let x = x.ident.clone().unwrap();
                    (x.to_string(), quote!(#x))
                }).collect(),
            Fields::Unnamed(..) => s
                .fields
                .iter()
                .enumerate()
                .map(|(i, _)| (i.to_string(), quote!(#i)))
                .collect(),
            _ => panic!("Unit structs not supported for now, sorry."),
        },
        _ => panic!("Only structs not supported for now, sorry."),
    };
    let names = fields.iter().map(|(name, _)| name);
    let fields = fields.iter().map(|(_, ident)| ident);
    let exp = quote! {
        impl output::Render for #name {
            fn render_for_humans(&self, fmt: &mut output::human::Formatter) -> Result<(), output::Error> {
                let mut span = output::components::span();
                #(
                    span = span.add_item(#names);
                    span = span.add_item(": ");
                    span = span.add_item(output::components::text(&self.#fields.to_string()));
                    span = span.add_item("\n");
                )*
                span.render_for_humans(fmt)?;
                Ok(())
            }

            fn render_json(&self, fmt: &mut output::json::Formatter) -> Result<(), output::Error> {
                fmt.write(self)?;
                Ok(())
            }
        }
    };

    TokenStream::from(exp)
}
