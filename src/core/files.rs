//! File System utilities para escaneo y manipulación de archivos markdown.
//!
//! Proporciona funcionalidad para:
//! - Escanear directorios buscando archivos .md
//! - Leer y escribir archivos con manejo de errores
//! - Operaciones atómicas y backups

use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::time::SystemTime;
use walkdir::{WalkDir, DirEntry};
use crate::errors::{OcError, OcResult};

/// Opciones para escaneo de archivos.
#[derive(Debug, Clone, Default)]
pub struct ScanOptions {
    /// Patrones a excluir (glob-like).
    pub exclude_patterns: Vec<String>,
    /// Profundidad máxima de recursión (0 = infinito).
    pub max_depth: usize,
    /// Seguir symlinks.
    pub follow_symlinks: bool,
    /// Incluir archivos ocultos.
    pub include_hidden: bool,
}

impl ScanOptions {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_excludes(mut self, patterns: Vec<String>) -> Self {
        self.exclude_patterns = patterns;
        self
    }
    
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }
    
    pub fn with_hidden(mut self, include: bool) -> Self {
        self.include_hidden = include;
        self
    }
}

/// Metadata de un archivo.
#[derive(Debug, Clone)]
pub struct FileMetadata {
    /// Ruta al archivo.
    pub path: PathBuf,
    /// Tamaño en bytes.
    pub size: u64,
    /// Tiempo de modificación.
    pub modified: SystemTime,
    /// Es symlink.
    pub is_symlink: bool,
}

/// Escanea un directorio buscando archivos markdown.
pub fn get_all_md_files(dir: impl AsRef<Path>, options: &ScanOptions) -> OcResult<Vec<PathBuf>> {
    let dir = dir.as_ref();
    
    if !dir.exists() {
        return Err(OcError::DirectoryNotFound(dir.to_path_buf()));
    }
    
    let mut walker = WalkDir::new(dir)
        .follow_links(options.follow_symlinks);
    
    if options.max_depth > 0 {
        walker = walker.max_depth(options.max_depth);
    }
    
    let files: Vec<PathBuf> = walker
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_valid_md_file(e, options))
        .map(|e| e.path().to_path_buf())
        .collect();
    
    Ok(files)
}

/// Verifica si una entrada es un archivo markdown válido.
fn is_valid_md_file(entry: &DirEntry, options: &ScanOptions) -> bool {
    let path = entry.path();
    
    // Debe ser archivo
    if !path.is_file() {
        return false;
    }
    
    // Debe tener extensión .md
    let ext = path.extension().and_then(|e| e.to_str());
    if ext != Some("md") {
        return false;
    }
    
    // Verificar archivos ocultos
    if !options.include_hidden {
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.starts_with('.') {
                return false;
            }
        }
    }
    
    // Verificar patrones de exclusión
    let path_str = path.to_string_lossy();
    for pattern in &options.exclude_patterns {
        if path_str.contains(pattern) {
            return false;
        }
    }
    
    true
}

/// Lee el contenido de un archivo como UTF-8.
pub fn read_file_content(path: impl AsRef<Path>) -> OcResult<String> {
    let path = path.as_ref();
    
    if !path.exists() {
        return Err(OcError::FileNotFound(path.to_path_buf()));
    }
    
    fs::read_to_string(path).map_err(|e| OcError::FileRead {
        path: path.to_path_buf(),
        source: e,
    })
}

/// Lee el contenido de un archivo como bytes.
pub fn read_file_bytes(path: impl AsRef<Path>) -> OcResult<Vec<u8>> {
    let path = path.as_ref();
    
    if !path.exists() {
        return Err(OcError::FileNotFound(path.to_path_buf()));
    }
    
    fs::read(path).map_err(|e| OcError::FileRead {
        path: path.to_path_buf(),
        source: e,
    })
}

/// Escribe contenido a un archivo (crea directorios padre si es necesario).
pub fn write_file_content(path: impl AsRef<Path>, content: &str) -> OcResult<()> {
    let path = path.as_ref();
    
    // Crear directorio padre si no existe
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| OcError::FileWrite {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }
    }
    
    fs::write(path, content).map_err(|e| OcError::FileWrite {
        path: path.to_path_buf(),
        source: e,
    })
}

/// Escribe contenido a un archivo atómicamente (tmp + rename).
pub fn write_file_atomic(path: impl AsRef<Path>, content: &str) -> OcResult<()> {
    let path = path.as_ref();
    let tmp_path = path.with_extension("tmp");
    
    // Crear directorio padre si no existe
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| OcError::FileWrite {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }
    }
    
    // Escribir a archivo temporal
    fs::write(&tmp_path, content).map_err(|e| OcError::FileWrite {
        path: tmp_path.clone(),
        source: e,
    })?;
    
    // Renombrar atómicamente
    fs::rename(&tmp_path, path).map_err(|e| OcError::FileWrite {
        path: path.to_path_buf(),
        source: e,
    })
}

