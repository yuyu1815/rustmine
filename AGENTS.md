# AGENTS.md

This file is both a guide and a hard harness for agents working in this repository. The human operator is a visual-spatial learner, so user-facing explanations and final reports should use compact maps, relationships, tables, or flow descriptions when that helps understanding. This file itself is written primarily for AI agents and should stay concise.

## Project Roles

Treat `/Users/yuyu/Documents/rustmine/azalea` as the Migration Target. Production implementation work should normally happen there.

Treat `/Users/yuyu/Documents/rustmine/stevenarella` as the UI Reference. Read it for screen order, wording, layout, user interaction flow, and error timing. Do not treat it as the implementation target unless the user explicitly asks.

Treat `/Users/yuyu/Documents/rustmine/_analysis` as Jar Analysis. It contains raw Minecraft jar material and is the primary evidence for tiny behavior tuning or behavior missing from both Azalea and Stevenarella. You may read, inspect, decompile, or reason from it, but you must not overwrite jar/json/analysis material or use `_analysis` as an implementation location.

Treat `/Users/yuyu/Documents/rustmine/_research` as Research Reference. It can help with protocol notes and external comparisons, but it is not the highest source of truth and should not be edited without explicit instruction.

Use `CONTEXT.md` as the glossary for this migration effort. When a project term is resolved, update `CONTEXT.md`; do not turn `AGENTS.md` into a glossary or research log.

## Source Of Truth

For UI flow, screens, visual behavior, wording, and operation feel, prefer Stevenarella. For client internals already implemented in Azalea, prefer Azalea. For tiny behavior tuning or behavior absent from both Azalea and Stevenarella, prefer Jar Analysis. Use `_research` only as supporting context.

If Jar Analysis is used as evidence for a change, the final report must name the jar, class, method when available, and the reason it was consulted.

## Never

Never treat `stevenarella/` as the implementation target. Never overwrite `_analysis/` jar or json material. Never promote `_research/` above Azalea, Stevenarella, or Jar Analysis as a source of truth. Never build a future-proof UI framework before a concrete UI Migration Slice works. Never mark a screen-file copy as a completed migration. Never mix UI widgets/layout with packet or client internals in the same responsibility boundary. Never mark work complete when the required verification was not run and no reason or substitute check is reported.

## Before Starting Work

Before implementation, produce a short work map. It must name exactly one target UI Migration Slice, list the Stevenarella screen or behavior being referenced, list the Azalea crate/plugin areas likely involved, say whether Jar Analysis is needed, identify the responsibility boundaries that will be touched, and list the subagent investigations and implementation drafts that will run before the main agent edits code.

Do not change multiple screens, multiple user flows, or multiple crates without this work map. If the work map grows broad, split the work before editing.

## Subagent Delegation

Implementation investigation and implementation drafting must be delegated to subagents before code is changed. The main agent is an orchestrator and reviewer, not the primary implementer. The main agent owns the work map, task splitting, integration of subagent findings, `AGENTS.md` and `CONTEXT.md` updates, final decisions, and final reporting. The main agent must not perform broad implementation research, cross-codebase comparison, Jar Analysis, or first-pass code creation by itself.

The only allowed main-agent investigation exceptions are tiny checks needed to maintain the thread, update documentation, confirm tool availability, or verify a specific subagent result. If the main agent uses an exception, it must state the exception in the work map or final report.

Before making implementation edits, the main agent must have either received the required subagent findings and patch proposal, or explicitly reported that subagent tools are unavailable. If subagent tools are unavailable, the main agent may continue only after saying which investigations and implementation draft would have been delegated and why local work is being used as a fallback.

For every implementation task, the main agent must ask: "What investigation or code drafting can be separated from my critical path?" If the answer is not "none", spawn at least one subagent before editing. Do not use "I can inspect it faster myself" or "I can implement it faster myself" as a reason to skip delegation.

Subagents are investigation and bounded implementation helpers, not release owners. A subagent may inspect files, run read-only analysis, run verification commands, and prepare a bounded patch when the main agent explicitly assigns a disjoint write scope. A subagent must not run `git add`, `git commit`, `git push`, create branches, open pull requests, or perform any final repository publication step. The main agent must review the subagent's findings or patch, decide what to integrate, and own all final reporting and git actions.

The main agent should not write new production code directly when subagent tools are available. The main agent may make only small integration edits after reviewing subagent output, such as resolving a narrow conflict, applying a reported patch, fixing formatting fallout, or updating documentation. If the main agent writes production code directly, the final report must label it as a harness exception and explain why no subagent could draft it.

When delegating implementation work, the main agent must state the subagent's write scope and must say that the subagent is not alone in the codebase, must not revert unrelated changes, and must not touch files outside its assigned scope. If the task is investigation-only, the prompt must say that no files should be edited. If the task is patch-producing, the prompt must say that the subagent should report changed paths and verification, but should not commit.

