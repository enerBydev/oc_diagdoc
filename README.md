# 🦀⚛️☢️ oc_diagdoc

> **Motor algorítmico nuclear para diagnóstico y gestión de documentación técnica**

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

```
╔═══════════════════════════════════════════════════════════════╗
║   ██████╗  ██████╗    ██████╗ ██╗ █████╗  ██████╗             ║
║  ██╔═══██╗██╔════╝    ██╔══██╗██║██╔══██╗██╔════╝             ║
║  ██║   ██║██║         ██║  ██║██║███████║██║  ███╗            ║
║  ██║   ██║██║         ██║  ██║██║██╔══██║██║   ██║            ║
║  ╚██████╔╝╚██████╗    ██████╔╝██║██║  ██║╚██████╔╝            ║
║   ╚═════╝  ╚═════╝    ╚═════╝ ╚═╝╚═╝  ╚═╝ ╚═════╝  DOC v3.0.1 ║
╚═══════════════════════════════════════════════════════════════╝
```

---

## 📋 Descripción

**oc_diagdoc** es un motor de diagnóstico documental de alto rendimiento escrito en Rust. Diseñado para proyectos de documentación técnica extensos, proporciona:

- 🔍 **Verificación integral** con 21 fases de análisis
- 📊 **Dashboard de estadísticas** en tiempo real
- 🌳 **Visualización jerárquica** de estructura documental
- 🔗 **Análisis de dependencias** y detección de ciclos
- 🩺 **Diagnóstico cuántico** con auto-reparación
- ⚡ **Alto rendimiento** - procesa miles de archivos en segundos
- 📁 **Escaneo recursivo** - detecta archivos en todos los subdirectorios (WalkDir)

---

## ✨ Features

| Feature | Descripción |
|---------|-------------|
| **21 Fases de Verificación** | Validación completa de YAML, links, estructura, contenido |
| **Parser YAML Robusto** | Extracción de frontmatter con validación de esquema |
| **Escaneo Recursivo** | Detecta archivos .md en toda la jerarquía de directorios |
| **Grafos de Dependencias** | Detección de ciclos, huérfanos y componentes aislados |
| **Heatmaps de Cobertura** | Visualización de cobertura por módulo |
| **Auto-healing Cuántico** | Sugerencias de reparación automática |
| **Exportación Multi-formato** | Markdown, HTML, JSON, LaTeX |
| **Watch Mode** | Monitoreo en tiempo real de cambios |
| **CI/CD Ready** | Exit codes semánticos y reportes JUnit |

---

## 🚀 Instalación

### Desde código fuente

```bash
# Clonar repositorio
git clone https://github.com/enerBydev/oc_diagdoc.git
cd oc_diagdoc

# Compilar release
cargo build --release

# Instalar globalmente
cargo install --path .
```

### Requisitos
- Rust 1.75+
- Cargo

---

## 🎯 Quick Start

```bash
# Verificar documentación completa
oc_diagdoc verify ./Datos

# Ver estadísticas del proyecto
oc_diagdoc stats

# Mostrar árbol jerárquico
oc_diagdoc tree --colored

# Buscar en documentación
oc_diagdoc search "término"

# Generar reporte de cobertura
oc_diagdoc coverage --min-words 300

# Exportar a HTML
oc_diagdoc export --format html --output ./export
```

---

## 📖 Comandos CLI

### Analíticos
| Comando | Descripción |
|---------|-------------|
| `verify` | Verificación integral (21 fases) |
| `stats` | Dashboard de estadísticas |
| `tree` | Árbol jerárquico visual |
| `search` | Búsqueda en contenido y metadata |
| `deps` | Análisis de dependencias |

### Diagnóstico
| Comando | Descripción |
|---------|-------------|
| `lint` | Validación de formato Markdown |
| `health` | Score de salud del proyecto |
| `coverage` | Cobertura de contenido (palabras) |
| `trace` | Trazabilidad documento→requisito |
| `audit` | Auditoría forense YAML |

### Modificación
| Comando | Descripción |
|---------|-------------|
| `sync` | Sincronizar metadatos y fechas |
| `batch` | Operaciones en lote |
| `gen` | Generación automática |
| `export` | Exportación multi-formato |

### Gestión
| Comando | Descripción |
|---------|-------------|
| `init` | Inicializar proyecto nuevo |
| `migrate` | Migración entre versiones |
| `snapshot` | Crear snapshot del estado |
| `restore` | Restaurar desde snapshot |
| `ci` | Integración CI/CD |

---

## ⚙️ Configuración

Crear archivo `.oc-diagdoc.yaml` en la raíz del proyecto:

```yaml
# .oc-diagdoc.yaml
project:
  name: "Mi Proyecto"
  data_dir: "./Datos"
  
validation:
  min_words: 300
  required_fields:
    - id
    - title
    - parent
    - breadcrumb
    - status
    
output:
  colors: true
  verbose: false
  format: "table"
```

---

## 💡 Ejemplos

### Verificación con filtro por módulo

```bash
oc_diagdoc verify ./Datos --module 3 --quick
```

### Exportar solo documentos activos

```bash
oc_diagdoc export --format json --status active
```

### Lint con auto-fix

```bash
oc_diagdoc lint --fix --backup
```

### CI/CD Pipeline

```bash
# En GitHub Actions, retorna exit code apropiado
oc_diagdoc verify ./Datos --ci --junit-output report.xml
```

---

## 📚 API (Biblioteca)

```rust
use oc_diagdoc_lib::{
    core::{OcConfig, load_project},
    commands::verify::VerifyCommand,
};

fn main() -> anyhow::Result<()> {
    // Cargar configuración
    let config = OcConfig::from_file(".oc-diagdoc.yaml")?;
    
    // Ejecutar verificación
    let cmd = VerifyCommand::default();
    let result = cmd.run(&config.data_dir)?;
    
    println!("Fases pasadas: {}/{}", 
        result.phases_passed(), 
        result.phases.len()
    );
    
    Ok(())
}
```

---

## 🤝 Contributing

¡Contribuciones bienvenidas! Ver [CONTRIBUTING.md](docs/CONTRIBUTING.md).

1. Fork el repositorio
2. Crear rama feature (`git checkout -b feature/nueva-feature`)
3. Commit cambios (`git commit -am 'Add nueva feature'`)
4. Push a la rama (`git push origin feature/nueva-feature`)
5. Crear Pull Request

---

## 📝 Changelog

Ver [CHANGELOG.md](CHANGELOG.md) para historial de versiones.

---

## 📄 Licencia

MIT License - © 2026 enerBydev

Ver [LICENSE](LICENSE) para más detalles.
