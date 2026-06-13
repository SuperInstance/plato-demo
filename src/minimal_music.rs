/// Minimal music-cognitive sync — groove tracking and counterpoint analysis
/// Demonstrates: rooms as ensemble, groove = alignment, counterpoint = interaction quality

#[derive(Debug, Clone)]
pub struct MinimalMusic {
    num_rooms: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Motion {
    Contrary,  // one up, one down — productive
    Parallel,  // both same direction — potentially redundant
    Oblique,   // one stable, one changing — normal
    Static,    // both stable — idle
}

impl MinimalMusic {
    pub fn new(num_rooms: usize) -> Self {
        Self { num_rooms }
    }

    /// Compute groove score (0.0 chaos → 1.0 perfect sync)
    /// Based on how well room phases align to expected positions
    pub fn compute_groove(&self, phases: &[f64]) -> f64 {
        if phases.is_empty() {
            return 1.0;
        }
        // Groove = 1 - average pairwise phase difference
        // When rooms are in sync, phases cluster → small difference → high groove
        let n = phases.len();
        if n < 2 {
            return 1.0;
        }
        let mut total_diff = 0.0;
        let mut count = 0;
        for i in 0..n {
            for j in (i+1)..n {
                let diff = (phases[i] - phases[j]).abs();
                let diff = diff.min(1.0 - diff); // circular distance
                total_diff += diff;
                count += 1;
            }
        }
        let avg_diff = total_diff / count as f64;
        // avg_diff ranges from 0 (perfect sync) to 0.5 (anti-sync)
        (1.0 - avg_diff * 2.0).max(0.0).min(1.0)
    }

    /// Detect motion between two sensor change pairs
    pub fn detect_motion_pair(&self, a: (f64, f64), b: (f64, f64)) -> Motion {
        let delta_a = a.1 - a.0;
        let delta_b = b.1 - b.0;
        let threshold = 0.1;

        if delta_a.abs() < threshold && delta_b.abs() < threshold {
            Motion::Static
        } else if delta_a.abs() < threshold || delta_b.abs() < threshold {
            Motion::Oblique
        } else if (delta_a > 0.0 && delta_b > 0.0) || (delta_a < 0.0 && delta_b < 0.0) {
            Motion::Parallel
        } else {
            Motion::Contrary
        }
    }

    /// Detect motion across all sensor pairs
    pub fn detect_motion(&self, changes: &[(f64, f64)]) -> Vec<(usize, usize, Motion)> {
        let mut motions = Vec::new();
        for i in 0..changes.len() {
            for j in (i+1)..changes.len() {
                let motion = self.detect_motion_pair(changes[i], changes[j]);
                motions.push((i, j, motion));
            }
        }
        motions
    }

    /// Render groove as a bar
    pub fn groove_bar(&self, groove: f64) -> String {
        let filled = (groove * 10.0).round() as usize;
        let empty = 10 - filled;
        format!("{}{}", "■".repeat(filled), "·".repeat(empty))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_groove() {
        let m = MinimalMusic::new(5);
        // All phases equal = perfect sync
        let phases = vec![0.5, 0.5, 0.5, 0.5, 0.5];
        let groove = m.compute_groove(&phases);
        assert!(groove > 0.9, "Perfect sync should have groove > 0.9, got {}", groove);
    }

    #[test]
    fn test_chaotic_groove() {
        let m = MinimalMusic::new(5);
        // Evenly spaced = maximum anti-sync
        let phases = vec![0.0, 0.2, 0.4, 0.6, 0.8];
        let groove = m.compute_groove(&phases);
        assert!(groove < 0.7, "Evenly spaced should have moderate groove, got {}", groove);
    }

    #[test]
    fn test_contrary_motion() {
        let m = MinimalMusic::new(2);
        let motion = m.detect_motion_pair((80.0, 90.0), (1.0, 0.5)); // one up, one down
        assert_eq!(motion, Motion::Contrary);
    }

    #[test]
    fn test_parallel_motion() {
        let m = MinimalMusic::new(2);
        let motion = m.detect_motion_pair((80.0, 90.0), (0.3, 0.5)); // both up
        assert_eq!(motion, Motion::Parallel);
    }

    #[test]
    fn test_oblique_motion() {
        let m = MinimalMusic::new(2);
        let motion = m.detect_motion_pair((80.0, 90.0), (0.3, 0.3)); // one moving, one stable
        assert_eq!(motion, Motion::Oblique);
    }

    #[test]
    fn test_static_motion() {
        let m = MinimalMusic::new(2);
        let motion = m.detect_motion_pair((80.0, 80.05), (0.3, 0.3));
        assert_eq!(motion, Motion::Static);
    }

    #[test]
    fn test_groove_bar() {
        let m = MinimalMusic::new(5);
        let bar = m.groove_bar(0.8);
        assert_eq!(bar, "■■■■■■■■··");
    }

    #[test]
    fn test_detect_motion_multiple() {
        let m = MinimalMusic::new(3);
        let changes = vec![(80.0, 90.0), (0.3, 0.1), (22.0, 22.0)];
        let motions = m.detect_motion(&changes);
        assert_eq!(motions.len(), 3); // C(3,2) = 3 pairs
        assert!(motions.iter().any(|(_, _, mt)| *mt == Motion::Contrary));
    }
}
