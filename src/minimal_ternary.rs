/// Minimal ternary bridge — sensor values → {-1, 0, +1}
/// Demonstrates the core insight: 8 sensors = 2 bytes packed.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MinimalTernary {
    thresholds: HashMap<String, (f64, f64)>, // (low, high) — normal range
}

impl MinimalTernary {
    pub fn new() -> Self {
        let mut thresholds = HashMap::new();
        thresholds.insert("engine_temp_c".into(), (80.0, 95.0));
        thresholds.insert("bilge_level_m".into(), (0.1, 1.0));
        thresholds.insert("rpm".into(), (1200.0, 2200.0));
        thresholds.insert("fuel_pct".into(), (15.0, 100.0));
        thresholds.insert("backdeck_temp_c".into(), (10.0, 40.0));
        thresholds.insert("wheelhouse_humidity".into(), (30.0, 80.0));
        thresholds.insert("galley_temp_c".into(), (18.0, 35.0));
        thresholds.insert("vibration".into(), (0.0, 0.7));
        Self { thresholds }
    }

    /// Convert sensor values to a ternary vector
    pub fn to_trits(&self, sensors: &HashMap<String, f64>) -> TritState {
        let mut trits = [0i8; 8];
        let keys = ["engine_temp_c", "bilge_level_m", "rpm", "fuel_pct",
                     "backdeck_temp_c", "wheelhouse_humidity", "galley_temp_c", "vibration"];
        for (i, key) in keys.iter().enumerate() {
            if let Some(&value) = sensors.get(*key) {
                if let Some(&(low, high)) = self.thresholds.get(*key) {
                    trits[i] = if value < low { -1 } else if value > high { 1 } else { 0 };
                }
            }
        }
        TritState(trits)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TritState(pub [i8; 8]);

impl TritState {
    /// Pack 8 trits into a u32 (2 trits per byte)
    pub fn pack(&self) -> u32 {
        let mut packed = 0u32;
        for (i, &trit) in self.0.iter().enumerate() {
            let shifted = (trit as i32 + 1) as u32; // map -1,0,1 → 0,1,2
            packed |= shifted << (i * 2);
        }
        packed
    }

    /// Unpack from u32 back to trits
    pub fn unpack(packed: u32) -> Self {
        let mut trits = [0i8; 8];
        for i in 0..8 {
            let shifted = ((packed >> (i * 2)) & 0x3) as i32;
            trits[i] = (shifted - 1) as i8; // map 0,1,2 → -1,0,1
        }
        TritState(trits)
    }

    /// Number of non-zero (alarm) trits
    pub fn magnitude(&self) -> i32 {
        self.0.iter().map(|&t| i32::from(t.abs())).sum()
    }

    /// Display as {-1, 0, +1}
    pub fn display_trits(&self) -> String {
        let parts: Vec<String> = self.0.iter().map(|t| {
            match t {
                -1 => "-1".into(),
                0 => " 0".into(),
                1 => "+1".into(),
                _ => "??".into(),
            }
        }).collect();
        format!("{{{}}}", parts.join(", "))
    }
}

impl std::fmt::Display for TritState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_trits())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_state_all_zeros() {
        let t = MinimalTernary::new();
        let mut sensors = HashMap::new();
        sensors.insert("engine_temp_c".into(), 82.0);
        sensors.insert("bilge_level_m".into(), 0.3);
        sensors.insert("rpm".into(), 1800.0);
        sensors.insert("fuel_pct".into(), 80.0);
        sensors.insert("backdeck_temp_c".into(), 22.0);
        sensors.insert("wheelhouse_humidity".into(), 55.0);
        sensors.insert("galley_temp_c".into(), 24.0);
        sensors.insert("vibration".into(), 0.2);
        let state = t.to_trits(&sensors);
        assert_eq!(state.0, [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(state.magnitude(), 0);
    }

    #[test]
    fn test_overheat_produces_positive_trit() {
        let t = MinimalTernary::new();
        let mut sensors = HashMap::new();
        sensors.insert("engine_temp_c".into(), 96.0);
        sensors.insert("bilge_level_m".into(), 0.3);
        sensors.insert("rpm".into(), 1800.0);
        sensors.insert("fuel_pct".into(), 80.0);
        sensors.insert("backdeck_temp_c".into(), 22.0);
        sensors.insert("wheelhouse_humidity".into(), 55.0);
        sensors.insert("galley_temp_c".into(), 24.0);
        sensors.insert("vibration".into(), 0.2);
        let state = t.to_trits(&sensors);
        assert_eq!(state.0[0], 1); // engine over threshold
        assert_eq!(state.magnitude(), 1);
    }

    #[test]
    fn test_pack_roundtrip() {
        let state = TritState([1, -1, 0, 1, 0, -1, 0, 1]);
        let packed = state.pack();
        let unpacked = TritState::unpack(packed);
        assert_eq!(state, unpacked);
    }

    #[test]
    fn test_display_format() {
        let state = TritState([1, -1, 0, 0, 0, 0, 0, 0]);
        let s = format!("{}", state);
        assert!(s.contains("+1"));
        assert!(s.contains("-1"));
    }

    #[test]
    fn test_low_value_negative_trit() {
        let t = MinimalTernary::new();
        let mut sensors = HashMap::new();
        sensors.insert("fuel_pct".into(), 10.0); // below 15% threshold
        sensors.insert("engine_temp_c".into(), 82.0);
        sensors.insert("bilge_level_m".into(), 0.3);
        sensors.insert("rpm".into(), 1800.0);
        sensors.insert("backdeck_temp_c".into(), 22.0);
        sensors.insert("wheelhouse_humidity".into(), 55.0);
        sensors.insert("galley_temp_c".into(), 24.0);
        sensors.insert("vibration".into(), 0.2);
        let state = t.to_trits(&sensors);
        assert_eq!(state.0[3], -1); // fuel below low threshold
    }
}
