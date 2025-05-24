use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse::Parse, Attribute, Ident, LitStr};

pub fn get_attribute<T: Parse>(attrs: &[Attribute], name: &str) -> Option<T> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident(name))
        .and_then(|attr| attr.parse_args::<T>().ok())
}

pub fn is_vec_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Vec";
        }
    }
    false
}

pub fn is_option_type(ty: &syn::Type) -> bool {
    let syn::Type::Path(type_path) = ty else {
        return false;
    };

    if let Some(segment) = type_path.path.segments.last() {
        return segment.ident == "Option";
    }

    false
}

fn get_inner_type(ty: &syn::Type) -> String {
    let str = ty.to_token_stream().to_string().replace(" ", "");

    if str.starts_with("Vec<") {
        str.replace("Vec<", "").replace(">", "")
    } else if str.starts_with("Option<") {
        str.replace("Option<", "").replace(">", "")
    } else {
        str
    }
}

pub fn get_struct_field_serialization(field: &syn::Field) -> TokenStream {
    let ident = &field.ident;
    let ty = &field.ty;

    if let Some(encoding) = get_attribute::<LitStr>(&field.attrs, "encoding") {
        let ty_str = ty.to_token_stream().to_string();
        let encoding = encoding.value();

        // Some encoding methods require special implementations
        if ["json", "metadatas"].contains(&encoding.as_str()) {
            let string = format!("serialize_{encoding}");
            let serialize_fn = Ident::new(&string, Span::call_site());
            return quote! { #serialize_fn(&self.#ident, buf)?; };
        }

        // Serialize complex types
        if is_option_type(ty) || is_vec_type(ty) {
            let string = match encoding.as_str() {
                "json" => "serialize_json".to_string(),
                "metadatas" => "serialize_metadatas".to_string(),
                "custom" => return quote! { Serialize::serialize(&self.#ident, buf)?; },
                _ => format!("serialize_{encoding}_{}", get_inner_type(ty).to_lowercase()),
            };

            let serialize_fn = Ident::new(&string, Span::call_site());

            // Serialize Option<T>
            if is_option_type(ty) {
                return quote! {
                    if let Some(value) = &self.#ident {
                        #serialize_fn(value, buf)?;
                    }
                };
            }

            // Serialize Vec<T>
            if is_vec_type(ty) {
                return quote! {
                    serialize_varint_i32(&(self.#ident.len() as i32), buf)?;
                    for item in &self.#ident {
                        #serialize_fn(item, buf)?;
                    }
                };
            }
        }

        if is_option_type(ty) {
            let inner = get_inner_type(ty);

            return quote! {
                if let Some(value) = &self.#ident {
                    #inner::serialize(value, buf)?;
                }
            };
        }

        // Primitive types
        let string = format!("serialize_{encoding}_{ty_str}");
        let serialize_fn = Ident::new(&string, Span::call_site());
        return quote! { #serialize_fn(&self.#ident, buf)?; };
    }

    quote! { Serialize::serialize(&self.#ident, buf)?; }
}

pub fn get_struct_field_deserialization(field: &syn::Field) -> TokenStream {
    let ty = &field.ty;

    if let Some(encoding) = get_attribute::<LitStr>(&field.attrs, "encoding") {
        let ty_str = ty.to_token_stream().to_string().replace(" ", "");
        let encoding = encoding.value();

        // Some encoding methods require special implementations
        if ["json", "metadatas"].contains(&encoding.as_str()) {
            let string = format!("deserialize_{encoding}");
            let deserialize_fn = Ident::new(&string, Span::call_site());
            return quote! { #deserialize_fn(reader)?; };
        }

        // Deserialize complex types
        if is_option_type(ty) || is_vec_type(ty) {
            let string = match encoding.as_str() {
                "json" => "deserialize_json".to_string(),
                "metadatas" => "deserialize_metadatas".to_string(),
                "custom" => return quote! { Deserialize::deserialize(reader)? },
                _ => format!(
                    "deserialize_{encoding}_{}",
                    get_inner_type(ty).to_lowercase()
                ),
            };

            let deserialize_fn = Ident::new(&string, Span::call_site());

            // Deserialize Option<T>
            if is_option_type(ty) {
                return quote! { Some(#deserialize_fn(reader)?) };
            }

            // Deserialize Vec<T>
            if is_vec_type(ty) {
                return quote! {
                    {
                        let length = deserialize_varint_i32(reader)?;
                        let mut vec = Vec::with_capacity(length as usize);
                        for _ in 0..length {
                            vec.push(#deserialize_fn(reader)?);
                        }
                        vec
                    }
                };
            }
        }

        // Primitive types
        let string = format!("deserialize_{encoding}_{ty_str}");
        let deserialize_fn = Ident::new(&string, Span::call_site());
        return quote! { #deserialize_fn(reader)? };
    }

    if is_option_type(ty) {
        let inner = get_inner_type(ty);
        let tokens: TokenStream = inner.parse().unwrap();

        return quote! {
            Some(#tokens::deserialize(reader)?)
        };
    }

    quote! { Deserialize::deserialize(reader)? }
}
