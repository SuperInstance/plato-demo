//! # Plato Demo — The Matrix in 60 Seconds
//!
//! Clone this. Run it. Watch a fishing boat think.
//!
//! This crate simulates the complete Plato Matrix thesis: 5 rooms on a boat,
//! ternary state compression, music-cognitive sync, flux-compiled alarms,
//! and a terminal dashboard — all in pure Rust, no hardware needed.
//!
//! ```
//! use plato_demo::FishingBoatDemo;
//!
//! let demo = FishingBoatDemo::new();
//! demo.run(); // Watch the story unfold
//! ```

pub mod scenario;
pub mod minimal_engine;
pub mod minimal_ternary;
pub mod minimal_music;
pub mod minimal_dashboard;
pub mod narrator;

pub use scenario::FishingBoatScenario;
pub use minimal_engine::MinimalEngine;
pub use minimal_ternary::MinimalTernary;
pub use minimal_music::MinimalMusic;
pub use minimal_dashboard::MinimalDashboard;
pub use narrator::Narrator;

/// The full demo — simulates a fishing boat crisis and recovery
pub struct FishingBoatDemo {
    pub ticks: usize,
    pub verbose: bool,
}

impl FishingBoatDemo {
    pub fn new() -> Self {
        Self { ticks: 80, verbose: false }
    }

    pub fn with_ticks(mut self, n: usize) -> Self {
        self.ticks = n;
        self
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// Run the complete demo scenario
    pub fn run(&self) -> DemoResult {
        let scenario = FishingBoatScenario::new(self.ticks);
        let mut engine = MinimalEngine::new();
        let ternary = MinimalTernary::new();
        let music = MinimalMusic::new(5); // 5 rooms
        let narrator = Narrator::new(self.verbose);

        let mut result = DemoResult {
            min_groove: 1.0,
            ..DemoResult::default()
        };

        narrator.header();

        for tick_data in scenario.iter() {
            engine.tick(&tick_data);

            let trit_state = ternary.to_trits(engine.sensor_values());
            let groove = music.compute_groove(&engine.room_phases());
            let _counterpoint = music.detect_motion(&engine.recent_changes());
            let alarms = engine.evaluate_alarms();

            result.total_ticks += 1;
            if alarms.is_empty() {
                if result.alarm_ticks > 0 && result.cadence == CadenceType::None {
                    result.cadence = CadenceType::Perfect;
                }
                result.normal_ticks += 1;
            } else {
                result.alarm_ticks += 1;
            }
            result.min_groove = result.min_groove.min(groove);
            result.max_trit_magnitude = result.max_trit_magnitude.max(trit_state.magnitude());

            narrator.render_tick(tick_data.tick, &engine, &trit_state, groove, &alarms);
        }

        narrator.footer(&result);
        result
    }
}

impl Default for FishingBoatDemo {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct DemoResult {
    pub total_ticks: usize,
    pub normal_ticks: usize,
    pub alarm_ticks: usize,
    pub min_groove: f64,
    pub max_trit_magnitude: i32,
    pub cadence: CadenceType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CadenceType {
    None,
    Perfect,   // alarm → action → resolve
    Deceptive, // alarm → action → new alarm
    Half,      // alarm → unresolved
}

impl Default for CadenceType {
    fn default() -> Self {
        CadenceType::None
    }
}
