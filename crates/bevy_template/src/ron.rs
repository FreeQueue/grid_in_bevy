use bevy::reflect::TypeRegistryArc;
use serde::de::DeserializeSeed;
use serde::Deserialize;

use crate::content::{TemplateContent, TemplateContentDeserializer};

#[derive(Debug, Deserialize)]
pub struct TemplateRon {
    #[serde(default)]
    pub dep: Vec<String>,
    pub content: ron::Value,
}

impl TemplateRon {
    pub fn load_content(
        self,
        type_registry: TypeRegistryArc,
    ) -> Result<(Vec<String>, TemplateContent), ron::Error> {
        let type_registry = type_registry.read();
        // let content = if let Some(map) = self.content {
        //     let content_deserializer = TemplateContentDeserializer {
        //         registry: &type_registry,
        //     };
        //     content_deserializer.deserialize(map)?
        // } else {
        //     TemplateContent::default()
        // };
        let content_deserializer = TemplateContentDeserializer {
            registry: &type_registry,
        };
        let content = content_deserializer.deserialize(self.content)?;

        Ok((self.dep, content))
    }
}

#[cfg(test)]
mod test {
    const RON: &str = r#"
    (
        // dep:["base.RON"],
        content:{
            TestInfo:{value:1},
            TestTrait:{value:5},
        }
    )
    "#;

    #[test]
    fn test_ron() {
        let ron: crate::ron::TemplateRon = ron::de::from_str(RON).unwrap();
        println!("{:?}", ron);
    }
}
