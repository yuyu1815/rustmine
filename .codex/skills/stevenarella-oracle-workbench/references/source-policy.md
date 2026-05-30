# Source Policy

## Priority

```text
official client.jar / server.jar function
  -> decompiled source path for explanation
  -> reference repositories as witnesses
  -> Stevenarella implementation
```

Official jars win over reference repositories. Stevenarella never proves the
expected answer.

## Client And Server Jar Roles

| Flow | Primary official question |
|---|---|
| Clientbound packet | What does the official server write, and what does the official client read/react to? |
| Serverbound packet | What does the official client write, and what does the official server accept/react to? |
| Shared codec | Do both sides expose the same `STREAM_CODEC` field order and body bytes? |
| State table | Which `ProtocolInfo` table assigns the packet id in that protocol state and flow? |

## Reference Repositories

These are known witnesses, not a closed or complete witness set. Add a named
reference only when the work package needs it, and keep official jars as the
source of expected answers.

| Repo | Role |
|---|---|
| MCProtocolLib | Java witness for packet/data modeling |
| Azalea | Rust witness for protocol/client domain shape |
| minecraft-data | Versioned schema/data witness |
| node-minecraft-protocol | Data-driven state/serializer witness |

Reference disagreement is a finding, not permission to guess.

Local witness checkouts live under
`_research/protocol/versions/<protocol>/witnesses/<repo>`. Treat those paths as
cache locations for reference repositories only, not official answer sources.

## Expected Values

Allowed:

```text
official function output
generated oracle answer artifact
decompiled source path explaining the function
```

Forbidden:

```text
AI memory
wiki-only summaries
current Stevenarella output
hand-written packet bytes
```
