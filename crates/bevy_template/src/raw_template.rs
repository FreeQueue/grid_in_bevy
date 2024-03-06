use crate::content::{TemplateContent, TemplateContentDeserializer};
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;
use bevy::utils::BoxedFuture;
use serde::de::DeserializeSeed;
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Debug)]
pub struct TemplateRon {
    #[serde(default)]
    pub dep: Vec<String>,
    pub content: String,
}

#[derive(Asset, TypePath, Debug)]
pub struct RawTemplate {
    #[dependency]
    pub dependencies: Vec<Handle<RawTemplate>>,
    pub content: TemplateContent,
}

pub struct RawTemplateLoader {
    pub type_registry: TypeRegistryArc,
}

#[derive(Debug, Error)]
pub enum RawTemplateLoaderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    RonSpannedError(#[from] ron::error::SpannedError),
    #[error(transparent)]
    LoadDirectError(#[from] bevy::asset::LoadDirectError),
}

impl FromWorld for RawTemplateLoader {
    fn from_world(world: &mut World) -> Self {
        let type_registry = world.resource::<AppTypeRegistry>().0.clone();
        Self { type_registry }
    }
}

impl AssetLoader for RawTemplateLoader {
    type Asset = RawTemplate;
    type Settings = ();
    type Error = RawTemplateLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let ron: TemplateRon = ron::de::from_bytes(&bytes)?;
            let mut dependencies = Vec::new();
            for dep in ron.dep {
                let loaded = load_context.load(&dep);
                dependencies.push(loaded);
            }

            let type_registry = self.type_registry.read();
            let content_deserializer = TemplateContentDeserializer {
                registry: &type_registry,
            };
            let mut deserializer = ron::de::Deserializer::from_str(&ron.content)?;
            let content = content_deserializer
                .deserialize(&mut deserializer)
                .map_err(move |e| deserializer.span_error(e))?;
            Ok(RawTemplate {
                dependencies,
                content,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["temp.ron"]
    }
}
