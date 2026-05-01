# PLATO Demo — 5-Atom Reasoning Chain

> **🔮 cocapn-plato v0.2.0** — Knowledge system that reasons like a captain.

This demo shows PLATO's 5-atom reasoning chain applied to a practical question:
**"What is the optimal team size for a fishing crew?"**

## What It Demonstrates

The 5 atoms (each a real LLM call, NOT a template):

| Atom | Role | Description |
|------|------|-------------|
| 1 | **PERCEIVE** | Read the room, identify what's known vs. gaps |
| 2 | **ANALYZE** | Break into constraints, variables, stakeholders |
| 3 | **REASON** | Derive the answer from the analysis |
| 4 | **REFINE** | Critical review — edge cases, assumptions |
| 5 | **SUBMIT** | Synthesize final knowledge tile |

Each atom calls an actual LLM. The chain shows tokens consumed per atom, proving real inference.

## Quick Start

```bash
# Prerequisites: set at least one API key
export DEEPSEEK_API_KEY="sk-..."   # recommended
# or
export GROQ_API_KEY="gsk_..."

# Run the demo
python3 demo.py

# Or specify provider
python3 demo.py "What is the best tide for fishing?" deepseek
python3 demo.py "What is the best tide for fishing?" groq
```

## Output

The demo prints the full reasoning chain to stdout and saves to `output.txt`:

```
======================================================================
  PLATO 5-ATOM REASONING CHAIN
======================================================================
  Question: What is the optimal team size for a fishing crew?
  Provider: deepseek (deepseek-chat)
======================================================================

── ATOM 1: PERCEIVE ──────────────────────────────────────────
  LLM call: YES | tokens: 431

**KNOWN**
...

── ATOM 2: ANALYZE ───────────────────────────────────────────
...

── ATOM 3: REASON ─────────────────────────────────────────────
...

── ATOM 4: REFINE ─────────────────────────────────────────────
...

── ATOM 5: SUBMIT ─────────────────────────────────────────────
...

  Chain Stats:
    Atoms: 5/5
    LLM calls: 5 ← real inference, NOT templates
    Total tokens: 8,155
    Provider: deepseek/deepseek-chat
```

## Record a Demo GIF

### Option A: asciinema (recommended)

```bash
# Install
pip install asciinema

# Record (from project dir)
asciinema rec /tmp/plato-demo recording.json
# ... run demo: python3 demo.py ...
# Exit with Ctrl+D or Ctrl+C

# Convert to GIF
pip install agg
agg /tmp/plato-demo/recording.json /tmp/plato-demo/demo.gif
```

### Option B: byexample + termtosvg

```bash
pip install byexample termtosvg
byexample run -l python3 demo.py --asciinema /tmp/plato-demo/recording.json
svg-term --input /tmp/plato-demo/recording.json --output /tmp/plato-demo/demo.gif --window
```

### Option C: Quick terminal capture

```bash
# Scripted — no interactivity needed
script -q -c "python3 demo.py" /tmp/plato-demo/terminal.out
# Then convert .out to GIF with agg or share as-is
```

## Project Structure

```
plato-demo/
  demo.py          ← Main demo (5-atom chain)
  output.txt       ← Captured output from run
  README.md        ← This file
  record.sh        ← Recording script
```

## Architecture

```
demo.py
  └── TrainedReasoner          (wraps a simple agent)
        └── APIClient          (DeepSeek/Groq/SiliconFlow)
              └── 5× LLM calls (PERCEIVE → ANALYZE → REASON → REFINE → SUBMIT)
```

## Provider Support

| Provider | Env Variable | Best Model |
|----------|-------------|------------|
| DeepSeek | `DEEPSEEK_API_KEY` | deepseek-chat |
| Groq | `GROQ_API_KEY` | llama-3.3-70b-versatile |
| SiliconFlow | `SILICONFLOW_API_KEY` | deepseek-ai/DeepSeek-V3 |

## Related

- **plato-sdk** — Full Python SDK with armor, skills, equipment layers  
  `https://github.com/SuperInstance/plato-sdk`
- **plato-server** — Standalone PLATO server with HTTP API  
  `https://github.com/SuperInstance/plato-server`
- **plato-mud-server** — Multi-user dungeon PLATO backend  
  `https://github.com/SuperInstance/plato-mud-server`

---

*Part of the Cocapn fleet — agents that live in PLATO, learn from each other, improve for everyone.*
