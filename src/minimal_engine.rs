/// Minimal inline Plato Engine — just enough to tick, store history, evaluate alarms
/// No dependency on the real plato-engine-block crate.

use crate::scenario::TickData;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct MinimalEngine {
    pub sensor_values: HashMap<String, f64>,
    pub history: Vec<HashMap<String, f64>>,
    pub max_history: usize,
}

impl MinimalEngine {
    pub fn new() -> Self {
        Self {
            sensor_values: HashMap::new(),
            history: Vec::new(),
            max_history: 100,
        }
    }

    pub fn tick(&mut self, data: &TickData) {
        self.sensor_values.insert("engine_temp_c".into(), data.engine_temp_c);
        self.sensor_values.insert("bilge_level_m".into(), data.bilge_level_m);
        self.sensor_values.insert("rpm".into(), data.rpm);
        self.sensor_values.insert("fuel_pct".into(), data.fuel_pct);
        self.sensor_values.insert("backdeck_temp_c".into(), data.backdeck_temp_c);
        self.sensor_values.insert("wheelhouse_humidity".into(), data.wheelhouse_humidity);
        self.sensor_values.insert("galley_temp_c".into(), data.galley_temp_c);
        self.sensor_values.insert("vibration".into(), data.vibration);

        self.history.push(self.sensor_values.clone());
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }

    pub fn sensor_values(&self) -> &HashMap<String, f64> {
        &self.sensor_values
    }

    pub fn evaluate_alarms(&self) -> Vec<Alarm> {
        let mut alarms = Vec::new();
        if let Some(&temp) = self.sensor_values.get("engine_temp_c") {
            if temp > 95.0 {
                alarms.push(Alarm {
                    name: "engine_overheat".into(),
                    message: format!("Engine temp {:.1}°C > 95°C threshold", temp),
                    severity: Severity::Critical,
                });
            }
        }
        if let Some(&bilge) = self.sensor_values.get("bilge_level_m") {
            if bilge > 1.0 {
                alarms.push(Alarm {
                    name: "bilge_high".into(),
                    message: format!("Bilge level {:.2}m > 1.0m threshold", bilge),
                    severity: Severity::Warning,
                });
            }
        }
        alarms
    }

    pub fn room_phases(&self) -> Vec<f64> {
        // Simulated phase for each of 5 rooms
        // Phase = fractional position in current tick cycle
        let t = self.history.len() as f64;
        vec![
            (t * 0.2).fract(),  // engine room: 0.2 Hz
            (t * 2.0).fract(),  // backdeck: 2 Hz
            (t * 1.0).fract(),  // wheelhouse: 1 Hz
            (t * 0.017).fract(), // galley: 0.017 Hz
            (t * 0.5).fract(),  // bilge: 0.5 Hz
        ]
    }

    pub fn recent_changes(&self) -> Vec<(f64, f64)> {
        // (old, new) for each sensor — used for counterpoint
        if self.history.len() < 2 {
            return vec![(0.0, 0.0); 8];
        }
        let prev = &self.history[self.history.len() - 2];
        let curr = &self.history[self.history.len() - 1];
        let keys = ["engine_temp_c", "bilge_level_m", "rpm", "fuel_pct",
                     "backdeck_temp_c", "wheelhouse_humidity", "galley_temp_c", "vibration"];
        keys.iter().map(|k| {
            let old = prev.get(*k).copied().unwrap_or(0.0);
            let new = curr.get(*k).copied().unwrap_or(0.0);
            (old, new)
        }).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Alarm {
    pub name: String,
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Severity {
    Warning,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tick(t: usize) -> TickData {
        crate::scenario::FishingBoatScenario::new(80).iter().nth(t - 1).unwrap()
    }

    #[test]
    fn test_engine_stores_sensors() {
        let mut e = MinimalEngine::new();
        e.tick(&make_tick(1));
        assert!(e.sensor_values.contains_key("engine_temp_c"));
        assert_eq!(e.history.len(), 1);
    }

    #[test]
    fn test_no_alarm_normal() {
        let mut e = MinimalEngine::new();
        e.tick(&make_tick(10)); // normal tick
        assert!(e.evaluate_alarms().is_empty());
    }

    #[test]
    fn test_alarm_on_overheat() {
        let mut e = MinimalEngine::new();
        e.tick(&make_tick(42)); // crisis tick
        let alarms = e.evaluate_alarms();
        assert!(!alarms.is_empty());
        assert_eq!(alarms[0].name, "engine_overheat");
        assert_eq!(alarms[0].severity, Severity::Critical);
    }

    #[test]
    fn test_history_overflow() {
        let mut e = MinimalEngine::new();
        e.max_history = 5;
        for i in 1..=10 {
            e.tick(&make_tick(i));
        }
        assert_eq!(e.history.len(), 5);
    }

    #[test]
    fn test_room_phases() {
        let mut e = MinimalEngine::new();
        e.tick(&make_tick(1));
        let phases = e.room_phases();
        assert_eq!(phases.len(), 5);
        for p in &phases {
            assert!(*p >= 0.0 && *p <= 1.0);
        }
    }
}
