use bevy::prelude::*;

use crate::loader::TemplateLoader;
use crate::prelude::*;
use crate::template_spawner::{insert_template_system, TemplateSpawner};

mod bundle;
mod command;
mod content;
mod info;
mod loader;
mod ron;
mod template;
mod template_spawner;

#[allow(unused_imports)]
pub mod prelude {
    pub use crate::command::{InsertTemplateCommandExt, SpawnTemplateCommandExt};
    pub use crate::info::{Info, InfoAny, Infos, ReflectInfoAny, Trait};
    pub use crate::template::Template;
    pub use crate::TemplatePlugin;
}

pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    fn build(&self, app: &mut App) {
        app.world.init_component::<Handle<Template>>();
        app.init_asset::<Template>()
            .init_asset_loader::<TemplateLoader>()
            .init_resource::<TemplateSpawner>()
            .add_systems(SpawnScene, insert_template_system);
    }
}
