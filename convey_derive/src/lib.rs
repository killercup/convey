//! Custom derives for `output.rs`
//!
//! # Examples
//!
//! ```rust
//! extern crate convey;
//! #[macro_use] extern crate convey_derive;
//! #[macro_use] extern crate serde_derive;
//!
//! #[derive(Serialize, RenderOutput)]
//! struct Message {
//!     code: i32,
//!     message: String,
//! }
//!
//! # fn main() -> Result<(), convey::Error> {
//! # use convey::human;
//! # let test_target = human::test();
//! let mut out = convey::new().add_target(test_target.target());
//! out.print(&Message {
//!     code: 42,
//!     message: String::from("Derive works"),
//! })?;
//! # out.flush()?;
//! # assert_eq!(test_target.to_string(), "code: 42\nmessage: Derive works\n\n");
//! # Ok(()) }
//! ```

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
    let render_span = match ast.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(..) => {
                let fields = s.fields.iter().map(|f| {
                    let x = f.ident.clone().unwrap();
                    quote!(#x)
                });
                let names = s
                    .fields
                    .iter()
                    .map(|f| f.ident.clone().unwrap().to_string());
                quote! {
                    let mut span = convey::components::span();
                    #(
                        span = span.add_item(#names);
                        span = span.add_item(": ");
                        span = span.add_item(convey::components::text(&self.#fields.to_string()));
                        span = span.add_item("\n");
                    )*
                    span.render_for_humans(fmt)?;
                }
            }
            Fields::Unnamed(..) => {
                let field_count = s.fields.iter().count();
                let fields = (0..field_count)
                        .fold(Vec::new(), |mut res, i| {
                            res.push(quote! { span = span.add_item(convey::components::text(&self.#i.to_string())); });
                            if i < field_count - 1 {
                                res.push(quote! { span = span.add_item(", "); });
                            }
                            res
                        });

                quote! {
                    let mut span = convey::components::span();
                    span = span.add_item("(");
                    #(#fields)*
                    span = span.add_item(")");
                    span.render_for_humans(fmt)?;
                }
            }
            _ => panic!("Unit structs not supported for now, sorry."),
        },
        _ => panic!("Only structs supported for now, sorry."),
    };
    let exp = quote! {
        impl convey::Render for #name {
            fn render_for_humans(&self, fmt: &mut convey::human::Formatter) -> Result<(), convey::Error> {
                #render_span
                Ok(())
            }

            fn render_json(&self, fmt: &mut convey::json::Formatter) -> Result<(), convey::Error> {
                fmt.write(self)?;
                Ok(())
            }
        }
    };

    TokenStream::from(exp)
}
