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
- Read `ROADMAP.md` to find the current milestone.
- Read `VERSION` to identify the current release.
- Read the active spec in `specs/active/` (if present) to summarize the current mission.
- Provide a "Next Step" recommendation.

### 2. Check Current Status
For a quick summary of progress:
```
/skill:lume-manager status
```
The skill will report the current phase, completion percentage of the current milestone, and the last entry in `CHANGELOG.md`.

### 3. Complete a Phase
When a feature is implemented and the Definition of Done (DoD) is strictly met:
```
/skill:lume-manager complete [phase-name]
```
**Prerequisites for completion (hard gates):**
1. `/skill:lume-tester` returns **PASS**.
2. `/skill:lume-reviewer` returns **LGTM**.

If either gate fails, **refuse** the operation and report which gate blocked it. Do not start mutating files.

On PASS + LGTM, perform in order. The spec directories `specs/active/` and `specs/completed/` are created if they don't exist:
1. **Archive Spec**: Move `specs/active/[phase-name].md` → `specs/completed/`.
2. **Update Roadmap**: Mark the task ✅ in `ROADMAP.md`.
3. **Update Changelog**: Append the phase completion to `CHANGELOG.md`.
4. **Wiki Ingestion** *(pending — `wiki-assistant` skill not yet defined; emit the structured summary to stdout for now)*.
5. **Version Bump**: Patch bump on phase completion within a milestone; minor on milestone completion. Confirm with the user before writing `VERSION`.

If any step above fails mid-sequence, **stop and report the partial state**. Do not auto-rollback — let the user decide.

## Phase Completion Summary
On every phase completion, generate a structured summary with:
- Original goal.
- Technical solution implemented.
- Deviations from the spec.
- Links to the resulting code modules.

This summary is the artifact `wiki-assistant` will ingest once that skill exists. Until then, print it to stdout so the user can paste it into the wiki by hand.

## Safety & Constraints
- **Do not move specs** unless both gates returned PASS / LGTM.
- **Preserve formatting** in `ROADMAP.md` and `CHANGELOG.md` — only flip checkboxes and append entries; never reflow surrounding text.
- **No git commits** — leave commits to the user.
- **Idempotent reads, non-idempotent writes**: `help` and `status` are safe to repeat; `complete` mutates files and should be run once per phase.
