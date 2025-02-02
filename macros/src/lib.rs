use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    braced, parse_macro_input, Attribute, Data, DeriveInput, Expr, ExprLit, Fields, Ident, Lit,
    LitInt, LitStr, Path, Token,
};

#[proc_macro]
pub fn packets_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as PacketGroups);

    let variants = input.groups.iter().flat_map(|group| {
        let mod_path = &group.module;
        group.items.iter().map(move |item| {
            let variant_name = &item.alias;
            let struct_name = &item.struct_name;
            quote! { #variant_name(#mod_path::#struct_name) }
        })
    });

    let serialize_arms = input.groups.iter().flat_map(|group| {
        group.items.iter().map(|item| {
            let variant_name = &item.alias;
            quote! {
                Packets::#variant_name(packet) => packet.serialize(),
            }
        })
    });

    let output = quote! {
        #[derive(Debug, PartialEq)]
        pub enum Packets {
            #(#variants),*
        }

        impl Packets {
            pub fn serialize(&self) -> Result<()> {
                match self {
                    #(#serialize_arms)*
                }
            }
        }
    };

    output.into()
}

struct PacketGroups {
    groups: Vec<PacketGroup>,
}

struct PacketGroup {
    module: Path,
    items: Vec<PacketItem>,
}

struct PacketItem {
    alias: Ident,
    struct_name: Ident,
}

impl Parse for PacketGroups {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut groups = Vec::new();

        while !input.is_empty() {
            let module = input.parse()?;
            let _: syn::Token![:] = input.parse()?;
            let content;
            braced!(content in input);

            let items = Punctuated::<PacketItem, syn::Token![,]>::parse_terminated(&content)?;

            groups.push(PacketGroup {
                module,
                items: items.into_iter().collect(),
            });

            let _ = input.parse::<syn::Token![,]>();
        }

        Ok(PacketGroups { groups })
    }
}

impl Parse for PacketItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let struct_name: Ident = input.parse()?;

        if input.peek(syn::Token![as]) {
            let _: syn::Token![as] = input.parse()?;
            let alias: Ident = input.parse()?;
            Ok(PacketItem { alias, struct_name })
        } else {
            Ok(PacketItem {
                alias: struct_name.clone(),
                struct_name,
            })
        }
    }
}

// Packets Macros
fn get_attribute<T: Parse>(attrs: &[Attribute], name: &str) -> Option<T> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident(name))
        .and_then(|attr| attr.parse_args::<T>().ok())
}

fn is_vec_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Vec";
        }
    }
    false
}

fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn parse_discriminant_as_typed_value(expr: &Expr, ty: &str) -> proc_macro2::TokenStream {
    // Parse base value as i64 for range checking
    let base_value = match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(lit_int),
            ..
        }) => lit_int
            .base10_parse::<i64>()
            .expect("Failed to parse integer literal"),
        Expr::Cast(cast_expr) => match &*cast_expr.expr {
            Expr::Lit(ExprLit {
                lit: Lit::Int(lit_int),
                ..
            }) => lit_int
                .base10_parse::<i64>()
                .expect("Failed to parse integer literal in cast"),
            _ => panic!("Complex casts are not supported in enum discriminants"),
        },
        _ => panic!("Unsupported discriminant expression"),
    };

    // Convert to properly typed literal
    let value_str = format!("{}", base_value);
    let typed_value = syn::parse_str::<LitInt>(&format!("{}{}", value_str, ty))
        .unwrap_or_else(|_| panic!("Failed to create typed literal for {}", ty));

    quote! { #typed_value }
}

