use crate::loader::TemplateLoader;
use bevy::prelude::*;

use crate::prelude::*;

mod bundle;
mod content;
mod entity_command;
mod info;
mod loader;
mod ron;
mod template;

#[allow(unused_imports)]
pub mod prelude {
    pub use crate::entity_command::TemplateCommandExt;
    pub use crate::info::{Info, InfoAny, InfoComponent, Infos, ReflectInfoAny};
    pub use crate::template::Template;
    pub use crate::TemplatePlugin;
}

pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Template>()
            .init_asset_loader::<TemplateLoader>();
    }
}
