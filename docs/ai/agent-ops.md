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
      -> rustmine_nested_* leaf workers for bounded artifacts, patches, docs rewrites, maps, reviews, or blockers
        -> Parent Codex verifies, compresses, and updates recovery state
```

For small tasks, skip the planner and leaf workers. Parent-only work is the
default only when the task already fits one owner, one evidence surface, and one
verification loop.

## Delegation Gate

Before spawning a subagent, the parent must be able to fill this capsule:

```text
context_capsule:
  schema_version:
  batch:
    id:
    leaf_index:
    leaf_count:
  agent_role:
  objective:
  allowed_reads:
    - path:
      kind:
      scope:
      reason:
  allowed_writes:
    - path:
      kind:
      scope:
      reason:
  required_evidence:
  exit_condition:
  stop_boundary:
  fragile_preconditions:
  write_policy:
    mode:
    post_run_diff_check:
    diff_baseline:
      before_command:
      after_command:
      delta_rule:
  rust_fix_task_path:
    path:
    kind:
    scope:
    reason:
  return_contract:
    must_report:
    must_not_claim:
```

Validate planner-to-leaf capsules against
`.codex/skills/stevenarella-oracle-workbench/schemas/context-capsule.schema.json`
before spawning. Use:

```text
python3 .codex/skills/stevenarella-oracle-workbench/scripts/validate_context_capsule.py CAPSULE_JSON [RUST_FIX_TASK_JSON]
```

`RUST_FIX_TASK_JSON` is required when `agent_role` is
`rustmine_nested_rust_implementer`; the validator rejects Rust task
`allowed_write_scope` entries outside the capsule `allowed_writes`.

`allowed_reads`, `allowed_writes`, and required evidence are a machine
contract, not prompt decoration. Path entries are normalized project-relative
paths only: no absolute paths, `.` entries, `..` segments, broad owner roots, or
glob patterns. Write entries must be exact files, not directories.

For workspace-write capsules, the diff baseline must use:

```text
before = git status --porcelain=v1 --untracked-files=all -- .
after  = git status --porcelain=v1 --untracked-files=all -- .
check  = after_minus_before_status_paths_must_be_subset_of_allowed_writes
```

Use status paths rather than `git diff --name-only` so new untracked artifacts
and staged files are included in the boundary check.

For direct parent-to-worker delegation, use a `worker_capsule` instead of
re-sending broad startup docs or old chat history:

```text
worker_capsule:
  schema_version: worker-capsule/v1
  worker_role:
  objective:
  startup_context:
    current_location:
    known_facts:
    do_not_read_by_default:
  allowed_reads:
  allowed_writes:
  required_evidence:
  required_checks:
  exit_condition:
  stop_boundary:
  write_policy:
    mode:
    post_run_diff_check:
    diff_baseline:
  rust_fix_task_path:
  return_contract:
    must_report:
    must_not_claim:
```

Validate it before spawning:

```text
python3 .codex/skills/stevenarella-oracle-workbench/scripts/validate_worker_capsule.py WORKER_CAPSULE_JSON [RUST_FIX_TASK_JSON]
```

When a worker receives a validated `worker_capsule`, the capsule is the startup
context. The worker should not read `AGENTS.md`, `docs/ai/`, `docs/next/`, broad
`docs/analysis/`, full skill files, jars, or decompiled sources unless those
paths are explicitly listed in `allowed_reads` or named by the capsule role.

Delegate when the work is genuinely large, parallel, ambiguous, or needs
separate evidence surfaces. Do not spawn a subagent for a file read, a simple
route lookup, or durable memory storage.

## Cost And Model Gradient

Use stronger models at the top of the tree for product direction,
responsibility boundaries, decomposition, and ambiguous compatibility judgment.
Push execution into the smallest safe lane for the work; safety and evidence
quality override cost.

| Layer | Model posture | Best use |
|---|---|---|
| Parent Codex | strongest judgment when needed | user intent, final route, recovery, final answer |
| Planner / mapper | strong but short-lived | split the task, choose lanes, create context capsules |
| Nested leaf worker | smallest safe model for the evidence risk | execute one artifact, patch, test, map, review, or blocker from a capsule |

The planner should reduce supervision cost by keeping execution bounded. It
must not become a general execution pool, and high-risk oracle/review leaves may
still need a high-capacity model.

## Spawn Budget

Use one bounded batch at a time.

```text
Parent Codex
  -> at most one Lead/planner for the current decision
    -> at most two nested leaf workers in one batch
      -> Parent Codex stops, checks changed files, and chooses the next action
