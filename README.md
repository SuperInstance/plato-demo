# plato-demo

> **Clone this. Run it. Watch a fishing boat think.**

```bash
git clone https://github.com/SuperInstance/plato-demo
cd plato-demo
cargo run
```

That's it. In 60 seconds you'll see a complete Plato scenario unfold in your terminal — a fishing boat's engine overheats, the ternary bridge detects it, the groove drops, the agent acts, the boat recovers. Every concept from the Plato Matrix, demonstrated live.

No hardware. No cloud. No config files. Just the thesis in motion.

## What You'll See

```
═══════════════════════════════════════════════════════════
  PLATO DEMO — Fishing Boat "The Ermentrude"
  5 rooms · 8 sensors · $75 total hardware · 0 cloud dollars
═══════════════════════════════════════════════════════════

  Tick 1-20: All normal
    Engine: 80°C ●  Bilge: 0.3m ●  RPM: 1800 ●  Fuel: 80%
    Ternary: { 0,  0,  0,  0,  0,  0,  0,  0} = 0x5555
    Fleet groove: 0.52 ■■■■■·····

  Tick 36: 🔴 ENGINE OVERHEAT — 95.3°C > 95°C threshold
    Ternary: {+1,  0,  0,  0,  0,  0,  0,  0} = 0x5556
    Fleet groove: 0.56 ■■■■■■····
    Counterpoint: Engine ↑ Bilge ↑ (parallel — correlated!)

  Tick 44: 🔴 Multi-sensor crisis — engine + bilge + vibration
    Ternary: {+1, +1,  0,  0,  0,  0,  0, +1} = 0x955A  (magnitude: 3)

  Tick 48: Agent acts — bilge pump ON, RPM reduced
    ⚡ ACTUATOR: bilge_pump → 1.0
    ⚡ ACTUATOR: rpm_limit → 1500

  Tick 80: All clear — perfect cadence
    Engine: 82°C ●  Groove: 0.71 ■■■■■■■···
    Cadence: alarm → action → resolve ✓ (PERFECT)

═══════════════════════════════════════════════════════════
  80 ticks. 16 crisis. Min groove: 0.40. Max trit magnitude: 3.
  Cadence: alarm → action → resolve ✓ (PERFECT)
  The room persisted. The agent listened. The boat floated.
═══════════════════════════════════════════════════════════
```

## What's Happening

The demo runs through five phases of a real scenario:

| Phase | Ticks | What | Ternary | Groove |
|-------|-------|------|---------|--------|
| **Normal** | 1-20 | All sensors nominal | `{0,0,0,0,0,0,0,0}` | ~0.50 |
| **Rising** | 21-35 | Engine temp climbing | Still all zeros | ~0.45 |
| **Crisis** | 36-47 | Engine >95°C, bilge rising | `{+1,...,+1}` magnitude 3 | ~0.56 |
| **Action** | 48-60 | Agent activates pump, reduces RPM | Back to zeros | Rising |
| **Recovery** | 61-80 | All clear, perfect cadence | `{0,0,0,0,0,0,0,0}` | ~0.98 |

Each concept demonstrated:

- **Ternary bridge**: Sensor values → `{-1, 0, +1}`. 8 sensors packed into a single u32.
- **Groove tracking**: Fleet alignment score. Drops during crisis, recovers after.
- **Counterpoint**: Detects parallel motion (engine ↑ bilge ↑ = correlated crisis).
- **Flux-compiled alarms**: Threshold crossing triggers alarm evaluation.
- **Cadence**: Alarm → action → resolve = perfect cadence (borrowed from music theory).
- **Dashboard**: Unicode sparklines, severity indicators, fleet health panel.

## Architecture

This crate is self-contained — it has **zero dependencies** on other Plato crates. Every concept is reimplemented inline in the `minimal_*` modules. This is intentional: the demo should clone and run anywhere.

