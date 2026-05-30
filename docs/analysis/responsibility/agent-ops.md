# Agent Operations

Purpose: separate vocabulary, orientation routes, fixed workflows, agent roles,
and mutable evidence.

## Map

```text
AGENTS.md
  -> CONTEXT.md project vocabulary
    -> docs/ai/ orientation and recovery pointers
      -> .codex/skills/ fixed workflows and read-only lenses
        -> docs/analysis/* mutable domain facts
          -> oracle/* machine-readable evidence
```

## Responsibilities

| Surface | Owns | Must not own |
|---|---|---|
| `CONTEXT.md` | project glossary, preferred terms, short term boundaries, resolved vocabulary introduced or sharpened by docs work | workflow, proof status, evidence histories, implementation details, current task state |
| `docs/ai/` | current location, next action, route pointers, recovery pointer | fixed workflow, packet facts, proof status, long evidence histories |
| `.codex/skills/` | stable procedures, schemas, role contracts; Yuzu is a read-only operator lens | mutable proof status, current phase completion facts, glossary terms, task state |
| `.codex/agents/` | Codex app/CLI project-scoped subagent role definitions | canonical oracle facts or implementation changes |
| `docs/analysis/` shards | mutable human-readable evidence, responsibility, and traceability | generated official answers |

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
depends on the parent preserving the route and next action in `docs/ai/`.

## Glossary Updates

`CONTEXT.md` owns resolved project vocabulary. When documentation work
introduces, sharpens, or repeatedly depends on a project term, update
`CONTEXT.md` with the stable meaning and a short boundary note.

```text
new or sharpened term
  -> resolve meaning in the owning docs work
    -> update CONTEXT.md only when it is durable project language
      -> route evidence, workflow, or current state to the owning shard instead
```

Do not store new terms in Yuzu. Yuzu is only the operator mirror used to check
whether the route, evidence posture, and responsibility boundary feel right.

## Living Document Desire Paths

Docs are updated to preserve recoverability, evidence traceability, and future
routing. They are not updated to satisfy ceremony.

Use the document that naturally owns the knowledge:

```text
durable fact changed
  -> update the smallest owning domain shard

durable vocabulary changed
  -> update CONTEXT.md

new durable area appeared
  -> add a shard or index route so future agents can find it

only current location, next action, or recovery route changed
  -> update docs/ai/00-RESUME.md

nothing durable changed
  -> do not write docs
```

## Route And Name Maintenance

Names are routes. When a file or directory name no longer reflects the current
concept:

1. Check references.
2. Rename or delete the stale route.
3. Prefer canonical paths that express domain shape, such as
   `protocol/versions/<version>/...`.
4. Keep compatibility pointers only when they prevent real breakage; otherwise
   remove them before they train AI toward the old model.

Consequence examples:

| Missed update | Future failure |
|---|---|
| Evidence change not recorded in the owning shard | A later agent re-reads or re-interprets jars, probes, or smoke evidence instead of using the reusable proof trail. |
| New area has no index route | The work becomes invisible after compaction, handoff, or fresh session recovery. |
| Durable facts are stuffed into `docs/ai/` | Future agents fixate on orientation notes and bypass the canonical owner. |
| Project terms are left only in chat | A later agent repeats the terminology debate or uses the wrong boundary. |
| Uncertainty is not documented where the claim lives | Unknown or partial work is mistaken for completion. |

## Recovery Note

Completed cleanup efforts, including prior `docs/ai/` orientation cleanup, are
historical tasks, not standing responsibility areas. If similar topology work
returns, route it through Agent operations and the responsibility index before
editing.

## Current Topology

| Domain | Canonical shard |
|---|---|
| Project vocabulary | root `CONTEXT.md`; glossary only, not workflow or proof state |
| `docs/ai/` orientation | `docs/ai/README.md` route map and `docs/ai/00-RESUME.md` recovery pointer only; add another file only if it carries real orientation value beyond paths |
| Client-load phase lens | `docs/analysis/client-load/README.md` |
| Current evidence | `docs/analysis/current-evidence/README.md` |
| Protocol version traceability | `docs/analysis/protocol/versions/775/traceability.md` for the current populated 775 shard |
| Responsibility | `docs/analysis/responsibility/README.md` |
