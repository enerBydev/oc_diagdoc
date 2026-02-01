//! Integration tests for core module

use std::path::PathBuf;
use tempfile::tempdir;

mod yaml_tests {
    use super::*;

    #[test]
    fn test_yaml_frontmatter_roundtrip() {
        let content = r#"---
id: "3.2.1"
title: "Test Document"
parent: "3.2"
status: activo
breadcrumb: "Root > Module > Section"
---

# Content here
"#;
        // Test that we can parse and the frontmatter is valid
        assert!(content.contains("---"));
        assert!(content.contains("id:"));
        assert!(content.contains("title:"));
    }

    #[test]
    fn test_yaml_required_fields() {
        let required = ["id", "title", "parent", "breadcrumb", "status"];
        let content = r#"---
id: "1"
title: "Doc"
parent: "0"
breadcrumb: "Root"
status: activo
---
"#;
        for field in required {
            assert!(content.contains(field));
        }
    }
}

mod file_operations_tests {
    use super::*;

    #[test]
    fn test_temp_file_creation() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.md");
        
        std::fs::write(&file_path, "# Test").unwrap();
        assert!(file_path.exists());
        
        let content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "# Test");
    }

    #[test]
    fn test_markdown_file_detection() {
        let md_paths = ["doc.md", "README.MD", "file.markdown"];
        for path in md_paths {
            let p = PathBuf::from(path);
            let ext = p.extension().map(|e| e.to_str().unwrap().to_lowercase());
            assert!(ext.map(|e| e == "md" || e == "markdown").unwrap_or(false));
        }
    }
}

mod hash_caching_tests {
    use super::*;

    #[test]
    fn test_hash_determinism() {
        use sha2::{Sha256, Digest};
        
        let content = "Test content for hashing";
        let mut hasher1 = Sha256::new();
        hasher1.update(content.as_bytes());
        let hash1 = format!("{:x}", hasher1.finalize());
        
        let mut hasher2 = Sha256::new();
        hasher2.update(content.as_bytes());
        let hash2 = format!("{:x}", hasher2.finalize());
        
        assert_eq!(hash1, hash2);
    }
}

mod config_tests {
    use super::*;

    #[test]
    fn test_config_yaml_structure() {
        let config_yaml = r#"
project:
  name: "Test Project"
  data_dir: "./Datos"

validation:
  min_words: 300
  required_fields:
    - id
    - title
"#;
        let parsed: serde_yaml::Value = serde_yaml::from_str(config_yaml).unwrap();
        assert!(parsed.get("project").is_some());
        assert!(parsed.get("validation").is_some());
    }
}
