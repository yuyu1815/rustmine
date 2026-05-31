# Agent Operations

Purpose: record ownership decisions for AI operating surfaces. This is shared
analysis memory, not the startup manual and not the next-task card.

## Responsibilities

| Surface | Owns | Must not own |
|---|---|---|
| `CONTEXT.md` | project glossary, preferred terms, short term boundaries, resolved vocabulary introduced or sharpened by docs work | operating instructions, proof status, evidence histories, implementation details, current task state |
| `docs/ai/` | fixed startup route, low-token reading map, stable safety posture, parent-side subagent operation rules | mutable current location, next action, packet facts, proof status, long evidence histories |
| `docs/next/` | compact mutable recovery state: current location, immediate next action, blocker, stop boundary | durable evidence, proof ledgers, protocol facts, long analysis |
| `.codex/skills/` | stable instructions, schemas, role contracts; Yuzu is a read-only operator lens | mutable proof status, current phase completion facts, glossary terms, task state |
| `.codex/agents/` | Codex app/CLI project-scoped subagent role definitions and role-specific runtime rules | canonical oracle facts or implementation changes |
| `docs/analysis/` shards | AI-shared memory: mutable human-readable evidence, responsibility, uncertainty, decisions, and traceability | generated official answers, immediate next-task recovery |

## Subagent Ownership

The parent Codex instance remains the operator-facing collaborator. Subagents
may do bounded work, but they do not own the conversation.

| Surface | Owns | Must not own |
|---|---|---|
| Parent Codex | user questions, route decision, final answer, recovery pointer, compaction-safe handoff state | pretending helper output is proof without verification |
| Subagent | bounded extraction, review, mapping, or implementation task named by parent | final user response, full conversation state, route ownership, durable recovery memory |

This file does not decide when to spawn subagents. That fixed parent-side
guidance belongs in `docs/ai/agent-ops.md`.

## Runtime Placement

The standing subagent rules belong where they are read before use:

| Instruction type | Owner |
|---|---|
| Parent-side spawn, nesting, return, and wait rules | `docs/ai/agent-ops.md` |
| Role-specific subagent behavior | `.codex/agents/*.toml` |
| Ownership decision and failure evidence | this file |

Do not make this responsibility shard the operational manual. It records the
ownership decision and failure evidence so the startup and agent-prompt surfaces
can stay short.

## 2026-05-31 Loop Evidence

Session `019e7a17-7b2c-74e3-8762-9135ebb39353` is the current failure example
for agent topology. It ran from `2026-05-30T18:13:45Z` to
`2026-05-31T15:37:38Z`, about 21 hours.

| Pattern element | Observed effect |
|---|---|
| Wide goal | Parent decision did not converge before delegation expanded. |
| Many `spawn_agent` calls | Exploration and execution split across too many helpers. |
| Many `wait_agent` calls | Parent work became supervision and wall-clock waiting. |
| Repeated Lead / Route / Scope regeneration | Same evidence was summarized and reclassified repeatedly. |
| Large helper outputs returned to parent | Parent context filled with helper detail instead of durable state. |
| Automatic next-packet selection | The loop continued without a fresh parent stop decision. |

| Surface | Evidence | Failure mode |
|---|---:|---|
| Parent `spawn_agent` | 114 calls | Exploration and execution were expanded before the parent converged the route. |
| Parent `wait_agent` | 279 calls | Parent work became supervision and wall-clock waiting. |
| `wait_agent(timeout_ms=300000)` | 210 calls | Five-minute waits repeated as a control loop. |
| `rustmine_compatibility_lead` children | 54 sessions | Planner role became repeated implementation/coordination work. |
| Lead / Route / Scope cycles | 18 cycles | Same evidence was repeatedly summarized and reclassified. |
| Function outputs in parent | 1,843 outputs / about 5.5 MB | Helper output bulk flowed back into the parent context. |

## Current Guardrails

The topology now has both a depth fence and a width fence.

```text
Parent Codex
  -> max one Lead/planner for the current decision
    -> max two nested leaf workers in one batch
      -> parent checks after-before status paths against allowed_writes
        -> parent chooses stop / next batch / final answer
```

