//! Comando links - Gestión de enlaces.
//!
//! Analiza y repara enlaces entre documentos.

use std::path::PathBuf;
use clap::Parser;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// LINK TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Estado de un enlace.
#[derive(Debug, Clone, PartialEq)]
pub enum LinkStatus {
    /// Enlace válido con formato correcto (solo nombre de archivo sin path).
    Valid,
    /// Enlace roto - el archivo destino no existe.
    Broken,
    /// Enlace externo (http/https).
    External,
    /// Enlace circular - apunta a sí mismo.
    Circular,
    /// Enlace no-estándar - tiene path completo en lugar de solo nombre.
    /// Ejemplo: [[Proyecto OnlyCarNLD/Datos/2.8.1 Politicas_Seguridad]]
    /// Debería ser: [[2.8.1 Politicas_Seguridad]]
    NonStandard,
}

/// Un enlace encontrado.
#[derive(Debug, Clone)]
pub struct Link {
    pub source: PathBuf,
    pub target: String,
    pub line: usize,
    pub status: LinkStatus,
    /// Nombre normalizado (solo el nombre del archivo sin path).
    pub normalized: Option<String>,
}

impl Link {
    pub fn is_broken(&self) -> bool {
        self.status == LinkStatus::Broken
    }
    
    pub fn is_circular(&self) -> bool {
        self.status == LinkStatus::Circular
    }
    
    pub fn is_nonstandard(&self) -> bool {
        self.status == LinkStatus::NonStandard
    }
}

/// Resultado del análisis de enlaces.
#[derive(Debug, Clone)]
pub struct LinksResult {
    pub links: Vec<Link>,
    pub total_valid: usize,
    pub total_broken: usize,
    pub total_external: usize,
    pub total_circular: usize,
    pub total_nonstandard: usize,
}

impl LinksResult {
    pub fn new() -> Self {
        Self {
            links: Vec::new(),
            total_valid: 0,
            total_broken: 0,
            total_external: 0,
            total_circular: 0,
            total_nonstandard: 0,
        }
    }
    
    pub fn add_link(&mut self, link: Link) {
        match link.status {
            LinkStatus::Valid => self.total_valid += 1,
            LinkStatus::Broken => self.total_broken += 1,
            LinkStatus::External => self.total_external += 1,
            LinkStatus::Circular => self.total_circular += 1,
            LinkStatus::NonStandard => self.total_nonstandard += 1,
        }
        self.links.push(link);
    }
    
    pub fn broken_links(&self) -> Vec<&Link> {
        self.links.iter().filter(|l| l.is_broken()).collect()
    }
    
    pub fn health_score(&self) -> f64 {
        let total = self.links.len();
        if total == 0 {
            100.0
        } else {
            let healthy = self.total_valid + self.total_external;
            (healthy as f64 / total as f64) * 100.0
        }
    }
}

impl Default for LinksResult {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// LINKS COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de análisis de enlaces.
#[derive(Parser, Debug, Clone)]
#[command(name = "links", about = "Análisis de enlaces")]
pub struct LinksCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    
    /// Solo mostrar enlaces rotos.
    #[arg(long)]
    pub broken_only: bool,
    
    /// Intentar reparar enlaces rotos.
    #[arg(long)]
    pub fix: bool,
    
    /// Incluir enlaces externos.
    #[arg(long)]
    pub include_external: bool,
}

