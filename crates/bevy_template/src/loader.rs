use crate::info::{InfoAny, Infos, ReflectInfoAny};
use crate::prelude::Template;
use crate::ron::TemplateRon;
use crate::template::TemplateComponents;
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::prelude::{AppTypeRegistry, FromWorld, Reflect, World};
use bevy::reflect::TypeRegistryArc;
use bevy::utils::TypeIdMap;
use ron::de::SpannedError;
use std::sync::Arc;
use thiserror::Error;

pub struct TemplateLoader {
    pub type_registry: TypeRegistryArc,
}

#[derive(Debug, Error)]
pub enum TemplateLoaderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
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

            read_lock(
                self.type_registry.clone(),
                &ron,
                &mut components,
                &mut infos,
            )?;

            //反向迭代去重，后覆盖前
            for dep in ron.dep.iter().rev() {
                let loaded = load_context.load_direct(dep).await?;
                let template = loaded.get::<Template>().unwrap();
                for component in template.components.iter() {
                    let _ = components.try_insert(component.type_id(), component.clone());
                }
                for (type_id, info) in template.infos.infos.iter() {
                    let _ = infos.try_insert(*type_id, info.clone());
                }
            }

            Ok(Template {
                infos: Infos { infos },
                components: TemplateComponents::Uninit { components },
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["temp.ron"]
    }
}

fn read_lock(
    type_registry: TypeRegistryArc,
    ron: &TemplateRon,
    components: &mut TypeIdMap<Arc<dyn Reflect>>,
    infos: &mut TypeIdMap<Arc<dyn InfoAny>>,
) -> Result<(), SpannedError> {
    let type_registry = type_registry.read();
    let content = ron.load_content(&type_registry)?;

    //反向迭代去重，后覆盖前
    for item in content.items.into_iter().rev() {
        let type_id = item.type_id();
        //如果component已有，无需检测info，没有component必然没有info
        if components.contains_key(&type_id) {
            continue;
        }
        if let Some(reflect) = type_registry.get_type_data::<ReflectInfoAny>(type_id) {
            let info = reflect.get_boxed(item).unwrap();
            let component = info.gen_reflect();
            if components
                .try_insert(component.type_id(), component.into())
                .is_ok()
            {
                infos.insert(type_id, info.into());
            }
        } else {
            components.insert(type_id, item.into());
        };
    }
    Ok(())
}
