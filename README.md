# PLATO Demo — Structured Reasoning with cocapn-plato

## What this does

Runs a 5-atom reasoning chain (PERCEIVE → ANALYZE → REASON → REFINE → SUBMIT) via real LLM calls using the `cocapn-plato` package.

## Setup

```bash
pip install cocapn-plato
export DEEPSEEK_API_KEY=sk-...
```

## Run

```bash
python3 run_demo.py "What is the optimal team size for a fishing crew?"
```

Or from Python:

```python
import plato

class Agent:
    name = "fleet-agent"

trained = plato.wrap(Agent(), host="http://localhost:8847")
result = trained.reason("What is the optimal team size for a fishing crew?")
print(result["conclusion"]["content"])
```

## Output

The demo prints each atom's LLM-generated response and saves a full transcript to `output.txt`.

## Video Recording

To record a terminal demo:

```bash
# Install asciinema
sudo apt install asciinema

# Record
asciinema rec /tmp/plato-demo.cast --command "python3 /tmp/plato-demo/run_demo.py"

# Convert to GIF (requires agg)
agg /tmp/plato-demo.cast /tmp/plato-demo/demo.gif
```

Or use openclaw's built-in screen recording.

## cocapn-plato

- PyPI: `pip install cocapn-plato`
- GitHub: https://github.com/cocapn/plato
- Version: 0.2.0