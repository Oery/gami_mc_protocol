use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident};

pub fn get_entity_traits_impl(
    traits: &Vec<syn::Path>,
    name: &Ident,
) -> Vec<proc_macro2::TokenStream> {
    traits
        .iter()
        .map(
            |t| match t.get_ident().to_token_stream().to_string().as_str() {
                "LivingEntity" => quote! {
                    use crate::registry::entities::{LivingEntity, EntityKind};
                    use crate::packets::play::server::{SpawnMob, SpawnPlayer};

                    impl LivingEntity for #name {
                        fn ai_disabled(&self) -> bool {
                            self.ai_disabled
                        }

                        fn update(&mut self, metadatas: &[Metadata]) {
                            for metadata in metadatas {
                                if let Metadata::Byte(15, value) = metadata {
                                    self.ai_disabled = *value == 1;
                                }
                            }
                        }
                    }

                    impl From<&SpawnMob> for #name {
                        fn from(mob: &SpawnMob) -> Self {
                            let mut entity = Self {
                                id: mob.entity_id,
                                ..Default::default()
                            };

                            Entity::update(&mut entity, &mob.metadatas);

                            entity
                        }
                    }

                    impl From<&SpawnPlayer> for #name {
                        fn from(player: &SpawnPlayer) -> Self {
                            let mut entity = Self {
                                id: player.entity_id,
                                ..Default::default()
                            };

                            Entity::update(&mut entity, &player.metadatas);

                            entity
                        }
                    }

                    impl Into<EntityKind> for #name {
                        fn into(self) -> EntityKind {
                            EntityKind::#name(self)
                        }
                    }
                },

                "Ageable" => quote! {
                    use crate::registry::entities::Ageable;

                    impl Ageable for #name {
                        fn age(&self) -> u8 {
                            self.age
                        }

                        fn update(&mut self, metadatas: &[Metadata]) {
                            for metadata in metadatas {
                                if let Metadata::Byte(12, value) = metadata {
                                    self.age = *value;
                                }
                            }
                        }
                    }
                },

                "Tameable" => quote! {
                    use crate::registry::entities::Tameable;

                    impl Tameable for #name {
                        fn is_tame(&self) -> bool {
                            self.is_tame
                        }

                        fn is_sitting(&self) -> bool {
                            self.is_sitting
                        }

                        fn owner_name(&self) -> &str {
                            &self.owner_name
                        }

                        fn update(&mut self, metadatas: &[Metadata]) {
                            for metadata in metadatas {
                                match metadata {
                                    Metadata::Byte(16, value) => self.is_tame = *value == 1,
                                    Metadata::Byte(17, value) => self.is_sitting = *value == 1,
                                    Metadata::String(18, value) => self.owner_name = value.clone(),
                                    _ => (),
                                }
                            }
                        }
                    }
                },

                "Object" => quote! {
                    use crate::registry::entities::{Object, EntityKind};
                    use crate::packets::play::server::SpawnObject;

                    impl Object<'_> for #name {
                        fn update(&mut self, metadatas: &[Metadata]) {}
                    }

                    impl From<&SpawnObject> for #name {
                        fn from(object: &SpawnObject) -> Self {
                            Self {
                                id: object.entity_id,
                                ..Default::default()
                            }
                        }
                    }

                    impl Into<EntityKind> for #name {
                        fn into(self) -> EntityKind {
                            EntityKind::#name(self)
                        }
                    }
                },

                _ => panic!("Unknown Entity Trait"),
            },
        )
        .collect()
}

pub fn get_living(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::DeriveInput);
    let name = &item.ident;

    let expanded = quote! {
        use crate::registry::entities::LivingEntity;

        impl LivingEntity for #name {
            fn ai_disabled(&self) -> bool {
                self.ai_disabled
            }

            fn update(&mut self, metadatas: &[Metadata]) {
                for metadata in metadatas {
                    if let Metadata::Byte(15, value) = metadata {
                        self.ai_disabled = *value == 1;
                    }
                }
            }
        }
    };

    expanded.into()
}

pub fn get_ageable(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::DeriveInput);
    let name = &item.ident;

    quote! {
        use crate::registry::entities::Ageable;

        impl Ageable for #name {
            fn age(&self) -> u8 {
                self.age
            }

            fn update(&mut self, metadatas: &[Metadata]) {
                for metadata in metadatas {
                    if let Metadata::Byte(12, value) = metadata {
                        self.age = *value;
                    }
                }
            }
        }
    }
    .into()
}

pub fn get_tameable(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::DeriveInput);
    let name = &item.ident;

    let expanded = quote! {
        use crate::registry::entities::Tameable;

        impl Tameable for #name {
            fn owner_name(&self) -> &str {
                &self.owner_name
            }

            fn is_sitting(&self) -> bool {
                self.is_sitting
            }

            fn is_tame(&self) -> bool {
                self.is_tame
            }

            fn update(&mut self, metadatas: &[Metadata]) {
                for metadata in metadatas {
                    match metadata {
                        Metadata::Byte(16, value) => {
                            self.is_sitting = (*value & 0x01) != 0;
                            self.is_tame = (*value & 0x04) != 0;
                        }
                        Metadata::String(17, value) => self.owner_name = value.clone(),
                        _ => (),
                    }
                }
            }
        }
    };

    expanded.into()
}
