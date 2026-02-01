//! Comando search - Búsqueda en documentación.
//!
//! Búsqueda por contenido y metadata YAML.

use crate::errors::OcResult;
use clap::Parser;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// SEARCH RESULT
// ═══════════════════════════════════════════════════════════════════════════

/// Resultado de búsqueda.
#[derive(Debug, Clone)]
pub struct SearchMatch {
    /// Ruta del archivo.
    pub file_path: PathBuf,
    /// Número de línea.
    pub line_number: usize,
    /// Contenido de la línea.
    pub line_content: String,
    /// Posición del match en la línea.
    pub match_start: usize,
    /// Longitud del match.
    pub match_length: usize,
    /// Contexto (líneas antes/después).
    pub context_before: Vec<String>,
    pub context_after: Vec<String>,
}

impl SearchMatch {
    pub fn new(
        file_path: PathBuf,
        line_number: usize,
        line_content: String,
        match_start: usize,
        match_length: usize,
    ) -> Self {
        Self {
            file_path,
            line_number,
            line_content,
            match_start,
            match_length,
            context_before: Vec::new(),
            context_after: Vec::new(),
        }
    }

    /// Línea con highlight.
    pub fn highlighted_line(&self) -> String {
        let before = &self.line_content[..self.match_start];
        let matched = &self.line_content[self.match_start..self.match_start + self.match_length];
        let after = &self.line_content[self.match_start + self.match_length..];

        format!("{}\x1b[1;33m{}\x1b[0m{}", before, matched, after)
    }
}

/// Resultados de búsqueda.
#[derive(Debug, Clone)]
pub struct SearchResults {
    pub query: String,
    pub matches: Vec<SearchMatch>,
    pub files_searched: usize,
    pub total_lines_searched: usize,
}

impl SearchResults {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            matches: Vec::new(),
            files_searched: 0,
            total_lines_searched: 0,
        }
    }

    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    pub fn files_with_matches(&self) -> usize {
        let mut paths: Vec<_> = self.matches.iter().map(|m| &m.file_path).collect();
        paths.sort();
        paths.dedup();
        paths.len()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SEARCH COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de búsqueda.
#[derive(Parser, Debug, Clone)]
#[command(name = "search", about = "Búsqueda en documentación")]
pub struct SearchCommand {
    /// Patrón de búsqueda.
    pub pattern: String,

    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Usar regex.
    #[arg(short, long)]
    pub regex: bool,

    /// Búsqueda case-insensitive.
    #[arg(short, long)]
    pub ignore_case: bool,

    /// Buscar en YAML frontmatter.
    #[arg(long)]
    pub yaml: bool,

    /// Buscar solo en contenido.
    #[arg(long)]
    pub content_only: bool,

    /// Líneas de contexto.
    #[arg(short = 'C', long, default_value = "2")]
    pub context: usize,

    /// Máximo de resultados.
    #[arg(short, long)]
    pub max_results: Option<usize>,
}

impl SearchCommand {
    /// Ejecuta la búsqueda.
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<SearchResults> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};

        let mut results = SearchResults::new(&self.pattern);

        // Compilar regex si es necesario
        let _regex_pattern = if self.regex {
            if self.ignore_case {
                regex::Regex::new(&format!("(?i){}", self.pattern)).ok()
            } else {
                regex::Regex::new(&self.pattern).ok()
            }
        } else {
            None
        };

        // Escanear todos los archivos
        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;

        results.files_searched = files.len();

        for file_path in files {
            if let Ok(content) = read_file_content(&file_path) {
                results.total_lines_searched += content.lines().count();

                let matches = self.search_in_content(&content, &file_path);

                for m in matches {
                    results.matches.push(m);

                    // Limitar resultados si hay máximo
                    if let Some(max) = self.max_results {
                        if results.matches.len() >= max {
                            return Ok(results);
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// Busca en un contenido.
    pub fn search_in_content(&self, content: &str, file_path: &PathBuf) -> Vec<SearchMatch> {
        let mut matches = Vec::new();
        let pattern = if self.ignore_case {
            self.pattern.to_lowercase()
        } else {
            self.pattern.clone()
        };

        for (line_idx, line) in content.lines().enumerate() {
            let search_line = if self.ignore_case {
                line.to_lowercase()
            } else {
                line.to_string()
            };

            if let Some(pos) = search_line.find(&pattern) {
                matches.push(SearchMatch::new(
                    file_path.clone(),
                    line_idx + 1,
                    line.to_string(),
                    pos,
                    pattern.len(),
                ));

                if let Some(max) = self.max_results {
                    if matches.len() >= max {
                        break;
                    }
                }
            }
        }

        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_match_new() {
        let m = SearchMatch::new(PathBuf::from("test.md"), 1, "hello world".to_string(), 0, 5);

        assert_eq!(m.line_number, 1);
    }

    #[test]
    fn test_search_results() {
        let results = SearchResults::new("test");
        assert_eq!(results.match_count(), 0);
    }

    #[test]
    fn test_search_in_content() {
        let cmd = SearchCommand {
            pattern: "hello".to_string(),
            path: None,
            regex: false,
            ignore_case: false,
            yaml: false,
            content_only: false,
            context: 2,
            max_results: None,
        };

        let content = "line1\nhello world\nline3";
        let matches = cmd.search_in_content(content, &PathBuf::from("test.md"));

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].line_number, 2);
    }

    #[test]
    fn test_highlighted_line() {
        let m = SearchMatch::new(PathBuf::from("test.md"), 1, "hello world".to_string(), 0, 5);

        let highlighted = m.highlighted_line();
        assert!(highlighted.contains("\x1b[1;33m"));
    }
}

/// Función de ejecución para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: SearchCommand, cli: &crate::CliConfig) -> anyhow::Result<()> {
    // F6: Corregir path handling
    let default_dir = std::path::PathBuf::from(&cli.data_dir);
    let data_dir = cmd.path.as_ref().unwrap_or(&default_dir);
    let results = cmd.run(data_dir)?;

    if results.matches.is_empty() {
        println!("🔍 No se encontraron resultados para: {}", results.query);
    } else {
        println!(
            "🔍 {} resultados para '{}' en {} archivos ({} líneas buscadas)\n",
            results.match_count(),
            results.query,
            results.files_with_matches(),
            results.total_lines_searched
        );

        for m in &results.matches {
            println!(
                "{}:{}: {}",
                m.file_path.display(),
                m.line_number,
                m.highlighted_line()
            );
        }
    }

    Ok(())
}
