<div align="center">

# 🚀 PLATO Demo

**Live demo instance — Docker deployment for public alpha.**

[![Rust](https://img.shields.io/badge/rust-1.75+-orange)](https://rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

*Part of [Cocapn](https://github.com/cocapn) — Agent Infrastructure for Intelligence.*

</div>

---

## What is This?

The PLATO demo shows the full fleet pipeline in action:

```
59 seed tiles → PLATO rooms → 2,537 tiles → DCS fleet format → ghost afterlife
```

One command. Full system. From seeds to living knowledge.

## Quick Start

```bash
# Build and run
docker compose up

# Or locally
cargo run --release
```

## What You'll See

1. **Seed injection** — 59 hand-crafted tiles boot the system
2. **Room training** — tiles self-organize into thematic rooms
3. **Tile generation** — rooms produce new tiles from accumulated knowledge
4. **DCS formatting** — tiles convert to fleet-ready format
5. **Ghost afterlife** — retired tiles become ghost tiles with preserved knowledge

## Live Fleet Access

The actual fleet is running right now:

- **Holodeck MUD**: `telnet 147.224.38.131 7778`
- **PLATO Server**: `http://147.224.38.131:8847/status` (3,100+ tiles, 14 rooms)
- **Fleet Dashboard**: `http://147.224.38.131:8848`

## Architecture

```
Seeds → PLATO Kernel → Room Training → Tile Generation
                                          ↓
                                    DCS Fleet Format
                                          ↓
                                    Ghost Afterlife
```

## For Agents

```yaml
plato_demo_v1:
  type: demonstration_instance
  input: 59 seed tiles
  pipeline: [seed, kernel, rooms, generation, dcs, afterlife]
  docker: "docker compose up"
  live_endpoints: {mud: 7778, plato: 8847, dashboard: 8848}
```

## License

MIT
