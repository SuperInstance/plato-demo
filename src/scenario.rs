/// Fishing boat scenario — pre-scripted sensor values that tell a story
///
/// Phase 1 (ticks 1-20): Normal operation
/// Phase 2 (ticks 21-35): Engine temp rising
/// Phase 3 (ticks 36-47): Crisis — engine overheats, bilge rises
/// Phase 4 (ticks 48-60): Agent acts, stabilization
/// Phase 5 (ticks 61-80): Recovery, back to normal

#[derive(Debug, Clone)]
pub struct TickData {
    pub tick: usize,
    pub engine_temp_c: f64,
    pub bilge_level_m: f64,
    pub rpm: f64,
    pub fuel_pct: f64,
    pub backdeck_temp_c: f64,
    pub wheelhouse_humidity: f64,
    pub galley_temp_c: f64,
    pub vibration: f64,
    pub resolved: bool,
    pub phase: ScenarioPhase,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScenarioPhase {
    Normal,
    Rising,
    Crisis,
    Action,
    Recovery,
}

pub struct FishingBoatScenario {
    total_ticks: usize,
}

impl FishingBoatScenario {
    pub fn new(total_ticks: usize) -> Self {
        Self { total_ticks }
    }

    pub fn iter(&self) -> ScenarioIterator {
        ScenarioIterator {
            current: 0,
            total: self.total_ticks,
        }
    }
}

pub struct ScenarioIterator {
    current: usize,
    total: usize,
}

impl Iterator for ScenarioIterator {
    type Item = TickData;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.total {
            return None;
        }
        self.current += 1;
        Some(generate_tick(self.current))
    }
}

fn generate_tick(t: usize) -> TickData {
    let (temp, bilge, rpm, fuel, vibration, phase, resolved) = match t {
        1..=20 => {
            // Normal operation
            let temp = 80.0 + (t as f64 * 0.1).sin() * 2.0;
            let bilge = 0.3 + (t as f64 * 0.05).sin() * 0.1;
            (temp, bilge, 1800.0, 80.0 - t as f64 * 0.1, 0.2, ScenarioPhase::Normal, false)
        }
        21..=35 => {
            // Rising — engine temp climbing
            let progress = (t - 20) as f64 / 15.0;
            let temp = 82.0 + progress * 13.0; // 82 → 95
            let bilge = 0.3 + progress * 0.2;
            (temp, bilge, 1800.0 + progress * 100.0, 78.0 - t as f64 * 0.1, 0.2 + progress * 0.3, ScenarioPhase::Rising, false)
        }
        36..=47 => {
            // Crisis — engine overheated, bilge rising
            let progress = (t - 35) as f64 / 12.0;
            let temp = 95.0 + progress * 3.5; // 95 → 98.5
            let bilge = 0.5 + progress * 0.7; // 0.5 → 1.2
            (temp, bilge, 1900.0 - progress * 200.0, 76.0 - t as f64 * 0.1, 0.5 + progress * 0.3, ScenarioPhase::Crisis, false)
        }
        48..=60 => {
            // Action — agent turns on bilge pump, reduces RPM
            let progress = (t - 47) as f64 / 13.0;
            let temp = 98.5 - progress * 10.0; // 98.5 → 88.5
            let bilge = 1.2 - progress * 0.6; // 1.2 → 0.6
            (temp, bilge, 1700.0 - progress * 100.0, 74.0 - t as f64 * 0.05, 0.8 - progress * 0.5, ScenarioPhase::Action, progress > 0.5)
        }
        _ => {
            // Recovery
            let progress = ((t - 60) as f64 / 20.0).min(1.0);
            let temp = 88.5 - progress * 6.5; // 88.5 → 82
            let bilge = 0.6 - progress * 0.3; // 0.6 → 0.3
            (temp, bilge, 1600.0 + progress * 200.0, 73.0 - t as f64 * 0.05, 0.3 - progress * 0.1, ScenarioPhase::Recovery, true)
        }
    };

    TickData {
        tick: t,
        engine_temp_c: temp,
        bilge_level_m: bilge,
        rpm,
        fuel_pct: fuel,
        backdeck_temp_c: 22.0 + (t as f64 * 0.3).sin() * 3.0,
        wheelhouse_humidity: 55.0 + (t as f64 * 0.2).sin() * 5.0,
        galley_temp_c: 24.0 + (t as f64 * 0.1).sin() * 1.5,
        vibration,
        resolved,
        phase,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_produces_ticks() {
        let s = FishingBoatScenario::new(80);
        let ticks: Vec<_> = s.iter().collect();
        assert_eq!(ticks.len(), 80);
    }

    #[test]
    fn test_normal_phase_temps() {
        for t in 1..=20 {
            let tick = generate_tick(t);
            assert_eq!(tick.phase, ScenarioPhase::Normal);
            assert!(tick.engine_temp_c < 85.0, "Tick {}: temp {} should be normal", t, tick.engine_temp_c);
        }
    }

    #[test]
    fn test_crisis_phase() {
        let tick42 = generate_tick(42);
        assert_eq!(tick42.phase, ScenarioPhase::Crisis);
        assert!(tick42.engine_temp_c > 95.0, "Crisis temp should exceed 95°C");
    }

    #[test]
    fn test_recovery_phase() {
        let tick80 = generate_tick(80);
        assert_eq!(tick80.phase, ScenarioPhase::Recovery);
        assert!(tick80.engine_temp_c < 85.0, "Recovery temp should be back to normal");
        assert!(tick80.resolved);
    }

    #[test]
    fn test_all_phases_present() {
        let phases: Vec<_> = (1..=80).map(|t| generate_tick(t).phase).collect();
        assert!(phases.contains(&ScenarioPhase::Normal));
        assert!(phases.contains(&ScenarioPhase::Rising));
        assert!(phases.contains(&ScenarioPhase::Crisis));
        assert!(phases.contains(&ScenarioPhase::Action));
        assert!(phases.contains(&ScenarioPhase::Recovery));
    }

    #[test]
    fn test_bilge_rises_during_crisis() {
        let normal = generate_tick(10);
        let crisis = generate_tick(42);
        assert!(crisis.bilge_level_m > normal.bilge_level_m);
    }
}
