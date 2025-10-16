use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(AutoNew)]
pub fn auto_new(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input.clone()).unwrap();

    let name = &ast.ident;

    if let syn::Data::Struct(strc) = &ast.data {
        if let syn::Fields::Named(field) = &strc.fields {
            let field_names = field.named.iter().map(|f|{&f.ident});
            let field_type = field.named.iter().map(|f|{&f.ty});

            let field_names_clone = field_names.clone();

            let to_return = quote! {
                impl #name {
                    pub fn new(#(#field_names: #field_type),*) -> Self {
                        return #name { #(#field_names_clone),* };
                    }
                }
            };
            return to_return.into();
        }
    }

    input
}
