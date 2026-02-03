//! Pipeline de procesamiento de documentos.
//!
//! Proporciona un sistema de pipeline composable para procesar documentos.

use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// PIPELINE TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Etapa del pipeline.
pub trait PipelineStage: Send + Sync {
    /// Nombre de la etapa.
    fn name(&self) -> &str;

    /// Procesa un documento.
    fn process(&self, ctx: &mut PipelineContext) -> PipelineResult;
}

/// Contexto del pipeline.
#[derive(Debug, Clone)]
pub struct PipelineContext {
    pub path: PathBuf,
    pub content: String,
    pub metadata: std::collections::HashMap<String, String>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl PipelineContext {
    pub fn new(path: PathBuf, content: String) -> Self {
        Self {
            path,
            content,
            metadata: std::collections::HashMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, msg: &str) {
        self.errors.push(msg.to_string());
    }

    pub fn add_warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

/// Resultado del pipeline.
#[derive(Debug, Clone)]
pub enum PipelineResult {
    Continue,
    Skip,
    Abort(String),
}

/// Pipeline de procesamiento.
pub struct Pipeline {
    stages: Vec<Box<dyn PipelineStage>>,
    #[allow(dead_code)]
    name: String,
}

impl Pipeline {
    pub fn new(name: &str) -> Self {
        Self {
            stages: Vec::new(),
            name: name.to_string(),
        }
    }

    pub fn add_stage<S: PipelineStage + 'static>(&mut self, stage: S) {
        self.stages.push(Box::new(stage));
    }

    pub fn run(&self, ctx: &mut PipelineContext) -> PipelineResult {
        for stage in &self.stages {
            match stage.process(ctx) {
                PipelineResult::Continue => continue,
                PipelineResult::Skip => return PipelineResult::Skip,
                PipelineResult::Abort(msg) => return PipelineResult::Abort(msg),
            }
        }
        PipelineResult::Continue
    }

    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// BUILT-IN STAGES
// ═══════════════════════════════════════════════════════════════════════════

/// Etapa de validación de frontmatter.
pub struct FrontmatterValidationStage;

impl PipelineStage for FrontmatterValidationStage {
    fn name(&self) -> &str {
        "frontmatter_validation"
    }

    fn process(&self, ctx: &mut PipelineContext) -> PipelineResult {
        if !ctx.content.starts_with("---") {
            ctx.add_warning("Missing frontmatter");
        }
        PipelineResult::Continue
    }
}

/// Etapa de conteo de palabras.
pub struct WordCountStage;

impl PipelineStage for WordCountStage {
    fn name(&self) -> &str {
        "word_count"
    }

    fn process(&self, ctx: &mut PipelineContext) -> PipelineResult {
        let words = ctx.content.split_whitespace().count();
        ctx.metadata
            .insert("word_count".to_string(), words.to_string());
        PipelineResult::Continue
    }
}

/// Etapa de detección de enlaces rotos.
pub struct LinkValidationStage;

impl PipelineStage for LinkValidationStage {
    fn name(&self) -> &str {
        "link_validation"
    }

    fn process(&self, ctx: &mut PipelineContext) -> PipelineResult {
        // Usar patrón precompilado centralizado
        use crate::core::patterns::RE_MD_LINK;
        let link_pattern = &*RE_MD_LINK;

        // Clonar content para evitar borrow conflict
        let content = ctx.content.clone();
        let parent = ctx.path.parent().map(|p| p.to_path_buf());

        let mut broken_links = Vec::new();
        for cap in link_pattern.captures_iter(&content) {
            let target = &cap[2];
            if target.starts_with("http") {
                continue;
            }
            if let Some(ref parent_path) = parent {
                let link_path = parent_path.join(target);
                if !link_path.exists() {
                    broken_links.push(target.to_string());
                }
            }
        }

        for link in broken_links {
            ctx.add_warning(&format!("Broken link: {}", link));
        }
        PipelineResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_context_new() {
        let ctx = PipelineContext::new(PathBuf::from("test.md"), "content".to_string());
        assert!(!ctx.has_errors());
    }

    #[test]
    fn test_pipeline_context_errors() {
        let mut ctx = PipelineContext::new(PathBuf::from("test.md"), "content".to_string());
        ctx.add_error("Error 1");
        assert!(ctx.has_errors());
    }

    #[test]
    fn test_pipeline_new() {
        let pipeline = Pipeline::new("test");
        assert_eq!(pipeline.stage_count(), 0);
    }

    #[test]
    fn test_word_count_stage() {
        let stage = WordCountStage;
        let mut ctx = PipelineContext::new(PathBuf::from("t.md"), "hello world test".to_string());
        stage.process(&mut ctx);
        assert_eq!(ctx.metadata.get("word_count"), Some(&"3".to_string()));
    }
}
