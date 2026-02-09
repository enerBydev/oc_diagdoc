//! Dashboard TUI - Interfaz interactiva para oc_diagdoc
//!
//! ADD#1: Dashboard con ratatui para visualización de issues

use crate::commands::verify::{VerificationResult, VerificationPhase};
use crate::errors::OcResult;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph, Tabs},
    Frame, Terminal,
};
use std::io;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// ISSUE Y SEVERIDAD (ADD#2)
// ═══════════════════════════════════════════════════════════════════════════

/// Nivel de severidad de un issue
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Error = 3,
    Warning = 2,
    Info = 1,
    Hint = 0,
}

impl Severity {
    pub fn icon(&self) -> &'static str {
        match self {
            Severity::Error => "❌",
            Severity::Warning => "⚠️",
            Severity::Info => "ℹ️",
            Severity::Hint => "💡",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Severity::Error => Color::Red,
            Severity::Warning => Color::Yellow,
            Severity::Info => Color::Blue,
            Severity::Hint => Color::Gray,
        }
    }
}

/// Issue individual con severidad
#[derive(Debug, Clone)]
pub struct Issue {
    pub id: String,
    pub message: String,
    pub severity: Severity,
    pub fixable: bool,
    pub phase: u8,
    pub file: Option<String>,
}

impl Issue {
    pub fn from_phase(phase: &VerificationPhase) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        for error in &phase.errors {
            issues.push(Issue {
                id: format!("E{:02}", phase.id),
                message: error.clone(),
                severity: Severity::Error,
                fixable: false,
                phase: phase.id,
                file: None,
            });
        }
        
        for warning in &phase.warnings {
            issues.push(Issue {
                id: format!("W{:02}", phase.id),
                message: warning.clone(),
                severity: Severity::Warning,
                fixable: true,
                phase: phase.id,
                file: None,
            });
        }
        
        issues
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DASHBOARD APP STATE
// ═══════════════════════════════════════════════════════════════════════════

/// Estado de la aplicación Dashboard
pub struct DashboardApp {
    pub issues: Vec<Issue>,
    pub list_state: ListState,
    pub filter: FilterMode,
    pub should_quit: bool,
    pub total_errors: usize,
    pub total_warnings: usize,
    pub phases_passed: usize,
    pub phases_total: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterMode {
    All,
    Errors,
    Warnings,
    Fixable,
}

impl DashboardApp {
    pub fn new(result: &VerificationResult) -> Self {
        let mut issues = Vec::new();
        
        for phase in &result.phases {
            issues.extend(Issue::from_phase(phase));
        }
        
        let mut list_state = ListState::default();
        if !issues.is_empty() {
            list_state.select(Some(0));
        }
        
        Self {
            issues,
            list_state,
            filter: FilterMode::All,
            should_quit: false,
            total_errors: result.total_errors,
            total_warnings: result.total_warnings,
            phases_passed: result.phases.iter().filter(|p| p.passed).count(),
            phases_total: result.phases.len(),
        }
    }

    pub fn filtered_issues(&self) -> Vec<Issue> {
        self.issues.iter().filter(|i| match self.filter {
            FilterMode::All => true,
            FilterMode::Errors => i.severity == Severity::Error,
            FilterMode::Warnings => i.severity == Severity::Warning,
            FilterMode::Fixable => i.fixable,
        }).cloned().collect()
    }

    pub fn filtered_count(&self) -> usize {
        self.filtered_issues().len()
    }

