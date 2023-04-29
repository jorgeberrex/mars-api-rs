use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, Fields, Field};

#[proc_macro_derive(IdentifiableDocument, attributes(id))]
pub fn identifiable_document_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_identifiable_document_macro(&ast)
}

fn impl_identifiable_document_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data_struct = match &ast.data {
        Data::Struct(data_struct) => data_struct,
        _ => panic!("Automatic derive of IdentifiableDocument only applies to structs!")
    };
    let named_fields = match &data_struct.fields {
        Fields::Named(fields_named) => fields_named,
        _ => panic!("Automatic derive of IdentifiableDocument only supports non-tuple structs")
    };
    let mut id_field : Option<&Field> = None;
    let mut id_count = 0;
    for named_field in named_fields.named.iter() {
        let attrs = &named_field.attrs;
        for attr in attrs.iter() {
            let segments = &attr.path.segments;
            if segments.len() == 1 && segments.first().unwrap().ident.to_string() == "id" {
                id_field = Some(named_field);
                id_count += 1;
            }
        }
    }
    if id_field.is_none() {
        panic!("No ID field present")
    };
    if id_count != 1 {
        panic!("There can only be one identifier")
    };

    let id_ident = &id_field.unwrap().ident;

    let gen = quote! {
        impl IdentifiableDocument for #name {
            fn get_id_value(&self) -> String {
                 self.#id_ident.clone()
            }
        }
    };
    gen.into()
}