/// Obtiene metadata de un archivo.
pub fn get_file_metadata(path: impl AsRef<Path>) -> OcResult<FileMetadata> {
    let path = path.as_ref();
    
    if !path.exists() {
        return Err(OcError::FileNotFound(path.to_path_buf()));
    }
    
    let metadata = fs::metadata(path).map_err(|e| OcError::FileRead {
        path: path.to_path_buf(),
        source: e,
    })?;
    
    let symlink_metadata = fs::symlink_metadata(path).ok();
    let is_symlink = symlink_metadata.map(|m| m.file_type().is_symlink()).unwrap_or(false);
    
    Ok(FileMetadata {
        path: path.to_path_buf(),
        size: metadata.len(),
        modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
        is_symlink,
    })
}

/// Crea un backup de un archivo.
pub fn backup_file(path: impl AsRef<Path>) -> OcResult<PathBuf> {
    let path = path.as_ref();
    
    if !path.exists() {
        return Err(OcError::FileNotFound(path.to_path_buf()));
    }
    
    let backup_path = path.with_extension("md.bak");
    
    fs::copy(path, &backup_path).map_err(|e| OcError::FileWrite {
        path: backup_path.clone(),
        source: e,
    })?;
    
    Ok(backup_path)
}

/// Elimina un archivo de backup si existe.
pub fn remove_backup(path: impl AsRef<Path>) -> OcResult<bool> {
    let backup_path = path.as_ref().with_extension("md.bak");
    
    if backup_path.exists() {
        fs::remove_file(&backup_path).map_err(|e| OcError::FileWrite {
            path: backup_path,
            source: e,
        })?;
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Restaura un archivo desde su backup.
pub fn restore_from_backup(path: impl AsRef<Path>) -> OcResult<()> {
    let path = path.as_ref();
    let backup_path = path.with_extension("md.bak");
    
    if !backup_path.exists() {
        return Err(OcError::FileNotFound(backup_path));
    }
    
    fs::copy(&backup_path, path).map_err(|e| OcError::FileWrite {
        path: path.to_path_buf(),
        source: e,
    })?;
    
    Ok(())
}

/// Cuenta archivos markdown en un directorio.
pub fn count_md_files(dir: impl AsRef<Path>) -> OcResult<usize> {
    let files = get_all_md_files(dir, &ScanOptions::default())?;
    Ok(files.len())
}

/// Obtiene el tamaño total de archivos markdown en un directorio.
pub fn total_md_size(dir: impl AsRef<Path>) -> OcResult<u64> {
    let files = get_all_md_files(dir, &ScanOptions::default())?;
    let mut total = 0u64;
    
    for file in files {
        if let Ok(meta) = get_file_metadata(&file) {
            total += meta.size;
        }
    }
    
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_scan_md_files() {
        let dir = tempdir().unwrap();
        let md_file = dir.path().join("test.md");
        let txt_file = dir.path().join("test.txt");
        
        fs::write(&md_file, "# Test").unwrap();
        fs::write(&txt_file, "Not markdown").unwrap();
        
        let files = get_all_md_files(dir.path(), &ScanOptions::default()).unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("test.md"));
    }

    #[test]
    fn test_read_write_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("content.md");
        
        let content = "# Hello World\n\nThis is a test.";
        write_file_content(&file_path, content).unwrap();
        
        let read_content = read_file_content(&file_path).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_atomic_write() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("atomic.md");
        
        let content = "Atomic content";
        write_file_atomic(&file_path, content).unwrap();
        
        assert!(file_path.exists());
        assert!(!file_path.with_extension("tmp").exists());
        
        let read_content = read_file_content(&file_path).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_file_metadata() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("meta.md");
        
        let content = "Some content here";
        fs::write(&file_path, content).unwrap();
        
        let meta = get_file_metadata(&file_path).unwrap();
        assert_eq!(meta.size, content.len() as u64);
        assert!(!meta.is_symlink);
    }

    #[test]
    fn test_backup_restore() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("original.md");
        
        let original = "Original content";
        fs::write(&file_path, original).unwrap();
        
        // Create backup
        let backup_path = backup_file(&file_path).unwrap();
        assert!(backup_path.exists());
        
        // Modify original
        fs::write(&file_path, "Modified content").unwrap();
        
        // Restore
        restore_from_backup(&file_path).unwrap();
        let restored = read_file_content(&file_path).unwrap();
        assert_eq!(restored, original);
    }

    #[test]
    fn test_exclude_patterns() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("node_modules");
        fs::create_dir(&subdir).unwrap();
        
        let good_file = dir.path().join("good.md");
        let excluded_file = subdir.join("excluded.md");
        
        fs::write(&good_file, "Good").unwrap();
        fs::write(&excluded_file, "Excluded").unwrap();
        
        let options = ScanOptions::default()
            .with_excludes(vec!["node_modules".to_string()]);
        
        let files = get_all_md_files(dir.path(), &options).unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("good.md"));
    }
}