All subagent prompts must be written in English to reduce token cost and improve compression. Prompts must be concise, self-contained, and bounded. A good prompt states the exact question, relevant project context, paths to inspect, evidence required, and desired output shape. Do not send vague prompts such as "investigate this"; send a small answerable task.

Use a frontier model such as `gpt-5.5` for difficult investigations. A task is difficult when it crosses multiple crates or subsystems, compares Azalea and Stevenarella architecture, requires Jar Analysis, investigates Minecraft behavior missing from both codebases, touches protocol/auth/rendering/ECS/concurrency, affects responsibility boundaries, or could change the implementation plan if answered incorrectly.

Use a mini model for simple investigations. A task is simple when it can be answered with `rg` plus a few files, locates symbols or modules, summarizes one screen/plugin/crate, lists referenced screens, checks existing tests or dependencies, or confirms a small local impact that is unlikely to change the design.

When unsure, choose `gpt-5.5` for investigations that affect design or correctness, and choose a mini model for disposable fact-finding.

If a subagent remains running without returning useful output, interrupt it with a short English status-check prompt. The status check should ask for current task, blocker status, what it is waiting on, and next concrete step. Close completed agents after integrating their findings.

## UI Migration

Perform UI Migration by user operation flow, not by file. Good migration slices include flows such as startup to account selection to login, server list to connection to failure display, in-game chat input to send/display, and death to respawn. Do not copy `stevenarella/src/screen/*.rs` directly into Azalea or force Stevenarella's file structure onto the Migration Target.

Stevenarella screens are storyboards, not architecture. Read them for screen order, wording, input fields, buttons, lists, layout, operation feel, and error timing. Do not copy their rendering loop, OpenGL/glutin assumptions, global state structure, screen trait/stack structure, or UI implementations that mix rendering, async work, connection logic, and persistence.

The first UI implementation should live as a small module or plugin inside an existing Azalea crate, normally under `azalea/azalea-client/src/ui/` or `azalea/azalea-client/src/plugins/ui/`. Do not create a new `azalea-ui` crate before repeated UI Migration Slices prove that a separate crate is needed.

## Design Harness

Follow KISS. The first implementation should be the smallest vertical slice that makes the target user operation flow work. Prefer Azalea's existing crates, plugins, events, components, and state. Keep state transitions and conditions visible. Do not generalize until at least two real slices need the same abstraction. Do not add unused traits, generics, managers, or frameworks.

Follow the Single Responsibility Principle as a change-boundary rule, not as "one file equals one responsibility." A boundary should have one reason to change. Keep UI presentation, flow state, client action, persistence, and Minecraft behavior separate.

UI presentation may handle display, input, focus, button state, and local visual state. Flow state may decide which screen or state comes next, including waiting, success, failure, and retry states. Client action should hand work to Azalea events, plugins, or packet-facing APIs without knowing UI layout. Persistence should only save and load accounts, server lists, settings, or equivalent durable data. Minecraft behavior should prefer Azalea and use Jar Analysis only when Azalea and Stevenarella do not answer the behavior.

Before creating a new boundary, check whether the work belongs in an existing `azalea-client/src/plugins/*` area. A UI layer must not reimplement Azalea client internals. A client plugin must not know about UI widgets or screen layout. Flow state must not construct packet internals. A UI screen must not directly build protocol packets.

## Completion Criteria

A UI Migration Slice is complete only when the target user operation flow works from start to finish, the Stevenarella screens or behaviors referenced are reported, the Azalea crate/plugin areas changed are reported, and the touched responsibility boundaries can be explained. Completion also requires that UI, flow state, client action, persistence, and Minecraft behavior are not collapsed into one mixed module, and that no future-only UI framework or abstraction was added.

## Verification Harness

For Rust code changes, run `cargo fmt` and `cargo check -p <changed-crate>` when practical. For Azalea plugin or client behavior changes, run relevant tests; if no relevant tests exist, run `cargo check` and report a manual verification note. For UI behavior changes, report the referenced Stevenarella screen/behavior and the user operation flow that was exercised. If Jar Analysis was used, report the jar, class, method when available, and reason.

Run full `cargo test` only when the impact is broad enough to justify it. Prefer focused tests for narrow changes. If a required verification cannot be run, report the reason and the substitute check.

## Final Report

Final reports should be short but structured. Include what changed, the target flow map, source references, responsibility boundaries touched, verification performed, and remaining risks. If Jar Analysis was used, include the jar/class/method/reason. If verification was skipped or failed, say so directly.

## Maintaining This File

Keep `AGENTS.md` short and forceful. It should contain settled behavior rules, hard constraints, start-work checks, verification rules, and final report rules. Do not use it for implementation notes, TODOs, long background explanation, undecided ideas, or temporary investigation logs.

Put resolved domain language in `CONTEXT.md`. Create an ADR only for decisions that are hard to reverse, surprising without context, and the result of a real trade-off.
