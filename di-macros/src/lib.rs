extern crate proc_macro;
use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Injectable)]
pub fn injectable_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_injectable_macro(&ast)
}

fn impl_injectable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Injectable for #name {
            fn inject(self, c: &mut Container) {
                c.svcs.insert(self.type_id(), Box::new(self));
            }
        }
    };
    gen.into()
}
