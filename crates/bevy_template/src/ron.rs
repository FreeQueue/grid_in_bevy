use crate::content::{TemplateContent, TemplateContentDeserializer};
use bevy::reflect::TypeRegistry;
use ron::de::SpannedError;
use serde::de::DeserializeSeed;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TemplateRon {
    #[serde(default)]
    pub dep: Vec<String>,
    pub content: String,
}

impl TemplateRon {
    pub fn load_content(
        &self,
        type_registry: &TypeRegistry,
    ) -> Result<TemplateContent, SpannedError> {
        let content_deserializer = TemplateContentDeserializer {
            registry: type_registry,
        };
        let mut deserializer = ron::de::Deserializer::from_str(&self.content)?;
        let content = content_deserializer
            .deserialize(&mut deserializer)
            .map_err(move |e| deserializer.span_error(e))?;
        Ok(content)
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
