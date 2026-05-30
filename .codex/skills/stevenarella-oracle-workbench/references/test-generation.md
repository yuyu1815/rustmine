# Jar-Backed Oracle Test Generation

Purpose: turn official Minecraft jar behavior into Stevenarella tests without
guessing, hand-written expected bytes, or repeated reinterpretation.

## Ownership

```text
official jar function
  -> oracle case
  -> oracle contract
  -> generated official answer
  -> oracle test manifest
  -> Rust oracle contract test
  -> traceability row
```

This workflow belongs to the Oracle Workbench. It may write `oracle/**`,
project-level oracle tests, and root traceability docs. It must not edit
Stevenarella Rust implementation behavior.

## Generation Levels

| Level | Name | What is automated | Stop boundary |
|---|---|---|---|
| 0 | Hand-selected oracle case | A named case calls a named official function and writes an answer artifact | Do not claim broad packet coverage |
| 1 | Case-driven factory | All `oracle/cases/<protocol_version>/*.json` are validated, dispatched, answered, and tested | Do not discover new packet meaning |
| 2 | Manifest-driven Rust tests | `oracle/test-manifests/<protocol_version>/*.json` drive generated or table-driven Rust oracle tests | Do not put expected bytes in Rust |
| 3 | Discovery proposals | Official packet tables/decompiled sources produce candidate case proposals | Proposals are not facts until an official function generates an answer |
| 4 | Initialized behavior harness | Fabric Loader JUnit/GameTest proves registry/gameplay/runtime reactions | Do not fake initialized Minecraft state |

This reference describes the generation shape. Mutable bootstrap status and
next targets belong in the relevant analysis shard. Protocol 775 is the current
populated example, not a fixed workflow limit.

## Client And Server Jar Selection

| Question | Primary jar/function | Secondary proof |
|---|---|---|
| Serverbound packet body | Official client writes it | Official server accepts or reacts to it when testing runtime reaction |
| Clientbound packet body | Official server writes it | Official client reads or reacts to it when testing runtime reaction |
| Shared codec | The side exposing the `STREAM_CODEC` for the packet class | The opposite side only when it has the same class/function |
| Packet id | `ProtocolInfo` table for exact state and flow | Reference repos only as witnesses |
| State transition | Official login/configuration/play protocol code | Runtime harness if initialized state is required |

Never choose jar side by convenience. Record the jar role in the oracle answer
and the test manifest.

## Factory Contract

A factory run must execute this shape:

```text
for each oracle/cases/<protocol_version>/*.json
  -> perform runner structural validation for required case fields
  -> validate reciprocal case/contract/manifest references
  -> find extractor by oracle_kind + question.packet_type + official_sources
  -> remove or mark the target answer path
  -> call official jar function, not Stevenarella
  -> write oracle/answers/<protocol_version>/<case>.answer.jsonl
  -> perform bespoke generated-answer JSONL validation
  -> reject missing, empty, malformed, duplicate, or mismatched answer rows
  -> write/update oracle/test-manifests/<protocol_version>/<case>.test-manifest.json
  -> run the manifest-declared Rust oracle test by exact name
  -> fail if that exact Rust test is filtered out or not executed
  -> for shared Rust test names, require test code or explicit metadata to verify the intended manifest/case
  -> update docs/analysis/protocol/versions/<protocol_version>/traceability.md
  -> update docs/analysis/protocol/versions/<protocol_version>/cases/<case>.md
```

If no extractor exists, stop with a missing-extractor result. Do not hand-write
the answer, do not create a Rust expected value, and do not let the Rust worker
read jars to fill the gap.

Use the current runner for all existing cases:

```text
bash oracle/scripts/run_jar_backed_oracle_tests.sh
```

Runner output is scoped evidence. A line such as `PASS: answer regenerated` or
`PASS: exact Rust oracle test executed` proves only the named artifact
relationship or exact Rust test execution for the named case and manifest. It
does not prove broader packet coverage, client-load readiness, initialized
runtime behavior, inventory, combat, or a future protocol version. The current
helper performs structural and cross-reference checks; do not describe it as
full JSON Schema validation unless that is actually implemented.

## Rust Test Generation Contract

Generated or table-driven Rust tests must read only:

```text
oracle/answers/<protocol_version>/*.answer.jsonl
oracle/test-manifests/<protocol_version>/*.test-manifest.json
```

They must not read:

```text
client.jar
server.jar
decompiled sources
reference repositories
```

The preferred Rust shape is one public integration test surface per contract
family, not one mirrored test file per packet:

```text
oracle/rust-tests/tests/oracle_contracts.rs
  -> reset-proof project-level codec and packet-id contracts

oracle/rust-tests/tests/oracle_runtime_contracts.rs
  -> future reset-proof multi-module runtime reaction contracts
```

New cases should move toward typed Rust readers for answer/manifest rows. Avoid
copying expected packet ids or bytes into test source.

## Traceability Requirement

Every generated test must have one traceability row:

```text
case id
  -> official source function
  -> answer artifact
  -> Rust test target
  -> internal packet/runtime owner
  -> corridor milestone or active target
  -> current proof state
```

The traceability row is the anti-reinterpretation artifact. If the row cannot be
filled, the case is not ready for Rust implementation work.
The manifest, answer, Rust test target, and traceability row must be reciprocal:
adding one without the others is a failing helper state, not a partial pass.

## Failure Routing

| Failure | Owner |
|---|---|
| Missing extractor | Oracle Workbench |
| Official call requires initialized Minecraft state | Explicit initialized-harness follow-up through responsibility owner or lead |
| Answer structural or JSONL validation invalid | Oracle Workbench |
| Rust test cannot read manifest/answer | Oracle Workbench |
| Rust output differs from official answer | Rust Worker through `rust-fix-task` |
| Contract scope is too small or too broad | Direction/lead review |

## Done For One Case

```text
case json exists
contract json exists
official answer jsonl regenerated from jar
test manifest exists
Manifest-declared exact Rust oracle test executes and reads answer/manifest
traceability row names the official and internal owners
case note in docs/analysis/protocol/versions/<protocol_version>/cases/ names the stop boundary
verification command is recorded in the relevant domain shard
```
