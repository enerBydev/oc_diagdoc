# 📊 Sistema de Severidad

Tipos estructurados para clasificar issues de verificación.

## Descripción

El módulo `severity` define los niveles de severidad para categorizar issues detectados durante la verificación.

## Niveles

| Nivel | Valor | Icon | Color |
|-------|-------|------|-------|
| Error | 3 | ❌ | Rojo |
| Warning | 2 | ⚠️ | Amarillo |
| Info | 1 | ℹ️ | Azul |
| Hint | 0 | 💡 | Gris |

## Uso en Código

```rust
use oc_diagdoc_lib::types::{Severity, Issue};

// Crear issue con severidad
let issue = Issue::new(
    "V008".to_string(),
    "1.2.3 doc.md".to_string(),
    "Fecha desincronizada".to_string(),
    Severity::Error,
    true, // fixable
);

// Acceder a propiedades
println!("{} {}", issue.severity.icon(), issue.severity);
println!("Color: {:?}", issue.severity.color());
```

## Struct Issue

```rust
pub struct Issue {
    pub id: String,
    pub file: String,
    pub message: String,
    pub severity: Severity,
    pub fixable: bool,
}
```

## Métodos de Severity

| Método | Descripción |
|--------|-------------|
| `icon()` | Emoji correspondiente |
| `color()` | Color ANSI para terminal |
| `value()` | Valor numérico (0-3) |

## Tests

El módulo incluye 7 tests unitarios:

- `test_severity_ordering`
- `test_severity_display`
- `test_severity_icon`
- `test_severity_color`
- `test_issue_creation`
- `test_issue_display`
- `test_severity_from_str`

## Ubicación

`src/types/severity.rs` (215 LOC)

## Desde v3.1.0
