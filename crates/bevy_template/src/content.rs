use std::fmt::Formatter;
use std::sync::Arc;

use bevy::prelude::Reflect;
use bevy::reflect::serde::{TypeRegistrationDeserializer, TypedReflectDeserializer};
use bevy::reflect::{TypeRegistry, TypeRegistryArc};
use bevy::utils::TypeIdMap;
use serde::de::{DeserializeSeed, MapAccess, Visitor};
use serde::Deserializer;

use crate::info::{InfoAny, ReflectInfoAny};

#[derive(Debug, Default)]
pub struct TemplateContent {
    pub items: Vec<Box<dyn Reflect>>,
}

impl TemplateContent {
    pub(crate) fn parse(
        self,
        type_registry: TypeRegistryArc,
        components: &mut TypeIdMap<Arc<dyn Reflect>>,
        infos: &mut TypeIdMap<Arc<dyn InfoAny>>,
    ) {
        let type_registry = type_registry.read();
        //反向迭代去重，后覆盖前
        for item in self.items.into_iter().rev() {
            let type_id = item.type_id();
            //如果component已有，无需检测info，没有component必然没有info
            if components.contains_key(&type_id) {
                continue;
            }
            if let Some(reflect) = type_registry.get_type_data::<ReflectInfoAny>(type_id) {
                let info = reflect.get_boxed(item).unwrap();
                let component = info.gen_reflect();
                if components
                    .try_insert(component.type_id(), component.into())
                    .is_ok()
                {
                    infos.insert(type_id, info.into());
                }
            } else {
                components.insert(type_id, item.into());
            };
        }
    }
}

pub struct TemplateContentDeserializer<'a> {
    pub(crate) registry: &'a TypeRegistry,
}

impl<'a, 'de> DeserializeSeed<'de> for TemplateContentDeserializer<'a> {
    type Value = TemplateContent;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(TemplateContentVisitor {
            registry: self.registry,
        })
    }
}

struct TemplateContentVisitor<'a> {
    registry: &'a TypeRegistry,
}

impl<'a, 'de> Visitor<'de> for TemplateContentVisitor<'a> {
    type Value = TemplateContent;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("template.content")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let size = map.size_hint().unwrap_or(0);
        let mut items = Vec::with_capacity(size);
        while let Some(registration) =
            map.next_key_seed(TypeRegistrationDeserializer::new(self.registry))?
        {
            let item =
                map.next_value_seed(TypedReflectDeserializer::new(registration, self.registry))?;

            items.push(item);
        }
        Ok(TemplateContent { items })
    }
}
