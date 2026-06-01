# Rustmine

This context describes the local migration effort that combines Azalea's Minecraft client internals with Stevenarella's existing graphical user interface.

## Language

**Migration Target**:
The codebase that receives new behavior in this repository. For this project, the Migration Target is `azalea/`.
_Avoid_: destination, new side, main side

**UI Reference**:
The codebase used as the source of existing graphical interface behavior and visual workflows. For this project, the UI Reference is `stevenarella/`.
_Avoid_: old side, source, sample

**Research Reference**:
Material used to confirm protocol behavior or external implementation details without becoming the primary edit target.
_Avoid_: implementation target, working tree

**Jar Analysis**:
The raw Minecraft jar material under `_analysis/`, used as the primary evidence when tuning behavior or investigating behavior missing from both Azalea and Stevenarella.
_Avoid_: protocol reference, generated docs, implementation target

**UI Migration**:
The act of recreating Stevenarella's user-facing interface behavior on top of Azalea's more accurate client implementation.
_Avoid_: rewrite, copy, port everything

**UI Migration Slice**:
A single user operation flow migrated end-to-end into the Migration Target. A slice is complete when the user can perform the flow from start to finish.
_Avoid_: screen file, page, partial copy

**Responsibility Boundary**:
A project boundary that has exactly one reason to change, such as presentation, flow state, client action, persistence, or Minecraft behavior.
_Avoid_: one file, helper bucket, manager

**UI Home**:
The initial location for migrated UI code inside the Migration Target. UI Home starts as a small module or plugin inside an existing Azalea crate, not as a new UI crate.
_Avoid_: future UI framework, premature crate

## Example Dialogue

Developer: Should I change Stevenarella's login screen directly?

Domain Expert: No. Stevenarella is the UI Reference. Recreate the relevant login flow in the Migration Target.

Developer: Can I inspect research files while doing that?

Domain Expert: Yes. Use research files as a Research Reference when protocol behavior is unclear, but keep the implementation centered on Azalea.

Developer: What if Azalea and Stevenarella both lack the exact behavior?

Domain Expert: Use Jar Analysis as the primary evidence, then implement the result in the Migration Target.

Developer: Should I copy a Stevenarella screen file into Azalea?

Domain Expert: No. Treat the screen as a storyboard and migrate one UI Migration Slice across clear Responsibility Boundaries.

Developer: Should I create `azalea-ui` before the first screen works?

Domain Expert: No. Put the first slice in the UI Home inside an existing Azalea crate, then extract a crate only when repeated responsibilities make that necessary.
