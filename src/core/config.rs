//! Configuración global del sistema oc_diagdoc.
//!
//! Soporta múltiples fuentes de configuración:
//! - Archivo `.oc_diagdoc/config.yaml`
//! - Variables de entorno `OC_*`
//! - Argumentos de línea de comandos

use crate::errors::{OcError, OcResult};
use crate::DEFAULT_DATA_DIR;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Nombre del directorio de configuración.
pub const CONFIG_DIR: &str = ".oc_diagdoc";
/// Nombre del archivo de configuración.
pub const CONFIG_FILE: &str = "config.yaml";

/// Configuración principal de oc_diagdoc.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OcConfig {
    /// Directorio de datos (default: "Datos/").
    pub data_dir: PathBuf,
    /// Directorio de salida para reportes.
    pub output_dir: PathBuf,
    /// Habilitar cache persistente.
    pub cache_enabled: bool,
    /// Directorio de cache.
    pub cache_dir: PathBuf,
    /// Modo verbose.
    pub verbose: bool,
    /// Procesamiento paralelo.
    pub parallel: bool,
    /// Número de threads (0 = auto).
    pub threads: usize,
    /// Configuración de validación.
    pub validation: ValidationConfig,
    /// Configuración de cobertura.
    pub coverage: CoverageConfig,
}

impl Default for OcConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from(DEFAULT_DATA_DIR),
            output_dir: PathBuf::from("./output"),
            cache_enabled: true,
            cache_dir: PathBuf::from(".oc_diagdoc/cache"),
            verbose: false,
            parallel: true,
            threads: 0,
            validation: ValidationConfig::default(),
            coverage: CoverageConfig::default(),
        }
    }
}

impl OcConfig {
    /// Crea configuración desde builder.
    pub fn builder() -> OcConfigBuilder {
        OcConfigBuilder::new()
    }

    /// Carga configuración desde archivo YAML.
    pub fn from_file(path: impl AsRef<Path>) -> OcResult<Self> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).map_err(|e| OcError::FileRead {
            path: path.to_path_buf(),
            source: e,
        })?;

        serde_yaml::from_str(&content).map_err(|e| OcError::YamlParse {
            path: path.to_path_buf(),
            message: e.to_string(),
        })
    }

    /// Carga configuración desde directorio de trabajo.
    pub fn from_cwd() -> OcResult<Self> {
        let config_path = Path::new(CONFIG_DIR).join(CONFIG_FILE);

        if config_path.exists() {
            Self::from_file(&config_path)
        } else {
            Ok(Self::default())
        }
    }

    /// Carga configuración desde variables de entorno.
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(val) = env::var("OC_DATA_DIR") {
            config.data_dir = PathBuf::from(val);
        }
        if let Ok(val) = env::var("OC_OUTPUT_DIR") {
            config.output_dir = PathBuf::from(val);
        }
        if let Ok(val) = env::var("OC_CACHE_ENABLED") {
            config.cache_enabled = val.parse().unwrap_or(true);
        }
        if let Ok(val) = env::var("OC_VERBOSE") {
            config.verbose = val.parse().unwrap_or(false);
        }
        if let Ok(val) = env::var("OC_PARALLEL") {
            config.parallel = val.parse().unwrap_or(true);
        }
        if let Ok(val) = env::var("OC_THREADS") {
            config.threads = val.parse().unwrap_or(0);
        }
        if let Ok(val) = env::var("OC_MIN_WORDS") {
            config.coverage.min_words = val.parse().unwrap_or(300);
        }

        config
    }

    /// Merge con otra configuración (otra tiene prioridad).
    pub fn merge(&mut self, other: Self) {
        if other.data_dir != PathBuf::from(DEFAULT_DATA_DIR) {
            self.data_dir = other.data_dir;
        }
        if other.output_dir != PathBuf::from("./output") {
            self.output_dir = other.output_dir;
        }
        if other.verbose {
            self.verbose = true;
        }
        if !other.parallel {
            self.parallel = false;
        }
        if other.threads > 0 {
            self.threads = other.threads;
        }
    }

    /// Valida que la configuración sea válida.
    pub fn validate(&self) -> OcResult<()> {
        // Verificar que data_dir existe
        if !self.data_dir.exists() {
            return Err(OcError::DirectoryNotFound(self.data_dir.clone()));
        }

        // Crear output_dir si no existe
        if !self.output_dir.exists() {
            fs::create_dir_all(&self.output_dir).map_err(|e| OcError::FileWrite {
                path: self.output_dir.clone(),
                source: e,
            })?;
        }

        // Crear cache_dir si cache está habilitado
        if self.cache_enabled && !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir).map_err(|e| OcError::FileWrite {
                path: self.cache_dir.clone(),
                source: e,
            })?;
        }

        Ok(())
    }

    /// Guarda configuración a archivo.
    pub fn save(&self, path: impl AsRef<Path>) -> OcResult<()> {
        let path = path.as_ref();

        // Crear directorio padre si no existe
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| OcError::FileWrite {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        let yaml = serde_yaml::to_string(self).map_err(|e| OcError::Custom(e.to_string()))?;

        fs::write(path, yaml).map_err(|e| OcError::FileWrite {
            path: path.to_path_buf(),
            source: e,
        })
    }

    /// Ruta completa al archivo de configuración.
    pub fn config_path() -> PathBuf {
        PathBuf::from(CONFIG_DIR).join(CONFIG_FILE)
    }
}

