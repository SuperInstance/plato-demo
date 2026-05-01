#!/usr/bin/env python3
"""PLATO Demo runner — runs the 5-atom reasoning chain."""
import json, os, sys, time, urllib.request, urllib.error

PROVIDERS = {
    "deepseek": {"base_url": "https://api.deepseek.com/v1", "env_key": "DEEPSEEK_API_KEY", "model": "deepseek-chat"},
    "groq": {"base_url": "https://api.groq.com/openai/v1", "env_key": "GROQ_API_KEY", "model": "llama-3.3-70b-versatile"},
    "siliconflow": {"base_url": "https://api.siliconflow.com/v1", "env_key": "SILICONFLOW_API_KEY", "model": "deepseek-ai/DeepSeek-V3"},
}

def chat(client, system, message, temp=0.7):
    payload = {"model": client["model"], "messages": [{"role": "system", "content": system}, {"role": "user", "content": message}], "temperature": temp, "max_tokens": 1500}
    req = urllib.request.Request(f"{client['base_url']}/chat/completions", data=json.dumps(payload).encode(), method="POST")
    req.add_header("Content-Type", "application/json")
    req.add_header("User-Agent", "plato-demo/1.0")
    req.add_header("Authorization", f"Bearer {client['api_key']}")
    resp = urllib.request.urlopen(req, timeout=120)
    data = json.loads(resp.read())
    return data["choices"][0]["message"]["content"], data.get("usage", {})

def run(question, provider):
    client_cfg = PROVIDERS[provider]
    api_key = os.environ.get(client_cfg["env_key"], "")
    if not api_key:
        print(f"ERROR: {client_cfg['env_key']} not set")
        sys.exit(1)
    client = {"base_url": client_cfg["base_url"], "model": client_cfg["model"], "api_key": api_key}

    print(f"\n{'='*70}")
    print(f"  PLATO 5-ATOM REASONING CHAIN")
    print(f"{'='*70}")
    print(f"  Question: {question}")
    print(f"  Provider: {provider} ({client['model']})")
    print(f"{'='*70}\n")

    atoms = []
    total_tokens = 0

    # ATOM 1: PERCEIVE
    s1 = "You are a perceptive analyst. Identify what's known and what gaps exist. If no data, say so. Never hallucinate."
    m1 = f"Room: 'fishing-crew'\nCaptain asks: \"{question}\"\nWhat existing knowledge helps? List KNOWN, GAPS, BIASES."
    r1, u1 = chat(client, s1, m1)
    atoms.append({"atom": 1, "name": "PERCEIVE", "response": r1, "llm_call": True, "tokens": u1})
    total_tokens += u1.get("total_tokens", 0)
    print(f"\n── ATOM 1: PERCEIVE ──────────────────────────────────────────")
    print(f"  LLM call: YES | tokens: {u1.get('total_tokens', '?')}")
    print(f"\n{r1}\n")

    # ATOM 2: ANALYZE
    s2 = "Break complex questions into constraints, variables, stakeholders, tradeoffs, edge cases. Be specific."
    m2 = f"Question: \"{question}\"\n\nRoom context:\n{r1}\n\nBreak down into: CONSTRAINTS, VARIABLES, STAKEHOLDERS, TRADEOFFS, EDGE CASES"
    r2, u2 = chat(client, s2, m2)
    atoms.append({"atom": 2, "name": "ANALYZE", "response": r2, "llm_call": True, "tokens": u2})
    total_tokens += u2.get("total_tokens", 0)
    print(f"── ATOM 2: ANALYZE ───────────────────────────────────────────")
    print(f"  LLM call: YES | tokens: {u2.get('total_tokens', '?')}")
    print(f"\n{r2}\n")

    # ATOM 3: REASON
    s3 = "Given constraints and patterns, derive a specific, actionable answer. Give numbers and conditions. Be direct."
    m3 = f"Question: \"{question}\"\n\nKnown:\n{r1}\n\nConstraints:\n{r2}\n\nDerive the optimal answer with specific numbers and conditions."
    r3, u3 = chat(client, s3, m3)
    atoms.append({"atom": 3, "name": "REASON", "response": r3, "llm_call": True, "tokens": u3})
    total_tokens += u3.get("total_tokens", 0)
    print(f"── ATOM 3: REASON ─────────────────────────────────────────────")
    print(f"  LLM call: YES | tokens: {u3.get('total_tokens', '?')}")
    print(f"\n{r3}\n")

    # ATOM 4: REFINE
    s4 = "Critically review the answer. Find edge cases, assumptions, oversimplifications. Be constructively critical."
    m4 = f"Original: \"{question}\"\n\nProposed answer:\n{r3}\n\nCritically review: edge cases? assumptions? too vague? missing exceptions?"
    r4, u4 = chat(client, s4, m4)
    atoms.append({"atom": 4, "name": "REFINE", "response": r4, "llm_call": True, "tokens": u4})
    total_tokens += u4.get("total_tokens", 0)
    print(f"── ATOM 4: REFINE ─────────────────────────────────────────────")
    print(f"  LLM call: YES | tokens: {u4.get('total_tokens', '?')}")
    print(f"\n{r4}\n")

    # ATOM 5: SUBMIT
    s5 = "Synthesize into a final knowledge tile. Specific, practical, condition-aware, no absolutes."
    m5 = f"Question: \"{question}\"\n\nCritical review:\n{r4}\n\nFinal synthesized answer (knowledge tile):"
    r5, u5 = chat(client, s5, m5)
    atoms.append({"atom": 5, "name": "SUBMIT", "response": r5, "llm_call": True, "tokens": u5})
    total_tokens += u5.get("total_tokens", 0)
    print(f"── ATOM 5: SUBMIT ─────────────────────────────────────────────")
    print(f"  LLM call: YES | tokens: {u5.get('total_tokens', '?')}")
    print(f"\n{r5}\n")

    print(f"{'='*70}")
    print(f"  FINAL ANSWER")
    print(f"{'='*70}")
    print(f"\n  {r5}\n")
    print(f"  Chain Stats:")
    print(f"    Atoms: 5/5")
    print(f"    LLM calls: 5 ← real inference, NOT templates")
    print(f"    Total tokens: {total_tokens:,}")
    print(f"    Provider: {provider}/{client['model']}")
    print(f"{'='*70}\n")

    return {"question": question, "answer": r5, "chain": atoms, "total_tokens": total_tokens}

if __name__ == "__main__":
    q = sys.argv[1] if len(sys.argv) > 1 else "What is the optimal team size for a fishing crew?"
    p = sys.argv[2] if len(sys.argv) > 2 else "deepseek"
    output_file = sys.argv[3] if len(sys.argv) > 3 else "/tmp/plato-demo/output.txt"
    result = run(q, p)
    os.makedirs(os.path.dirname(output_file), exist_ok=True)
    with open(output_file, "w") as f:
        for atom in result["chain"]:
            f.write(f"\n{'='*70}\n")
            f.write(f"  ATOM {atom['atom']}: {atom['name']}\n")
            f.write(f"{'='*70}\n")
            f.write(atom["response"] + "\n")
        f.write(f"\n{'='*70}\n")
        f.write(f"  FINAL ANSWER: {result['answer']}\n")
        f.write(f"  Total tokens: {result['total_tokens']:,}\n")
        f.write(f"{'='*70}\n")
    print(f"\n  Output saved to: {output_file}")