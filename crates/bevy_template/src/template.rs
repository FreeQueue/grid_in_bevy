use crate::info::Infos;
use bevy::ecs::component::{ComponentId, Components};
use bevy::prelude::*;
use bevy::utils::TypeIdMap;
use either::Either;
use std::mem;
use std::sync::Arc;

#[derive(Debug)]
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
        } //.map(|info| {info.as_ref()})
    }
}

#[derive(Asset, TypePath, Debug)]
pub struct Template {
    pub(crate) components: TemplateComponents,
    pub infos: Infos,
}

impl Template {
    pub fn is_initialized(&self) -> bool {
        matches!(self.components, TemplateComponents::Init { .. })
    }

    pub(crate) fn initialize(&mut self, components: &Components) {
        if let TemplateComponents::Uninit {
            components: ref mut component_map,
        } = self.components
        {
            let component_map = mem::take(component_map);
            let mut component_map: Vec<_> = component_map
                .into_iter()
                .map(|(id, component)| (components.get_id(id).unwrap(), component))
                .collect();
            component_map.sort_by_key(|(id, _)| *id);
            self.components = TemplateComponents::Init {
                component_ids: component_map.iter().map(|(id, _)| *id).collect(),
                components: component_map
                    .into_iter()
                    .map(|(_, component)| component)
                    .collect(),
            }
        }
    }
}
