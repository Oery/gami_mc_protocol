use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use convert_case::{Case, Casing};
use proc_macro2::*;
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, *};
use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/packets/handshake/client");
    println!("cargo:rerun-if-changed=src/packets/login/client");
    println!("cargo:rerun-if-changed=src/packets/login/server");
    println!("cargo:rerun-if-changed=src/packets/play/client");
    println!("cargo:rerun-if-changed=src/packets/play/server");
    println!("cargo:rerun-if-changed=src/packets/status/client");
    println!("cargo:rerun-if-changed=src/packets/status/server");

    let mut packets: Vec<PacketInfo> = list_packets()?;

    tag_duplicates(&mut packets);

    generate_packet_enum(&packets)?;
    generate_modules_defs(&packets);

    Command::new("cargo").arg("fmt").status().unwrap();

    Ok(())
}

#[derive(Debug, Clone)]
struct PacketInfo {
    state: String,
    origin: String,
    id: i32,
    struct_name: String,
    is_duplicate: bool,
}

impl PacketInfo {
    fn sort_key(&self) -> (String, String, i32) {
        (self.state.clone(), self.origin.clone(), self.id)
    }
}

fn find_packet_attr(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|a| a.path().is_ident("packet"))
}

fn parse_path_components(path: &str) -> (String, String) {
    let components = path.split("/").collect::<Vec<_>>();
    let state = components.first().unwrap_or(&"unknown").to_string();
    let origin = components.get(1).unwrap_or(&"unknown").to_string();
    (state, origin)
}

fn parse_packet_attr(attr: &Attribute) -> (i32, Option<String>) {
    let list: Punctuated<Expr, Token![,]> =
        attr.parse_args_with(Punctuated::parse_terminated).unwrap();
    let mut iter = list.iter();

    let id = match iter.next().unwrap() {
        Expr::Lit(ExprLit {
            lit: Lit::Int(i), ..
        }) => i.base10_parse().unwrap(),
        _ => panic!("Invalid packet ID format"),
    };

    let origin = iter.next().and_then(|e| {
        if let Expr::Path(ExprPath { path, .. }) = e {
            path.get_ident().map(|i| i.to_string())
        } else {
            None
        }
    });

    (id, origin)
}

