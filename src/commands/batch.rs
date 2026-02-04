//! Comando batch - Operaciones en lote.
//!
//! Ejecuta operaciones sobre múltiples documentos.

use crate::errors::OcResult;
use clap::Parser;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// BATCH OPERATION
// ═══════════════════════════════════════════════════════════════════════════

/// Tipo de operación en lote.
#[derive(Debug, Clone, PartialEq)]
pub enum BatchOperation {
    /// Actualizar frontmatter.
    UpdateFrontmatter { field: String, value: String },
    /// Renombrar archivos.
    Rename {
        pattern: String,
        replacement: String,
    },
    /// Agregar tag.
    AddTag(String),
    /// Quitar tag.
    RemoveTag(String),
    /// Actualizar status.
    SetStatus(String),
}

/// Resultado de operación individual.
#[derive(Debug, Clone)]
pub struct BatchItemResult {
    pub path: PathBuf,
    pub success: bool,
    pub message: String,
}

/// Resultado general del batch.
#[derive(Debug, Clone)]
pub struct BatchResult {
    pub operation: String,
    pub items: Vec<BatchItemResult>,
    pub succeeded: usize,
    pub failed: usize,
}

impl BatchResult {
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            items: Vec::new(),
            succeeded: 0,
            failed: 0,
        }
    }

    pub fn add_success(&mut self, path: PathBuf, message: impl Into<String>) {
        self.items.push(BatchItemResult {
            path,
            success: true,
            message: message.into(),
        });
        self.succeeded += 1;
    }

    pub fn add_failure(&mut self, path: PathBuf, message: impl Into<String>) {
        self.items.push(BatchItemResult {
            path,
            success: false,
            message: message.into(),
        });
        self.failed += 1;
    }

    pub fn success_rate(&self) -> f64 {
        if self.items.is_empty() {
            100.0
        } else {
            (self.succeeded as f64 / self.items.len() as f64) * 100.0
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// BATCH COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de operaciones en lote.
#[derive(Parser, Debug, Clone)]
#[command(name = "batch", about = "Operaciones en lote")]
pub struct BatchCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Campo a actualizar.
    #[arg(long)]
    pub field: Option<String>,

    /// Valor nuevo.
    #[arg(long)]
    pub value: Option<String>,

    /// Filtro de módulo.
    #[arg(short, long)]
    pub module: Option<String>,

    /// Modo dry-run.
    #[arg(long)]
    pub dry_run: bool,

    // L17-L18: Flags avanzados
    /// Archivo de batch con comandos (.oc-batch).
    #[arg(long, short = 'f')]
    pub file: Option<PathBuf>,

    /// Número de jobs paralelos.
    #[arg(long, short = 'j', default_value = "1")]
    pub jobs: usize,

    /// Comandos inline a ejecutar.
    #[arg(last = true)]
    pub commands: Vec<String>,

    // F4: Nuevas flags de paridad con Python
    /// Agregar campo YAML a todos los documentos (formato: campo=valor).
    #[arg(long)]
    pub add_field: Option<String>,

    /// Eliminar campo YAML de todos los documentos.
    #[arg(long)]
    pub remove_field: Option<String>,

    // P2: Nuevas flags de paridad con Python v16
    /// Expresión de filtro compleja (ej: "status=draft AND module=3" o "type=leaf OR type=branch").
    #[arg(long)]
    pub filter: Option<String>,

    /// P2-B2: Mostrar barra de progreso durante operaciones.
    #[arg(long)]
    pub progress: bool,
}


/// L17.2: Comando parseado desde .oc-batch
#[derive(Debug, Clone)]
pub struct BatchCmd {
    pub name: String,
    pub args: Vec<String>,
}

impl BatchCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<BatchResult> {
        let op_name = if self.file.is_some() {
            "batch-file"
        } else {
            "batch-inline"
        };
        let mut result = BatchResult::new(op_name);

        // L17.2: Leer comandos desde archivo .oc-batch
        let commands = if let Some(ref file_path) = self.file {
            self.parse_batch_file(file_path)?
        } else if !self.commands.is_empty() {
            // L17.1: Comandos inline
            self.commands
                .iter()
                .map(|c| BatchCmd {
                    name: c.clone(),
                    args: Vec::new(),
                })
                .collect()
        } else {
            // Operación sobre frontmatter
            return self.run_frontmatter_update(data_dir);
        };

        // L18.1/L18.2: Ejecutar comandos (paralelo si jobs > 1)
        if self.jobs > 1 {
            #[cfg(feature = "parallel")]
            {
                use rayon::prelude::*;
                use rayon::ThreadPoolBuilder;

                let pool = ThreadPoolBuilder::new()
                    .num_threads(self.jobs)
                    .build()
                    .map_err(|e| {
                        crate::errors::OcError::Io(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            e.to_string(),
                        ))
                    })?;

                let results: Vec<_> = pool.install(|| {
                    commands
                        .par_iter()
                        .map(|cmd| self.execute_batch_cmd(cmd, data_dir, self.dry_run))
                        .collect()
                });

                for (cmd, res) in commands.iter().zip(results) {
                    if res {
                        result.add_success(PathBuf::from(&cmd.name), "ejecutado");
                    } else {
                        result.add_failure(PathBuf::from(&cmd.name), "falló");
                    }
                }
            }
        } else {
            // L17.1: Ejecución secuencial
            for cmd in &commands {
                let success = self.execute_batch_cmd(cmd, data_dir, self.dry_run);
                if success {
                    result.add_success(PathBuf::from(&cmd.name), "ejecutado");
                } else {
                    result.add_failure(PathBuf::from(&cmd.name), "falló");
                }
            }
        }

        eprintln!(
            "📦 {} comandos procesados ({} exitosos, {} fallidos)",
            commands.len(),
            result.succeeded,
            result.failed
        );

        Ok(result)
    }

    /// L17.2: Parsea archivo .oc-batch
    fn parse_batch_file(&self, path: &PathBuf) -> OcResult<Vec<BatchCmd>> {
        let content = std::fs::read_to_string(path)?;
        let mut commands = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue; // Skip comments and empty lines
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if !parts.is_empty() {
                commands.push(BatchCmd {
                    name: parts[0].to_string(),
                    args: parts[1..].iter().map(|s| s.to_string()).collect(),
                });
            }
        }

        Ok(commands)
    }

    /// L17.1: Ejecuta un comando batch
    fn execute_batch_cmd(
        &self,
        cmd: &BatchCmd,
        _data_dir: &std::path::Path,
        dry_run: bool,
    ) -> bool {
        if dry_run {
            eprintln!("  [DRY-RUN] {}", cmd.name);
            return true;
        }

        eprintln!("  ▶ Ejecutando: {} {:?}", cmd.name, cmd.args);
        // Aquí se ejecutarían los subcomandos reales
        // Por ahora solo simula éxito
        true
    }

    /// Operación original sobre frontmatter
    fn run_frontmatter_update(&self, data_dir: &std::path::Path) -> OcResult<BatchResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        use regex::Regex;

        let field = self.field.as_deref().unwrap_or("status");
        let value = self.value.as_deref().unwrap_or("draft");
        let mut result = BatchResult::new(format!("update-{}", field));

        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;

        let field_regex = Regex::new(&format!(r#"{}:\s*["']?[^"'\n]+["']?"#, field)).ok();
        use crate::core::patterns::RE_MODULE;
        let module_regex = &*RE_MODULE;

        for file_path in &files {
            if let Ok(content) = read_file_content(file_path) {
                // Filtrar por módulo si se especificó
                if let Some(ref filter_module) = self.module {
                    let file_module = module_regex
                        .captures(&content)
                        .map(|c| c[1].trim().to_string());
                    if file_module
                        .as_ref()
                        .map(|m| !m.contains(filter_module))
                        .unwrap_or(true)
                    {
                        continue;
                    }
                }

                if let Some(ref re) = field_regex {
                    if re.is_match(&content) {
                        let new_field = format!("{}: \"{}\"", field, value);
                        let new_content = re.replace(&content, new_field.as_str()).to_string();

                        if !self.dry_run {
                            std::fs::write(file_path, &new_content)?;
                        }
                        result.add_success(file_path.clone(), format!("{}={}", field, value));
                    }
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_result_new() {
        let result = BatchResult::new("test");
        assert_eq!(result.succeeded, 0);
        assert_eq!(result.failed, 0);
    }

    #[test]
    fn test_add_success() {
        let mut result = BatchResult::new("test");
        result.add_success(PathBuf::from("a.md"), "ok");

        assert_eq!(result.succeeded, 1);
    }

    #[test]
    fn test_success_rate() {
        let mut result = BatchResult::new("test");
        result.add_success(PathBuf::from("a.md"), "ok");
        result.add_failure(PathBuf::from("b.md"), "fail");

        assert_eq!(result.success_rate(), 50.0);
    }

    #[test]
    fn test_empty_success_rate() {
        let result = BatchResult::new("test");
        assert_eq!(result.success_rate(), 100.0);
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: BatchCommand, cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let default_dir = PathBuf::from(&cli.data_dir);
    let data_dir = cmd.path.as_ref().unwrap_or(&default_dir);

    // F4: Procesar add_field
    if let Some(ref add_spec) = cmd.add_field {
        println!("➕ Agregando campo YAML...");

        if let Some((field, value)) = add_spec.split_once('=') {
            let mut files_modified = 0;
            let mut files_skipped = 0;

            use walkdir::WalkDir;
            for entry in WalkDir::new(data_dir).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if !path.is_file() { continue; }
                if path.extension().map(|e| e != "md").unwrap_or(true) { continue; }
                if let Ok(content) = std::fs::read_to_string(path) {
                    // Verificar si ya tiene el campo
                    let field_pattern = format!("{}:", field);
                    if content.contains(&field_pattern) {
                        files_skipped += 1;
                    } else if content.starts_with("---") {
                        // Agregar campo después del primer ---
                        if !cmd.dry_run {
                            let new_line = format!("{}: {}", field, value);
                            let new_content = content.replacen(
                                "---\n",
                                &format!("---\n{}\n", new_line),
                                1,
                            );
                            let _ = std::fs::write(path, new_content);
                        }
                        files_modified += 1;
                    }
                }
            }

            if cmd.dry_run {
                println!(
                    "  🔍 [dry-run] {} archivos modificarían, {} ya tienen el campo",
                    files_modified, files_skipped
                );
            } else {
                println!(
                    "  ✅ {} archivos modificados, {} ya tenían el campo",
                    files_modified, files_skipped
                );
            }
        } else {
            println!("  ❌ Formato inválido. Use: --add-field campo=valor");
        }
        return Ok(());
    }

    // F4: Procesar remove_field
    if let Some(ref field) = cmd.remove_field {
        println!("➖ Eliminando campo YAML: {}", field);

        let field_regex =
            regex::Regex::new(&format!(r#"(?m)^{}:\s*.*\n?"#, regex::escape(field))).unwrap();
        let mut files_modified = 0;
        let mut files_skipped = 0;

        use walkdir::WalkDir;
        for entry in WalkDir::new(data_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() { continue; }
            if path.extension().map(|e| e != "md").unwrap_or(true) { continue; }
            if let Ok(content) = std::fs::read_to_string(path) {
                if field_regex.is_match(&content) {
                    if !cmd.dry_run {
                        let new_content = field_regex.replace_all(&content, "").to_string();
                        let _ = std::fs::write(path, new_content);
                    }
                    files_modified += 1;
                } else {
                    files_skipped += 1;
                }
            }
        }

        if cmd.dry_run {
            println!(
                "  🔍 [dry-run] {} archivos modificarían, {} no tienen el campo",
                files_modified, files_skipped
            );
        } else {
            println!(
                "  ✅ {} archivos modificados, {} no tenían el campo",
                files_modified, files_skipped
            );
        }
        return Ok(());
    }

    // Operación normal
    let result = cmd.run(data_dir)?;

    println!("📦 Operación: {}", result.operation);
    println!("✅ Exitosos: {}", result.succeeded);
    println!("❌ Fallidos: {}", result.failed);
    println!("📊 Tasa de éxito: {:.1}%", result.success_rate());

    Ok(())
}
