/// Narrator — terminal output that tells the Plato story
/// Prints formatted narrative as the demo progresses.

use crate::minimal_engine::{Alarm, MinimalEngine, Severity};
use crate::minimal_music::MinimalMusic;
use crate::minimal_ternary::TritState;
use crate::scenario::ScenarioPhase;
use crate::{DemoResult, CadenceType};

pub struct Narrator {
    verbose: bool,
}

impl Narrator {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    pub fn header(&self) {
        println!();
        println!("═══════════════════════════════════════════════════════════");
        println!("  PLATO DEMO — Fishing Boat \"The Ermentrude\"");
        println!("  5 rooms · 8 sensors · $75 total hardware · 0 cloud dollars");
        println!("═══════════════════════════════════════════════════════════");
        println!();
    }

    pub fn render_tick(
        &self,
        tick: usize,
        engine: &MinimalEngine,
        trit_state: &TritState,
        groove: f64,
        alarms: &[Alarm],
    ) {
        let sensors = engine.sensor_values();
        let music = MinimalMusic::new(5);
        let temp = sensors.get("engine_temp_c").copied().unwrap_or(0.0);
        let bilge = sensors.get("bilge_level_m").copied().unwrap_or(0.0);
        let rpm = sensors.get("rpm").copied().unwrap_or(0.0);
        let fuel = sensors.get("fuel_pct").copied().unwrap_or(0.0);

        let groove_bar = music.groove_bar(groove);

        // Only print at key moments (not every tick)
        match tick {
            1 => {
                println!("  Tick 1-20: All normal");
                println!("    Engine: {:.0}°C ●  Bilge: {:.1}m ●  RPM: {:.0} ●  Fuel: {:.0}%", temp, bilge, rpm, fuel);
                println!("    Ternary: {} = 0x{:04X}", trit_state, trit_state.pack());
                println!("    Fleet groove: {:.2} {}", groove, groove_bar);
                println!("    ───────────────────────────────────────");
            }
            21 => {
                println!();
                println!("  Tick 21: Engine temp rising...");
                println!("    Engine: {:.0}°C ▲  Ternary: {}", temp, trit_state);
                println!("    Fleet groove: {:.2} {}", groove, groove_bar);
            }
            35 => {
                println!();
                println!("  Tick 35: Engine at {:.0}°C — approaching threshold", temp);
                println!("    Ternary: {}  Groove: {:.2} {}", trit_state, groove, groove_bar);
            }
            t if !alarms.is_empty() && (tick == 36 || tick == 42 || tick == 44 || self.verbose) => {
                let icon = match alarms[0].severity {
                    Severity::Critical => "🔴",
                    Severity::Warning => "🟡",
                };
                if t == alarms.first().map(|_| tick).unwrap_or(0) || self.verbose {
                    println!();
                    println!("  Tick {}: {} {}", tick, icon, alarms[0].message);
                    println!("    Ternary: {} = 0x{:04X}  (magnitude: {})", trit_state, trit_state.pack(), trit_state.magnitude());
                    println!("    Fleet groove: {:.2} {}", groove, groove_bar);

                    // Show counterpoint
                    let changes = engine.recent_changes();
                    let motions = music.detect_motion(&changes);
                    let parallel: Vec<_> = motions.iter().filter(|(_, _, m)| matches!(m, crate::minimal_music::Motion::Parallel)).collect();
                    if !parallel.is_empty() {
                        println!("    Counterpoint: Engine ↑ Bilge ↑ (parallel — correlated!)");
                    }
                }
            }
            48 => {
                println!();
                println!("  Tick 48: Agent acts — bilge pump ON, RPM reduced");
                println!("    ⚡ ACTUATOR: bilge_pump → 1.0");
                println!("    ⚡ ACTUATOR: rpm_limit → 1500");
            }
            60 => {
                println!();
                println!("  Tick 60: Stabilizing");
                println!("    Engine: {:.0}°C ▼  Bilge: {:.1}m ▼", temp, bilge);
                println!("    Ternary: {} = 0x{:04X}", trit_state, trit_state.pack());
                println!("    Fleet groove: {:.2} {}", groove, groove_bar);
            }
            80 => {
                println!();
                println!("  Tick 80: All clear — perfect cadence");
                println!("    Engine: {:.0}°C ●  Groove: {:.2} {}", temp, groove, groove_bar);
                println!("    Cadence: alarm → action → resolve ✓ (PERFECT)");
                println!("    ───────────────────────────────────────");
            }
            _ if self.verbose => {
                println!("  Tick {}: {:.0}°C groove={:.2}", tick, temp, groove);
            }
            _ => {}
        }
    }

    pub fn footer(&self, result: &DemoResult) {
        println!();
        println!("═══════════════════════════════════════════════════════════");
        println!("  {} ticks. {} crisis ticks. Min groove: {:.2}. Max trit magnitude: {}.",
            result.total_ticks, result.alarm_ticks, result.min_groove, result.max_trit_magnitude);
        let cadence = match result.cadence {
            CadenceType::Perfect => "alarm → action → resolve ✓ (PERFECT)",
            CadenceType::Deceptive => "alarm → action → new alarm ✗ (DECEPTIVE)",
            CadenceType::Half => "alarm → unresolved (HALF)",
            CadenceType::None => "no alarm cycle",
        };
        println!("  Cadence: {}", cadence);
        println!("  The room persisted. The agent listened. The boat floated.");
        println!("═══════════════════════════════════════════════════════════");
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_narrator_creates() {
        let n = Narrator::new(false);
        assert!(!n.verbose);
    }

    #[test]
    fn test_narrator_verbose() {
        let n = Narrator::new(true);
        assert!(n.verbose);
    }
}
