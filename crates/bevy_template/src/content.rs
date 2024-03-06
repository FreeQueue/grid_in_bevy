use bevy::prelude::Reflect;
use bevy::reflect::serde::{TypeRegistrationDeserializer, TypedReflectDeserializer};
use bevy::reflect::TypeRegistry;
use serde::de::{DeserializeSeed, MapAccess, Visitor};
use serde::Deserializer;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct TemplateContent {
    pub items: Vec<Box<dyn Reflect>>,
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
            let info =
                map.next_value_seed(TypedReflectDeserializer::new(registration, self.registry))?;
            items.push(info);
        }
        Ok(TemplateContent { items })
    }
}
