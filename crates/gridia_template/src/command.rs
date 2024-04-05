use bevy::asset::AssetPath;
use bevy::ecs::system::{Command, EntityCommands};
use bevy::prelude::*;

use crate::template_spawner::TemplateSpawner;

pub struct InsertTemplate {
    pub entity: Entity,
    pub template: AssetPath<'static>,
}

impl Command for InsertTemplate {
    fn apply(self, world: &mut World) {
        world.resource_scope(move |world, mut template_spawner: Mut<TemplateSpawner>| {
            let handle = world.resource::<AssetServer>().load(self.template);
            template_spawner.insert_template(handle, self.entity);
        });
    }
}

pub trait InsertTemplateCommandExt {
    fn insert_template<'a>(&mut self, template: impl Into<AssetPath<'a>>) -> &mut Self;
}

impl InsertTemplateCommandExt for EntityCommands<'_> {
    fn insert_template<'a>(&mut self, template: impl Into<AssetPath<'a>>) -> &mut Self {
        let entity = self.id();
        self.commands().add(InsertTemplate {
            entity,
            template: template.into().into_owned(),
        });
        self
    }
}
pub trait SpawnTemplateCommandExt {
    fn spawn_template<'a>(&mut self, template: impl Into<AssetPath<'a>>) -> EntityCommands;
}
impl<'w, 's> SpawnTemplateCommandExt for Commands<'w, 's> {
    fn spawn_template<'a>(&mut self, template: impl Into<AssetPath<'a>>) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.insert_template(template);
        e
    }
}
