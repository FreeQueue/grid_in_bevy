use bevy::prelude::*;
use bevy::utils::TypeIdMap;
use std::any::TypeId;

pub trait Info: Reflect {
    type Trait: Trait<Info = Self>;

    fn init(&self) -> Self::Trait;

    fn init_reflect(&self) -> Box<dyn Reflect> {
        Box::new(self.init())
    }
}

pub trait Trait: Component + Reflect {
    type Info: Info<Trait = Self>;
}

#[derive(Asset, TypePath)]
pub struct Infos {
    infos: TypeIdMap<Box<dyn Reflect>>,
}

impl Infos {
    pub fn get<T: Trait>(&self) -> Option<&T::Info> {
        let info = self.infos.get(&TypeId::of::<T>()).unwrap();
        info.downcast_ref()
    }
}

#[cfg(test)]
mod test {
    use crate::info::{Info, Trait};
    use bevy::prelude::*;

    #[derive(Reflect)]
    struct TestInfo {
        value: i32,
    }

    impl Info for TestInfo {
        type Trait = TestTrait;

        fn init(&self) -> Self::Trait {
            TestTrait { value: self.value }
        }
    }

    #[derive(Component, Reflect)]
    struct TestTrait {
        value: i32,
    }

    impl Trait for TestTrait {
        type Info = TestInfo;
    }

    fn test() {
        App::new()
            .add_plugins(MinimalPlugins)
            .register_type::<TestInfo>()
            .register_type::<TestTrait>()
            .add_systems(Startup, test)
            .run();
    }

    fn setup(type_registry: Res<AppTypeRegistry>) {}
}
