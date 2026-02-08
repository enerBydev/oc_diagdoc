//! RFC-03: Lint Documentation - Documentación exhaustiva de reglas de lint
//!
//! Provee documentación detallada para cada regla de lint (L001-L010).

use std::collections::HashMap;

/// Documentación de una regla de lint.
#[derive(Debug, Clone)]
pub struct LintRuleDoc {
    pub code: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub impact: &'static str,
    pub example_bad: &'static str,
    pub example_good: &'static str,
    pub auto_fixable: bool,
    pub suggestion: &'static str,
}

/// Obtiene documentación de todas las reglas.
pub fn get_all_rules() -> HashMap<&'static str, LintRuleDoc> {
    let mut rules = HashMap::new();
    
    rules.insert("L001", LintRuleDoc {
        code: "L001",
        name: "Frontmatter",
        description: "El archivo debe tener frontmatter YAML al inicio (delimitado por ---).",
        impact: "⚠️ Medio - Los archivos sin frontmatter no pueden ser procesados correctamente.",
        example_bad: "# Mi Documento\n\nContenido...",
        example_good: "---\nid: mi_doc\ntitle: Mi Documento\n---\n\n# Mi Documento",
        auto_fixable: false,
        suggestion: "Agregar frontmatter YAML con campos id, title, parent, type, status.",
    });
    
    rules.insert("L002", LintRuleDoc {
        code: "L002",
        name: "Header Hierarchy",
        description: "Los headers deben seguir jerarquía correcta (no saltar niveles).",
        impact: "⚠️ Medio - Afecta la estructura semántica del documento.",
        example_bad: "# Título\n\n### Subtema (salta H2)",
        example_good: "# Título\n\n## Sección\n\n### Subtema",
        auto_fixable: false,
        suggestion: "Revisar que los headers desciendan gradualmente: H1 → H2 → H3.",
    });
    
    rules.insert("L003", LintRuleDoc {
        code: "L003",
        name: "Trailing Whitespace",
        description: "Las líneas no deben terminar con espacios en blanco.",
        impact: "ℹ️ Bajo - Cosmético, no afecta funcionalidad.",
        example_bad: "Esta línea tiene espacios al final   ",
        example_good: "Esta línea está limpia",
        auto_fixable: true,
        suggestion: "Ejecutar: oc_diagdoc lint --fix",
    });
    
    rules.insert("L004", LintRuleDoc {
        code: "L004",
        name: "Final Newline",
        description: "Los archivos deben terminar con una línea vacía (newline final).",
        impact: "ℹ️ Bajo - Convención de archivos de texto.",
        example_bad: "Última línea sin newline<EOF>",
        example_good: "Última línea\n<EOF>",
        auto_fixable: true,
        suggestion: "Ejecutar: oc_diagdoc lint --fix",
    });
    
    rules.insert("L005", LintRuleDoc {
        code: "L005",
        name: "Line Length",
        description: "Las líneas no deben exceder 300 caracteres.",
        impact: "⚠️ Medio - Afecta legibilidad en editores.",
        example_bad: "[línea muy larga de más de 300 caracteres...]",
        example_good: "Línea de longitud razonable.",
        auto_fixable: false,
        suggestion: "Dividir líneas largas usando saltos de línea.",
    });
    
    rules.insert("L006", LintRuleDoc {
        code: "L006",
        name: "Code Block Language",
        description: "Los bloques de código deben especificar el lenguaje de programación.",
        impact: "ℹ️ Bajo - Cosmético, mejora el resaltado de sintaxis.",
        example_bad: "```\nconst x = 1;\n```",
        example_good: "```javascript\nconst x = 1;\n```",
        auto_fixable: false,
        suggestion: "Agregar lenguaje: markdown, javascript, rust, python, bash, sql, json, yaml.",
    });
    
    rules.insert("L007", LintRuleDoc {
        code: "L007",
        name: "Duplicate Headers",
        description: "Los headers no deben repetirse en el mismo documento.",
        impact: "⚠️ Medio - Dificulta navegación y referencias.",
        example_bad: "## Introducción\n...\n## Introducción",
        example_good: "## Introducción\n...\n## Contexto Adicional",
        auto_fixable: false,
        suggestion: "Renombrar headers duplicados para que sean únicos.",
    });
    
    rules.insert("L008", LintRuleDoc {
        code: "L008",
        name: "Required Fields",
        description: "El frontmatter debe contener campos obligatorios: id, title.",
        impact: "❌ Alto - Documentos sin identificador no pueden procesarse.",
        example_bad: "---\ntitle: Solo título\n---",
        example_good: "---\nid: mi_doc\ntitle: Mi Documento\n---",
        auto_fixable: false,
        suggestion: "Agregar campos faltantes: id, title, parent, type, status.",
    });
    
    rules.insert("L009", LintRuleDoc {
        code: "L009",
        name: "Table Header",
        description: "Las tablas deben tener fila de encabezado con separador.",
        impact: "⚠️ Medio - Tablas sin header no se renderizan correctamente.",
        example_bad: "| dato1 | dato2 |",
        example_good: "| Col1 | Col2 |\n|------|------|\n| dato1 | dato2 |",
        auto_fixable: false,
        suggestion: "Agregar fila de encabezado y separador |---|.",
    });
    
    rules.insert("L010", LintRuleDoc {
        code: "L010",
        name: "Image Alt Text",
        description: "Las imágenes deben tener texto alternativo (alt text).",
        impact: "⚠️ Medio - Afecta accesibilidad y SEO.",
        example_bad: "![](imagen.png)",
        example_good: "![Descripción de la imagen](imagen.png)",
        auto_fixable: false,
        suggestion: "Agregar descripción dentro de los corchetes: ![descripción](url).",
    });
    
    rules.insert("L011", LintRuleDoc {
        code: "L011",
        name: "Table Double Separator",
        description: "Las tablas solo deben tener UN separador |---| después del header, no después de cada fila.",
        impact: "❌ Alto - Tablas corruptas no se renderizan correctamente.",
        example_bad: "| Col1 | Col2 |\\n|---|---|\\n| dato1 | dato2 |\\n|---|---|",
        example_good: "| Col1 | Col2 |\\n|---|---|\\n| dato1 | dato2 |\\n| dato3 | dato4 |",
        auto_fixable: true,
        suggestion: "Ejecutar: oc_diagdoc lint --fix --rule L011",
    });
    
    rules.insert("L012", LintRuleDoc {
        code: "L012",
        name: "Unescaped Pipe in Table Wikilink",
        description: "Los wikilinks dentro de tablas deben escapar el pipe: [[X\\|Y]] no [[X|Y]].",
        impact: "❌ Alto - El pipe sin escapar rompe la estructura de columnas de la tabla.",
        example_bad: "| [[1.1. identidad|1.1]] | Detalle |",
        example_good: "| [[1.1. identidad\\|1.1]] | Detalle |",
        auto_fixable: true,
        suggestion: "Ejecutar: oc_diagdoc lint --fix --rule L012",
    });
    
    rules.insert("L013", LintRuleDoc {
        code: "L013",
        name: "Nietos Count Mismatch",
        description: "La columna Nietos debe coincidir con descendants_count del archivo enlazado.",
        impact: "⚠️ Medio - Información de jerarquía incorrecta en tablas de navegación.",
        example_bad: "| [[1.1. identidad\\|1.1]] | ... | 0 |",
        example_good: "| [[1.1. identidad\\|1.1]] | ... | 23 |",
        auto_fixable: true,
        suggestion: "Ejecutar: oc_diagdoc lint --fix --rule L013",
    });
    
    rules.insert("L014", LintRuleDoc {
        code: "L014",
        name: "Wikilink Absolute Path",
        description: "Los wikilinks no deben usar paths absolutos con prefijo de proyecto.",
        impact: "ℹ️ Bajo - Afecta portabilidad y legibilidad.",
        example_bad: "[[Proyecto OnlyCarNLD/Datos/1.1. identidad]]",
        example_good: "[[1.1. identidad]]",
        auto_fixable: false,
        suggestion: "Revisar manualmente y usar paths relativos.",
    });
    
    rules
}