#[proc_macro_derive(Serialize, attributes(encoding, condition))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = match &input.data {
        Data::Struct(data_struct) => {
            // Existing struct handling code
            let fields = match &data_struct.fields {
                Fields::Named(fields_named) => &fields_named.named,
                _ => panic!("Only named fields are supported for structs"),
            };

            let serializations = fields.iter().map(|field| {
                let ident = &field.ident;
                let ty = &field.ty;

                let ser_expr =
                    if let Some(encoding_method) = get_attribute::<LitStr>(&field.attrs, "encoding") {
                        let mut string = format!("serialize_{}", encoding_method.value());

                        if is_option_type(ty) {
                            string.push_str("_option");
                        } else if is_vec_type(ty) {
                            string.push_str("_vec");
                        }

                        let serialize_fn = Ident::new(&string, Span::call_site().into());
                        quote! { 
                            #serialize_fn(&self.#ident, buf)?;
                        }
                    } else {
                        quote! {
                            Serialize::serialize(&self.#ident, buf)?;
                        }
                    };

                match get_attribute::<Expr>(&field.attrs, "condition") {
                    Some(condition) => quote! { 
                        if #condition {
                            #ser_expr
                        }
                        else {
                            eprintln!("[WARNING] Invalid packet {}, skipping field {} to avoid crashing. Condition {} was not met.", stringify!(#name), stringify!(self.#ident), stringify!(#condition));
                        }
                    },
                    None => ser_expr,
                }
            });

            quote! {
                impl Serialize for #name {
                    fn serialize(&self, buf: &mut dyn std::io::Write) -> std::io::Result<()> {
                        #(#serializations)*
                        Ok(())
                    }
                }
            }
        }

        Data::Enum(data_enum) => {
            let encoding_type = get_attribute::<LitStr>(&input.attrs, "encoding")
                .expect("Enums must specify encoding attribute, e.g. #[encoding(\"u8\")]").value();

            let repr_ident = syn::Ident::new(&encoding_type, Span::call_site());
            let variants = &data_enum.variants;

            // Ensure all variants have explicit discriminants
            for variant in variants {
                if variant.discriminant.is_none() {
                    panic!("Enum variant {} has no discriminant", variant.ident);
                }
            }

            // Generate match arms for each variant
            let match_arms = variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let discriminant_expr = &variant.discriminant.as_ref().unwrap().1;

                // Handle different field types (unit, tuple, struct)
                let pattern = match &variant.fields {
                    Fields::Unit => quote! { #name::#variant_ident },
                    Fields::Unnamed(_) => quote! { #name::#variant_ident(..) },
                    Fields::Named(_) => quote! { #name::#variant_ident { .. } },
                };

                quote! {
                    #pattern => {
                        Serialize::serialize(&(#discriminant_expr as #repr_ident), buf)?;
                    }
                }
            });

            quote! {
                impl Serialize for #name {
                    fn serialize(&self, buf: &mut dyn std::io::Write) -> std::io::Result<()> {
                        match self {
                            #(#match_arms)*
                        }
                        Ok(())
                    }
                }
            }
        }
        _ => panic!("Only structs and enums are supported"),
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Deserialize, attributes(encoding))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = match &input.data {
        Data::Struct(data_struct) => {
            let mut let_statements = Vec::new();
            let mut field_initializers = Vec::new();

            for field in &data_struct.fields {
                let ident = &field.ident;
                let ty = &field.ty;

                let mut deser_code = match get_attribute::<LitStr>(&field.attrs, "encoding") {
                    Some(encoding_method) => {
                        let mut string = format!("deserialize_{}", encoding_method.value());

                        if is_vec_type(ty) {
                            string.push_str("_vec");
                        }

                        let deserialize_fn = Ident::new(&string, Span::call_site().into());
                        quote! {#deserialize_fn(reader)?}
                    },
                    None => quote! {<#ty as Deserialize>::deserialize(reader)?},
                };

                if let Some(mut condition) = get_attribute::<Expr>(&field.attrs, "condition") {
                    let mut condition_str = condition.to_token_stream().to_string().replace(" ", "");
                    condition_str = condition_str.replace("self.", "");
                    let tokens: TokenStream = condition_str.parse().unwrap();
                    condition = syn::parse::<Expr>(tokens).unwrap();
                    
                    deser_code = quote! {
                        match #condition {
                            true => #deser_code,
                            false => None,
                        }
                    }
                };

                let_statements.push(quote! {
                    let #ident = #deser_code;
                });
                field_initializers.push(quote! {
                    #ident,
                });
            }

            quote! {
                impl Deserialize for #name {
                    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
                        #(#let_statements)*
                        Ok(Self {
                            #(#field_initializers)*
                        })
                    }
                }
            }
        }

        Data::Enum(data_enum) => {
            let encoding_type = get_attribute::<LitStr>(&input.attrs, "encoding")
                .expect("Enums must specify encoding attribute, e.g. #[encoding(\"u8\")]").value();

            let repr_ident = syn::Ident::new(&encoding_type, Span::call_site());

            // Validate encoding type
            let valid_types = ["u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64"];
            if !valid_types.contains(&&*encoding_type) {
                panic!("Invalid encoding type '{}' for enum", encoding_type);
            }

            let match_arms = data_enum.variants.iter().map(|variant| {
                if !variant.fields.is_empty() {
                    panic!("Enum variant have fields.");
                }

                let (_, expr) = variant
                    .discriminant
                    .as_ref()
                    .expect("Missing discriminant for variant");

                let value = parse_discriminant_as_typed_value(expr, &encoding_type);
                let variant_ident = &variant.ident;

                quote! { 
                    #value => Ok(#name::#variant_ident),
                }
            });

            quote! {
                impl Deserialize for #name {
                    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
                        let discriminant: #repr_ident = Deserialize::deserialize(reader)?;
                        match discriminant {
                            #(#match_arms)*
                            _ => Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                format!("Invalid discriminant value: {}", discriminant)
                            )),
                        }
                    }
                }
            }
        }
        _ => panic!("Only structs and enums are supported"),
    };

    TokenStream::from(expanded)
}

