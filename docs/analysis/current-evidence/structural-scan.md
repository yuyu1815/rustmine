# Structural Scan

Purpose: describe the reset-time scan without confusing it with proof.

## Command

```text
bash oracle/scripts/scan_current_load_surfaces.sh
```

Run this after a checkout reset when a task needs fresh structural
observations. The scan reports whether candidate paths exist, including paths
under `stevenarella/`, but every reported phase remains `observed_only`.

## Interpretation

```text
scan output
  -> candidate owner paths
    -> observed_only
      -> choose evidence surface
        -> root-owned proof
```

The scan can help find code. It cannot prove implementation, compatibility, or
phase completion.

## SRP / KISS Readability Rule

For 26-only readability cleanup, use this map before moving Rust code:

```text
same feature / same reason to change
  -> keep together

different lifecycle step or feature family
  -> split at that boundary

one function or one packet only
  -> not enough reason by itself
```

Single responsibility means a cohesive reason to change. It does not mean
one function per file. KISS means avoiding module hops that make the 26 load
path harder to follow. Prefer feature-family files such as configuration,
play, movement, inventory, or entity when they make the route easier to scan.

## Reset Boundary

`stevenarella/` is a reset-prone checkout under test. Do not store persistent
AI docs, oracle answers, test manifests, or canonical project-level tests
there.