/// Obtiene documentación de una regla específica.
pub fn get_rule_doc(code: &str) -> Option<LintRuleDoc> {
    get_all_rules().remove(code)
}

/// Imprime explicación detallada de una regla.
pub fn print_rule_explanation(code: &str) {
    if let Some(doc) = get_rule_doc(code) {
        println!();
        println!("📘 REGLA {}: {}", doc.code, doc.name);
        println!("═══════════════════════════════════════════════════════════════");
        println!();
        println!("📋 DESCRIPCIÓN:");
        println!("   {}", doc.description);
        println!();
        println!("❌ INCORRECTO:");
        for line in doc.example_bad.lines() {
            println!("   {}", line);
        }
        println!();
        println!("✅ CORRECTO:");
        for line in doc.example_good.lines() {
            println!("   {}", line);
        }
        println!();
        println!("🔧 IMPACTO: {}", doc.impact);
        println!("📊 AUTO-FIX: {}", if doc.auto_fixable { "Disponible" } else { "No disponible" });
        println!();
        println!("💡 SUGERENCIA:");
        println!("   {}", doc.suggestion);
        println!();
    } else {
        eprintln!("❌ Regla '{}' no encontrada.", code);
        eprintln!("   Reglas válidas: L001-L014");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_all_rules() {
        let rules = get_all_rules();
        assert_eq!(rules.len(), 14);
        assert!(rules.contains_key("L006"));
        assert!(rules.contains_key("L011"));
        assert!(rules.contains_key("L012"));
        assert!(rules.contains_key("L013"));
        assert!(rules.contains_key("L014"));
    }

    
    #[test]
    fn test_get_rule_doc() {
        let doc = get_rule_doc("L006");
        assert!(doc.is_some());
        assert_eq!(doc.unwrap().name, "Code Block Language");
    }
}
