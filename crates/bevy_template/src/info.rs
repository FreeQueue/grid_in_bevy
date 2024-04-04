use bevy::prelude::*;
use bevy::utils::TypeIdMap;
use core::fmt;
use std::any::TypeId;
use std::fmt::Debug;
use std::sync::Arc;

#[reflect_trait]
pub trait InfoAny: Reflect {
    fn gen_reflect(&self) -> Box<dyn Reflect>;
}

impl Debug for dyn InfoAny {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(self.reflect_type_ident().unwrap())
            .finish_non_exhaustive()
    }
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

#[derive(Component, Debug)]
pub struct Infos {
    pub(crate) infos: TypeIdMap<Arc<dyn InfoAny>>,
}

impl Infos {
    #[allow(unused)]
    pub fn get<T: Info>(&self) -> Option<&T> {
        self.get_by_type_id(TypeId::of::<T>()).map(|info| {
            info.as_reflect()
                .downcast_ref::<T>()
                .expect("downcast info fail")
        })
    }

    pub fn get_by_type_id(&self, type_id: TypeId) -> Option<&dyn InfoAny> {
        self.infos.get(&type_id).map(|info| info.as_ref())
    }

    #[allow(dead_code)]
    pub fn get_by_component<T: InfoComponent>(&self) -> Option<&T::Info> {
        self.infos.get(&TypeId::of::<T::Info>()).map(|info| {
            info.as_ref()
                .as_reflect()
                .downcast_ref::<T::Info>()
                .expect("Failed to downcast info to component info")
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &dyn InfoAny> {
        self.infos.values().map(|info| info.as_ref())
    }
}
