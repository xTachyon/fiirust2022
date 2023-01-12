# Recap

Declarative macros:
```rs
macro_rules! println_hex {
    ($num:expr) => {
        println!("{:x}", $num);
    };
}

println_hex!(5);
println_hex!(100);
```

Procedural macros helpers:
1. syn: this library parses the tokens into a mini AST that can be used more easily
2. proc_macro2: extends the functionality of the `proc_macro` crate offered by the compiler
3. quote: easily transform hardcoded code into a token stream

Example (the full impl [here](hello_world)):
```rs
use proc_macro2::TokenStream;
use syn::{DeriveInput, Data};
use quote::quote;

// Implement a derive macro named Dbg
// This takes a TokenStream with the tokens of the type, and returns another TokenStream with the tokens to be added to the code
#[proc_macro_derive(Dbg)]
pub fn dbg_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let fields = do_struct(&input.data);

    // Implement the Debug trait with a custom implementation
    let result = quote!(
        impl std::fmt::Debug for S {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // will be replaced by the fields variable
                #fields
                Ok(())
            }
        }
    );
    result.into()
}

fn do_struct(data: &Data) -> TokenStream {
    let mut result = TokenStream::new();

    // A derived type can be a struct, an enum or an union. We're only interested in the struct
    match data {
        Data::Struct(data) => match &data.fields {
            // A struct's fields can be named, unnamed or missing. Only deal with the named fields
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
```

# Problem

Go back to the problem at [L10](../l10/readme.md). Use the following trait:
```rs
trait ArchiveSerializer {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()>;
}
```
This trait is supposed to be implemented for all the types that can be written into an archive:
- for buffer and string, implement the trait manually
- for numbers, make a declarative macro that takes a type as argument, and implements the trait using the `to_ne_bytes` function to turn the number into bytes
- for structs, make a procedural macro that iterates over the fields and calls the `serialize` function for each field

For this `struct`:
```rs
struct S {
    x: u32,
    y: String,
}
```
the generated code of the proc macro should look something like:
```rs
impl ArchiveSerializer for S {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.x.serialize(writer)?;
        self.y.serialize(writer)?;
        Ok(())
    }
}
```