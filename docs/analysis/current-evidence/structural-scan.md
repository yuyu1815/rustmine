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

## Reset Boundary

`stevenarella/` is a reset-prone checkout under test. Do not store persistent
AI docs, oracle answers, test manifests, or canonical project-level tests
there.
