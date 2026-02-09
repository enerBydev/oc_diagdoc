//! Módulo de comandos CLI.

use crate::CliConfig;
#[cfg(feature = "cli")]
use clap::Subcommand;

// Comandos analíticos
pub mod deps;
pub mod search;
pub mod stats;
pub mod tree;
pub mod verify;

// Comandos de modificación
pub mod batch;
pub mod fix;  // RFC-07
pub mod links;
pub mod sync;

// Comandos de diagnóstico
pub mod audit;
pub mod coverage;
pub mod health;
pub mod lint;
pub mod module;
pub mod report;
pub mod trace;
pub mod watch;

// Comandos de generación
pub mod compress;
pub mod export;
pub mod gen;
pub mod template;

// Comandos de producción
pub mod archive;
pub mod ci;
pub mod diff;
pub mod init;
pub mod migrate;
pub mod restore;
pub mod snapshot;

// Comandos de sistema
pub mod help;
pub mod readme;
pub mod dashboard;  // ADD#1: TUI Dashboard

#[cfg(feature = "cli")]
#[derive(Subcommand, Debug)]
pub enum Command {
    // Analíticos
    Verify(verify::VerifyCommand),
    Stats(stats::StatsCommand),
    Search(search::SearchCommand),
    Deps(deps::DepsCommand),
    Tree(tree::TreeCommand),

    // Modificación
    Batch(batch::BatchCommand),
    Fix(fix::FixCommand),  // RFC-07
    Sync(sync::SyncCommand),
    Links(links::LinksCommand),

    // Diagnóstico
    Lint(lint::LintCommand),
    Health(health::HealthCommand),
    Coverage(coverage::CoverageCommand),
    Trace(trace::TraceCommand),
    Audit(audit::AuditCommand),
    Report(report::ReportCommand),
    Module(module::ModuleCommand),
    Watch(watch::WatchCommand),

    // Generación
    Gen(gen::GenCommand),
    Template(template::TemplateCommand),
    Export(export::ExportCommand),
    Compress(compress::CompressCommand),

    // Producción
    Init(init::InitCommand),
    Migrate(migrate::MigrateCommand),
    Diff(diff::DiffCommand),
    Snapshot(snapshot::SnapshotCommand),
    Restore(restore::RestoreCommand),
    Archive(archive::ArchiveCommand),
    Ci(ci::CiCommand),

    // Sistema
    Readme(readme::ReadmeCommand),
    Help(help::HelpCommand),
    Dashboard(dashboard::DashboardCommand),  // ADD#1: TUI Dashboard
}

#[cfg(feature = "cli")]
pub fn execute(cmd: Command, cli: &CliConfig) -> anyhow::Result<()> {
    match cmd {
        Command::Verify(args) => verify::run(args, cli),
        Command::Stats(args) => stats::run(args, cli),
        Command::Search(args) => search::run(args, cli),
        Command::Deps(args) => deps::run(args, cli),
        Command::Tree(args) => tree::run(args, cli),
        Command::Batch(args) => batch::run(args, cli),
        Command::Fix(args) => fix::run(args, cli),  // RFC-07
        Command::Sync(args) => sync::run(args, cli),
        Command::Links(args) => links::run(args, cli),
        Command::Lint(args) => lint::run(args, cli),
        Command::Health(args) => health::run(args, cli),
        Command::Coverage(args) => coverage::run(args, cli),
        Command::Trace(args) => trace::run(args, cli),
        Command::Audit(args) => audit::run(args, cli),
        Command::Report(args) => report::run(args, cli),
        Command::Module(args) => module::run(args, cli),
        Command::Watch(args) => watch::run(args, cli),
        Command::Gen(args) => gen::run(args, cli),
        Command::Template(args) => template::run(args, cli),
        Command::Export(args) => export::run(args, cli),
        Command::Compress(args) => compress::run(args, cli),
        Command::Init(args) => init::run(args, cli),
        Command::Migrate(args) => migrate::run(args, cli),
        Command::Diff(args) => diff::run(args, cli),
        Command::Snapshot(args) => snapshot::run(args, cli),
        Command::Restore(args) => restore::run(args, cli),
        Command::Archive(args) => archive::run(args, cli),
        Command::Ci(args) => ci::run(args, cli),
        Command::Readme(args) => readme::run(args, cli),
        Command::Help(args) => help::run(args, cli),
        Command::Dashboard(args) => dashboard::run(args, cli),  // ADD#1
    }
}
