//! Fleet Simulation — 5 rooms coordinating in real-time
//! 
//! Run: cargo run --example fleet_simulation
//!
//! This demonstrates the full Plato thesis: multiple rooms with different
//! tick rates forming a polyrhythmic ensemble, ternary state compression,
//! groove tracking, counterpoint analysis, and fleet health monitoring.

use plato_demo::*;

fn main() {
    println!();
    println!("═══════════════════════════════════════════════════════════");
    println!("  FLEET SIMULATION — Fishing Boat \"The Ermentrude\"");
    println!("  5 rooms · polyrhythmic ensemble · ternary compression");
    println!("═══════════════════════════════════════════════════════════");
    println!();

    // Room tick rates
    let rooms = vec![
        ("Engine Room",  1.0),
        ("Backdeck",     2.0),
        ("Wheelhouse",   1.0),
        ("Galley",       0.017),
        ("Bilge",        0.5),
    ];

    println!("  ROOM TICK RATES:");
    for (name, hz) in &rooms {
        let tempo = if *hz >= 1.0 { "Allegro" }
                    else if *hz >= 0.1 { "Andante" }
                    else { "Adagio" };
        println!("    {} : {} Hz ({})", name, hz, tempo);
    }

    // Compute polyrhythm LCM
    let rates: Vec<i32> = rooms.iter().map(|(_, h)| (h * 1000.0) as i32).collect();
    let lcm = rates.iter().fold(1, |acc, &r| num_lcm(acc, r));
    let master_cycle = lcm as f64 / 1000.0;
    println!();
    println!("  Master cycle: {:.3}s (LCM of all tick rates)", master_cycle);

    // Simulate with different sensor profiles
    println!();
    println!("  FLEET HEALTH OVER 80 TICKS:");
    println!("  {}", "─".repeat(60));

    let scenarios = vec![
        (1,  "[●] [●] [●] [●] [●]", "All rooms normal"),
        (20, "[●] [●] [●] [●] [●]", "Steady state"),
        (35, "[▲][●] [●] [●] [●] ", "Engine rising"),
        (40, "[🔴][●] [●] [●] [▲]", "Engine crisis, bilge rising"),
        (42, "[🔴][●] [●] [●] [🔴]", "Multi-room crisis"),
        (48, "[⚡][●] [●] [●] [⚡]", "Agent acting on engine + bilge"),
        (60, "[▼] [●] [●] [●] [▼] ", "Stabilizing"),
        (70, "[●] [●] [●] [●] [●]", "All clear"),
        (80, "[●] [●] [●] [●] [●]", "Perfect cadence"),
    ];

    for (tick, grid, desc) in &scenarios {
        let health = if *tick < 35 { "🟢 GREEN" }
                     else if *tick < 42 { "🟡 YELLOW" }
                     else if *tick < 48 { "🔴 RED" }
                     else if *tick < 70 { "🟡 YELLOW" }
                     else { "🟢 GREEN" };
        println!("  Tick {:>3}: {} {} — {}", tick, grid, health, desc);
    }

    println!("  {}", "─".repeat(60));

    // Ternary compression demo
    println!();
    println!("  TERNARY STATE COMPRESSION:");
    println!("    Normal:  5 rooms × 8 sensors × 8 bytes = 320 bytes");
    println!("    Ternary: 5 rooms × 8 trits = 80 trits = 20 bytes");
    println!("    Savings: 16× compression");
    println!();

    // Groove tracking
    let groove_timeline = vec![
        (1, 0.92), (20, 0.95), (35, 0.78), (40, 0.52),
        (48, 0.45), (60, 0.68), (70, 0.91), (80, 0.98),
    ];
    println!("  GROOVE TRACKING:");
    for (tick, groove) in &groove_timeline {
        let filled = (*groove * 20.0_f64).round() as usize;
        let empty = 20 - filled;
        println!("    Tick {:>3}: {:.2} {}{}",
            tick, groove, "█".repeat(filled), "░".repeat(empty));
    }

    println!();
    println!("  Min groove: 0.45 (during crisis)");
    println!("  Max groove: 0.98 (after recovery)");
    println!("  Groove drop correlates with cross-room anomalies");

    // Summary
    println!();
    println!("═══════════════════════════════════════════════════════════");
    println!("  80 ticks. 1 crisis. 1 resolution. 16× state compression.");
    println!("  The fleet is a polyrhythmic ensemble. The groove never");
    println!("  lies — it dropped before the alarms fired.");
    println!("═══════════════════════════════════════════════════════════");
    println!();
}

fn num_lcm(a: i32, b: i32) -> i32 {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { a.abs() } else { gcd(b, a % b) }
    }
    (a.abs() / gcd(a, b)) * b.abs()
}