impl LinksCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<LinksResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        use regex::Regex;
        
        let mut result = LinksResult::new();
        
        // Patrones para detectar enlaces
        let wiki_link = Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
        let md_link = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
        
        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;
        
        for file_path in &files {
            if let Ok(content) = read_file_content(file_path) {
                // Buscar wiki links [[target]]
                for (line_idx, line) in content.lines().enumerate() {
                    for cap in wiki_link.captures_iter(line) {
                        let target = &cap[1];
                        
                        // Detectar si tiene path (contiene /) - esto es no-estándar
                        let has_path = target.contains('/');
                        
                        // Extraer nombre normalizado (sin path, sin alias)
                        let normalized_name = target
                            .split('/')
                            .last()
                            .unwrap_or(target)
                            .split('|')
                            .next()
                            .unwrap_or(target)
                            .trim()
                            .to_string();
                        
                        // Si tiene path, es NonStandard (aunque el archivo exista)
                        let status = if has_path {
                            // Verificar si el archivo destino existe para dar info adicional
                            let _exists = self.file_exists(&normalized_name, &files);
                            LinkStatus::NonStandard
                        } else {
                            self.check_link_status(data_dir, file_path, target, &files)
                        };
                        
                        result.add_link(Link {
                            source: file_path.clone(),
                            target: target.to_string(),
                            line: line_idx + 1,
                            status,
                            normalized: if has_path { Some(normalized_name) } else { None },
                        });
                    }
                    
                    // Buscar markdown links [text](path)
                    for cap in md_link.captures_iter(line) {
                        let target = &cap[2];
                        
                        // Skip external links
                        if target.starts_with("http://") || target.starts_with("https://") {
                            if self.include_external {
                                result.add_link(Link {
                                    source: file_path.clone(),
                                    target: target.to_string(),
                                    line: line_idx + 1,
                                    status: LinkStatus::External,
                                    normalized: None,
                                });
                            }
                            continue;
                        }
                        
                        let status = self.check_link_status(data_dir, file_path, target, &files);
                        result.add_link(Link {
                            source: file_path.clone(),
                            target: target.to_string(),
                            line: line_idx + 1,
                            status,
                            normalized: None,
                        });
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    /// Verifica si un archivo existe en la lista de archivos.
    fn file_exists(&self, name: &str, files: &[std::path::PathBuf]) -> bool {
        let name_lower = name.to_lowercase();
        for file in files {
            if let Some(stem) = file.file_stem() {
                if stem.to_string_lossy().to_lowercase() == name_lower {
                    return true;
                }
            }
        }
        false
    }
    
    /// Verifica si un enlace es válido.
    fn check_link_status(&self, data_dir: &std::path::Path, source: &std::path::Path, target: &str, files: &[std::path::PathBuf]) -> LinkStatus {
        // Detectar enlaces circulares (apuntan a sí mismos)
        if let Some(source_name) = source.file_stem() {
            let source_str = source_name.to_string_lossy();
            if target == source_str || target.ends_with(&format!("/{}", source_str)) {
                return LinkStatus::Circular;
            }
        }
        
        // Extraer solo el nombre del documento del target (ignorar path completo)
        // Para links tipo [[Proyecto OnlyCarNLD/Datos/2.8.1 Politicas_Seguridad]]
        let target_name = target.split('/').last().unwrap_or(target);
        // Remover pipe alias [[doc|alias]] -> doc
        let target_name = target_name.split('|').next().unwrap_or(target_name).trim();
        
        // Intentar resolver el path
        let resolved = if target.starts_with('/') {
            data_dir.join(&target[1..])
        } else {
            source.parent().unwrap_or(data_dir).join(target)
        };
        
        // Verificar si existe directamente
        if resolved.exists() {
            return LinkStatus::Valid;
        }
        
        // Verificar con extensión .md
        let with_md = resolved.with_extension("md");
        if with_md.exists() {
            return LinkStatus::Valid;
        }
        
        // Buscar por nombre en todos los archivos (fuzzy matching mejorado)
        let target_lower = target_name.to_lowercase();
        for file in files {
            if let Some(name) = file.file_stem() {
                let name_lower = name.to_string_lossy().to_lowercase();
                // Match exacto o match parcial
                if name_lower == target_lower || name_lower.ends_with(&target_lower) {
                    return LinkStatus::Valid;
                }
                // Match por ID numérico (2.8.1 == 2.8.1 algo)
                if target_lower.starts_with(char::is_numeric) && name_lower.starts_with(&target_lower) {
                    return LinkStatus::Valid;
                }
            }
        }
        
        // Último intento: buscar el target sin extensión en el path del archivo
        for file in files {
            if file.to_string_lossy().to_lowercase().contains(&target_lower) {
                return LinkStatus::Valid;
            }
        }
        
        LinkStatus::Broken
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_links_result_new() {
        let result = LinksResult::new();
        assert_eq!(result.health_score(), 100.0);
    }

    #[test]
    fn test_add_link() {
        let mut result = LinksResult::new();
        result.add_link(Link {
            source: PathBuf::from("a.md"),
            target: "b.md".to_string(),
            line: 10,
            status: LinkStatus::Valid,
            normalized: None,
        });
        
        assert_eq!(result.total_valid, 1);
    }

    #[test]
    fn test_broken_links() {
        let mut result = LinksResult::new();
        result.add_link(Link {
            source: PathBuf::from("a.md"),
            target: "missing.md".to_string(),
            line: 10,
            status: LinkStatus::Broken,
            normalized: None,
        });
        
        assert_eq!(result.broken_links().len(), 1);
    }

    #[test]
    fn test_health_score() {
        let mut result = LinksResult::new();
        result.add_link(Link {
            source: PathBuf::from("a.md"),
            target: "b.md".to_string(),
            line: 10,
            status: LinkStatus::Valid,
            normalized: None,
        });
        result.add_link(Link {
            source: PathBuf::from("a.md"),
            target: "missing.md".to_string(),
            line: 20,
            status: LinkStatus::Broken,
            normalized: None,
        });
        
        assert_eq!(result.health_score(), 50.0);
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: LinksCommand, cli: &crate::CliConfig) -> anyhow::Result<()> {
    let data_dir = std::path::Path::new(&cli.data_dir);
    let result = cmd.run(data_dir)?;
    
    if cmd.broken_only {
        for link in result.broken_links() {
            println!("❌ {}:{} → {}", 
                link.source.display(), 
                link.line, 
                link.target
            );
        }
    }
    
    println!("\n🔗 Resumen de enlaces:");
    println!("  ✅ Válidos: {}", result.total_valid);
    println!("  ❌ Rotos: {}", result.total_broken);
    println!("  ⚠️  No-estándar (con path): {}", result.total_nonstandard);
    println!("  🌐 Externos: {}", result.total_external);
    println!("  🔄 Circulares: {}", result.total_circular);
    println!("  📊 Salud: {:.1}%", result.health_score());
    
    if result.total_nonstandard > 0 {
        println!("\n⚠️  Los enlaces no-estándar tienen path completo.");
        println!("  Formato correcto: [[nombre_archivo]]");
        println!("  Formato incorrecto: [[Proyecto/Datos/nombre_archivo]]");
    }
    
    Ok(())
}
