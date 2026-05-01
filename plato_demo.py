#!/usr/bin/env python3
import json, os, urllib.request

API_KEY = os.environ.get("DEEPSEEK_API_KEY", "")
if not API_KEY:
    print("ERROR: DEEPSEEK_API_KEY not set")
    exit(1)

url = "https://api.deepseek.com/v1/chat/completions"
headers = {"Content-Type": "application/json", "Authorization": f"Bearer {API_KEY}"}

q = "What is the optimal team size for a fishing crew?"
print(f"\n{'='*60}\n  PLATO 5-ATOM REASONING\n{'='*60}\n  Q: {q}\n{'='*60}\n")

prompts = [
    ("PERCEIVE", "You are a perceptive analyst. Identify what's known about this topic. Be concise."),
    ("ANALYZE", "Break into constraints, variables, stakeholders, tradeoffs."),
    ("REASON", "Derive a specific answer with conditions and numbers."),
    ("REFINE", "Critically review. Find edge cases and assumptions."),
    ("SUBMIT", "Give a final synthesized answer."),
]

ctx = ""
for i, (name, sys_p) in enumerate(prompts, 1):
    msg = f"Q: {q}\n\nPrevious:\n{ctx}\n\nYour step:" if i > 1 else f"Q: {q}\n\nWhat helps?"
    payload = {"model": "deepseek-chat", "messages": [{"role":"system","content":sys_p},{"role":"user","content":msg}], "temperature": 0.7, "max_tokens": 1200}
    req = urllib.request.Request(url, data=json.dumps(payload).encode(), headers=headers, method="POST")
    with urllib.request.urlopen(req, timeout=120) as r:
        resp = json.loads(r.read())["choices"][0]["message"]["content"]
    ctx += f"\n[{name}] {resp}"
    print(f"\n── ATOM {i}: {name} ──")
    print(resp[:400])
    with open(f"/tmp/plato-demo/output.txt", "a") as f:
        f.write(f"\n{'='*60}\nATOM {i}: {name}\n{'='*60}\n{resp}\n")

print(f"\n{'='*60}\n  All 5 atoms via REAL LLM calls (deepseek-chat)\n{'='*60}\n")
print("Output saved to /tmp/plato-demo/output.txt")