use crate::info::Infos;
use bevy::ecs::component::{ComponentId, Components};
use bevy::prelude::*;
use bevy::utils::TypeIdMap;
use either::Either;
use std::any::Any;
use std::mem;
use std::sync::Arc;

pub(crate) enum TemplateComponents {
    Init {
        component_ids: Vec<ComponentId>,
        components: Vec<Arc<dyn Reflect>>,
    },
    Uninit {
        components: TypeIdMap<Arc<dyn Reflect>>,
    },
}

impl TemplateComponents {
    pub fn iter(&self) -> impl Iterator<Item = &Arc<dyn Reflect>> {
        match self {
            TemplateComponents::Init { components, .. } => Either::Left(components.iter()),
            TemplateComponents::Uninit { components } => Either::Right(components.values()),
        }
    }
}

#[derive(Asset, TypePath)]
pub struct Template {
    pub(crate) components: TemplateComponents,
    pub infos: Infos,
}

impl Template {
    pub fn is_initialized(&self) -> bool {
        matches!(self.components, TemplateComponents::Init { .. })
    }

    pub(crate) fn initialize(&mut self, self_handle: Handle<Template>, components: &Components) {
        if let TemplateComponents::Uninit {
            components: ref mut component_map,
        } = self.components
        {
            let mut component_map = mem::take(component_map);
            let get_component_id = |component: &Arc<dyn Reflect>| -> ComponentId {
                components.get_id(component.type_id()).unwrap()
            };
            //只会保留这个Handle<Template>的组件
            component_map.insert(self_handle.type_id(), self_handle.clone_value().into());
            let mut components: Vec<Arc<dyn Reflect>> = component_map.into_values().collect();
            components.sort_by_cached_key(get_component_id);
            self.components = TemplateComponents::Init {
                component_ids: components.iter().map(get_component_id).collect(),
                components,
            }
        }
    }
}