    pub fn next(&mut self) {
        let count = self.filtered_count();
        if count == 0 {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => (i + 1) % count,
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let count = self.filtered_count();
        if count == 0 {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    count - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn health_score(&self) -> f64 {
        if self.phases_total == 0 {
            return 100.0;
        }
        (self.phases_passed as f64 / self.phases_total as f64) * 100.0
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DASHBOARD COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando dashboard - Interfaz TUI interactiva
#[derive(Parser, Debug, Clone)]
#[command(name = "dashboard", about = "Interfaz TUI interactiva para visualizar issues")]
pub struct DashboardCommand {
    /// Ruta al directorio de datos
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Filtro inicial (all, errors, warnings, fixable)
    #[arg(short, long, default_value = "all")]
    pub filter: String,

    /// Ejecutar verificación rápida
    #[arg(long)]
    pub quick: bool,
}

// ═══════════════════════════════════════════════════════════════════════════
// UI RENDERING
// ═══════════════════════════════════════════════════════════════════════════

fn ui(frame: &mut Frame, app: &mut DashboardApp) {
    // Layout principal: Header, Content, Footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(5),  // Summary
            Constraint::Min(10),    // Issues list
            Constraint::Length(3),  // Footer
        ])
        .split(frame.area());

    // Header con tabs de filtro
    render_header(frame, chunks[0], app);
    
    // Summary con métricas
    render_summary(frame, chunks[1], app);
    
    // Lista de issues
    render_issues(frame, chunks[2], app);
    
    // Footer con comandos
    render_footer(frame, chunks[3]);
}

fn render_header(frame: &mut Frame, area: Rect, app: &DashboardApp) {
    let titles = vec!["[A]ll", "[E]rrors", "[W]arnings", "[F]ixable"];
    let selected = match app.filter {
        FilterMode::All => 0,
        FilterMode::Errors => 1,
        FilterMode::Warnings => 2,
        FilterMode::Fixable => 3,
    };
    
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" 📊 oc_diagdoc Dashboard "))
        .select(selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    
    frame.render_widget(tabs, area);
}

fn render_summary(frame: &mut Frame, area: Rect, app: &DashboardApp) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    // Health Gauge
    let health = app.health_score();
    let health_color = if health >= 80.0 {
        Color::Green
    } else if health >= 50.0 {
        Color::Yellow
    } else {
        Color::Red
    };
    
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Health "))
        .gauge_style(Style::default().fg(health_color))
        .percent(health as u16)
        .label(format!("{:.0}%", health));
    frame.render_widget(gauge, chunks[0]);

    // Phases passed
    let phases_text = format!("{}/{}", app.phases_passed, app.phases_total);
    let phases = Paragraph::new(phases_text)
        .block(Block::default().borders(Borders::ALL).title(" Phases "))
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(phases, chunks[1]);

    // Errors
    let errors_text = format!("❌ {}", app.total_errors);
    let errors = Paragraph::new(errors_text)
        .block(Block::default().borders(Borders::ALL).title(" Errors "))
        .style(Style::default().fg(Color::Red));
    frame.render_widget(errors, chunks[2]);

    // Warnings
    let warnings_text = format!("⚠️ {}", app.total_warnings);
    let warnings = Paragraph::new(warnings_text)
        .block(Block::default().borders(Borders::ALL).title(" Warnings "))
        .style(Style::default().fg(Color::Yellow));
    frame.render_widget(warnings, chunks[3]);
}

fn render_issues(frame: &mut Frame, area: Rect, app: &mut DashboardApp) {
    let filtered = app.filtered_issues();
    
    let items: Vec<ListItem> = filtered
        .iter()
        .map(|issue| {
            let style = Style::default().fg(issue.severity.color());
            let fixable_marker = if issue.fixable { " 🔧" } else { "" };
            let content = Line::from(vec![
                Span::styled(
                    format!("{} ", issue.severity.icon()),
                    style,
                ),
                Span::styled(
                    format!("[{}]", issue.id),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::raw(" "),
                Span::styled(&issue.message, style),
                Span::styled(fixable_marker, Style::default().fg(Color::Green)),
            ]);
            ListItem::new(content)
        })
        .collect();

    let issues_count = filtered.len();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!(" Issues ({}) ", issues_count)))
        .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let help_text = " ↑/↓ or j/k: Navigate | a/e/w/f: Filter | q: Quit ";
    let footer = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(footer, area);
}

// ═══════════════════════════════════════════════════════════════════════════
// EVENT HANDLING
// ═══════════════════════════════════════════════════════════════════════════

fn handle_events(app: &mut DashboardApp) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Char('a') => app.filter = FilterMode::All,
                    KeyCode::Char('e') => app.filter = FilterMode::Errors,
                    KeyCode::Char('w') => app.filter = FilterMode::Warnings,
                    KeyCode::Char('f') => app.filter = FilterMode::Fixable,
                    _ => {}
                }
            }
        }
    }
    Ok(!app.should_quit)
}

// ═══════════════════════════════════════════════════════════════════════════
// RUN FUNCTION
// ═══════════════════════════════════════════════════════════════════════════

pub fn run(cmd: DashboardCommand, cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    use crate::commands::verify::VerifyCommand;
    
    let data_dir = PathBuf::from(&cli.data_dir);
    
    // Ejecutar verificación
    let verify_cmd = VerifyCommand {
        path: Some(data_dir.clone()),
        phase: None,
        quick: cmd.quick,
        json: false,
        quiet: true,
        schema_strict: false,
        progress: false,
        cache: false,
        root_only: false,
        exclude: vec![],
    };
    
    let result = verify_cmd.run(&data_dir)?;
    
    // Iniciar TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = DashboardApp::new(&result);
    
    // Configurar filtro inicial
    app.filter = match cmd.filter.as_str() {
        "errors" => FilterMode::Errors,
        "warnings" => FilterMode::Warnings,
        "fixable" => FilterMode::Fixable,
        _ => FilterMode::All,
    };

    // Main loop
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        
        if !handle_events(&mut app)? {
            break;
        }
    }

    // Restaurar terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
