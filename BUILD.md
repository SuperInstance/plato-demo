# BUILD plato-demo — The HN Demo Binary

## What to Build
A SINGLE binary that a Hacker News reader can:
1. `cargo build --release` (no GPU, no apt install, no API keys)
2. `./plato-demo` and watch the fleet come alive in their terminal

## What It Demonstrates (in ~55 seconds on CPU)
1. **Tile Forge**: 59 seed tiles expand to 2,501+ tiles in real-time
2. **DCS Fleet**: Specialist agents outperform generalist baseline (5.88× ratio)
3. **Trust-Routed Messaging**: Messages flow through trust-weighted BFS
4. **Belief-Filtered Deployment**: Tiles classified into Live/Monitored/HumanGated tiers
5. **Ghost Tile Afterlife**: Dead tiles fade, relevant ones resurrect

## Output Format
Clean terminal output with:
- Progress bars or tick counters
- Ratio assertions printed live (5.88× specialist, 21.87× generalist, 880:1 compression)
- Tier distribution at the end
- Total time elapsed
- Every number comes from actual computation, not sleep+println

## Architecture
- This is an integration demo crate that depends on plato-dcs, plato-forge-pipeline, plato-relay-tidepool, plato-deploy-policy, plato-unified-belief, plato-afterlife
- Each demo phase calls into the real crate APIs
- NO mocking — use the actual implementations
- Since these crates are on GitHub, add them as git dependencies with the SuperInstance URLs

## Dependencies (all from SuperInstance, zero external deps)
- plato-dcs (DCS engine)
- plato-forge-pipeline (tile forge)
- plato-unified-belief (belief scoring)
- plato-deploy-policy (tier classification)
- plato-relay-tidepool (trust routing)
- plato-afterlife (ghost tiles)

## Cargo.toml
```toml
[package]
name = "plato-demo"
version = "0.1.0"
edition = "2021"
description = "The PLATO HN Demo — 59 seeds → 2,501 tiles → 4 ensigns in 55s on CPU"

[dependencies]
plato-dcs = { git = "https://github.com/SuperInstance/plato-dcs.git" }
plato-forge-pipeline = { git = "https://github.com/SuperInstance/plato-forge-pipeline.git" }
plato-unified-belief = { git = "https://github.com/SuperInstance/plato-unified-belief.git" }
plato-deploy-policy = { git = "https://github.com/SuperInstance/plato-deploy-policy.git" }
plato-relay-tidepool = { git = "https://github.com/SuperInstance/plato-relay-tidepool.git" }
plato-afterlife = { git = "https://github.com/SuperInstance/plato-afterlife.git" }
```

## Quality Requirements
- This is Casey's "download that makes people go wow"
- Clean, readable output — someone should understand what's happening without reading docs
- Every ratio is computed live and printed
- Error handling: if a dependency crate API doesn't match, provide a helpful error message
- The binary should work on first try after `cargo build`

BUILD IT NOW. Write Cargo.toml, src/main.rs with all 5 demo phases. 
Use the actual crate APIs — read their source if needed.
Make the terminal output beautiful and informative.
Push to GitHub when it compiles.