| Prior risk | Guardrail |
|---|---|
| Capsule text drifted away from machine schema | `.codex/skills/stevenarella-oracle-workbench/schemas/context-capsule.schema.json` is the `context_capsule` schema. |
| Depth was limited but width could still fan out | `.codex/config.toml` sets `max_threads = 3`, interpreted as one Lead plus at most two leaves; `context_capsule.batch.leaf_count` is capped at 2. |
| Workspace-write leaves relied on prompt discipline | Every capsule must set the standard `write_policy.diff_baseline` using `git status --porcelain=v1 --untracked-files=all -- .`; parent verifies only `after - before` status paths against `allowed_writes`. |
| Rust leaf write scopes could split | The capsule validator can take a rust-fix task and rejects `allowed_write_scope` outside capsule `allowed_writes`; Rust leaves may edit only the intersection. |
| Lead output implied verification | Lead returns `reported_checks`; final proof remains Parent Codex responsibility. |
| Model lanes and agent definitions drifted | Rust, oracle-workbench, and review agents now declare the matching model lane in their TOML. |
| Parent-facing and nested roles could drift | The shared delegation contract is the `context_capsule` schema; agent TOML adds only role-specific behavior. |
| Direct parent-facing workspace-write agents bypassed capsules | Parent direct delegation must still name write scope, take before/after status snapshots, and reject new or changed status paths outside that scope. |
| Parent-facing workers repeated broad startup reads | Direct workers can now receive a validated `worker-capsule/v1` with startup context, allowed reads/writes, checks, stop boundary, and return contract; worker role TOML treats the capsule as startup context when supplied. |
| Documentation updates consumed planner tokens | `rustmine_nested_docs_rewriter` is the bounded typist/editor leaf for supplied wording; Parent/Lead checks only transcription mistakes, formatting, link/path breakage, duplicate or missing rows, and scope drift after the write. |

The important split is:

| Cause owner | Runtime cost | Token cost |
|---|---|---|
| Parent | Broad goal, too many spawns, repeated waits | Huge handoffs, helper outputs imported into chat |
| Subagents | Long-running Lead/Oracle workers, repeated planner cycles | High-capacity model lanes used for execution-like work |

## Rationale

Keeping startup rules, next-task state, and shared evidence in separate places
prevents fresh agents from treating long evidence history as required startup
context.

| Misplaced content | Failure mode |
|---|---|
| Evidence change not recorded in the owning shard | A later agent re-reads or re-interprets jars, probes, or smoke evidence instead of using the reusable proof record. |
| New area has no index route | The work becomes invisible after compaction, handoff, or fresh session recovery. |
| Mutable facts in `docs/ai/` | Future agents fixate on fixed startup docs and bypass the canonical owner. |
| Long proof ledgers in `docs/next/` | Startup becomes expensive and the next action is hidden inside evidence history. |
| Project terms left only in chat | A later agent repeats the terminology debate or uses the wrong boundary. |
| Uncertainty missing from the owning shard | Unknown or partial work is mistaken for completion. |

Fixed guidance for update destinations, route hygiene, startup token
budget, and subagent topology lives in `docs/ai/`, because those are fixed AI
startup rules rather than analysis memory.

## Historical Note

Completed cleanup efforts, including prior `docs/ai/` orientation cleanup, are
historical tasks, not standing responsibility areas. Similar topology work still
belongs to Agent operations, but the next action itself belongs in
`docs/next/README.md`.

## Current Topology

| Domain | Canonical shard |
|---|---|
| Project vocabulary | root `CONTEXT.md`; glossary only, not operating instructions or proof state |
| `docs/ai/` startup | `docs/ai/README.md` fixed low-token route map; `docs/ai/agent-ops.md` fixed subagent operation map; `docs/ai/00-RESUME.md` is a compatibility pointer to `docs/next/README.md` |
| Next task recovery | `docs/next/README.md`; compact current location, next action, blocker, and stop boundary only |
| Client-load phase lens | `docs/analysis/client-load/README.md` |
| Current evidence | `docs/analysis/current-evidence/README.md` |
| Protocol version traceability | `docs/analysis/protocol/versions/775/traceability.md` for the current populated 775 shard |
| Responsibility | `docs/analysis/responsibility/README.md` |
