//! Telemetría local para métricas.
//!
//! Almacena métricas localmente sin envío externo.

use parking_lot::RwLock;
use std::collections::HashMap;
use std::time::{Duration, Instant};

// ═══════════════════════════════════════════════════════════════════════════
// TELEMETRY EVENT
// ═══════════════════════════════════════════════════════════════════════════

/// Tipo de evento.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    Command,
    Error,
    Warning,
    Performance,
    Custom(String),
}

/// Evento de telemetría.
#[derive(Debug, Clone)]
pub struct TelemetryEvent {
    pub event_type: EventType,
    pub name: String,
    pub timestamp: Instant,
    pub duration: Option<Duration>,
    pub metadata: HashMap<String, String>,
}

impl TelemetryEvent {
    pub fn command(name: impl Into<String>, duration: Duration) -> Self {
        Self {
            event_type: EventType::Command,
            name: name.into(),
            timestamp: Instant::now(),
            duration: Some(duration),
            metadata: HashMap::new(),
        }
    }

    pub fn error(name: impl Into<String>) -> Self {
        Self {
            event_type: EventType::Error,
            name: name.into(),
            timestamp: Instant::now(),
            duration: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TELEMETRY
// ═══════════════════════════════════════════════════════════════════════════

/// Sistema de telemetría local.
#[derive(Debug, Default)]
pub struct Telemetry {
    /// Eventos registrados.
    events: RwLock<Vec<TelemetryEvent>>,
    /// Contadores por tipo.
    counters: RwLock<HashMap<String, usize>>,
    /// Duraciones acumuladas por comando.
    durations: RwLock<HashMap<String, Duration>>,
}

impl Telemetry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registra un comando.
    pub fn track_command(&self, name: &str, duration: Duration) {
        let event = TelemetryEvent::command(name, duration);
        self.events.write().push(event);

        *self
            .counters
            .write()
            .entry(format!("cmd:{}", name))
            .or_insert(0) += 1;
        *self
            .durations
            .write()
            .entry(name.to_string())
            .or_insert(Duration::ZERO) += duration;
    }

    /// Registra un error.
    pub fn track_error(&self, error: &str) {
        let event = TelemetryEvent::error(error);
        self.events.write().push(event);
        *self
            .counters
            .write()
            .entry("errors:total".to_string())
            .or_insert(0) += 1;
    }

    /// Incrementa un contador.
    pub fn increment(&self, name: &str) {
        *self.counters.write().entry(name.to_string()).or_insert(0) += 1;
    }

    /// Obtiene un contador.
    pub fn get_counter(&self, name: &str) -> usize {
        *self.counters.read().get(name).unwrap_or(&0)
    }

    /// Duración total de un comando.
    pub fn total_duration(&self, command: &str) -> Duration {
        *self
            .durations
            .read()
            .get(command)
            .unwrap_or(&Duration::ZERO)
    }

    /// Número de eventos.
    pub fn event_count(&self) -> usize {
        self.events.read().len()
    }

    /// Resumen de estadísticas.
    pub fn summary(&self) -> HashMap<String, String> {
        let mut summary = HashMap::new();

        let counters = self.counters.read();
        for (key, value) in counters.iter() {
            summary.insert(key.clone(), value.to_string());
        }

        let durations = self.durations.read();
        for (cmd, dur) in durations.iter() {
            summary.insert(format!("duration:{}", cmd), format!("{:?}", dur));
        }

        summary
    }

    /// Limpia todos los datos.
    pub fn clear(&self) {
        self.events.write().clear();
        self.counters.write().clear();
        self.durations.write().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_command() {
        let telemetry = Telemetry::new();
        telemetry.track_command("diagnose", Duration::from_secs(1));

        assert_eq!(telemetry.event_count(), 1);
        assert_eq!(telemetry.get_counter("cmd:diagnose"), 1);
    }

    #[test]
    fn test_track_error() {
        let telemetry = Telemetry::new();
        telemetry.track_error("file not found");

        assert_eq!(telemetry.get_counter("errors:total"), 1);
    }

    #[test]
    fn test_increment() {
        let telemetry = Telemetry::new();
        telemetry.increment("custom:metric");
        telemetry.increment("custom:metric");

        assert_eq!(telemetry.get_counter("custom:metric"), 2);
    }

    #[test]
    fn test_total_duration() {
        let telemetry = Telemetry::new();
        telemetry.track_command("cmd", Duration::from_secs(1));
        telemetry.track_command("cmd", Duration::from_secs(2));

        assert_eq!(telemetry.total_duration("cmd"), Duration::from_secs(3));
    }
}
