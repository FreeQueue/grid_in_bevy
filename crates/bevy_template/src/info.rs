use bevy::prelude::*;
use bevy::utils::TypeIdMap;
use std::any::TypeId;
use std::sync::Arc;

#[reflect_trait]
pub trait InfoAny: Reflect {
    fn gen_reflect(&self) -> Box<dyn Reflect>;
}

pub trait Info: InfoAny {
    type Component: InfoComponent<Info = Self>;

    fn gen(&self) -> Self::Component;
}

impl<T: Info> InfoAny for T {
    fn gen_reflect(&self) -> Box<dyn Reflect> {
        Box::new(self.gen())
    }
}

pub trait InfoComponent: Component + Reflect {
    type Info: Info<Component = Self>;
}

pub struct Infos {
    pub(crate) infos: TypeIdMap<Arc<dyn InfoAny>>,
}

impl Infos {
    #[allow(dead_code)]
    pub fn get<T: InfoComponent>(&self) -> Option<&T::Info> {
        self.infos.get(&TypeId::of::<T>()).map(|info| {
            info.as_ref()
                .as_reflect()
                .downcast_ref::<T::Info>()
                .expect("Failed to downcast info to component info")
        })
    }

    pub fn get_reflect(&self, type_id: TypeId) -> Option<&dyn InfoAny> {
        self.infos.get(&type_id).map(|info| info.as_ref())
    }
}
