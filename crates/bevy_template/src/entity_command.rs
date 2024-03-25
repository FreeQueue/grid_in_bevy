use crate::Template;
use bevy::ecs::system::{Command, EntityCommands};
use bevy::prelude::*;
use bevy::ptr::OwningPtr;
use std::ptr::NonNull;
use crate::template::TemplateComponents;

pub trait TemplateCommandExt {
    fn insert_template(&mut self, template: Handle<Template>) -> &mut Self;
}

impl TemplateCommandExt for EntityCommands<'_> {
    fn insert_template(&mut self, template: Handle<Template>) -> &mut Self {
        let entity = self.id();
        self.commands().add(InsertTemplate { entity, template });
        self
    }
}
pub struct InsertTemplate {
    pub entity: Entity,
    pub template: Handle<Template>,
}

impl Command for InsertTemplate {
    fn apply(self, world: &mut World) {
        world.resource_scope(move |world,mut templates:Mut<Assets<Template>>| {
            let Some(template) = templates.get_mut(&self.template) else {
                panic!("Could not insert template {:?} for entity because it doesn't exist in this World.",self.template);
            };
            let Some(mut entity) = world.get_entity_mut(self.entity) else {
                panic!("Could not insert template for entity {:?} because it doesn't exist in this World.",self.entity);
            };
            
            if !template.is_initialized() {
                template.initialize(self.template,entity.world().components());
            }
            
            match &template.components {  
                TemplateComponents::Init { components, component_ids } => {
                    let components = components
                        .iter()
                        .map(|component| unsafe { OwningPtr::new(NonNull::from(component).cast()) });
                    unsafe { entity.insert_by_ids(component_ids, components) };
                }
                _ => panic!("Template is not initialized"),
            }
        });
    }
}
