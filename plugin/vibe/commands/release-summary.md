# vibe release-summary

## Command Contract Only

This file documents the intended workflow contract and does not claim executable Codex command binding.

## Purpose

Draft local release notes from git history and Vibe artifacts without publishing
to a remote service.

## Inputs

- `--from` commit, tag, or ref.
- `--to` commit, tag, or ref.
- Optional output path for the local release notes draft.
- Optional title or version label.

## Reads

- Local git history for the selected range.
- `.vibe/tasks/`, `.vibe/runs/`, and `.vibe/reviews/` when present.
- `DELIVERY.md` or prior release notes when useful context exists.

## Writes

- A local release notes draft in Markdown.
- Optional metadata linking commit ranges to completed Vibe tasks.

## Local Only

If `--from` is omitted, the future script may use a latest tag fallback. The
output is a local release notes draft only. Phase 20 requires no network-required GitHub publishing, API token, or remote release mutation.

## Expected Output

- Selected commit range, including `--from` and `--to` resolution.
- Summary grouped by user-visible changes, fixes, documentation, and migration
  notes.
- Review or verification caveats when Vibe artifacts show unresolved risk.
- Path to the local release notes draft.
