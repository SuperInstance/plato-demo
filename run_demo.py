#!/usr/bin/env python3
"""PLATO Demo - runs 5-atom reasoning chain via DeepSeek."""
import json, os, sys, urllib.request

PROVIDERS = {
    "deepseek": {"base_url": "https://api.deepseek.com/v1", "env_key": "DEEPSEEK_API_KEY", "model": "deepseek-chat"},
}

def chat(client, system, message, temp=0.7):
    payload = {
        "model": client["model"],
        "messages": [{"role": "system", "content": system}, {"role": "user", "content": message}],
        "temperature": temp,
        "max_tokens": 1500
    }
    req = urllib.request.Request(
        f"{client['base_url']}/chat/completions",
        data=json.dumps(payload).encode(),
        headers={"Content-Type": "application/json", "Authorization": f"Bearer {client['api_key']}"},
        method="POST"
    )
    resp = urllib.request.urlopen(req, timeout=120)
    data = json.loads(resp.read())
    return data["choices"][0]["message"]["content"], data.get("usage", {})

def run(question, provider="deepseek"):
    client_cfg = PROVIDERS[provider]
    api_key = os.environ.get(client_cfg["env_key"], "")
    if not api_key:
        print(f"ERROR: {client_cfg['env_key']} not set")
        sys.exit(1)
    client = {"base_url": client_cfg["base_url"], "model": client_cfg["model"], "api_key": api_key}

    print(f"\n{'='*60}")
    print(f"  PLATO 5-ATOM REASONING CHAIN")
    print(f"{'='*60}")
    print(f"  Question: {question}")
    print(f"  Provider: {provider} ({client['model']})")
    print(f"{'='*60}\n")

    atoms = []
    prompts = [
        ("PERCEIVE", "You are a perceptive analyst. Identify what's known and gaps. Be concise."),
        ("ANALYZE", "Break the question into constraints, variables, stakeholders, tradeoffs. Be specific."),
        ("REASON", "Derive a specific, actionable answer. Give numbers and conditions."),
        ("REFINE", "Critically review. Find edge cases, assumptions, oversimplifications."),
        ("SUBMIT", "Synthesize into a final answer. Specific, practical, condition-aware."),
    ]

    context = ""
    for i, (name, sys_prompt) in enumerate(prompts, 1):
        if i == 1:
            msg = f"Question: \"{question}\"\nWhat helps answer this?"
        else:
            msg = f"Question: \"{question}\"\n\nPrevious:\n{context}\n\nYour analysis:"
        resp, usage = chat(client, sys_prompt, msg)
        atoms.append({"atom": i, "name": name, "response": resp, "tokens": usage.get("total_tokens", 0)})
        context += f"\n[{name}] {resp}"
        print(f"ATOM {i}: {name}")
        print(resp[:300])
        print()

    print(f"{'='*60}")
    print(f"  LLM calls: 5/5 (real inference, not templates)")
    print(f"  Total tokens: {sum(a['tokens'] for a in atoms):,}")
    print(f"{'='*60}\n")

    with open("/tmp/plato-demo/output.txt", "w") as f:
        for atom in atoms:
            f.write(f"\n{'='*60}\n")
            f.write(f"  ATOM {atom['atom']}: {atom['name']}\n")
            f.write(f"{'='*60}\n")
            f.write(atom["response"] + "\n")
    print("Output saved to /tmp/plato-demo/output.txt")

if __name__ == "__main__":
    q = sys.argv[1] if len(sys.argv) > 1 else "What is the optimal team size for a fishing crew?"
    run(q)