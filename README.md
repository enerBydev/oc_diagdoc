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
║   ╚═════╝  ╚═════╝    ╚═════╝ ╚═╝╚═╝  ╚═╝ ╚═════╝  DOC v3.1.0 ║
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
| **Modo Quiet** | Flag global `-q/--quiet` para suprimir output no esencial |
| **Progress Bars** | Barras de progreso interactivas con indicatif |
| **Caché Sled** | Caché persistente para verificaciones repetidas |
| **Búsqueda Fuzzy** | Búsqueda aproximada tolerante a errores tipográficos |

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

## 📖 Comandos CLI (30)

### Analíticos

| Comando | Descripción |
|---------|-------------|
| `verify` | Verificación integral (21 fases) |
| `stats` | Dashboard de estadísticas |
| `tree` | Árbol jerárquico visual |
| `search` | Búsqueda en contenido y metadata |
| `deps` | Análisis de dependencias |
| `links` | Análisis de enlaces internos/externos |
| `dashboard` | Interfaz TUI interactiva para visualizar issues |

### Diagnóstico

| Comando | Descripción |
|---------|-------------|
| `lint` | Validación de formato Markdown |
| `health` | Score de salud del proyecto |
| `coverage` | Cobertura de contenido (palabras) |
| `trace` | Trazabilidad documento→requisito |
| `audit` | Auditoría forense YAML |
| `report` | Generación de reportes |
| `diff` | Comparar estados del proyecto |

### Modificación

| Comando | Descripción |
|---------|-------------|
| `fix` | Corregir anomalías estructurales (fechas, hashes, tablas) |
| `sync` | Sincronizar metadatos y fechas |
| `batch` | Operaciones en lote |
| `gen` | Generación automática de documentos |
| `export` | Exportación multi-formato |
| `compress` | Compilar documentación en archivo único |

### Gestión

| Comando | Descripción |
|---------|-------------|
| `init` | Inicializar proyecto nuevo |
| `migrate` | Migración entre versiones |
| `snapshot` | Crear snapshot del estado |
| `restore` | Restaurar desde snapshot |
| `archive` | Archivar documentos obsoletos |
| `ci` | Integración CI/CD |

### Utilidades

| Comando | Descripción |
|---------|-------------|
| `module` | Operaciones sobre módulos |
| `watch` | Observar cambios en tiempo real |
| `template` | Gestión de plantillas |
| `readme` | Generar README automático |
| `help` | Ayuda extendida |

---

## 🚩 Flags Globales

Estos flags están disponibles para todos los comandos:

| Flag | Descripción |
|------|-------------|
| `-q, --quiet` | Modo silencioso, suprime output no esencial |
| `-v, --verbose` | Modo detallado con información extra |
| `--data-dir <PATH>` | Directorio de datos (override del config) |
| `--config <FILE>` | Archivo de configuración personalizado |

---

## 🔧 Flags Avanzados por Comando

### `verify`

| Flag | Descripción |
|------|-------------|
| `--progress` | Mostrar barra de progreso interactiva |
| `--cache` | Usar caché sled para acelerar verificaciones |
| `--quick` | Verificación rápida (solo fases críticas) |
| `--strict` | Fallar en cualquier warning |

### `batch`

| Flag | Descripción |
|------|-------------|
| `--progress` | Mostrar barra de progreso |
| `--filter <PATTERN>` | Filtrar archivos por patrón glob |
| `--dry-run` | Simular sin modificar archivos |

### `search`

| Flag | Descripción |
|------|-------------|
| `--fuzzy` | Búsqueda aproximada tolerante a errores |
| `--module <ID>` | Filtrar por módulo específico |
| `--field <NAME>` | Buscar solo en campo YAML específico |
| `--format <FMT>` | Formato de salida (text/json/table) |

### `stats`

| Flag | Descripción |
|------|-------------|
| `--cache` | Usar caché para estadísticas |
| `--heatmap` | Generar heatmap de cobertura |

### `tree`

| Flag | Descripción |
|------|-------------|
| `--root <ID>` | Nodo raíz para visualización (matching flexible) |
| `--show-status` | Mostrar estado de cada documento |
| `--format <FMT>` | Formato de salida (ascii/json/md) |
| `--output <FILE>` | Guardar resultado en archivo |
| `--depth <N>` | Profundidad máxima del árbol |

### `lint`

| Flag | Descripción |
|------|-------------|
| `--show-fixes` | Mostrar sugerencias de corrección detalladas |
| `--fix` | Aplicar correcciones automáticamente |

### `fix`

| Flag | Descripción |
|------|-------------|
| `--dates` | Sincronizar campo last_updated con fecha del filesystem |
| `--hashes` | Recalcular campo content_hash basado en contenido actual |
| `--tables` | Corregir tablas de contenido (columna Nietos) |
| `--dry-run` | Modo dry-run: mostrar cambios sin aplicar |
| `-v, --verbose` | Mostrar detalles de cada corrección |

### `dashboard`

| Flag | Descripción |
|------|-------------|
| `-f, --filter` | Filtro inicial: all, errors, warnings, fixable |
| `--quick` | Ejecutar verificación rápida |
| `-p, --path` | Ruta al directorio de datos |

### `compress`

| Flag | Descripción |
|------|-------------|
| `--preview` | Mostrar output sin escribir archivo |
| `--pdf` | Generar versión PDF (requiere pandoc) |

### `sync`

| Flag | Descripción |
|------|-------------|
| `--force` | Forzar actualización de todas las fechas |
| Estadísticas extendidas: `hashes_initialized`, `hashes_updated` |

### `deps`

| Flag | Descripción |
|------|-------------|
| Reporte detallado de huérfanos: `reason`, `invalid_parent` |

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

### Verificación con progreso y caché (v3.1.0)

```bash
oc_diagdoc verify ./Datos --progress --cache
```

### Búsqueda fuzzy tolerante a errores (v3.1.0)

```bash
oc_diagdoc search "configracion" --fuzzy
```

### Árbol jerárquico con root flexible (v3.1.0)

```bash
oc_diagdoc tree --root 1.1 --show-status --format json --output tree.json
```

### Preview de compresión sin escribir (v3.1.0)

```bash
oc_diagdoc compress --preview --format md
```

### Lint con sugerencias de corrección (v3.1.0)

```bash
oc_diagdoc lint --show-fixes
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
