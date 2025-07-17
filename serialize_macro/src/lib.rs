use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Fields};

#[proc_macro_derive(SerializeNumberStruct)]
pub fn serialise_number_struct(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let serialize_fields = match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let field_serializations = fields.named.iter().map(|field| {
                        let field_name = &field.ident;
                        quote! {
                            result.extend_from_slice(&self.#field_name.to_be_bytes());
                        }
                    });
                    /*
                        field_serializeations = [quote!(result.extend_from_slice(&self.qty_1.to_be_bytes())), quote!(result.extend_from_slice(&self.qty_2.to_be_bytes()))]
                     */
                    quote! {
                        #(#field_serializations)*
                    }
                }
                _ => panic!("Only named fields are supported"),
            }
        }
        _ => panic!("Only structs are supported"),
    };
    /*
        serialize_fields ->
        result.extend_from_slice(&self.qty_1.to_be_bytes())
        result.extend_from_slice(&self.qty_2.to_be_bytes())
        result.extend_from_slice(&self.qty_3.to_be_bytes())
     */

    let generated = quote! {
        impl Serialize for #name {
            fn serialize(&self) -> Vec<u8> {
                let mut result = Vec::new();
                #serialize_fields
                result
            }
        }
    };
    generated.into()
}

#[proc_macro_derive(DeserializeNumberStruct)]
pub fn deserialise_number_struct(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let (deserialize_fields, field_assignments, total_size) = match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let mut offset: usize = 0;
                    let mut field_deserializations = Vec::new();
                    let mut field_assignments = Vec::new();
                    
                    for field in &fields.named {
                        let field_name = &field.ident;
                        let field_size = 4;
                        let start_offset = offset;
                        let end_offset = offset + field_size;
                        
                        field_deserializations.push(quote! {
                            let #field_name = {
                                let bytes: [u8; 4] = base[#start_offset..#end_offset]
                                    .try_into()
                                    .map_err(|_| Error)?;
                                i32::from_be_bytes(bytes)
                            };
                        });
                        
                        field_assignments.push(quote! {
                            #field_name
                        });
                        
                        offset += field_size;
                    }
                    
                    (field_deserializations, field_assignments, offset)
                }
                _ => panic!("Only named fields are supported"),
            }
        }
        _ => panic!("Only structs are supported"),
    };

    let generated = quote! {
        impl Deserialize for #name {
            fn deserialize(base: &[u8]) -> Result<Self, Error> {
                if base.len() < #total_size {
                    return Err(Error);
                }
                
                #(#deserialize_fields)*
                
                Ok(#name {
                    #(#field_assignments,)*
                })
            }
        }
    };
    generated.into()
}