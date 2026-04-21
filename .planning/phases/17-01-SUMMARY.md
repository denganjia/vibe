# Plan 17-01 Summary: TTY Encoder & Throttling

## Achievements
- Implemented `TTYEncoder` in `crates/vibe-core/src/adapter/encoder.rs` to provide throttled text injection (64-byte chunks with 5ms delay).
- Refactored `TerminalAdapter` trait to include `inject_raw` and a default `inject_text` implementation using the encoder.
- Updated `WezTermAdapter` to use `--no-paste` for literal text injection, bypassing Bracketed Paste Mode issues.
- Updated `TmuxAdapter` to use `-l` (literal) for raw injection and ensured `\r` is used for triggering submission in `send_keys`.
- Fixed a regression in `vibe-core` unit tests related to the `ProjectConfig` refactoring.

## Verification
- Unit test for `TTYEncoder` passed successfully.
- Codebase compiles and core traits are aligned.