enum PacketDirection {
    Client,
    Server,
}

struct PacketAttr {
    id: u8,
    direction: PacketDirection,
}

// Custom parser for the attribute
impl Parse for PacketAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the ID
        let id_lit = input.parse::<LitInt>()?;
        let id = id_lit.base10_parse::<u8>()?;

        // Expect a comma
        input.parse::<Token![,]>()?;

        // Parse the direction identifier
        let direction_ident = input.parse::<syn::Ident>()?;
        let direction = match direction_ident.to_string().as_str() {
            "client" => PacketDirection::Client,
            "server" => PacketDirection::Server,
            _ => {
                return Err(syn::Error::new(
                    direction_ident.span(),
                    "Expected 'client' or 'server'",
                ))
            }
        };

        Ok(PacketAttr { id, direction })
    }
}

#[proc_macro_attribute]
pub fn packet(attr: TokenStream, item: TokenStream) -> TokenStream {
    let PacketAttr { id, direction } = parse_macro_input!(attr as PacketAttr);
    let item = parse_macro_input!(item as DeriveInput);
    let name = &item.ident;

    // Generate different implementations based on direction
    let direction_impl = match direction {
        PacketDirection::Client => quote! {
            impl ClientPacket for #name {}
        },
        PacketDirection::Server => quote! {
            impl ServerPacket for #name {}
        },
    };

    let origin = match direction {
        PacketDirection::Client => quote! {
            crate::registry::tcp::Origins::Client
        },
        PacketDirection::Server => quote! {
            crate::registry::tcp::Origins::Server
        },
    };

    let expanded = quote! {
        use crate::serialization::{Deserialize, Serialize};
        use crate::packets::{ClientPacket, ServerPacket};

        #[derive(PartialEq, Debug, Deserialize, Serialize)]
        #item

        impl Packet for #name {
            fn get_origin(&self) -> crate::registry::tcp::Origins {
                #origin
            }

            fn get_state(&self) -> crate::registry::tcp::States {
                crate::registry::tcp::States::from_module_path(module_path!())
            }

            fn get_id(&self) -> u8 {
                #id
            }

            fn get_name(&self) -> &'static str {
                stringify!(#name)
            }
        }

        #direction_impl
    };

    expanded.into()
}
