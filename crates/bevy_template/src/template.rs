use crate::content::TemplateContentDeserializer;
use crate::info::Infos;
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::ecs::component::ComponentId;
use bevy::prelude::{AppTypeRegistry, FromWorld, Reflect, TypePath, World};
use bevy::reflect::TypeRegistryArc;
use thiserror::Error;

#[derive(Asset, TypePath)]
pub struct Template {
    pub name: String,
    pub(crate) component_ids: Vec<ComponentId>,
    pub(crate) components: Vec<Box<dyn Reflect>>,
    pub infos: Infos,
}

pub struct TemplateLoader {
    type_registry: TypeRegistryArc,
}

#[derive(Debug, Error)]
pub enum TemplateLoaderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl FromWorld for TemplateLoader {
    fn from_world(world: &mut World) -> Self {
        let type_registry = world.resource::<AppTypeRegistry>().0.clone();
        Self { type_registry }
    }
}

impl AssetLoader for TemplateLoader {
    type Asset = Template;
    type Settings = ();
    type Error = TemplateLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut content = String::new();
            reader.read_to_string(&mut content).await?;
            let type_registry = self.type_registry.read();
            let template_deserializer = TemplateContentDeserializer {
                registry: &type_registry,
            };
            drop(type_registry);
            // let template = template_deserializer
            //     .deserialize(&mut deserializer)
            //     .map_err(|e| deserializer.span_error(e))?;
            // Ok(template)
            todo!()
        })
    }

    fn extensions(&self) -> &[&str] {
        &["temp"]
    }
}
