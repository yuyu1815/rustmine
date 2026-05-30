# Current Evidence

Purpose: record what is proven, what is only observed in the reset-prone
checkout, and what is unknown.

## Proof Rule

```text
observed file exists
  != implemented
  != compatible
  != done
```

Known proof kinds that can upgrade a claim include:

| Proof kind | Meaning |
|---|---|
| `jar_answer` | An official jar or initialized official harness generated the expected answer. |
| `project_test` | A reset-proof project-level test under root passed against the current checkout. |
| `smoke_probe` | A root-owned smoke/probe command reached a named milestone. |
| `visual_probe` | A root-owned render/screenshot/pixel proof exists for a render claim. |
| `manual_observation` | Explicitly marked human observation; never enough for official equivalence by itself. |

This list is extendable. Add a named proof kind here when a new evidence family
has a durable artifact, clear owner, and stop boundary.

Helper output is scoped evidence only. For example, `PASS: answer regenerated`
or `PASS: manifest consumed` can support the named case and artifact link, but
does not upgrade a wider compatibility or client-load claim without the matching
proof kind and domain shard entry.

## Index

| Need | Location |
|---|---|
| Client-load proof state | [client-load.md](client-load.md) |
| Structural scan rule and command | [structural-scan.md](structural-scan.md) |
| Client-load phase lens | [../client-load/README.md](../client-load/README.md) |
