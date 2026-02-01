//! Utilidades de release.
//!
//! Versionado, publicación y distribución.

use semver::Version;

// ═══════════════════════════════════════════════════════════════════════════
// RELEASE TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Información de versión.
#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub version: Version,
    pub git_hash: Option<String>,
    pub build_date: String,
}

impl VersionInfo {
    pub fn new(version: &str) -> Result<Self, semver::Error> {
        Ok(Self {
            version: Version::parse(version)?,
            git_hash: None,
            build_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        })
    }

    pub fn with_git_hash(mut self, hash: &str) -> Self {
        self.git_hash = Some(hash.to_string());
        self
    }

    pub fn display_string(&self) -> String {
        match &self.git_hash {
            Some(hash) => format!("{} ({})", self.version, &hash[..7.min(hash.len())]),
            None => self.version.to_string(),
        }
    }

    pub fn is_prerelease(&self) -> bool {
        !self.version.pre.is_empty()
    }
}

/// Tipo de bump de versión.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BumpType {
    Major,
    Minor,
    Patch,
}

/// Bumper de versión.
pub struct VersionBumper;

impl VersionBumper {
    pub fn bump(version: &Version, bump_type: BumpType) -> Version {
        match bump_type {
            BumpType::Major => Version::new(version.major + 1, 0, 0),
            BumpType::Minor => Version::new(version.major, version.minor + 1, 0),
            BumpType::Patch => Version::new(version.major, version.minor, version.patch + 1),
        }
    }
}

/// Checklist de release.
#[derive(Debug, Clone)]
pub struct ReleaseChecklist {
    items: Vec<(String, bool)>,
}

impl ReleaseChecklist {
    pub fn new() -> Self {
        Self {
            items: vec![
                ("Tests passing".to_string(), false),
                ("Changelog updated".to_string(), false),
                ("Version bumped".to_string(), false),
                ("Documentation updated".to_string(), false),
                ("Git tagged".to_string(), false),
            ],
        }
    }

    pub fn check(&mut self, index: usize) {
        if let Some(item) = self.items.get_mut(index) {
            item.1 = true;
        }
    }

    pub fn is_complete(&self) -> bool {
        self.items.iter().all(|(_, done)| *done)
    }

    pub fn progress(&self) -> f64 {
        let done = self.items.iter().filter(|(_, d)| *d).count();
        done as f64 / self.items.len() as f64
    }
}

impl Default for ReleaseChecklist {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info_new() {
        let info = VersionInfo::new("3.0.1").unwrap();
        assert_eq!(info.version.major, 3);
    }

    #[test]
    fn test_version_display() {
        let info = VersionInfo::new("1.2.3")
            .unwrap()
            .with_git_hash("abc1234567890");
        assert!(info.display_string().contains("abc1234"));
    }

    #[test]
    fn test_version_bumper() {
        let v = Version::new(1, 2, 3);
        let bumped = VersionBumper::bump(&v, BumpType::Minor);
        assert_eq!(bumped, Version::new(1, 3, 0));
    }

    #[test]
    fn test_release_checklist() {
        let mut checklist = ReleaseChecklist::new();
        assert!(!checklist.is_complete());
        for i in 0..5 {
            checklist.check(i);
        }
        assert!(checklist.is_complete());
    }
}
