# Agent Operations Startup

Purpose: fixed startup gate for parent/subagent posture. Read this at the start
of every AI turn before deciding whether parent Codex should work alone or
delegate a bounded packet. This file is not evidence memory and not task
recovery state.

## Placement Map

```text
parent/subagent posture before task routing
  -> docs/ai/agent-ops.md

role-specific behavior inside a spawned agent
  -> .codex/agents/*.toml

planner-to-leaf nested behavior
  -> .codex/agents/rustmine-nested-*.toml

why this topology exists / failure evidence
  -> docs/analysis/responsibility/agent-ops.md

current task location / next action / blocker
  -> docs/next/README.md
```

## Default Shape

Do not use subagents as exploratory memory. Use them as a controlled
decomposition tree when the active task is too large for one context window or
benefits from parallel evidence gathering. The parent Codex owns route choice,
recovery state, and the final answer.

```text
User
  -> Parent Codex
    -> high-capacity planner / mapper for large or ambiguous task shape
      -> rustmine_nested_* leaf workers for bounded artifacts, patches, maps, reviews, or blockers
        -> Parent Codex verifies, compresses, and updates recovery state
```

For small tasks, skip the planner and leaf workers. Parent-only work is the
default only when the task already fits one owner, one evidence surface, and one
verification loop.

## Delegation Gate

Before spawning a subagent, the parent must be able to fill this capsule:

```text
context_capsule:
  objective:
  allowed_reads:
  allowed_writes:
  required_evidence:
  exit_condition:
  stop_boundary:
```

Delegate when the work is genuinely large, parallel, ambiguous, or needs
separate evidence surfaces. Do not spawn a subagent for a file read, a simple
route lookup, or durable memory storage.

## Cost And Model Gradient

Use stronger models at the top of the tree for product direction,
responsibility boundaries, decomposition, and ambiguous compatibility judgment.
Push execution into the smallest capable lane.

| Layer | Model posture | Best use |
|---|---|---|
| Parent Codex | strongest judgment when needed | user intent, final route, recovery, final answer |
| Planner / mapper | strong but short-lived | split the task, choose lanes, create context capsules |
| Nested leaf worker | smallest capable model | execute one artifact, patch, test, map, review, or blocker from a capsule |

The planner should reduce cost by moving bounded execution to cheaper lanes. It
must not become a general execution pool.

## Middle Reasoning Budget

The planner is the middle layer. It may think enough to split the task, but it
must not turn its reasoning into parent context.

| Middle output | Rule |
|---|---|
| Reasoning traces | Do not return to Parent Codex. |
| Leaf prompts or raw leaf logs | Do not return to Parent Codex. |
| Rejected branches | Omit unless they are the blocker. |
| Work-package map | Keep internal unless the parent asked only for a plan. |
| Parent result | Return status, changed artifacts, proof/check status, blocker, and next step only. |

## Granularity Gate

Nested work is useful only at reviewable boundaries.

| Granularity | Decision |
|---|---|
| Too large | Split before execution; the result would mix multiple owners, phases, or proof surfaces. |
| Good | One worker can return one artifact, patch, test result, map, review, or blocker that the parent can verify. |
| Too small | Keep in parent or merge into a neighboring packet; the result would be only a file read, rename, or isolated fact. |

If a leaf discovers that its packet is still too large, it should stop with a
proposed split and blocker. It should not create another planner chain.

## Nesting Boundary

Project agent nesting is configured as `max_depth = 2`, meaning parent Codex
may use one planner layer and that planner may use `rustmine_nested_*` leaf
workers.

Allowed:

```text
Parent Codex
  -> parent-facing planner / mapper
    -> rustmine_nested_* leaf workers
```

Forbidden:

```text
Parent Codex
  -> Lead
    -> Route
      -> Scope
        -> worker
```

Planner agents may split work into multiple bounded leaf packets. Leaf agents
must not delegate further; they return one artifact, patch, test result, map,
review, proposed split, or blocker.

## Agent Selection Flow

```text
Parent Codex needs delegation
  -> choose parent-facing agent from .codex/agents/*.toml
    -> usually rustmine_compatibility_lead for large decomposition
      -> Lead creates context_capsule packets
        -> choose rustmine_nested_* agent for each leaf packet
          -> nested leaf reads capsule and named artifacts only
            -> nested leaf returns detailed result to Lead
              -> Lead returns compact status to Parent Codex
```

Do not use parent-facing agents as planner-to-leaf workers. They carry startup,
recovery, and user-facing context that nested leaves should not inherit.

## Role Boundary

| Layer | Owns | Must not own |
|---|---|---|
| Parent Codex | user intent, route, recovery state, final answer | treating helper output as verified proof |
| Planner / mapper | decomposition, lane choice, context capsules, compact integration of leaf results | implementation, durable memory, final user answer |
| Leaf worker | one bounded artifact, patch, test result, map, review, proposed split, or blocker | route reinterpretation, parent recovery state, nested delegation |

Use parent-facing agents when Parent Codex directly delegates a role. Use
`rustmine_nested_*` agents when a planner delegates leaf work. Nested agents are
capsule-driven and must not perform broad startup reads.

Use `rustmine_compatibility_lead` for large or ambiguous decomposition. Do not
use Lead as a compaction store, execution pool, or long-running supervisor.

## Context Loss Guard

The main weakness of nested delegation is context loss. Preserve context before
the tree expands.

| Risk | Required countermeasure |
|---|---|
| Planner forgets user intent | Parent prompt includes objective, stop boundary, and current `docs/next` state. |
| Leaf lacks necessary facts | Every leaf receives a complete context capsule and exact allowed reads/writes. |
| Parent receives too much detail | Planner compresses leaf results to status, changed artifacts, proof/check status, blocker, and next step. |
| Durable evidence stays in chat | Store durable facts in the owned artifact, `docs/next`, or the relevant `docs/analysis` shard. |
| Split changes the route | Parent updates `docs/next/README.md` before ending. |

## Return Boundary

```text
leaf -> planner / parent:
  detailed evidence is allowed

planner -> parent:
  compact status only
  include status, changed artifacts, proof/check status, blocker, and next step
  omit reasoning, raw logs, prompts, rejected branches, and full work-package maps

parent -> user:
  concise answer in user-facing language
```

If detailed helper context must survive compaction, store it in the naturally
owned artifact: oracle case, failure packet, traceability row, test output
reference, or the relevant `docs/analysis/` shard.

## Wait Boundary

The subagent surface is asynchronous. Do not supervise helpers with repeated
short waits.

```text
result blocks parent action
  -> one wait_agent call with timeout_ms=3600000

result does not block parent action
  -> continue non-overlapping parent work or return scoped status
```

Anti-pattern:

```text
spawn
  -> wait 300s
    -> timeout
      -> wait 300s
        -> timeout
          -> repeat
```
