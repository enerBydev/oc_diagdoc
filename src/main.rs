//! oc_diagdoc - Motor algorítmico nuclear para documentación OnlyCarNLD
//!
//! CLI ultra-potente en Rust puro para gestión de documentación.

use anyhow::Result;

#[cfg(feature = "cli")]
use clap::Parser;

use oc_diagdoc_lib::{CliConfig, commands};

#[cfg(feature = "cli")]
#[derive(Parser, Debug)]
#[command(
    name = "oc_diagdoc",
    version = "3.0.0",
    author = "enerbydev <dev@onlycar.mx>",
    about = "Motor algorítmico nuclear para documentación OnlyCarNLD",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: commands::Command,
    
    /// Modo verbose
    #[arg(short, long, global = true)]
    pub verbose: bool,
    
    /// Directorio de datos
    #[arg(long, global = true, default_value = "Datos")]
    pub data_dir: String,
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

#[cfg(feature = "cli")]
fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Inicializar logging
    if cli.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }
    
    // Ejecutar comando
    let config = cli.to_config();
    commands::execute(cli.command, &config)?;
    
    Ok(())
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("oc_diagdoc requiere feature 'cli'. Compila con: cargo build --features cli");
    std::process::exit(1);
}
