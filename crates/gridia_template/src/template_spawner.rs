use std::ptr::NonNull;

use bevy::prelude::*;
use bevy::ptr::OwningPtr;
use bevy::utils::HashMap;
use thiserror::Error;

use crate::prelude::{Infos, Template};
use crate::template::TemplateComponents;

#[derive(Error, Debug)]
pub enum TemplateSpawnError {
    #[error("template dose not exist")]
    NonExistentTemplate { id: AssetId<Template> },
    #[error("entity dose not exist")]
    NonExistentEntity { entity: Entity },
}

#[derive(Default, Resource)]
pub struct TemplateSpawner {
    templates: HashMap<AssetId<Template>, Handle<Template>>,
    templates_to_insert: Vec<(AssetId<Template>, Entity)>,
    // scene_asset_event_reader: ManualEventReader<AssetEvent<Template>>,
}

impl TemplateSpawner {
    pub fn insert_template(&mut self, handle: Handle<Template>, entity: Entity) {
        let id = handle.id();
        let _ = self.templates.try_insert(id, handle);
        self.templates_to_insert.push((id, entity));
    }

    pub fn update_queued_templates(&mut self, world: &mut World) -> Result<(), TemplateSpawnError> {
        let templates_to_spawn = std::mem::take(&mut self.templates_to_insert);

        for (id, entity) in templates_to_spawn {
            match insert_template_internal(world, id, entity) {
                Ok(_) => continue,
                Err(TemplateSpawnError::NonExistentTemplate { id }) => {
                    self.templates_to_insert.push((id, entity));
                }
                Err(TemplateSpawnError::NonExistentEntity { entity }) => {
                    warn!("{:?} does not exist, {:?} insert cancel", entity, id)
                } // Err(err) => {
                  //     return Err(err);
                  // }
            }
        }
        Ok(())
    }
}

pub fn insert_template_system(world: &mut World) {
    world.resource_scope(|world, mut template_spawner: Mut<TemplateSpawner>| {
        template_spawner.update_queued_templates(world).unwrap();
    })
}

fn insert_template_internal(
    world: &mut World,
    id: AssetId<Template>,
    entity: Entity,
) -> Result<(), TemplateSpawnError> {
    world.resource_scope(move |world, mut templates: Mut<Assets<Template>>| {
        let mut entity = world
            .get_entity_mut(entity)
            .ok_or(TemplateSpawnError::NonExistentEntity { entity })?;
        if let Some(template) = templates.get_mut(id) {
            if !template.is_initialized() {
                template.initialize(entity.world().components());
            }
            match &template.components {
                TemplateComponents::Init {
                    components,
                    component_ids,
                } => {
                    let components = components.iter().map(|component| unsafe {
                        OwningPtr::new(NonNull::from(component.as_reflect()).cast())
                    });
                    if let Some(mut infos) = entity.get_mut::<Infos>() {
                        for (type_id, info) in template.infos.iter() {
                            infos.insert(*type_id, info.clone()); // 当 key 已存在时，insert() 会自动覆盖原有值
                        }
                    } else if !template.infos.is_empty() {
                        entity.insert(template.infos.clone());
                    }
                    unsafe { entity.insert_by_ids(component_ids, components) };
                    Ok(())
                }
                _ => panic!("Template is not initialized, why?"),
            }
        } else {
            Err(TemplateSpawnError::NonExistentTemplate { id })
        }
    })
}
