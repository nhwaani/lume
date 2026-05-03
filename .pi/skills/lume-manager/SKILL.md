---
description: Manages the SDLC of the Lume project, coordinating between specs, code, and the knowledge wiki.
---

# Lume Manager Skill

This skill manages the Software Development Life Cycle (SDLC) of the Lume project. It coordinates between the codebase, the technical specifications, and the knowledge wiki.

## Usage

### 1. Get Project Overview
When the user needs a status update or guidance on what to do next:
```
/skill:lume-manager help
```
The skill will:
- Read `lume/ROADMAP.md` to find the current milestone.
- Read `lume/VERSION` to identify the current release.
- Read the active spec in `lume/specs/active/` to summarize the current mission.
- Provide a "Next Step" recommendation.

### 2. Check Current Status
For a quick summary of progress:
```
/skill:lume-manager status
```
The skill will report the current phase, completion percentage of the current milestone, and the last entry in `CHANGELOG.md`.

### 3. Complete a Phase
When a feature is verified and the Definition of Done (DoD) is met:
```
/skill:lume-manager complete [phase-name]
```
This is a critical operation that synchronizes the entire ecosystem. The skill will:
1. **Archive Spec**: Move `lume/specs/active/[phase-name].md` to `lume/specs/completed/`.
2. **Update Roadmap**: Mark the task as completed (✅) in `lume/ROADMAP.md`.
3. **Update Changelog**: Append the completion of the phase to `lume/CHANGELOG.md`.
4. **Wiki Ingestion**: Invoke `/skill:wiki-assistant ingest` with a summary of the phase's technical outcome to ensure the knowledge wiki is updated.
5. **Version Bump**: (Optional) Prompt the user if a version bump is required in `lume/VERSION`.

## Integration with Wiki Assistant
The `lume-manager` is the "orchestrator," and the `wiki-assistant` is the "archivist." 

Whenever `lume-manager` completes a phase, it must generate a structured summary including:
- The original goal.
- The technical solution implemented.
- Any deviations from the original spec.
- Links to the resulting code modules.

This summary is then passed to `/skill:wiki-assistant ingest` to ensure the project's history is preserved in the Obsidian wiki.

## Safety & Constraints
- **Do not move specs** unless the user has confirmed that the DoD is met.
- **Preserve formatting** in `ROADMAP.md` and `CHANGELOG.md`.
- **Coordinate** with the `wiki-assistant` to ensure no data is lost during the transition from "Active Spec" to "Wiki Record."
