//! Integration tests for types module

use oc_diagdoc_lib::types::{
    DocumentId, DocumentStatus, DocumentType, DataPath, OcDate, ContentHash, Breadcrumb,
};

mod document_id_tests {
    use super::*;

    #[test]
    fn test_document_id_parsing() {
        let id = DocumentId::from_string("3.2.1");
        assert!(id.is_ok());
        
        let parsed = id.unwrap();
        assert_eq!(parsed.to_string(), "3.2.1");
    }

    #[test]
    fn test_document_id_hierarchy() {
        let parent = DocumentId::from_string("3").unwrap();
        let child = DocumentId::from_string("3.1").unwrap();
        
        assert!(child.is_child_of(&parent));
    }

    #[test]
    fn test_document_id_master() {
        let master = DocumentId::from_string("0").unwrap();
        assert!(master.is_master());
    }
}

mod status_tests {
    use super::*;

    #[test]
    fn test_status_from_string() {
        let status = DocumentStatus::from_str_opt("activo");
        assert!(status.is_some());
        assert!(matches!(status.unwrap(), DocumentStatus::Active));
    }

    #[test]
    fn test_status_health() {
        assert!(DocumentStatus::Active.is_healthy());
        assert!(!DocumentStatus::Deprecated.is_healthy());
    }

    #[test]
    fn test_all_statuses() {
        let statuses = ["activo", "borrador", "revisado", "deprecado", "archivado"];
        for s in statuses {
            assert!(DocumentStatus::from_str_opt(s).is_some());
        }
    }
}

mod path_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_data_path_markdown_detection() {
        let path = DataPath::new(PathBuf::from("doc.md"));
        assert!(path.is_markdown());

        let path2 = DataPath::new(PathBuf::from("file.txt"));
        assert!(!path2.is_markdown());
    }

    #[test]
    fn test_data_path_id_extraction() {
        let path = DataPath::new(PathBuf::from("3.2.1. Documento.md"));
        let id = path.extract_id();
        assert!(id.is_some());
        assert_eq!(id.unwrap(), "3.2.1");
    }
}

mod date_tests {
    use super::*;

    #[test]
    fn test_date_parsing_iso() {
        let date = OcDate::parse("2026-02-01");
        assert!(date.is_some());
    }

    #[test]
    fn test_date_parsing_various_formats() {
        let formats = [
            "2026-02-01",
            "01/02/2026",
            "Feb 1, 2026",
        ];
        
        for fmt in formats {
            let parsed = OcDate::parse(fmt);
            assert!(parsed.is_some(), "Failed to parse: {}", fmt);
        }
    }

    #[test]
    fn test_date_now() {
        let now = OcDate::now();
        assert!(now.is_today());
    }
}

mod hash_tests {
    use super::*;

    #[test]
    fn test_hash_consistency() {
        let content = "Test content for hashing";
        let hash1 = ContentHash::from_content(content);
        let hash2 = ContentHash::from_content(content);
        
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_different_content() {
        let hash1 = ContentHash::from_content("content1");
        let hash2 = ContentHash::from_content("content2");
        
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hash_short_representation() {
        let hash = ContentHash::from_content("test");
        let short = hash.short();
        assert_eq!(short.len(), 8);
    }
}

mod breadcrumb_tests {
    use super::*;

    #[test]
    fn test_breadcrumb_parsing() {
        let bc = Breadcrumb::from_str("Root > Module > Section");
        assert!(bc.is_ok());
        
        let parsed = bc.unwrap();
        assert_eq!(parsed.depth(), 3);
    }

    #[test]
    fn test_breadcrumb_root() {
        let bc = Breadcrumb::from_str("Root > Child").unwrap();
        assert_eq!(bc.root(), "Root");
    }

    #[test]
    fn test_breadcrumb_current() {
        let bc = Breadcrumb::from_str("A > B > C").unwrap();
        assert_eq!(bc.current(), "C");
    }
}
