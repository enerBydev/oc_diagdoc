//! oc_diagdoc - Motor algorítmico nuclear para documentación OnlyCarNLD
//!
//! CLI ultra-potente en Rust puro para gestión de documentación.

use anyhow::Result;

#[cfg(feature = "cli")]
use clap::Parser;

use oc_diagdoc_lib::{commands, CliConfig};

#[cfg(feature = "cli")]
#[derive(Parser, Debug)]
#[command(
    name = "oc_diagdoc",
    version = "3.0.1",
    author = "enerbydev <dev@onlycar.mx>",
    about = "Motor algorítmico nuclear para documentación OnlyCarNLD",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<commands::Command>,

    /// Modo verbose
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Directorio de datos
    #[arg(long, global = true, default_value = "Datos")]
    pub data_dir: String,

    // F7: Flag global para generar README
    /// Generar documentación completa del CLI en formato Markdown
    #[arg(long)]
    pub readme: bool,
}

#[cfg(feature = "cli")]
impl Cli {
    pub fn to_config(&self) -> CliConfig {
        CliConfig {
            verbose: self.verbose,
            data_dir: self.data_dir.clone(),
        }
    }
}

/// F7: Genera documentación README completa del CLI
#[cfg(feature = "cli")]
fn generate_readme() {
    let readme = r#"# oc_diagdoc CLI v3.0.1

Motor algorítmico nuclear para documentación OnlyCarNLD.

## Instalación

```bash
cargo build --release --features cli
./target/release/oc_diagdoc --help
```

## Comandos Disponibles

### Verificación y Análisis
| Comando | Descripción |
|---------|-------------|
| `verify` | Verificación completa del proyecto (20 fases) |
| `lint` | Análisis de calidad y estilo |
| `audit` | Auditoría de metadata YAML |
| `stats` | Dashboard de estadísticas |

### Gestión de Dependencias
| Comando | Descripción |
|---------|-------------|
| `deps` | Análisis de dependencias entre documentos |
| `tree` | Visualización de estructura jerárquica |
| `search` | Búsqueda con regex y contexto |

### Operaciones en Lote
| Comando | Descripción |
|---------|-------------|
| `sync` | Sincronización de fechas y hashes |
| `batch` | Operaciones batch sobre frontmatter |
| `export` | Exportación a múltiples formatos |
| `watch` | Modo watch para desarrollo |

## Flags Globales

| Flag | Descripción |
|------|-------------|
| `-v, --verbose` | Modo verbose con debug |
| `--data-dir PATH` | Directorio de datos (default: Datos) |
| `--readme` | Generar esta documentación |

## Ejemplos de Uso

```bash
# Verificación rápida
oc_diagdoc verify --quick --path ./Datos

# Estadísticas por status
oc_diagdoc stats --by-status --by-type --path ./Datos

# Búsqueda con contexto
oc_diagdoc search "operador" --context 5 -p ./Datos

# Análisis de impacto
oc_diagdoc deps --impact "1.3.6" --path ./Datos

# Exportar documentación completa
oc_diagdoc export --single-file --output docs.md --path ./Datos

# Sincronización dry-run
oc_diagdoc sync --dry-run --fix-descendants --path ./Datos
```

## Flags por Comando

### verify
- `--quick, -Q` - Omitir fases lentas (V16, V17, V19)
- `--phase N` - Ejecutar solo fase específica
- `--json` - Salida en JSON
- `--quiet` - Sin output de progreso

### stats
- `--by-status` - Agrupar por status
- `--by-type` - Agrupar por tipo
- `--recent N` - Mostrar N archivos recientes
- `--size` - Incluir tamaño en bytes

### sync
- `--fix-descendants` - Propagar a hijos
- `--fix-total` - Recalcular totales
- `--tolerance N` - Tolerancia en segundos (default: 5)

### batch
- `--add-field campo=valor` - Agregar campo YAML
- `--remove-field campo` - Eliminar campo YAML
- `--dry-run` - Simular cambios

### deps
- `--direction up|down|both` - Dirección del análisis
- `--impact DOC_ID` - Análisis de impacto
- `--orphans` - Listar documentos huérfanos

### export
- `--template PATH` - Usar plantilla personalizada
- `--single-file` - Concatenar todo en un archivo
- `--zip` - Exportar como ZIP

---

**Generado por oc_diagdoc v3.0.1**
"#;

    println!("{}", readme);
}

#[cfg(feature = "cli")]
fn main() -> Result<()> {
    let cli = Cli::parse();

    // F7: Generar README si se pide
    if cli.readme {
        generate_readme();
        return Ok(());
    }

    // Inicializar logging
    if cli.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }

    // Obtener config ANTES de mover cli.command
    let config = cli.to_config();

    // Verificar que hay comando
    let command = match cli.command {
        Some(cmd) => cmd,
        None => {
            eprintln!(
                "Error: No se especificó un comando. Use --help para ver comandos disponibles."
            );
            std::process::exit(1);
        }
    };

    // Ejecutar comando
    commands::execute(command, &config)?;

    Ok(())
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("oc_diagdoc requiere feature 'cli'. Compila con: cargo build --features cli");
    std::process::exit(1);
}
