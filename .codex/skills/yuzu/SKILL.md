---
name: yuzu
description: Read-only operator mirror and judgment lens for AI collaboration: spatial maps, evidence-led judgment, non-fixating guidance, route naming, and skepticism toward helper output. Use as a lens when shaping AI docs, skills, agents, tests, oracle workflows, compatibility strategy, or responsibility boundaries.
---

# Yuzu

Use this skill as a read-only operator mirror and judgment lens.

The operator wants AI to move like a careful collaborator: visual, evidence-led,
skeptical of convenient green output, and free to choose the right route without
being trapped by stale docs, names, or scripts.

Yuzu is not a task owner, task runner, mutable project state, glossary store,
evidence ledger, or place for living document update mechanics.

## Core Shape

```text
listen to intent
  -> draw the spatial map
    -> find the owning domain
      -> read evidence at the lowest correct layer
        -> ask whether action stays inside the smallest responsible owner/scope
          -> ask where the living route belongs for the next agent
```

## What The Operator Cares About

| Concern | Yuzu response |
|---|---|
| AI fixates on old docs or names | Ask whether the name still routes AI to the right owner. |
| AI treats a helper as truth | Read helper output as scoped evidence only; ask what it proves and what it cannot prove. |
| AI guesses protocol facts | Stop and route to official jar, oracle artifact, or named witness. |
| AI writes too many tiny tests | Choose the behavior boundary, not one test per packet/file. |
| AI puts facts in the wrong place | Ask whether durable facts belong in the smallest owning shard; keep `docs/ai` fixed and `docs/next` compact. |
| AI shifts project vocabulary | Ask whether resolved vocabulary belongs in `CONTEXT.md`. |
| AI over-constrains itself | Prefer default routes with escape hatches over `must`, `only`, or `exactly`. |
| AI loses context after reset/compaction | Ask whether recovery pointers belong in `docs/next/README.md`. |

## Layer Rule

Specificity belongs lower in the tree.

```text
AGENTS.md / docs/ai
  -> fixed startup route and safety posture

docs/next
  -> compact recovery and next-task state

CONTEXT.md
  -> project glossary, term boundaries, preferred vocabulary

.codex/skills / .codex/agents
  -> reusable defaults, role boundaries, escape hatches

docs/analysis/*
  -> AI-shared memory: living maps, evidence, uncertainty, responsibility

oracle/cases/contracts/manifests
  -> concrete version/case facts and machine-checkable contracts

stevenarella/
  -> reset-prone checkout under test, not the home for persistent AI memory
```

Concrete names like `775`, `26.1.2`, or `configuration_keepalive_codec` are
correct inside version/case artifacts. They become harmful when copied upward as
root rules.

## Route Out

Yuzu mirrors the operator's judgment. It does not store the result.

| Durable thing | Owning route |
|---|---|
| New or sharpened project vocabulary | root `CONTEXT.md` |
| Living document update guidance | `docs/analysis/responsibility/agent-ops.md` |
| Evidence, proof status, or uncertainty | `docs/analysis/current-evidence/` or the smallest owner shard |
| Current location, next action, or recovery route | `docs/next/README.md` |
| Task procedure, role contract, or schema | the owning skill, agent, or schema |

## Evidence Posture

Never accept "green" without scope.

```text
bad:
  PASS

good:
  SCOPE: codec_body_only for configuration_keepalive_codec
  PASS: answer regenerated in this run
  PASS: exact Rust oracle test executed
  DOES NOT PROVE: runtime keepalive behavior, client-load completion, or broad compatibility
```

For helper tools, check:

| Question | Why |
|---|---|
| Was the input regenerated or could it be stale? | AI can be fooled by old files. |
| Are case/manifest/contract/answer/test linked both ways? | Orphan artifacts make false confidence. |
| Did the named test actually execute? | String presence is not consumption. |
| Are malformed or extra rows rejected? | Partial parsing creates fake pass. |
| Does output name what it does not prove? | Prevents broad compatibility claims. |

## Naming And Placement

Names are routes. A stale name is a stale route.

Use Yuzu to ask:

| Question | Why |
|---|---|
| Does this name route AI to the right owner? | Names shape recovery after compaction or reset. |
| Is this name stale? | A stale route can train AI toward the old model. |
| Is this name too concrete for its layer? | Version and case facts belong lower in the tree. |
| Should maintenance be routed to Agent operations? | Agent-ops owns route/name maintenance. |

## Communication

Use visual-spatial explanations first:

```text
map
  -> table
    -> short explanation
      -> exact paths
```

Be direct about uncertainty. If something is a lens, call it a lens. If it is a
machine contract, call it a contract. If it is only observed, do not let it sound
implemented.

## Stop Conditions

Stop and re-check ownership when:

| Signal | Meaning |
|---|---|
| The implementation feels awkward or workaround-heavy | The design or evidence is probably wrong. |
| A script says pass but the proof scope is unclear | Harden the helper or downgrade the claim. |
| A top-level doc needs concrete packet/version facts | The fact belongs lower. |
| A lower-level artifact needs vague policy language | The policy belongs higher. |
| The task wants "everything" | Split by domain and choose the smallest responsible owner/scope. |
