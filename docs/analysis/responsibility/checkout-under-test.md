# Checkout Under Test

Purpose: keep the reset-prone `stevenarella/` checkout from becoming the home
for persistent docs or oracle evidence.

## Boundary

| Field | Value |
|---|---|
| Checkout | `stevenarella/` |
| Status | reset-prone code under test |
| May contain | bounded implementation changes and local unit tests when a Rust task allows them |
| Must not contain | persistent AI docs, canonical oracle answers, test manifests, or reset-proof project-level tests |
| Evidence source | root-owned oracle artifacts, smoke/probe outputs, and domain-shard docs |

## Rust Worker Rule

A Rust implementation worker may read named oracle artifacts and failure
packets, but must not read jars, decompiled sources, reference repositories, or
edit expected answers.
