//! Trait para serialización uniforme.
//!
//! Proporciona serialización consistente a múltiples formatos.

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// SERIALIZATION FORMAT
// ═══════════════════════════════════════════════════════════════════════════

/// Formatos de serialización.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SerializationFormat {
    #[default]
    Json,
    JsonPretty,
    Yaml,
    Toml,
}

// ═══════════════════════════════════════════════════════════════════════════
// TRAIT SERIALIZABLE
// ═══════════════════════════════════════════════════════════════════════════

/// Trait para serialización uniforme.
pub trait Serializable: Serialize {
    /// Serializa a JSON.
    fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }

    /// Serializa a JSON pretty.
    fn to_json_pretty(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self).map_err(|e| e.to_string())
    }

    /// Serializa a YAML.
    fn to_yaml(&self) -> Result<String, String> {
        serde_yaml::to_string(self).map_err(|e| e.to_string())
    }

    /// Serializa en el formato especificado.
    fn serialize_to(&self, format: SerializationFormat) -> Result<String, String> {
        match format {
            SerializationFormat::Json => self.to_json(),
            SerializationFormat::JsonPretty => self.to_json_pretty(),
            SerializationFormat::Yaml => self.to_yaml(),
            SerializationFormat::Toml => Err("TOML not yet implemented".to_string()),
        }
    }
}

/// Implementación automática para todos los Serialize.
impl<T: Serialize> Serializable for T {}

// ═══════════════════════════════════════════════════════════════════════════
// TRAIT DESERIALIZABLE
// ═══════════════════════════════════════════════════════════════════════════

/// Trait para deserialización uniforme.
pub trait Deserializable: for<'de> Deserialize<'de> + Sized {
    /// Deserializa desde JSON.
    fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }

    /// Deserializa desde YAML.
    fn from_yaml(yaml: &str) -> Result<Self, String> {
        serde_yaml::from_str(yaml).map_err(|e| e.to_string())
    }
}

/// Implementación automática para todos los Deserialize.
impl<T: for<'de> Deserialize<'de> + Sized> Deserializable for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestStruct {
        name: String,
        value: i32,
    }

    #[test]
    fn test_to_json() {
        let obj = TestStruct {
            name: "test".to_string(),
            value: 42,
        };
        let json = obj.to_json().unwrap();

        assert!(json.contains("test"));
        assert!(json.contains("42"));
    }

    #[test]
    fn test_from_json() {
        let json = r#"{"name":"test","value":42}"#;
        let obj: TestStruct = TestStruct::from_json(json).unwrap();

        assert_eq!(obj.name, "test");
        assert_eq!(obj.value, 42);
    }

    #[test]
    fn test_to_yaml() {
        let obj = TestStruct {
            name: "test".to_string(),
            value: 42,
        };
        let yaml = obj.to_yaml().unwrap();

        assert!(yaml.contains("name:"));
        assert!(yaml.contains("test"));
    }
}