fn generate_packet_enum(packets: &[PacketInfo]) -> anyhow::Result<()> {
    let path = Path::new("./src/packets/packets_enum.rs");
    write_generated_warning(path);

    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;

    let variants = packets.iter().map(|p| {
        let variant_name = match p.is_duplicate {
            true => &format!("{}{}", p.origin.to_case(Case::Pascal), p.struct_name),
            false => &p.struct_name,
        };

        let variant = Ident::new(variant_name, Span::call_site());
        let ty =
            parse_str::<Type>(&format!("{}::{}::{}", p.state, p.origin, p.struct_name)).unwrap();
        quote! { #variant(#ty) }
    });

    let match_serialize_arms = packets.iter().map(|p| {
        let variant_name = match p.is_duplicate {
            true => &format!("{}{}", p.origin.to_case(Case::Pascal), p.struct_name),
            false => &p.struct_name,
        };

        let variant = Ident::new(variant_name, Span::call_site());
        quote! { Packets::#variant(packet) => Packet::serialize(packet, compression), }
    });

    let match_arms_deserialize = packets.iter().map(|p| {
        let variant_name = match p.is_duplicate {
            true => &format!("{}{}", p.origin.to_case(Case::Pascal), p.struct_name),
            false => &p.struct_name,
        };

        let variant = Ident::new(variant_name, Span::call_site());

        let state = Ident::new(&p.state.to_case(Case::Pascal), Span::call_site());
        let origin = Ident::new(&p.origin.to_case(Case::Pascal), Span::call_site());

        let id = p.id;

        quote! {
            (#state, #origin, #id) => {
                Ok(Packets::#variant(Packet::deserialize(bytes)?))
            }
        }
    });

    let match_type_id_arms = packets.iter().map(|p| {
        let ty =
            parse_str::<Type>(&format!("{}::{}::{}", p.state, p.origin, p.struct_name)).unwrap();

        let state = Ident::new(&p.state.to_case(Case::Pascal), Span::call_site());
        let origin = Ident::new(&p.origin.to_case(Case::Pascal), Span::call_site());

        let id = p.id;

        quote! {
            (#state, #origin, #id) => {
                Ok(TypeId::of::<#ty>())
            }
        }
    });

    writeln!(file, "use std::any::TypeId;\n\n")?;

    let imports = quote! {
        use crate::packets::*;
        use crate::registry::tcp::{Origin::*, State::*};
    };
    writeln!(file, "{imports}\n\n")?;

    let enum_decl = quote! {
        #[derive(Debug)]
        pub enum Packets {
            #(#variants),*
        }
    };
    writeln!(file, "{enum_decl}\n\n")?;

    let impl_decl = quote! {
        impl Packets {
            pub fn serialize(&self, compression: i32) -> Result<Vec<u8>> {
                match self {
                    #(#match_serialize_arms)*
                }
            }

            pub fn deserialize(id: i32, state: &State, origin: &Origin, bytes: &[u8]) -> Result<Packets> {
                match (state, origin, id) {
                    #(#match_arms_deserialize)*
                    _ => Err(Error::new(ErrorKind::InvalidData, "Unknown packet")),
                }
            }

            pub fn get_type_id(id: i32, state: &State, origin: &Origin) -> Result<TypeId> {
                match (state, origin, id) {
                    #(#match_type_id_arms)*
                    _ => Err(Error::new(ErrorKind::InvalidData, "Unknown packet")),
                }
            }
        }
    };

    writeln!(file, "{}", impl_decl)?;
    Ok(())
}

fn write_generated_warning(path: &Path) {
    fs::write(path, "// #################################################\n// ###### THIS FILE IS AUTOMATICALLY GENERATED #####\n// ########      DO NOT EDIT BY HAND     ###########\n// #################################################\n\n").unwrap();
}

fn list_packets() -> anyhow::Result<Vec<PacketInfo>> {
    let mut packets = Vec::new();

    for entry in WalkDir::new("src/packets") {
        let Ok(entry) = entry else {
            continue;
        };

        if entry.file_type().is_dir() {
            continue;
        }

        if entry.path().extension().is_some_and(|e| e != "rs") {
            continue;
        }

        let content = fs::read_to_string(entry.path())?;
        let ast = parse_file(&content)?;

        for item in ast.items {
            let Item::Struct(item_struct) = item else {
                continue;
            };

            let Some(packet_attr) = find_packet_attr(&item_struct.attrs) else {
                continue;
            };

            let path = entry.path().strip_prefix("src/packets")?;
            let (state, origin) = parse_path_components(path.to_str().unwrap());
            let (id, _origin) = parse_packet_attr(packet_attr);

            packets.push(PacketInfo {
                state,
                origin,
                id,
                struct_name: item_struct.ident.to_string(),
                is_duplicate: false,
            });
        }
    }

    packets.sort_by_key(|p| p.sort_key());

    Ok(packets)
}

fn tag_duplicates(packets: &mut [PacketInfo]) {
    let all_packets = packets.to_vec();

    for packet in packets.iter_mut() {
        if all_packets
            .iter()
            .filter(|p| p.struct_name == packet.struct_name)
            .count()
            > 1
        {
            packet.is_duplicate = true;
        }
    }
}

fn generate_modules_defs(packets: &[PacketInfo]) {
    let paths = [
        "./src/packets/handshake/client.rs",
        "./src/packets/login/client.rs",
        "./src/packets/login/server.rs",
        "./src/packets/play/client.rs",
        "./src/packets/play/server.rs",
        "./src/packets/status/client.rs",
        "./src/packets/status/server.rs",
    ];

    for path in paths {
        write_generated_warning(Path::new(path));

        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .unwrap();

        for packet in packets {
            if !(path.contains(&packet.origin) && path.contains(&packet.state)) {
                continue;
            }

            let module_name = format_ident!("{}", packet.struct_name.to_case(Case::Snake));
            let path_str = format!(
                "./{}/{}_{}.rs",
                packet.origin.to_case(Case::Snake),
                packet.id,
                packet.struct_name.to_case(Case::Snake)
            );
            let mod_decl = quote! {
                #[path = #path_str]
                mod #module_name;
                pub use #module_name::*;
            };

            writeln!(file, "{}\n\n", mod_decl).unwrap();
        }
    }
}
