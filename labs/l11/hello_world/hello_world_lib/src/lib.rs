use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

#[proc_macro_derive(Dbg)]
pub fn dbg_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let fields = do_struct(&input.data);
    let result = quote!(
        impl std::fmt::Debug for S {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #fields
                Ok(())
            }
        }
    );
    result.into()
}

fn do_struct(data: &Data) -> TokenStream {
    let mut result = TokenStream::new();
    match data {
        Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => {
                for i in fields.named.iter() {
                    let name = &i.ident;
                    result = quote! {
                        #result
                        write!(f, "{}={},", stringify!(#name), self.#name)?;
                    };
                }
            }
            _ => unimplemented!("only named fields are supported"),
        },
        _ => unimplemented!("only implemented for struct"),
    }

    result
}
