# Analysis Route Map

Purpose: route analysis readers to the right shard. This file is an index only,
not a fact store.

## Spatial Map

```text
docs/analysis/
  -> client-load/          phase map and load proof requirements
  -> current-evidence/     current proof state and observation rules
  -> protocol/             versioned protocol traceability indexes
  -> responsibility/       ownership and reset-boundary decisions
  -> legacy pointers       old flat paths retained for older links only
```

## Index

| Need | Canonical location |
|---|---|
| Client-load phase map | [client-load/README.md](client-load/README.md) |
| Current evidence | [current-evidence/README.md](current-evidence/README.md) |
| Protocol analysis index | [protocol/README.md](protocol/README.md) |
| Protocol 775 traceability | [protocol/versions/775/traceability.md](protocol/versions/775/traceability.md) |
| Responsibility decisions | [responsibility/README.md](responsibility/README.md) |

## Index Rule

Do not add evidence rows, packet facts, or current proof status here. Put those
in the shard named by the index.
