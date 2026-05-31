# Agent Operations

Purpose: record ownership decisions for AI operating surfaces. This is shared
analysis memory, not the startup manual and not the next-task card.

## Map

```text
AGENTS.md
  -> CONTEXT.md project vocabulary
    -> docs/ai/ fixed startup and routing map
      -> docs/next/ compact recovery and next-task state
        -> docs/analysis/* AI-shared memory, evidence, and decisions
          -> oracle/* machine-readable evidence
```

## Responsibilities

| Surface | Owns | Must not own |
|---|---|---|
| `CONTEXT.md` | project glossary, preferred terms, short term boundaries, resolved vocabulary introduced or sharpened by docs work | workflow, proof status, evidence histories, implementation details, current task state |
| `docs/ai/` | fixed startup route, low-token reading map, stable safety posture | mutable current location, next action, packet facts, proof status, long evidence histories |
| `docs/next/` | compact mutable recovery state: current location, immediate next action, blocker, stop boundary | durable evidence, proof ledgers, protocol facts, long analysis |
| `.codex/skills/` | stable procedures, schemas, role contracts; Yuzu is a read-only operator lens | mutable proof status, current phase completion facts, glossary terms, task state |
| `.codex/agents/` | Codex app/CLI project-scoped subagent role definitions | canonical oracle facts or implementation changes |
| `docs/analysis/` shards | AI-shared memory: mutable human-readable evidence, responsibility, uncertainty, decisions, and traceability | generated official answers, immediate next-task recovery |

## Subagent Boundary

The parent Codex instance remains the operator-facing collaborator. Subagents
may do bounded work, but they do not own the conversation.

```text
operator question
  -> parent Codex
    -> optional subagent task with explicit scope
      -> subagent returns scoped evidence or a proposed patch/result
        -> parent Codex verifies, updates recovery pointers, and answers user
```

| Surface | Owns | Must not own |
|---|---|---|
| Parent Codex | user questions, route decision, final answer, recovery pointer, compaction-safe handoff state | pretending helper output is proof without verification |
| Subagent | bounded extraction, review, mapping, or implementation task named by parent | final user response, full conversation state, route ownership, durable recovery memory |

Use subagents when they reduce bounded uncertainty. Do not use them as a full
delegation mechanism for the active conversation, because compaction recovery
depends on the parent preserving the route and next action in `docs/next/`.

## Rationale

Keeping startup rules, next-task state, and shared evidence in separate places
prevents fresh agents from treating long evidence history as required startup
context.

| Misplaced content | Failure mode |
|---|---|
| Evidence change not recorded in the owning shard | A later agent re-reads or re-interprets jars, probes, or smoke evidence instead of using the reusable proof trail. |
| New area has no index route | The work becomes invisible after compaction, handoff, or fresh session recovery. |
| Mutable facts in `docs/ai/` | Future agents fixate on fixed startup docs and bypass the canonical owner. |
| Long proof ledgers in `docs/next/` | Startup becomes expensive and the next action is hidden inside evidence history. |
| Project terms left only in chat | A later agent repeats the terminology debate or uses the wrong boundary. |
| Uncertainty missing from the owning shard | Unknown or partial work is mistaken for completion. |

Earlier operational guidance for update destinations, route hygiene, and startup
token budget moved to `docs/ai/README.md`, because those are fixed AI startup
rules rather than analysis memory.

## Historical Note

Completed cleanup efforts, including prior `docs/ai/` orientation cleanup, are
historical tasks, not standing responsibility areas. Similar topology work still
belongs to Agent operations, but the next action itself belongs in
`docs/next/README.md`.

## Current Topology

| Domain | Canonical shard |
|---|---|
| Project vocabulary | root `CONTEXT.md`; glossary only, not workflow or proof state |
| `docs/ai/` startup | `docs/ai/README.md` fixed low-token route map; `docs/ai/00-RESUME.md` is a compatibility pointer to `docs/next/README.md` |
| Next task recovery | `docs/next/README.md`; compact current location, next action, blocker, and stop boundary only |
| Client-load phase lens | `docs/analysis/client-load/README.md` |
| Current evidence | `docs/analysis/current-evidence/README.md` |
| Protocol version traceability | `docs/analysis/protocol/versions/775/traceability.md` for the current populated 775 shard |
| Responsibility | `docs/analysis/responsibility/README.md` |
