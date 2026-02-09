# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

- PDF export via pandoc integration
- WebSocket watch mode for live reload
- Prometheus metrics export

---

## [3.1.0] - 2026-02-09

### Added

- 🖥️ **Dashboard TUI** (ADD#1): Interfaz interactiva con ratatui/crossterm
  - Navegación con j/k/↑/↓
  - Filtros: all/errors/warnings/fixable
  - Gauge de salud del proyecto
  - Lista de issues con colores por severidad
  
- 📊 **Sistema de Severidad** (ADD#2): Tipos estructurados para issues
  - Enum Severity (Error, Warning, Info, Hint)
  - Struct Issue con metadata completa
  - Colores ANSI por nivel
  - 7 tests unitarios
  
- 📸 **Snapshots Mejorados** (ADD#3): Gestión de estados
  - Comando `snapshot save/list`
  - Soporte para comparación base

### Fixed

- 📅 **FIX#1**: Sincronización de fechas last_updated
  - Umbral de 24h para evitar falsos positivos
  - 20 archivos corregidos automáticamente
  - Flag `--dates` agregado a `fix`
  
- 🔐 **FIX#2**: Recálculo de content_hash
  - 4 archivos corregidos
  - Flag `--hashes` agregado a `fix`

### Dependencies

- Added: `ratatui 0.29`, `crossterm 0.28`

---

## [3.0.1] - 2026-02-01

### Fixed

- 🔴 **CRÍTICO**: Corregido bug de escaneo no recursivo en todos los comandos
  - Reemplazado `fs::read_dir` con `WalkDir` en 11 ubicaciones
  - Archivos afectados: verify.rs, stats.rs, batch.rs, sync.rs, deps.rs, report.rs, export.rs
  - El comando `verify` ahora detecta correctamente todos los archivos en subdirectorios

### Technical Details

- Bug root cause: `fs::read_dir` solo escaneaba el nivel raíz del directorio
- Solución: Uso de `WalkDir::new()` para escaneo recursivo completo
- Paridad Python-Rust restaurada (15 errores, 2373 warnings detectados)

---

## [3.0.0] - 2026-02-01

### Added

- 🦀 Complete rewrite in Rust for maximum performance
- ⚛️ Quantum module with Oracle predictions and auto-healing
- 📊 21-phase verification system
- 🌳 Hierarchical tree visualization with ANSI colors
- 🔗 Dependency graph with cycle detection
- 📈 Heatmap coverage visualization
- 🔄 Batch operations for bulk updates
- 💾 Snapshot/restore functionality
- 🔍 Full-text search in content and metadata
- 📤 Multi-format export (HTML, JSON, LaTeX)

### Changed

- Engine rewritten from Python to Rust
- Configuration format updated to YAML
- CLI arguments restructured with clap v4

### Performance

- 10x faster file scanning
- Parallel processing with rayon
- Incremental hash caching

---

## [2.0.0] - 2025-12-15

### Added

- Module-based organization
- YAML frontmatter validation
- Link extraction and validation
- Coverage calculation by word count
- Progress bars and colored output

### Changed

- Migrated to structured error handling
- Improved CLI with subcommands

---

## [1.0.0] - 2025-10-01

### Added

- Initial Python implementation
- Basic document verification
- Statistics generation
- Tree visualization
- Simple export to Markdown

---

## [0.1.0] - 2025-08-15

### Added

- Project scaffolding
- Basic file scanning
- YAML parsing prototype