```
src/
├── main.rs              — Entry point: run the demo
├── lib.rs               — FishingBoatDemo orchestrator
├── scenario.rs          — Pre-scripted sensor values (5 phases)
├── minimal_engine.rs    — Inline PlatoEngine (tick, history, alarms)
├── minimal_ternary.rs   — Inline ternary threshold + packing
├── minimal_music.rs     — Inline groove + counterpoint
├── minimal_dashboard.rs — Inline sparklines + panels
└── narrator.rs          — Terminal output with timing
```

Each `minimal_*` module is a simplified version of the real crate. After you understand the demo, you can graduate to the real implementations:

| Demo Module | Real Crate | What It Adds |
|-------------|-----------|--------------|
| `minimal_engine` | [plato-engine-block](https://github.com/SuperInstance/plato-engine-block) | TCP server, text protocol, subscriptions |
| `minimal_ternary` | [plato-ternary-bridge](https://github.com/SuperInstance/plato-ternary-bridge) | Dead zones, fleet voting, delta compression |
| `minimal_music` | [plato-music-sync](https://github.com/SuperInstance/plato-music-sync) | Polyrhythmic scheduling, tempo maps, cadence patterns |
| `minimal_dashboard` | [plato-dashboard](https://github.com/SuperInstance/plato-dashboard) | Room/fleet/alarm panels, keyboard input, responsive layout |

## API

```rust
use plato_demo::FishingBoatDemo;

// Default: 80 ticks, normal output
let result = FishingBoatDemo::new().run();
assert_eq!(result.total_ticks, 80);
assert!(result.alarm_ticks > 0);
assert_eq!(result.cadence, plato_demo::CadenceType::Perfect);

// Verbose: show every tick
FishingBoatDemo::new().verbose().run();

// Custom tick count
FishingBoatDemo::new().with_ticks(200).run();
```

## The Ternary Insight

Every sensor value reduces to a trit: **-1** (below normal), **0** (normal), **+1** (above normal).

```
Engine: 96.3°C  → threshold 95°C  → +1  (overheating)
Bilge:  0.3m    → range [0.1,1.0] →  0  (normal)
Fuel:   10%     → threshold 15%   → -1  (low)
```

Eight sensors = eight trits = **two bytes** packed. An entire room's state fits in a u16. A fleet of 100 rooms = 200 bytes of state. This is why ternary matters: you can reason about massive fleets in pocket-sized data.

## The Fishing Boat

The scenario is based on a real deployment architecture:

```
[ESP32-C3: Engine] ──WiFi──→ [RPi4: Fleet Manager] ──WiFi──→ [ESP32-S3: Backdeck]
   $4.27, 0.2 Hz               $55, 1 Hz                    $6.50, 2 Hz

Total hardware: ~$75
Total cloud: $0
Total intelligence: the captain's
```

This demo simulates exactly what would run on that hardware. The code you're reading is the same logic, just without the TCP layer.

## Tests

36 tests covering every module:

```bash
cargo test
# running 35 tests + 1 doc test
# test result: ok. 36 passed; 0 failed
```

Test coverage includes: scenario phases, ternary threshold/packing/groove computation, counterpoint motion detection, sparkline rendering, trend arrows, alarm evaluation, and full integration (80-tick pipeline).

## Why This Exists

The Plato Matrix is 10 crates, 365 tests, 11K lines of code across 3 languages. That's a lot to absorb. This demo is the on-ramp — the one thing you clone to understand why any of it matters.

The thesis is simple: **rooms persist, agents listen, the boat floats.** Everything else is implementation. This demo proves the thesis in 60 seconds.

## Related

- [PLATO_MASTER_GUIDE](https://github.com/SuperInstance/ai-writings/blob/master/PLATO_MASTER_GUIDE.md) — Complete stack documentation
- [plato-engine-block](https://github.com/SuperInstance/plato-engine-block) — The real room runtime (Rust, 22 tests)
- [plato-engine-block-c](https://github.com/SuperInstance/plato-engine-block-c) — Bare-metal C implementation (35 tests)
- [plato-agent-python](https://github.com/SuperInstance/plato-agent-python) — Python agent framework (55 tests)
- [SuperInstance](https://github.com/SuperInstance/SuperInstance) — The full ecosystem
