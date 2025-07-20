use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input,DeriveInput, Data, Fields};

#[proc_macro_derive(SerializeNumberStruct)]
pub fn serialize_number_struct(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  impl_serialize(&ast).into()
}

#[proc_macro_derive(DeserializeNumberStruct)]
pub fn deserialize_number_struct(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  impl_deserialize(&ast).into()
}

fn impl_serialize(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
  let name = &ast.ident;

  let fields = match &ast.data {
    Data::Struct(data_struct) => match &data_struct.fields {
      Fields::Named(fields_named) => &fields_named.named,
      _ => panic!("SerializeNumberStruct only works with named fields."),
    },
    _ => panic!("SerializeNumberStruct only works with structs."),
  };

  let serialize_fields = fields.iter().map(|f| {
    let ident = &f.ident;
    quote! {
      serialize_macro_traits::Serialize::serialize(&self.#ident, writer)?;
    }
  });

  quote! {
    impl serialize_macro_traits::Serialize for #name {
      fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        #(#serialize_fields)*
        Ok(())
      }
    }
  }
}

fn impl_deserialize(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
  let name = &ast.ident;

  let fields = match &ast.data {
    Data::Struct(data_struct) => match &data_struct.fields {
      Fields::Named(fields_named) => &fields_named.named,
      _ => panic!("DeserializeNumberStruct only works with named fields."),
    },
    _ => panic!("DeserializeNumberStruct only works with structs."),
  };

  let deserialize_fields = fields.iter().map(|f| {
    let ident = &f.ident;
    let ty = &f.ty;
    quote! {
      #ident: <#ty as serialize_macro_traits::Deserialize>::deserialize(reader)?,
    }
  });

  quote! {
    impl serialize_macro_traits::Deserialize for #name {
      fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        Ok(Self {
          #(#deserialize_fields)*
        })
      }
    }
  }
}