```

`max_depth = 2` prevents deeper chains. `max_threads = 3` prevents the previous
wide fan-out pattern from becoming normal supervision work. Treat `max_threads`
as the total non-parent agent budget, so one Lead plus two leaves is the maximum
single batch. A Lead must not start a second leaf batch without a fresh parent
decision.

## Ephemeral Agent Lifecycle

Planner, implementation, oracle, mapping, docs, and review agents are
single-use workers. Do not reuse an existing planner, worker, or review session
for a later task, later batch, or follow-up review, even when the role name and
target area are the same.

```text
spawn fresh agent with capsule
  -> wait once for result
    -> validate result / changed paths
      -> delete or discard that agent session and its cache
        -> next batch must spawn fresh agents from fresh capsules
```

Any helper cache is treated as contaminated after the result is consumed. The
next task must be reconstructed from durable project files, the new
`context_capsule` or `worker_capsule`, and the current parent prompt; it must
not inherit the previous helper's chat memory, scratch state, or cached
interpretation. This applies equally to implementation workers and review
workers.

If the platform exposes an explicit delete/close/clear operation for a spawned
agent, the caller must run it after accepting or blocking the result. If no
explicit deletion API is available, the caller must drop the session id, mark
the helper as expired, and never address it again.

## Middle Reasoning Budget

The planner is the middle layer. It may think enough to split the task, but it
must not turn its reasoning into parent context.

| Middle output | Rule |
|---|---|
| Reasoning traces | Do not return to Parent Codex. |
| Leaf prompts or raw leaf logs | Do not return to Parent Codex. |
| Rejected branches | Omit unless they are the blocker. |
| Work-package map | Keep internal unless the parent asked only for a plan. |
| Parent result | Return status, changed artifacts, reported checks, blocker, and next step only. |

## Granularity Gate

Nested work is useful only at reviewable boundaries.

| Granularity | Decision |
|---|---|
| Too large | Split before execution; the result would mix multiple owners, phases, or proof surfaces. |
| Good | One worker can return one artifact, patch, test result, map, review, or blocker that the parent can verify. |
| Too small | Keep in parent or merge into a neighboring packet; the result would be only a file read, rename, or isolated fact. |

If a leaf discovers that its packet is still too large, it should stop with a
proposed split and blocker. It should not create another planner chain.

## Docs Rewrite Leaf

Use `rustmine_nested_docs_rewriter` for documentation-update churn when the
planner or parent already knows the wording or exact rewrite intent.

```text
Parent / Lead decides content
  -> context_capsule carries target files and supplied wording
    -> rustmine_nested_docs_rewriter edits only allowed_writes
      -> Parent / Lead checks only write mistakes and scope drift
```

This leaf is a typist/editor lane, not a route or evidence lane. The capsule
must include the final text, replacement table rows, or exact rewrite intent.
After it writes, review only transcription mistakes, broken path/link
formatting, duplicate or missing rows, Markdown shape, and whether changed
paths are inside `allowed_writes`. Do not use the post-write check to
re-litigate evidence, ownership, protocol meaning, or the product route.

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
      -> Lead creates validated context_capsule packets
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
| Parent receives too much detail | Planner compresses leaf results to status, changed artifacts, reported checks, blocker, and next step. |
| Write-capable leaf drifts | Parent records `before = git status --porcelain=v1 --untracked-files=all -- .`, runs the leaf, records `after` with the same command, and confirms new or changed status paths in `after - before` are inside the capsule `allowed_writes`. |
| Rust task scope splits from capsule scope | For Rust leaves, validate the capsule together with the rust-fix task and require `allowed_write_scope` to be a subset of capsule `allowed_writes`. |
| Leaf omits required return fields | Lead/Parent wraps leaf output in `leaf-result/v1` and runs `python3 .codex/skills/stevenarella-oracle-workbench/scripts/validate_leaf_result.py CAPSULE_JSON LEAF_RESULT_JSON` before accepting the result. |
| Helper cache affects later work | Every planner, implementation worker, docs leaf, mapper, oracle worker, and review worker is single-use; delete/clear or discard the agent session after the result is consumed. |
| Durable evidence stays in chat | Store durable facts in the owned artifact, `docs/next`, or the relevant `docs/analysis` shard. |
| Split changes the route | Parent updates `docs/next/README.md` before ending. |

## Direct Write Gate

If Parent Codex directly delegates to a parent-facing workspace-write agent
instead of using a planner-to-leaf capsule, prefer a validated `worker_capsule`.
The parent still owns the same write boundary:

```text
Parent direct write delegation
  -> name allowed_write_scope / allowed_writes before spawn
  -> record before status with untracked files
  -> run one bounded parent-facing agent
  -> record after status with untracked files
  -> accept only after new or changed status paths are inside the named scope
  -> delete or discard that agent session and cache
```

This applies to direct `rustmine_oracle_cartographer` and
`rustmine_rust_implementer` use. `worker_capsule` is the direct-worker contract;
`context_capsule` remains the planner-to-leaf contract. The status-baseline and
allowed-write-scope check are not optional for direct write-capable agents.

## Return Boundary

```text
leaf -> planner / parent:
  detailed evidence is allowed

planner -> parent:
  compact status only
  include status, changed artifacts, reported checks, blocker, and next step
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
