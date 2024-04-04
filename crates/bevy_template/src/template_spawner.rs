use std::ptr::NonNull;

use bevy::prelude::*;
use bevy::ptr::OwningPtr;
use thiserror::Error;

use crate::prelude::Template;
use crate::template::TemplateComponents;

#[derive(Error, Debug)]
pub enum TemplateSpawnError {
    #[error("template dose not exist")]
    NonExistentTemplate { handle: Handle<Template> },
    #[error("entity dose not exist")]
    NonExistentEntity { entity: Entity },
}

#[derive(Default, Resource)]
pub struct TemplateSpawner {
    templates_to_insert: Vec<(Handle<Template>, Entity)>,
    // scene_asset_event_reader: ManualEventReader<AssetEvent<Template>>,
}

impl TemplateSpawner {
    pub fn insert_template(&mut self, handle: Handle<Template>, entity: Entity) {
        self.templates_to_insert.push((handle, entity));
    }

    pub fn update_queued_templates(&mut self, world: &mut World) -> Result<(), TemplateSpawnError> {
        let templates_to_spawn = std::mem::take(&mut self.templates_to_insert);

        for (handle, entity) in templates_to_spawn {
            let path = handle.path().unwrap().to_owned();
            match insert_template_internal(world, handle, entity) {
                Ok(_) => continue,
                Err(TemplateSpawnError::NonExistentTemplate { handle }) => {
                    self.insert_template(handle, entity);
                }
                Err(TemplateSpawnError::NonExistentEntity { entity }) => {
                    warn!("{:?} does not exist, {} insert cancel", entity, path)
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
    handle: Handle<Template>,
    entity: Entity,
) -> Result<(), TemplateSpawnError> {
    world.resource_scope(move |world, mut templates: Mut<Assets<Template>>| {
        let id = handle.id();
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
                    entity.insert(handle);
                    unsafe { entity.insert_by_ids(component_ids, components) };
                    Ok(())
                }
                _ => panic!("Template is not initialized, why?"),
            }
        } else {
            Err(TemplateSpawnError::NonExistentTemplate { handle })
        }
    })
}
