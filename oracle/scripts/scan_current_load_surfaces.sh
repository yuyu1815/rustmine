#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

node - "$ROOT" <<'NODE'
const fs = require("fs");
const path = require("path");

const root = process.argv[2];

// Snapshot of paths observed in this working tree. This is not a reusable
// phase topology, readiness model, or version taxonomy.
const phases = [
  {
    phase: "local_boot_resources",
    paths: [
      "stevenarella/src/resources.rs",
      "stevenarella/src/resources/tests.rs",
      "stevenarella/src/main.rs"
    ]
  },
  {
    phase: "network_login_configuration",
    paths: [
      "stevenarella/protocol/src/protocol/versions/v26_1_2.rs",
      "stevenarella/src/server/connect.rs",
      "stevenarella/src/server/configuration.rs",
      "oracle/rust-tests/tests/oracle_contracts.rs"
    ]
  },
  {
    phase: "registry_hydration",
    paths: [
      "stevenarella/protocol/src/protocol/versions/v26_1_2_play_registry_fixture.rs",
      "stevenarella/src/server/configuration.rs",
      "stevenarella/src/server/runtime_state.rs"
    ]
  },
  {
    phase: "play_entry",
    paths: [
      "stevenarella/src/server/session.rs",
      "stevenarella/src/server/play_dispatch.rs",
      "stevenarella/src/server/play_spawn.rs"
    ]
  },
  {
    phase: "world_hydration",
    paths: [
      "stevenarella/src/server/level_chunk.rs",
      "stevenarella/src/server/light_update.rs",
      "stevenarella/src/server/block_update.rs",
      "stevenarella/src/world/mod.rs"
    ]
  },
  {
    phase: "entity_player_hydration",
    paths: [
      "stevenarella/src/server/local_player_lifecycle.rs",
      "stevenarella/src/server/remote_player.rs",
      "stevenarella/src/server/remote_entity_movement.rs",
      "stevenarella/src/server/player_position.rs"
    ]
  },
  {
    phase: "render_ready",
    paths: [
      "stevenarella/src/server/render_lifecycle.rs",
      "stevenarella/src/server/sun.rs",
      "stevenarella/src/server/target.rs"
    ]
  },
  {
    phase: "control_interact_ready",
    paths: [
      "stevenarella/src/server/player_movement.rs",
      "stevenarella/src/server/interact.rs",
      "stevenarella/src/server/inventory.rs",
      "stevenarella/src/server/combat.rs"
    ]
  }
];

const result = {
  generated_by: "oracle/scripts/scan_current_load_surfaces.sh",
  root,
  observation_scope: "observed_path_snapshot",
  warning: "Path existence is observation only. It is not compatibility proof, reusable load topology, or a phase-completion signal.",
  phases: phases.map((phase) => ({
    phase: phase.phase,
    observation_status: "observed_only",
    paths: phase.paths.map((relativePath) => ({
      path: relativePath,
      path_role: "current_tree_snapshot_path",
      exists: fs.existsSync(path.join(root, relativePath))
    }))
  }))
};

console.log(JSON.stringify(result, null, 2));
NODE
