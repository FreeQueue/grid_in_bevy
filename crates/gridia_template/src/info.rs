use core::fmt;
use std::any::TypeId;
use std::fmt::Debug;
use std::sync::Arc;

use bevy::prelude::*;
use bevy::utils::TypeIdMap;

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
    type Trait: Trait<Info = Self>;

    fn gen(&self) -> Self::Trait;
}

impl<T: Info> InfoAny for T {
    fn gen_reflect(&self) -> Box<dyn Reflect> {
        Box::new(self.gen())
    }
}

pub trait Trait: Component + Reflect {
    type Info: Info<Trait = Self>;
}

#[derive(Component, Debug, Clone, DerefMut, Deref)]
pub struct Infos {
    infos: TypeIdMap<Arc<dyn InfoAny>>,
}

impl Infos {
    pub fn new(infos: TypeIdMap<Arc<dyn InfoAny>>) -> Infos {
        Infos { infos }
    }

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
    pub fn get_by_component<T: Trait>(&self) -> Option<&T::Info> {
        self.infos.get(&TypeId::of::<T::Info>()).map(|info| {
            info.as_ref()
                .as_reflect()
                .downcast_ref::<T::Info>()
                .expect("Failed to downcast info to component info")
        })
    }

    // pub fn iter(&self) -> impl Iterator<Item = (&TypeId, &Arc<dyn InfoAny>)> {
    //     self.infos.iter() //.map(|(id, info)| (*id, info.as_ref()))
    // }
    //
    // pub fn values(&self) -> impl Iterator<Item = &Arc<dyn InfoAny>> {
    //     self.infos.values() //.map(|info| info.as_ref())
    // }
}
