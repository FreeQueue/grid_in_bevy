use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::asset::io::Reader;
use bevy::prelude::{AppTypeRegistry, FromWorld, World};
use bevy::reflect::TypeRegistryArc;
use bevy::utils::TypeIdMap;
use ron::de::SpannedError;
use thiserror::Error;

use crate::info::Infos;
use crate::prelude::Template;
use crate::ron::TemplateRon;
use crate::template::TemplateComponents;

pub struct TemplateLoader {
    pub type_registry: TypeRegistryArc,
}

#[derive(Debug, Error)]
pub enum TemplateLoaderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    RonError(#[from] ron::Error),
    #[error(transparent)]
    RonSpannedError(#[from] SpannedError),
    #[error(transparent)]
    LoadDirectError(#[from] bevy::asset::LoadDirectError),
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
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let ron: TemplateRon = ron::de::from_bytes(&bytes)?;
            let mut components = TypeIdMap::default();
            let mut infos = TypeIdMap::default();

            let (dep, content) = ron.load_content(self.type_registry.clone())?;
            content.parse(self.type_registry.clone(), &mut components, &mut infos);

            //反向迭代去重，后覆盖前
            for dep in dep.iter().rev() {
                let loaded = load_context.load_direct(dep).await?;
                let template = loaded.get::<Template>().unwrap();
                for component in template.components.iter() {
                    let _ = components.try_insert(component.type_id(), component.clone());
                }
                for (type_id, info) in template.infos.iter() {
                    let _ = infos.try_insert(*type_id, info.clone());
                }
            }

            Ok(Template {
                infos: Infos::new(infos),
                components: TemplateComponents::Uninit { components },
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["temp.ron"]
    }
}