/// Configuración de validación.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ValidationConfig {
    /// Validar YAML frontmatter.
    pub check_yaml: bool,
    /// Validar links internos.
    pub check_links: bool,
    /// Validar huérfanos.
    pub check_orphans: bool,
    /// Modo estricto de esquema.
    pub strict_schema: bool,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            check_yaml: true,
            check_links: true,
            check_orphans: true,
            strict_schema: false,
        }
    }
}

/// Configuración de cobertura.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CoverageConfig {
    /// Mínimo de palabras para "aceptable".
    pub min_words: usize,
    /// Mínimo de secciones.
    pub min_sections: usize,
    /// Detectar placeholders.
    pub detect_placeholders: bool,
    /// Detectar stubs.
    pub detect_stubs: bool,
}

impl Default for CoverageConfig {
    fn default() -> Self {
        Self {
            min_words: 300,
            min_sections: 3,
            detect_placeholders: true,
            detect_stubs: true,
        }
    }
}

/// Builder para OcConfig.
#[derive(Debug, Default)]
pub struct OcConfigBuilder {
    data_dir: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    cache_enabled: Option<bool>,
    cache_dir: Option<PathBuf>,
    verbose: Option<bool>,
    parallel: Option<bool>,
    threads: Option<usize>,
}

impl OcConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn data_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.data_dir = Some(dir.into());
        self
    }

    pub fn output_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.output_dir = Some(dir.into());
        self
    }

    pub fn cache_enabled(mut self, enabled: bool) -> Self {
        self.cache_enabled = Some(enabled);
        self
    }

    pub fn cache_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.cache_dir = Some(dir.into());
        self
    }

    pub fn verbose(mut self, v: bool) -> Self {
        self.verbose = Some(v);
        self
    }

    pub fn parallel(mut self, p: bool) -> Self {
        self.parallel = Some(p);
        self
    }

    pub fn threads(mut self, t: usize) -> Self {
        self.threads = Some(t);
        self
    }

    pub fn build(self) -> OcConfig {
        let default = OcConfig::default();
        OcConfig {
            data_dir: self.data_dir.unwrap_or(default.data_dir),
            output_dir: self.output_dir.unwrap_or(default.output_dir),
            cache_enabled: self.cache_enabled.unwrap_or(default.cache_enabled),
            cache_dir: self.cache_dir.unwrap_or(default.cache_dir),
            verbose: self.verbose.unwrap_or(default.verbose),
            parallel: self.parallel.unwrap_or(default.parallel),
            threads: self.threads.unwrap_or(default.threads),
            validation: default.validation,
            coverage: default.coverage,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = OcConfig::default();
        assert_eq!(config.data_dir, PathBuf::from("Datos"));
        assert!(config.cache_enabled);
        assert!(config.parallel);
    }

    #[test]
    fn test_builder() {
        let config = OcConfig::builder()
            .data_dir("MisDatos")
            .verbose(true)
            .threads(4)
            .build();

        assert_eq!(config.data_dir, PathBuf::from("MisDatos"));
        assert!(config.verbose);
        assert_eq!(config.threads, 4);
    }

    #[test]
    fn test_save_and_load() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.yaml");

        let config = OcConfig::builder()
            .data_dir("TestDatos")
            .verbose(true)
            .build();

        config.save(&config_path).unwrap();

        let loaded = OcConfig::from_file(&config_path).unwrap();
        assert_eq!(loaded.data_dir, PathBuf::from("TestDatos"));
        assert!(loaded.verbose);
    }

    #[test]
    fn test_coverage_config() {
        let config = CoverageConfig::default();
        assert_eq!(config.min_words, 300);
        assert!(config.detect_placeholders);
    }
}
