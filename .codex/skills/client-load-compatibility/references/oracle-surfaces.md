# Oracle Surfaces For Client Load

Use one official root, but do not force one test shape across all load phases.

## Surface Selector

| Need | Evidence source | Test surface |
|---|---|---|
| Exact packet id/body/fields | Official jar `STREAM_CODEC` / `ProtocolInfo` | `oracle/rust-tests` golden/contract test |
| State transition | Official protocol source or initialized official harness | state contract or harness answer artifact |
| Registry/configuration storage | Official source plus initialized harness when stateful | registry hydration artifact + runtime contract |
| Chunk/light/world decode | Official server/client source, generated fixture, or reference witness after official proof | world hydration test/probe |
| Entity/player spawn state | Official packet/runtime source plus smoke/probe | entity/player runtime contract |
| Render readiness | Project runtime/render observation | screenshot, pixel, or milestone smoke |
| Control/interact readiness | Own-server behavior and probes | e2e/corridor smoke |

## Official Equivalence

```text
official root
  -> several answer/proof artifacts
    -> phase-specific tests
      -> one client-load phase entry
```

"Same as the jar" does not mean one test. It means every phase claim is tied to
the strongest official or observable evidence available for that phase.

## Routing

| Surface | Route |
|---|---|
| packet bytes or packet id | `.codex/skills/stevenarella-oracle-workbench/SKILL.md` |
| protocol state or registry protocol evidence | Oracle Workbench; create an explicit initialized-harness follow-up when stateful Minecraft behavior is required |
| runtime implementation after oracle failure | `.codex/skills/stevenarella-rust-worker/SKILL.md` |
| world/render/control readiness | project-level smoke/probe; do not invent official bytes |

If official jar evidence cannot produce the claim, record the gap in
`docs/analysis/client-load/README.md` and the relevant
`docs/analysis/client-load/phases/*.md` file instead of forcing a fake oracle.
