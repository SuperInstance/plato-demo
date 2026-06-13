/// Minimal dashboard — terminal rendering with sparklines
/// Pure ANSI escape codes, no external dependencies.

use crate::minimal_engine::{Alarm, Severity};

pub struct MinimalDashboard;

impl MinimalDashboard {
    pub fn new() -> Self {
        Self
    }

    /// Render a sparkline from a slice of values
    pub fn sparkline(&self, values: &[f64], width: usize) -> String {
        if values.is_empty() {
            return "░".repeat(width);
        }
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = max - min;
        let chars = "▁▂▃▄▅▆▇█";

        if range < 1e-10 {
            return "▄".repeat(width.min(values.len()));
        }

        let step = if values.len() <= width { 1 } else { values.len() / width };
        let sampled: Vec<f64> = values.iter().step_by(step).copied().collect();

        sampled.iter().map(|&v| {
            let idx = (((v - min) / range) * 7.0).round() as usize;
            let idx = idx.min(7);
            chars.chars().nth(idx).unwrap()
        }).collect()
    }

    /// Render trend arrow
    pub fn trend_arrow(&self, values: &[f64]) -> &'static str {
        if values.len() < 2 {
            return "→";
        }
        let recent = &values[values.len() - 5.min(values.len())..];
        let first = recent.first().unwrap();
        let last = recent.last().unwrap();
        let delta = last - first;
        if delta > 0.5 { "↑" } else if delta < -0.5 { "↓" } else { "→" }
    }

    /// Format alarm indicator
    pub fn alarm_indicator(&self, severity: &Severity) -> &'static str {
        match severity {
            Severity::Warning => "🟡",
            Severity::Critical => "🔴",
        }
    }

    /// Render sensor panel line
    pub fn sensor_line(&self, name: &str, value: f64, unit: &str, threshold: Option<(f64, f64)>) -> String {
        let status = match threshold {
            Some((low, high)) if value > high => "🔴",
            Some((low, high)) if value < low => "🟡",
            _ => "●",
        };
        format!("  {}: {:.1}{} {}", name, value, unit, status)
    }

    /// Render full room panel
    pub fn room_panel(&self, name: &str, sensors: &[(String, f64, String)]) -> String {
        let mut lines = vec![format!("┌─ {} ─", name)];
        for (s_name, value, unit) in sensors {
            lines.push(format!("│ {}: {:.1}{}", s_name, value, unit));
        }
        lines.push("└─────".into());
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparkline_empty() {
        let d = MinimalDashboard::new();
        let s = d.sparkline(&[], 5);
        assert_eq!(s, "░░░░░");
    }

    #[test]
    fn test_sparkline_single() {
        let d = MinimalDashboard::new();
        let s = d.sparkline(&[5.0], 1);
        assert_eq!(s, "▄");
    }

    #[test]
    fn test_sparkline_range() {
        let d = MinimalDashboard::new();
        let s = d.sparkline(&[0.0, 50.0, 100.0], 3);
        assert!(s.contains('▁'));
        assert!(s.contains('█'));
    }

    #[test]
    fn test_trend_rising() {
        let d = MinimalDashboard::new();
        assert_eq!(d.trend_arrow(&[80.0, 82.0, 85.0, 88.0, 92.0]), "↑");
    }

    #[test]
    fn test_trend_falling() {
        let d = MinimalDashboard::new();
        assert_eq!(d.trend_arrow(&[96.0, 93.0, 90.0, 87.0, 84.0]), "↓");
    }

    #[test]
    fn test_trend_flat() {
        let d = MinimalDashboard::new();
        assert_eq!(d.trend_arrow(&[82.0, 82.1, 81.9, 82.0, 82.1]), "→");
    }

    #[test]
    fn test_alarm_indicator() {
        let d = MinimalDashboard::new();
        assert_eq!(d.alarm_indicator(&Severity::Critical), "🔴");
        assert_eq!(d.alarm_indicator(&Severity::Warning), "🟡");
    }

    #[test]
    fn test_sensor_line() {
        let d = MinimalDashboard::new();
        let line = d.sensor_line("temp", 96.5, "°C", Some((80.0, 95.0)));
        assert!(line.contains("96.5"));
        assert!(line.contains("🔴"));
    }

    #[test]
    fn test_room_panel() {
        let d = MinimalDashboard::new();
        let panel = d.room_panel("Engine", &[
            ("temp".into(), 82.0, "°C".into()),
            ("rpm".into(), 1800.0, "".into()),
        ]);
        assert!(panel.contains("Engine"));
        assert!(panel.contains("82.0"));
    }
}
