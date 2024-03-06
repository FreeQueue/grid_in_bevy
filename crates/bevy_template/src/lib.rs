mod content;
mod entity_command;
mod info;
mod process;
mod raw_template;
mod template;

use crate::raw_template::RawTemplateLoader;
use crate::template::{Template, TemplateLoader};

use bevy::prelude::*;

use bevy::utils::NoOpTypeIdHash;
use indexmap::IndexMap;

use std::any::TypeId;

pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Template>()
            .init_asset_loader::<TemplateLoader>()
            .init_asset_loader::<RawTemplateLoader>();
        // .register_asset_processor::<process::TemplateProcessor>()
        // .set_default_asset_processor();
    }
}

type IndexTypeIdMap<V> = IndexMap<TypeId, V, NoOpTypeIdHash>;

pub trait MyBundle: Bundle {
    fn get_components(&self);
}

// pub struct TemplateBundle<'a> {
//     components: Vec<(StorageType, &'a dyn Reflect)>,
// }
//
// impl<'a> From<&'a Template> for TemplateBundle<'a> {
//     fn from(value: &'a Template) -> Self {
//         let mut components = Vec::new();
//         for (storage_type, component) in value.components.values() {
//             components.push((*storage_type, component.as_ref()));
//         }
//         TemplateBundle { components }
//     }
// }
//
// impl<'a> DynamicBundle for TemplateBundle<'a> {
//     fn get_components(self, func: &mut impl FnMut(StorageType, OwningPtr<'_>)) {
//         for (storage_type, component) in self.components {
//             let _= component.downcast();
//
//             func(storage_type, unsafe {
//                 OwningPtr::new(NonNull::from(component).cast())
//             });
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use bevy::ptr::{Aligned, OwningPtr};
    use std::cell::Cell;
    use std::ptr::NonNull;

    // 创建一个用于记录 drop 次数的辅助类型
    #[derive(Debug, Default, Clone)]
    struct DropCounter {
        count: Cell<usize>,
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            self.count.set(self.count.get() + 1);
            println!("DropCounter dropped, total drops: {}", self.count.get());
        }
    }

    #[test]
    fn double_drop() {
        let mut dd: Vec<Box<DropCounter>> = vec![Box::default(); 1];
        let c = dd.clone();

        let a = unsafe {
            OwningPtr::<'_, Aligned>::new(NonNull::from(dd.get_unchecked_mut(0).as_mut()).cast())
        };
        unsafe {
            a.drop_as::<DropCounter>();
        }
    }
}
